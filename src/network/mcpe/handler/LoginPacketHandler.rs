use std::net::SocketAddr;
use tokio::sync::mpsc;
use crate::raknet::server::RakNetCommand;
use crate::protocol::packet::*;
use crate::server::client::ClientState;

pub async fn handle_request_network_settings(
    addr: SocketAddr,
    payload: &[u8],
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    let req = RequestNetworkSettings::read(payload)?;
    log::info!(
        "[{}] [PROTOCOLO] Cliente usa protocol_version = {} (nuestro servidor habla 1001 = v1.26.31)",
        addr,
        req.protocol_version
    );

    let settings = NetworkSettings {
        compression_threshold: 0,
        compression_algorithm: 0,
        client_throttle: false,
        client_throttle_threshold: 0,
        client_throttle_scalar: 0.0,
    };
    let response = settings.write()?;
    let game_packet = GamePacket {
        id: ID_NETWORK_SETTINGS,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: response,
    };
    state.send_packets(addr, cmd_tx, &[game_packet]).await?;
    state.compression_enabled = true;
    log::info!("Compresion habilitada");
    Ok(())
}
