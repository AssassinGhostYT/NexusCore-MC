use std::net::SocketAddr;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

// RakNet offline packet magic
pub const MAGIC: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00,
    0xfe, 0xfe, 0xfe, 0xfe,
    0xfd, 0xfd, 0xfd, 0xfd,
    0x12, 0x34, 0x56, 0x78,
];

// Offline Packet IDs
pub const ID_UNCONNECTED_PING: u8 = 0x01;
pub const ID_UNCONNECTED_PING_OPEN_CONNECTIONS: u8 = 0x02;
pub const ID_OPEN_CONNECTION_REQUEST_1: u8 = 0x05;
pub const ID_OPEN_CONNECTION_REPLY_1: u8 = 0x06;
pub const ID_OPEN_CONNECTION_REQUEST_2: u8 = 0x07;
pub const ID_OPEN_CONNECTION_REPLY_2: u8 = 0x08;
pub const ID_UNCONNECTED_PONG: u8 = 0x1c;

// Online Packet IDs (encapsulated)
pub const ID_CONNECTED_PING: u8 = 0x00;
pub const ID_CONNECTED_PONG: u8 = 0x03;
pub const ID_CONNECTION_REQUEST: u8 = 0x09;
pub const ID_CONNECTION_REQUEST_ACCEPTED: u8 = 0x10;
pub const ID_NEW_INCOMING_CONNECTION: u8 = 0x13;
pub const ID_DISCONNECT: u8 = 0x15;
#[allow(dead_code)]
pub const ID_DETECT_LOST_CONNECTIONS: u8 = 0x04;

// Frame set packets range
pub const MIN_FRAME_SET: u8 = 0x80;
pub const MAX_FRAME_SET: u8 = 0x8d;

// ACK/NACK
pub const ID_ACK: u8 = 0xc0;
pub const ID_NACK: u8 = 0xa0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reliability {
    Unreliable = 0,
    UnreliableSequenced = 1,
    Reliable = 2,
    ReliableOrdered = 3,
    ReliableSequenced = 4,
    UnreliableWithAckReceipt = 5,
    ReliableWithAckReceipt = 6,
    ReliableOrderedWithAckReceipt = 7,
}

impl Reliability {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Unreliable),
            1 => Some(Self::UnreliableSequenced),
            2 => Some(Self::Reliable),
            3 => Some(Self::ReliableOrdered),
            4 => Some(Self::ReliableSequenced),
            5 => Some(Self::UnreliableWithAckReceipt),
            6 => Some(Self::ReliableWithAckReceipt),
            7 => Some(Self::ReliableOrderedWithAckReceipt),
            _ => None,
        }
    }

    pub fn is_reliable(self) -> bool {
        matches!(
            self,
            Self::Reliable
                | Self::ReliableOrdered
                | Self::ReliableSequenced
                | Self::ReliableWithAckReceipt
                | Self::ReliableOrderedWithAckReceipt
        )
    }

    pub fn is_ordered(self) -> bool {
        matches!(
            self,
            Self::ReliableOrdered | Self::ReliableOrderedWithAckReceipt
        )
    }

    pub fn is_sequenced(self) -> bool {
        matches!(
            self,
            Self::UnreliableSequenced | Self::ReliableSequenced
        )
    }
}

// Read/write u24 in little-endian
pub fn read_u24_le(buf: &mut &[u8]) -> Option<u32> {
    if buf.len() < 3 {
        return None;
    }
    let val = buf[0] as u32 | ((buf[1] as u32) << 8) | ((buf[2] as u32) << 16);
    *buf = &buf[3..];
    Some(val)
}

pub fn write_u24_le(buf: &mut Vec<u8>, val: u32) {
    buf.push((val & 0xff) as u8);
    buf.push(((val >> 8) & 0xff) as u8);
    buf.push(((val >> 16) & 0xff) as u8);
}

// Serialization/Deserialization of SocketAddr (RakNet format)
pub fn read_address(buf: &mut &[u8]) -> Option<SocketAddr> {
    if buf.is_empty() {
        return None;
    }
    let version = buf[0];
    *buf = &buf[1..];
    if version == 4 {
        if buf.len() < 6 {
            return None;
        }
        let ip1 = !buf[0];
        let ip2 = !buf[1];
        let ip3 = !buf[2];
        let ip4 = !buf[3];
        *buf = &buf[4..];
        let port = (&mut *buf).read_u16::<BigEndian>().ok()?;
        let ip = std::net::Ipv4Addr::new(ip1, ip2, ip3, ip4);
        Some(SocketAddr::new(std::net::IpAddr::V4(ip), port))
    } else if version == 6 {
        if buf.len() < 28 {
            return None;
        }
        let _family = (&mut *buf).read_u16::<BigEndian>().ok()?;
        let port = (&mut *buf).read_u16::<BigEndian>().ok()?;
        let _flowinfo = (&mut *buf).read_u32::<BigEndian>().ok()?;
        let mut ip_bytes = [0u8; 16];
        ip_bytes.copy_from_slice(&buf[..16]);
        *buf = &buf[16..];
        let _scope_id = (&mut *buf).read_u32::<BigEndian>().ok()?;
        let ip = std::net::Ipv6Addr::from(ip_bytes);
        Some(SocketAddr::new(std::net::IpAddr::V6(ip), port))
    } else {
        None
    }
}

pub fn write_address(buf: &mut Vec<u8>, addr: &SocketAddr) {
    match addr {
        SocketAddr::V4(addr_v4) => {
            buf.push(4);
            let octets = addr_v4.ip().octets();
            buf.push(!octets[0]);
            buf.push(!octets[1]);
            buf.push(!octets[2]);
            buf.push(!octets[3]);
            let mut port_buf = [0u8; 2];
            (&mut port_buf[..]).write_u16::<BigEndian>(addr_v4.port()).unwrap();
            buf.extend_from_slice(&port_buf);
        }
        SocketAddr::V6(addr_v6) => {
            buf.push(6);
            // Family AF_INET6 is 23
            let mut family_buf = [0u8; 2];
            (&mut family_buf[..]).write_u16::<BigEndian>(23).unwrap();
            buf.extend_from_slice(&family_buf);

            let mut port_buf = [0u8; 2];
            (&mut port_buf[..]).write_u16::<BigEndian>(addr_v6.port()).unwrap();
            buf.extend_from_slice(&port_buf);

            let mut flowinfo_buf = [0u8; 4];
            (&mut flowinfo_buf[..]).write_u32::<BigEndian>(addr_v6.flowinfo()).unwrap();
            buf.extend_from_slice(&flowinfo_buf);

            buf.extend_from_slice(&addr_v6.ip().octets());

            let mut scope_buf = [0u8; 4];
            (&mut scope_buf[..]).write_u32::<BigEndian>(addr_v6.scope_id()).unwrap();
            buf.extend_from_slice(&scope_buf);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub reliability: Reliability,
    pub split: bool,
    pub reliability_index: Option<u32>, // Message index
    pub sequence_index: Option<u32>,
    pub order_index: Option<u32>,
    pub order_channel: Option<u8>,
    pub split_count: Option<u32>,
    pub split_id: Option<u16>,
    pub split_index: Option<u32>,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn read(buf: &mut &[u8]) -> Option<Self> {
        if buf.is_empty() {
            return None;
        }
        let flags = buf[0];
        *buf = &buf[1..];

        let reliability_val = (flags & 0xE0) >> 5;
        let reliability = Reliability::from_u8(reliability_val)?;
        let split = (flags & 0x10) != 0;

        if buf.len() < 2 {
            return None;
        }
        let length_bits = (&mut *buf).read_u16::<BigEndian>().ok()?;
        let length_bytes = ((length_bits + 7) / 8) as usize;

        let mut reliability_index = None;
        let mut sequence_index = None;
        let mut order_index = None;
        let mut order_channel = None;

        if reliability.is_reliable() {
            reliability_index = Some(read_u24_le(buf)?);
        }

        if reliability.is_sequenced() {
            sequence_index = Some(read_u24_le(buf)?);
        }

        if reliability.is_ordered() || reliability.is_sequenced() {
            order_index = Some(read_u24_le(buf)?);
            if buf.is_empty() {
                return None;
            }
            order_channel = Some(buf[0]);
            *buf = &buf[1..];
        }

        let mut split_count = None;
        let mut split_id = None;
        let mut split_index = None;

        if split {
            if buf.len() < 10 {
                return None;
            }
            split_count = Some((&mut *buf).read_u32::<BigEndian>().ok()?);
            split_id = Some((&mut *buf).read_u16::<BigEndian>().ok()?);
            split_index = Some((&mut *buf).read_u32::<BigEndian>().ok()?);
        }

        if buf.len() < length_bytes {
            return None;
        }
        let payload = buf[..length_bytes].to_vec();
        *buf = &buf[length_bytes..];

        Some(Frame {
            reliability,
            split,
            reliability_index,
            sequence_index,
            order_index,
            order_channel,
            split_count,
            split_id,
            split_index,
            payload,
        })
    }

    pub fn write(&self, buf: &mut Vec<u8>) {
        let mut flags = (self.reliability as u8) << 5;
        if self.split {
            flags |= 0x10;
        }
        buf.push(flags);

        let length_bits = (self.payload.len() * 8) as u16;
        let mut length_buf = [0u8; 2];
        (&mut length_buf[..]).write_u16::<BigEndian>(length_bits).unwrap();
        buf.extend_from_slice(&length_buf);

        if self.reliability.is_reliable() {
            write_u24_le(buf, self.reliability_index.unwrap_or(0));
        }

        if self.reliability.is_sequenced() {
            write_u24_le(buf, self.sequence_index.unwrap_or(0));
        }

        if self.reliability.is_ordered() || self.reliability.is_sequenced() {
            write_u24_le(buf, self.order_index.unwrap_or(0));
            buf.push(self.order_channel.unwrap_or(0));
        }

        if self.split {
            let mut split_buf = [0u8; 10];
            let mut writer = &mut split_buf[..];
            writer.write_u32::<BigEndian>(self.split_count.unwrap_or(0)).unwrap();
            writer.write_u16::<BigEndian>(self.split_id.unwrap_or(0)).unwrap();
            writer.write_u32::<BigEndian>(self.split_index.unwrap_or(0)).unwrap();
            buf.extend_from_slice(&split_buf);
        }

        buf.extend_from_slice(&self.payload);
    }
}

#[derive(Debug, Clone)]
pub struct FrameSet {
    pub sequence_number: u32,
    pub frames: Vec<Frame>,
}

impl FrameSet {
    pub fn read(_packet_id: u8, mut buf: &[u8]) -> Option<Self> {
        let sequence_number = read_u24_le(&mut buf)?;
        let mut frames = Vec::new();
        while !buf.is_empty() {
            let frame = Frame::read(&mut buf)?;
            frames.push(frame);
        }
        Some(FrameSet {
            sequence_number,
            frames,
        })
    }

    pub fn write(&self, packet_id: u8, buf: &mut Vec<u8>) {
        buf.push(packet_id);
        write_u24_le(buf, self.sequence_number);
        for frame in &self.frames {
            frame.write(buf);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AckNack {
    Ack(Vec<u32>),
    Nack(Vec<u32>),
}

impl AckNack {
    pub fn read(packet_id: u8, mut buf: &[u8]) -> Option<Self> {
        if buf.len() < 2 {
            return None;
        }
        let count = (&mut buf).read_u16::<BigEndian>().ok()? as usize;
        let mut sequence_numbers = Vec::new();

        for _ in 0..count {
            if buf.is_empty() {
                return None;
            }
            let is_single = buf[0] == 1;
            buf = &buf[1..];

            if is_single {
                let seq = read_u24_le(&mut buf)?;
                sequence_numbers.push(seq);
            } else {
                let start = read_u24_le(&mut buf)?;
                let end = read_u24_le(&mut buf)?;
                if start > end {
                    return None;
                }
                for seq in start..=end {
                    sequence_numbers.push(seq);
                }
            }
        }

        if packet_id == ID_ACK {
            Some(AckNack::Ack(sequence_numbers))
        } else {
            Some(AckNack::Nack(sequence_numbers))
        }
    }

    pub fn write(&self, packet_id: u8, buf: &mut Vec<u8>) {
        buf.push(packet_id);

        let seqs = match self {
            AckNack::Ack(s) => s,
            AckNack::Nack(s) => s,
        };

        if seqs.is_empty() {
            let mut count_buf = [0u8; 2];
            (&mut count_buf[..]).write_u16::<BigEndian>(0).unwrap();
            buf.extend_from_slice(&count_buf);
            return;
        }

        let mut sorted_seqs = seqs.clone();
        sorted_seqs.sort_unstable();
        sorted_seqs.dedup();

        let mut ranges = Vec::new();
        let mut range_start = sorted_seqs[0];
        let mut prev = sorted_seqs[0];

        for &seq in sorted_seqs.iter().skip(1) {
            if seq == prev + 1 {
                prev = seq;
            } else {
                ranges.push((range_start, prev));
                range_start = seq;
                prev = seq;
            }
        }
        ranges.push((range_start, prev));

        let mut count_buf = [0u8; 2];
        (&mut count_buf[..]).write_u16::<BigEndian>(ranges.len() as u16).unwrap();
        buf.extend_from_slice(&count_buf);

        for &(start, end) in &ranges {
            if start == end {
                buf.push(1);
                write_u24_le(buf, start);
            } else {
                buf.push(0);
                write_u24_le(buf, start);
                write_u24_le(buf, end);
            }
        }
    }
}
