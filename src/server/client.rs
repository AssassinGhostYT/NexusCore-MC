use std::net::SocketAddr;
use tokio::sync::mpsc;
use crate::raknet::server::RakNetCommand;
use crate::raknet::protocol::Reliability;
use crate::network::mcpe::protocol::packets::{GamePacket, encode_batch};

pub struct ClientState {
    pub username: String,
    pub uuid: [u8; 16],
    pub compression_enabled: bool,
    pub encryption_state: Option<crate::protocol::encryption::EncryptionState>,
    pub last_chunk_x: Option<i32>,
    pub last_chunk_z: Option<i32>,
    pub loaded_chunks: std::collections::HashSet<(i32, i32)>,
    pub world_data_sent: bool,
    pub chunk_radius: u32,
    pub last_pitch: f32,
    pub last_yaw: f32,
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            uuid: [0u8; 16],
            compression_enabled: false,
            encryption_state: None,
            last_chunk_x: None,
            last_chunk_z: None,
            loaded_chunks: std::collections::HashSet::new(),
            world_data_sent: false,
            chunk_radius: 4,
            last_pitch: 0.0,
            last_yaw: 0.0,
        }
    }

    pub async fn send_packets(
        &mut self,
        addr: SocketAddr,
        cmd_tx: &mpsc::Sender<RakNetCommand>,
        packets: &[GamePacket],
    ) -> std::io::Result<()> {
        for p in packets {
            log::info!(
                "[{}] SEND OUTBOUND GAME PACKET: id={} (0x{:02x}), payload_len={}",
                addr,
                p.id,
                p.id,
                p.payload.len()
            );
        }
        let mut reply_payload = encode_batch(packets, self.compression_enabled);
        if let Some(ref mut crypto) = self.encryption_state {
            let encrypted_body = crypto.encrypt_packet(&reply_payload[1..]);
            reply_payload.truncate(1);
            reply_payload.extend_from_slice(&encrypted_body);
        }
        let res = cmd_tx.send(RakNetCommand::Send(
            addr,
            reply_payload,
            Reliability::ReliableOrdered,
        )).await;
        if let Err(e) = res {
            log::error!("[{}] Failed to send RakNetCommand: {:?}", addr, e);
        }
        Ok(())
    }
}
