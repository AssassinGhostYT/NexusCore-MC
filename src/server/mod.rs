pub mod client;
pub mod handler;

use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use crate::raknet::server::{RakNetEvent, RakNetCommand};
use self::client::ClientState;
use self::handler::handle_packet;

pub struct Server {
    cmd_tx: mpsc::Sender<RakNetCommand>,
    event_rx: mpsc::Receiver<RakNetEvent>,
    clients: HashMap<SocketAddr, ClientState>,
}

impl Server {
    pub fn new(cmd_tx: mpsc::Sender<RakNetCommand>, event_rx: mpsc::Receiver<RakNetEvent>) -> Self {
        Self {
            cmd_tx,
            event_rx,
            clients: HashMap::new(),
        }
    }

    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(event) = self.event_rx.recv().await {
            match event {
                RakNetEvent::Connected(addr, guid) => {
                    log::info!("Event: Client Connected: {} (GUID: {})", addr, guid);
                    self.clients.insert(addr, ClientState::new());
                }
                RakNetEvent::Disconnected(addr) => {
                    log::info!("Event: Client Disconnected: {}", addr);
                    self.clients.remove(&addr);
                }
                RakNetEvent::Packet(addr, payload) => {
                    if let Some(state) = self.clients.get_mut(&addr) {
                        if let Err(e) = handle_packet(addr, payload, state, &self.cmd_tx).await {
                            log::error!("Error handling packet from {}: {:?}", addr, e);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
