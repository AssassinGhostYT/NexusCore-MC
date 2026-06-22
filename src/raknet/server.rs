use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

use crate::raknet::protocol::{
    MAGIC, ID_UNCONNECTED_PING, ID_UNCONNECTED_PING_OPEN_CONNECTIONS,
    ID_OPEN_CONNECTION_REQUEST_1, ID_OPEN_CONNECTION_REPLY_1,
    ID_OPEN_CONNECTION_REQUEST_2, ID_OPEN_CONNECTION_REPLY_2,
    ID_UNCONNECTED_PONG, ID_CONNECTED_PING, ID_CONNECTED_PONG,
    ID_CONNECTION_REQUEST, ID_CONNECTION_REQUEST_ACCEPTED,
    ID_NEW_INCOMING_CONNECTION, ID_DISCONNECT, MIN_FRAME_SET, MAX_FRAME_SET,
    ID_ACK, ID_NACK, Reliability, Frame, FrameSet, AckNack, write_address, read_address
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Connecting,
    Connected,
    Disconnected,
}

pub struct SplitReassembler {
    pub count: u32,
    pub fragments: HashMap<u32, Vec<u8>>,
}

pub struct Session {
    pub address: SocketAddr,
    pub guid: u64,
    #[allow(dead_code)]
    pub mtu: u16,
    pub state: SessionState,
    
    // Sequence numbers
    pub send_seq_num: u32,
    pub message_index: u32,
    pub order_index: u32,
    
    // Ack / Nack tracking
    pub received_seq_nums: HashSet<u32>,
    pub pending_acks: Vec<u32>,
    pub sent_packets: HashMap<u32, Vec<u8>>, // seq_num -> raw serialized FrameSet
    
    // Fragmentation reassembly
    pub split_reassembler: HashMap<u16, SplitReassembler>,
    pub next_split_id: u16,
    
    pub last_activity: Instant,
}

impl Session {
    pub fn new(address: SocketAddr, guid: u64, mtu: u16) -> Self {
        log::info!("Creating new RakNet session for {} (GUID: {}, MTU: {})", address, guid, mtu);
        Session {
            address,
            guid,
            mtu,
            state: SessionState::Connecting,
            send_seq_num: 0,
            message_index: 0,
            order_index: 0,
            received_seq_nums: HashSet::new(),
            pending_acks: Vec::new(),
            sent_packets: HashMap::new(),
            split_reassembler: HashMap::new(),
            next_split_id: 0,
            last_activity: Instant::now(),
        }
    }

    pub fn handle_split_frame(&mut self, frame: Frame) -> Option<Vec<u8>> {
        let split_id = frame.split_id.unwrap();
        let split_count = frame.split_count.unwrap();
        let split_index = frame.split_index.unwrap();
        
        let reassembler = self.split_reassembler.entry(split_id).or_insert_with(|| SplitReassembler {
            count: split_count,
            fragments: HashMap::new(),
        });
        
        reassembler.fragments.insert(split_index, frame.payload);
        
        if reassembler.fragments.len() == reassembler.count as usize {
            let mut full_payload = Vec::new();
            for i in 0..reassembler.count {
                if let Some(frag) = reassembler.fragments.get(&i) {
                    full_payload.extend_from_slice(frag);
                } else {
                    return None;
                }
            }
            self.split_reassembler.remove(&split_id);
            Some(full_payload)
        } else {
            None
        }
    }

    pub fn create_frame_set(&mut self, payload: Vec<u8>, reliability: Reliability) -> FrameSet {
        let mut frame = Frame {
            reliability,
            split: false,
            reliability_index: None,
            sequence_index: None,
            order_index: None,
            order_channel: None,
            split_count: None,
            split_id: None,
            split_index: None,
            payload,
        };
        
        if reliability.is_reliable() {
            frame.reliability_index = Some(self.message_index);
            self.message_index += 1;
        }
        if reliability.is_ordered() {
            frame.order_index = Some(self.order_index);
            frame.order_channel = Some(0);
            self.order_index += 1;
        }
        
        let seq = self.send_seq_num;
        self.send_seq_num += 1;
        
        FrameSet {
            sequence_number: seq,
            frames: vec![frame],
        }
    }
}

pub enum RakNetEvent {
    Connected(SocketAddr, u64),
    Disconnected(SocketAddr),
    Packet(SocketAddr, Vec<u8>),
}

pub enum RakNetCommand {
    Send(SocketAddr, Vec<u8>, Reliability),
}

pub struct RakNetServer {
    socket: Arc<UdpSocket>,
    guid: u64,
    sessions: HashMap<SocketAddr, Session>,
    event_tx: mpsc::Sender<RakNetEvent>,
    cmd_rx: mpsc::Receiver<RakNetCommand>,
    port: u16,
}

impl RakNetServer {
    pub async fn new(addr: &str, event_tx: mpsc::Sender<RakNetEvent>) -> std::io::Result<(Self, mpsc::Sender<RakNetCommand>)> {
        let socket = UdpSocket::bind(addr).await?;
        let local_addr = socket.local_addr()?;
        let port = local_addr.port();
        let guid = rand::random::<u64>();
        log::info!("RakNet server listening on {} with Server GUID {}", local_addr, guid);
        
        let (cmd_tx, cmd_rx) = mpsc::channel(1024);
        
        let server = RakNetServer {
            socket: Arc::new(socket),
            guid,
            sessions: HashMap::new(),
            event_tx,
            cmd_rx,
            port,
        };
        Ok((server, cmd_tx))
    }

    pub async fn run(mut self) {
        let mut buf = vec![0u8; 65535];
        let mut interval = tokio::time::interval(Duration::from_millis(50)); // 20 ticks per second

        loop {
            tokio::select! {
                // Handle socket read
                recv_res = self.socket.recv_from(&mut buf) => {
                    match recv_res {
                        Ok((len, src)) => {
                            if let Err(e) = self.handle_packet(src, &buf[..len]).await {
                                log::error!("Error handling packet from {}: {:?}", src, e);
                            }
                        }
                        Err(e) => {
                            log::error!("Socket read error: {:?}", e);
                        }
                    }
                }
                
                // Handle commands
                cmd_opt = self.cmd_rx.recv() => {
                    if let Some(cmd) = cmd_opt {
                        match cmd {
                            RakNetCommand::Send(addr, payload, reliability) => {
                                self.send_packet(addr, payload, reliability).await;
                            }
                        }
                    }
                }
                
                // Handle tick processing (ACKs, keepalives, timeouts)
                _ = interval.tick() => {
                    self.tick().await;
                }
            }
        }
    }

    async fn handle_packet(&mut self, src: SocketAddr, packet: &[u8]) -> Result<(), &'static str> {
        if packet.is_empty() {
            return Err("Empty packet received");
        }

        let packet_id = packet[0];

        // Route offline / handshake packets
        match packet_id {
            ID_UNCONNECTED_PING | ID_UNCONNECTED_PING_OPEN_CONNECTIONS => {
                self.handle_unconnected_ping(src, packet).await;
                return Ok(());
            }
            ID_OPEN_CONNECTION_REQUEST_1 => {
                self.handle_open_connection_request_1(src, packet).await;
                return Ok(());
            }
            ID_OPEN_CONNECTION_REQUEST_2 => {
                self.handle_open_connection_request_2(src, packet).await;
                return Ok(());
            }
            _ => {}
        }

        // Handle ACK/NACK packets for established or connecting sessions
        if packet_id == ID_ACK || packet_id == ID_NACK {
            if let Some(session) = self.sessions.get_mut(&src) {
                session.last_activity = Instant::now();
                if let Some(ack_nack) = AckNack::read(packet_id, &packet[1..]) {
                    match ack_nack {
                        AckNack::Ack(seqs) => {
                            for seq in seqs {
                                session.sent_packets.remove(&seq);
                            }
                        }
                        AckNack::Nack(seqs) => {
                            for seq in seqs {
                                if let Some(raw_packet) = session.sent_packets.get(&seq).cloned() {
                                    if let Err(e) = self.socket.send_to(&raw_packet, src).await {
                                        log::error!("Failed to resend packet to {}: {:?}", src, e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return Ok(());
        }

        // Handle connected/encapsulated packets
        if (MIN_FRAME_SET..=MAX_FRAME_SET).contains(&packet_id) {
            if let Some(session) = self.sessions.get_mut(&src) {
                session.last_activity = Instant::now();
                if let Some(frame_set) = FrameSet::read(packet_id, &packet[1..]) {
                    // Queue ACK for this frame set sequence number
                    if !session.received_seq_nums.contains(&frame_set.sequence_number) {
                        session.received_seq_nums.insert(frame_set.sequence_number);
                        session.pending_acks.push(frame_set.sequence_number);
                        
                        // Process frames inside the frame set
                        for frame in frame_set.frames {
                            let payload = if frame.split {
                                if let Some(reassembled) = session.handle_split_frame(frame) {
                                    reassembled
                                } else {
                                    // Fragmented packet is not yet fully reassembled
                                    continue;
                                }
                            } else {
                                frame.payload
                            };

                            if let Err(e) = Self::handle_encapsulated(&self.socket, self.guid, &self.event_tx, session, &payload).await {
                                log::warn!("Failed to handle encapsulated frame: {}", e);
                            }
                        }
                    }
                }
            }
            return Ok(());
        }

        Ok(())
    }

    async fn handle_unconnected_ping(&self, src: SocketAddr, packet: &[u8]) {
        log::info!("Received Unconnected Ping from {}", src);
        let mut reader = packet;
        let _id = reader.read_u8().unwrap();
        let ping_id = match reader.read_u64::<BigEndian>() {
            Ok(v) => v,
            Err(_) => return,
        };
        
        // Skip magic
        if reader.len() < 16 {
            return;
        }
        reader = &reader[16..];
        
        let _client_guid = reader.read_u64::<BigEndian>().unwrap_or(0);
        
        // Prepare pong status string
        let motd = "§bNexusCore-BE§r Server";
        let sub_motd = "A Bedrock Server in Rust";
        let protocol = "1001"; // Minecraft v1.26.30 protocol version
        let version = "1.26.30";
        let players = "0";
        let max_players = "100";
        let gamemode = "Creative";
        let gamemode_int = "1";
        
        let status_str = format!(
            "MCPE;{};{};{};{};{};{};{};{};{};{};{};",
            motd, protocol, version, players, max_players, self.guid, sub_motd, gamemode, gamemode_int, self.port, self.port + 1
        );

        let mut reply = Vec::new();
        reply.push(ID_UNCONNECTED_PONG);
        reply.write_u64::<BigEndian>(ping_id).unwrap();
        reply.write_u64::<BigEndian>(self.guid).unwrap();
        reply.extend_from_slice(&MAGIC);
        
        let bytes_str = status_str.as_bytes();
        reply.write_u16::<BigEndian>(bytes_str.len() as u16).unwrap();
        reply.extend_from_slice(bytes_str);

        if let Err(e) = self.socket.send_to(&reply, src).await {
            log::error!("Failed to send Unconnected Pong to {}: {:?}", src, e);
        } else {
            log::info!("Sent Unconnected Pong to {}", src);
        }
    }

    async fn handle_open_connection_request_1(&self, src: SocketAddr, packet: &[u8]) {
        log::info!("Received Open Connection Request 1 from {}", src);
        let mut reader = packet;
        let _id = reader.read_u8().unwrap();
        
        // Verify magic
        if reader.len() < 16 || reader[..16] != MAGIC {
            log::warn!("Magic mismatch or packet too short from {}. Expected magic, got length {}, first 20 bytes: {:02X?}", src, reader.len(), &packet[..std::cmp::min(packet.len(), 20)]);
            return;
        }
        reader = &reader[16..];
        
        let protocol_version = reader.read_u8().unwrap_or(0);
        log::info!("Client protocol version in Request 1: {}", protocol_version);
        
        // Deduce MTU from packet size (with UDP and IP headers offset, but packet.len() is the standard RakNet MTU size)
        let client_mtu = packet.len() as u16;
        let accepted_mtu = client_mtu.clamp(576, 1492);
        log::info!("Negotiating MTU: client sent packet size {}, accepted MTU {}", client_mtu, accepted_mtu);
        
        // Generate stateless deterministic cookie based on peer port and server GUID
        let mut cookie = (src.port() as u32) ^ (self.guid as u32);
        let first_byte = (cookie >> 24) as u8;
        if first_byte == 4 || first_byte == 6 {
            cookie ^= 0x0100_0000;
        }
        log::info!("Generated security cookie 0x{:08X} for {}", cookie, src);

        let mut reply = Vec::new();
        reply.push(ID_OPEN_CONNECTION_REPLY_1);
        reply.extend_from_slice(&MAGIC);
        reply.write_u64::<BigEndian>(self.guid).unwrap();
        reply.push(1); // Security flag: enabled (Minecraft Bedrock expects a cookie)
        reply.write_u32::<BigEndian>(cookie).unwrap();
        reply.write_u16::<BigEndian>(accepted_mtu).unwrap();

        if let Err(e) = self.socket.send_to(&reply, src).await {
            log::error!("Failed to send Open Connection Reply 1 to {}: {:?}", src, e);
        } else {
            log::info!("Sent Open Connection Reply 1 to {}", src);
        }
    }

    async fn handle_open_connection_request_2(&mut self, src: SocketAddr, packet: &[u8]) {
        log::info!("Received Open Connection Request 2 from {}", src);
        let mut reader = packet;
        let _id = reader.read_u8().unwrap();
        
        if reader.len() < 16 || reader[..16] != MAGIC {
            log::warn!("Magic mismatch or packet too short in Request 2 from {}. Got length {}, first 20 bytes: {:02X?}", src, reader.len(), &packet[..std::cmp::min(packet.len(), 20)]);
            return;
        }
        reader = &reader[16..];
        
        // Dynamically parse cookie (4 bytes) and client proof (1 byte) if present.
        // If the first byte after MAGIC is neither 4 nor 6, it indicates cookie presence.
        let mut cookie = None;
        if !reader.is_empty() {
            let first = reader[0];
            if first != 4 && first != 6 {
                if reader.len() < 5 {
                    log::warn!("Request 2 packet too short to contain cookie from {}", src);
                    return;
                }
                let cookie_val = reader.read_u32::<BigEndian>().unwrap();
                let client_proof = reader.read_u8().unwrap();
                cookie = Some(cookie_val);
                log::info!("Decoded cookie 0x{:08X} and client proof byte {} from {}", cookie_val, client_proof, src);
            }
        }
        
        // Validate cookie
        let mut expected_cookie = (src.port() as u32) ^ (self.guid as u32);
        let first_byte = (expected_cookie >> 24) as u8;
        if first_byte == 4 || first_byte == 6 {
            expected_cookie ^= 0x0100_0000;
        }
        if let Some(cookie_val) = cookie {
            if cookie_val != expected_cookie {
                log::warn!("Invalid cookie received from {}: expected 0x{:08X}, got 0x{:08X}", src, expected_cookie, cookie_val);
                return;
            }
        } else {
            log::warn!("No cookie received in Request 2 from {}, but security was enabled", src);
        }

        let server_addr = match read_address(&mut reader) {
            Some(addr) => addr,
            None => {
                log::warn!("Failed to read server address in Request 2 from {}", src);
                return;
            }
        };
        log::info!("Server address sent in Request 2: {}", server_addr);
        
        let mtu = match reader.read_u16::<BigEndian>() {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Failed to read MTU in Request 2: {:?}", e);
                return;
            }
        };
        
        let client_guid = match reader.read_u64::<BigEndian>() {
            Ok(v) => v,
            Err(e) => {
                log::warn!("Failed to read client GUID in Request 2: {:?}", e);
                return;
            }
        };
        log::info!("MTU in Request 2: {}, Client GUID: {}", mtu, client_guid);

        // Send reply 2
        let mut reply = Vec::new();
        reply.push(ID_OPEN_CONNECTION_REPLY_2);
        reply.extend_from_slice(&MAGIC);
        reply.write_u64::<BigEndian>(self.guid).unwrap();
        write_address(&mut reply, &src);
        reply.write_u16::<BigEndian>(mtu).unwrap();
        reply.push(1); // Security flag: enabled (matching Request 2 security)

        match self.socket.send_to(&reply, src).await {
            Ok(_) => {
                log::info!("Sent Open Connection Reply 2 to {}", src);
                // Create a session for this client
                let session = Session::new(src, client_guid, mtu);
                self.sessions.insert(src, session);
            }
            Err(e) => {
                log::error!("Failed to send Open Connection Reply 2 to {}: {:?}", src, e);
            }
        }
    }

    async fn handle_encapsulated(
        socket: &UdpSocket,
        _server_guid: u64,
        event_tx: &mpsc::Sender<RakNetEvent>,
        session: &mut Session,
        payload: &[u8],
    ) -> Result<(), &'static str> {
        if payload.is_empty() {
            return Err("Empty encapsulated payload");
        }

        let packet_id = payload[0];

        match packet_id {
            ID_CONNECTION_REQUEST => {
                let mut reader = payload;
                let _id = reader.read_u8().unwrap();
                let _client_guid = reader.read_u64::<BigEndian>().map_err(|_| "Failed to read client GUID")?;
                let send_time = reader.read_u64::<BigEndian>().map_err(|_| "Failed to read send time")?;
                
                log::info!("Connection Request from client GUID: {} on {}", _client_guid, session.address);
                
                // Build connection request accepted packet
                let mut accept_payload = Vec::new();
                accept_payload.push(ID_CONNECTION_REQUEST_ACCEPTED);
                write_address(&mut accept_payload, &session.address);
                accept_payload.write_u16::<BigEndian>(0).unwrap(); // System index
                
                // 20 System addresses
                for _ in 0..20 {
                    write_address(&mut accept_payload, &SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), 0));
                }
                
                accept_payload.write_u64::<BigEndian>(send_time).unwrap();
                
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                accept_payload.write_u64::<BigEndian>(current_time).unwrap();

                // Send inside FrameSet
                let frame_set = session.create_frame_set(accept_payload, Reliability::ReliableOrdered);
                Self::send_frame_set(socket, session, frame_set).await;
            }
            ID_NEW_INCOMING_CONNECTION => {
                session.state = SessionState::Connected;
                log::info!("RakNet session successfully established for {}", session.address);
                let _ = event_tx.send(RakNetEvent::Connected(session.address, session.guid)).await;
            }
            ID_CONNECTED_PING => {
                let mut reader = payload;
                let _id = reader.read_u8().unwrap();
                let client_time = reader.read_u64::<BigEndian>().map_err(|_| "Failed to read client time")?;
                
                let mut pong_payload = Vec::new();
                pong_payload.push(ID_CONNECTED_PONG);
                pong_payload.write_u64::<BigEndian>(client_time).unwrap();
                
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                pong_payload.write_u64::<BigEndian>(current_time).unwrap();

                let frame_set = session.create_frame_set(pong_payload, Reliability::Reliable);
                Self::send_frame_set(socket, session, frame_set).await;
            }
            ID_DISCONNECT => {
                session.state = SessionState::Disconnected;
                log::info!("Client requested disconnect: {}", session.address);
                let _ = event_tx.send(RakNetEvent::Disconnected(session.address)).await;
            }
            _ => {
                // This is a game packet or other encapsulated packet, pass it to the upper layer
                if session.state == SessionState::Connected {
                    let _ = event_tx.send(RakNetEvent::Packet(session.address, payload.to_vec())).await;
                } else {
                    log::warn!("Received packet ID {} from {} before connection handshake complete", packet_id, session.address);
                }
            }
        }

        Ok(())
    }

    async fn send_frame_set(socket: &UdpSocket, session: &mut Session, frame_set: FrameSet) {
        let mut buf = Vec::new();
        frame_set.write(0x80, &mut buf);
        
        session.sent_packets.insert(frame_set.sequence_number, buf.clone());
        if let Err(e) = socket.send_to(&buf, session.address).await {
            log::error!("Failed to send FrameSet to {}: {:?}", session.address, e);
        }
    }

    pub async fn send_packet(&mut self, addr: SocketAddr, payload: Vec<u8>, reliability: Reliability) {
        if let Some(session) = self.sessions.get_mut(&addr) {
            let limit = (session.mtu as usize).saturating_sub(60);
            if payload.len() > limit {
                let split_id = session.next_split_id;
                session.next_split_id = session.next_split_id.wrapping_add(1);
                
                let chunks: Vec<&[u8]> = payload.chunks(limit).collect();
                let split_count = chunks.len();
                
                let order_index = if reliability.is_ordered() {
                    let idx = session.order_index;
                    session.order_index += 1;
                    Some(idx)
                } else {
                    None
                };

                for (chunk_idx, chunk) in chunks.into_iter().enumerate() {
                    let mut frame = Frame {
                        reliability,
                        split: true,
                        reliability_index: None,
                        sequence_index: None,
                        order_index,
                        order_channel: if reliability.is_ordered() { Some(0) } else { None },
                        split_count: Some(split_count as u32),
                        split_id: Some(split_id),
                        split_index: Some(chunk_idx as u32),
                        payload: chunk.to_vec(),
                    };

                    if reliability.is_reliable() {
                        frame.reliability_index = Some(session.message_index);
                        session.message_index += 1;
                    }

                    let seq = session.send_seq_num;
                    session.send_seq_num += 1;

                    let frame_set = FrameSet {
                        sequence_number: seq,
                        frames: vec![frame],
                    };
                    Self::send_frame_set(&self.socket, session, frame_set).await;
                }
            } else {
                let frame_set = session.create_frame_set(payload, reliability);
                Self::send_frame_set(&self.socket, session, frame_set).await;
            }
        }
    }

    async fn tick(&mut self) {
        let mut disconnected = Vec::new();
        
        for (addr, session) in self.sessions.iter_mut() {
            // Check session timeout
            if session.last_activity.elapsed() > Duration::from_secs(10) {
                log::info!("Session timed out for client {}", addr);
                session.state = SessionState::Disconnected;
                disconnected.push(*addr);
                continue;
            }

            // Send pending ACKs
            if !session.pending_acks.is_empty() {
                let ack = AckNack::Ack(session.pending_acks.drain(..).collect());
                let mut buf = Vec::new();
                ack.write(ID_ACK, &mut buf);
                if let Err(e) = self.socket.send_to(&buf, *addr).await {
                    log::error!("Failed to send ACK to {}: {:?}", addr, e);
                }
            }
        }

        // Clean up disconnected sessions
        for addr in disconnected {
            self.sessions.remove(&addr);
            let _ = self.event_tx.send(RakNetEvent::Disconnected(addr)).await;
        }
    }
}
