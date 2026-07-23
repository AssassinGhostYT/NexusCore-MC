use std::net::SocketAddr;
use tokio::sync::mpsc;
use crate::raknet::server::RakNetCommand;
use crate::protocol::packet::*;
use crate::protocol::types::*;
use crate::server::client::ClientState;

pub async fn handle_request_chunk_radius(
    addr: SocketAddr,
    payload: &[u8],
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(req) = RequestChunkRadius::read(payload) {
        let radius = req.chunk_radius;
        state.chunk_radius = radius as u32;
        log::info!("[{}] handle_request_chunk_radius: received RequestChunkRadius, client requests radius {}", addr, radius);

        // 1. ChunkRadiusUpdated
        log::info!("[{}] Sending ChunkRadiusUpdated (radius={})...", addr, radius);
        let radius_payload = ChunkRadiusUpdated { radius }.write()?;
        let radius_pkg = GamePacket {
            id: ID_CHUNK_RADIUS_UPDATED,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: radius_payload,
        };
        state.send_packets(addr, cmd_tx, &[radius_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        state.last_chunk_x = Some(0);
        state.last_chunk_z = Some(0);

        let chunk_payload = make_flat_chunk_payload();
        let mut built_chunks = Vec::new();
        let radius = (req.chunk_radius as u32).min(3);
        let r = radius as i32;
        let mut chunk_pkgs = Vec::new();
        for dx in -r..=r {
            for dz in -r..=r {
                state.loaded_chunks.insert((dx, dz));
                built_chunks.push(ChunkPos { x: dx, z: dz });

                let chunk_payload_written = LevelChunk {
                    chunk_x: dx,
                    chunk_z: dz,
                    sub_chunk_count: 24,  // Full 24 sub-chunks height
                    payload: chunk_payload.clone(),
                }.write()?;
                let chunk_pkg = GamePacket {
                    id: ID_LEVEL_CHUNK,
                    sender_subclient: 0,
                    recipient_subclient: 0,
                    payload: chunk_payload_written,
                };
                chunk_pkgs.push(chunk_pkg);
                if chunk_pkgs.len() >= 16 {
                    state.send_packets(addr, cmd_tx, &chunk_pkgs).await?;
                    chunk_pkgs.clear();
                    tokio::time::sleep(std::time::Duration::from_millis(2)).await;
                }
            }
        }
        if !chunk_pkgs.is_empty() {
            state.send_packets(addr, cmd_tx, &chunk_pkgs).await?;
        }

        // 2. NetworkChunkPublisherUpdate
        log::info!("[{}] Sending NetworkChunkPublisherUpdate...", addr);
        let publisher_payload = NetworkChunkPublisherUpdate {
            position: BlockPos { x: 0, y: 64, z: 0 },
            radius: (radius as u32) << 4,
            server_built_chunks: Vec::new(),
        }.write()?;
        let publisher_pkg = GamePacket {
            id: ID_NETWORK_CHUNK_PUBLISHER_UPDATE,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: publisher_payload,
        };
        state.send_packets(addr, cmd_tx, &[publisher_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        if !state.world_data_sent {
            state.world_data_sent = true;

            let spawn_payload = PlayStatus { status: 3 }.write()?;
            let spawn_pkg = GamePacket {
                id: ID_PLAY_STATUS,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: spawn_payload,
            };

            state.send_packets(addr, cmd_tx, &[spawn_pkg]).await?;
            log::info!("[{}] Sent PlayStatus(3) PlayerSpawn after chunks!", addr);
        }
    }
    Ok(())
}
