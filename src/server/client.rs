use std::net::SocketAddr;
use tokio::sync::mpsc;
use crate::raknet::server::RakNetCommand;
use crate::raknet::protocol::Reliability;
use crate::protocol;
use crate::protocol::packet::{GamePacket, encode_batch};

pub struct ClientState {
    pub compression_enabled: bool,
    pub encryption_state: Option<protocol::encryption::EncryptionState>,
    pub last_chunk_x: Option<i32>,
    pub last_chunk_z: Option<i32>,
    pub loaded_chunks: std::collections::HashSet<(i32, i32)>,
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            compression_enabled: false,
            encryption_state: None,
            last_chunk_x: None,
            last_chunk_z: None,
            loaded_chunks: std::collections::HashSet::new(),
        }
    }

    pub async fn send_packets(
        &mut self,
        addr: SocketAddr,
        cmd_tx: &mpsc::Sender<RakNetCommand>,
        packets: &[GamePacket],
    ) -> std::io::Result<()> {
        let mut reply_payload = encode_batch(packets, self.compression_enabled)?;
        if let Some(ref mut crypto) = self.encryption_state {
            let encrypted_body = crypto.encrypt_packet(&reply_payload[1..]);
            reply_payload.truncate(1);
            reply_payload.extend_from_slice(&encrypted_body);
        }
        let _ = cmd_tx.send(RakNetCommand::Send(
            addr,
            reply_payload,
            Reliability::ReliableOrdered,
        )).await;
        Ok(())
    }
}
