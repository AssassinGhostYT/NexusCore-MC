use std::net::SocketAddr;
use tokio::sync::mpsc;
use crate::raknet::server::RakNetCommand;
use crate::protocol::packet::*;
use crate::protocol::types::*;
use crate::protocol;
use super::client::ClientState;
use super::packets;
use crate::log_t;
use base64::{Engine as _, engine::general_purpose};

pub async fn handle_packet(
    addr: SocketAddr,
    payload: &[u8],
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut payload = payload.to_vec();

    if let Some(ref mut crypto) = state.encryption_state {
        match crypto.decrypt_packet(&mut payload[1..]) {
            Ok(decrypted_body) => {
                payload.truncate(1);
                payload.extend_from_slice(&decrypted_body);
            }
            Err(e) => {
                log::error!("[{}] Packet decryption failed: {:?}", addr, e);
                return Ok(());
            }
        }
    }

    if payload.is_empty() || payload[0] != ID_GAME_PACKET as u8 {
        return Ok(());
    }

    let batch_payload = &payload[1..];
    let packets_res = decode_batch(batch_payload, state.compression_enabled);
    match packets_res {
        Ok(pkt_list) => {
            for packet in pkt_list {
                match packet.id {
                    ID_REQUEST_NETWORK_SETTINGS => {
                        handle_request_network_settings(addr, &packet.payload, state, cmd_tx).await?;
                    }
                    ID_LOGIN => {
                        handle_login(addr, &packet.payload, state, cmd_tx).await?;
                    }
                    ID_CLIENT_TO_SERVER_HANDSHAKE => {
                        handle_client_to_server_handshake(addr, state, cmd_tx).await?;
                    }
                    ID_RESOURCE_PACK_CLIENT_RESPONSE => {
                        handle_resource_pack_client_response(addr, &packet.payload, state, cmd_tx).await?;
                    }
                    ID_MOVE_PLAYER => {
                        handle_move_player(addr, &packet.payload, state, cmd_tx).await?;
                    }
                    ID_PLAYER_AUTH_INPUT => {
                        handle_player_auth_input(addr, &packet.payload, state, cmd_tx).await?;
                    }
                    ID_REQUEST_CHUNK_RADIUS => {
                        handle_request_chunk_radius(addr, &packet.payload, state, cmd_tx).await?;
                    }
                    ID_CLIENT_CACHE_STATUS => {
                        log::info!("[{}] Received ClientCacheStatus (ID 129), ignoring...", addr);
                    }
                    ID_PACKET_VIOLATION_WARNING => {
                        if let Ok(violation) = PacketViolationWarning::read(&packet.payload) {
                            log_t!(error, VIOLATION_DETECTED, violation.packet_id, violation.severity, violation.context);
                            if violation.severity == 3 {
                                log::error!("Connection will be terminated by client.");
                            }
                        }
                    }
                    ID_SET_LOCAL_PLAYER_AS_INITIALISED => {
                        if let Ok(init_packet) = SetLocalPlayerAsInitialized::read(&packet.payload) {
                            let rid = init_packet.entity_runtime_id;
                            log::info!(
                                "[{}] Received SetLocalPlayerAsInitialised (ID 113) - Player Runtime ID: {} - PLAYER SPAWN COMPLETED!", 
                                addr, 
                                rid
                            );

                            if !state.world_data_sent {
                                state.world_data_sent = true;
                                
                                let gametype_pkg = packets::create_gametype_pkg(1); // Creative = 1
                                let abilities_pkg = packets::create_abilities_pkg(rid as i64);
                                let adventure_pkg = packets::create_adventure_pkg();

                                let mut spawn_batch = vec![gametype_pkg, abilities_pkg, adventure_pkg];
                                let inv_setup_pkgs = packets::get_inventory_setup_packets();
                                spawn_batch.extend(inv_setup_pkgs);

                                let teleport_pkg = packets::create_move_player_pkg(rid, 0.5, -58.38, 0.5);
                                spawn_batch.push(teleport_pkg);

                                let identity = vec![
                                    packets::create_update_attributes_pkg(rid),
                                    packets::create_set_actor_data_pkg(rid),
                                ];
                                spawn_batch.extend(identity);

                                let commands_pkg = packets::create_available_commands_pkg();
                                spawn_batch.push(commands_pkg);

                                state.send_packets(addr, cmd_tx, &spawn_batch).await?;
                                log::info!("[{}] Player active state synchronized!", addr);
                            }
                        }
                    }
                    ID_SERVER_BOUND_LOADING_SCREEN => {
                        if let Ok(loading) = ServerBoundLoadingScreen::read(&packet.payload) {
                            log::info!(
                                "[{}] Received ServerBoundLoadingScreen: event_type={}, screen_id={:?}",
                                addr,
                                loading.event_type,
                                loading.screen_id
                            );
                        }
                    }
                    ID_ITEM_STACK_REQUEST => {
                        if let Ok(req) = ItemStackRequest::read(&packet.payload) {
                            log::info!(
                                "[{}] Received ItemStackRequest (ID 147): raw_bytes_count={}",
                                addr,
                                req.raw.len()
                            );
                        }
                    }
                    ID_INVENTORY_TRANSACTION => {
                        let mut slice = &packet.payload[..];
                        if let Ok(tx) = InventoryTransaction::read(&mut slice) {
                            log::info!(
                                "[{}] Received InventoryTransaction (ID 30): actions_count={}",
                                addr,
                                tx.actions.len()
                            );
                        }
                    }
                    ID_CONTAINER_CLOSE => {
                        let mut slice = &packet.payload[..];
                        if let Ok(cc) = ContainerClose::read(&mut slice) {
                            log::info!(
                                "[{}] Received ContainerClose (ID 47): window_id={}, server_initiated={}",
                                addr,
                                cc.window_id,
                                cc.server_side
                            );
                        }
                    }
                    ID_SUB_CHUNK_REQUEST => {
                        if let Ok(req) = SubChunkRequest::read(&mut &packet.payload[..]) {
                            log::info!(
                                "[{}] SubChunkRequest: dimension={}, position=({:?}), offsets_count={}",
                                addr, req.dimension, req.position, req.offsets.len()
                            );
                        }
                    }
                    ID_CLIENT_CAMERA_AIM_ASSIST => {
                        log::info!("[{}] Received ClientCameraAimAssist (ID 338)", addr);
                    }
                    ID_INTERACT => {
                        if let Ok(interact) = Interact::read(&mut &packet.payload[..]) {
                            log::info!("[{}] Interact: action_type={}, target_runtime_id={}", addr, interact.action_type, interact.target_entity_runtime_id);
                        }
                    }
                    ID_EMOTE_LIST => {
                        if let Ok(emote) = EmoteList::read(&mut &packet.payload[..]) {
                            log::info!("[{}] EmoteList: runtime_id={}, count={}", addr, emote.player_runtime_id, emote.emotes.len());
                        }
                    }
                    ID_PLAYER_SKIN => {
                        if let Ok(_skin) = PlayerSkin::read(&mut &packet.payload[..]) {
                            log::info!("[{}] PlayerSkin received", addr);
                        }
                    }
                    ID_SERVER_BOUND_DIAGNOSTICS => {
                        if let Ok(diag) = ServerBoundDiagnostics::read(&packet.payload) {
                            log::info!("[{}] Client Diagnostics: FPS={:.1}, ServerTickTime={:.2}ms, ClientTickTime={:.2}ms", addr, diag.avg_fps, diag.avg_server_tick_time_ms, diag.avg_client_tick_time_ms);
                        }
                    }
                    other => {
                        log::debug!("[{}] Unhandled Game Packet ID: {} (0x{:02x})", addr, other, other);
                    }
                }
            }
        }
        Err(e) => {
            log::error!("[{}] Failed to decode batch: {:?}", addr, e);
        }
    }

    Ok(())
}

async fn handle_request_network_settings(
    addr: SocketAddr,
    payload: &[u8],
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(req) = RequestNetworkSettings::read(payload) {
        log::info!(
            "[PROTOCOLO] Cliente usa protocol_version = {} (nuestro servidor habla 1001 = v1.26.31)",
            req.protocol_version
        );
        
        let settings = NetworkSettings {
            compression_threshold: 256,
            compression_algorithm: 0, // Zlib
            client_throttle: false,
            client_throttle_threshold: 0,
            client_throttle_scalar: 0.0,
        };
        
        let response_packet = GamePacket {
            id: ID_NETWORK_SETTINGS,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: settings.write()?,
        };
        
        state.send_packets(addr, cmd_tx, &[response_packet]).await?;
        state.compression_enabled = true;
        log_t!(info, COMPRESSION_ENABLED);
    } else {
        log::warn!("Failed to read RequestNetworkSettings payload");
    }
    Ok(())
}

async fn handle_login(
    addr: SocketAddr,
    payload: &[u8],
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(login) = Login::read(payload) {
        log::info!("Player Login Request: Username: {}, UUID: {}", login.username, login.uuid);
        
        if !login.identity_public_key.is_empty() {
            log_t!(info, XBOX_AUTH_REQUESTED);
            match protocol::encryption::parse_client_public_key(&login.identity_public_key) {
                Ok(client_pub) => {
                    let server_secret = p384::SecretKey::random(&mut rand::rngs::OsRng);
                    let mut salt = [0u8; 16];
                    rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut salt);
                    
                    let shared_secret = protocol::encryption::compute_shared_secret(&server_secret, &client_pub);
                    
                    match protocol::encryption::generate_handshake_jwt(&server_secret, &salt) {
                        Ok(handshake_jwt) => {
                            let crypto_state = protocol::encryption::EncryptionState::new(&shared_secret, &salt);
                            
                            let handshake_pkg = GamePacket {
                                id: ID_SERVER_TO_CLIENT_HANDSHAKE,
                                sender_subclient: 0,
                                recipient_subclient: 0,
                                payload: ServerToClientHandshake { jwt: handshake_jwt }.write()?,
                            };
                            
                            let mut temp_state = ClientState {
                                compression_enabled: state.compression_enabled,
                                encryption_state: None,
                                last_chunk_x: None,
                                last_chunk_z: None,
                                loaded_chunks: std::collections::HashSet::new(),
                                world_data_sent: false,
                                chunk_radius: 4,
                                last_pitch: 0.0,
                                last_yaw: 0.0,
                            };
                            temp_state.send_packets(addr, cmd_tx, &[handshake_pkg]).await?;
                            
                            state.encryption_state = Some(crypto_state);
                            log_t!(info, ENCRYPTION_ENABLED);
                        }
                        Err(e) => {
                            log::error!("Failed to generate handshake JWT: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to parse client public key: {:?}", e);
                }
            }
        } else {
            log_t!(info, OFFLINE_LOGIN);
            let play_status_pkg = GamePacket {
                id: ID_PLAY_STATUS,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: PlayStatus { status: 0 }.write()?,
            };
            let packs_info_pkg = GamePacket {
                id: ID_RESOURCE_PACKS_INFO,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: ResourcePacksInfo {
                    must_accept: false,
                    has_addons: false,
                    has_scripts: false,
                }.write()?,
            };
            
            state.send_packets(addr, cmd_tx, &[play_status_pkg, packs_info_pkg]).await?;
        }
    } else {
        log::warn!("Failed to parse Login packet payload");
    }
    Ok(())
}

async fn handle_client_to_server_handshake(
    addr: SocketAddr,
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    let play_status_pkg = GamePacket {
        id: ID_PLAY_STATUS,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: PlayStatus { status: 0 }.write()?,
    };
    let packs_info_pkg = GamePacket {
        id: ID_RESOURCE_PACKS_INFO,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: ResourcePacksInfo {
            must_accept: false,
            has_addons: false,
            has_scripts: false,
        }.write()?,
    };
    
    state.send_packets(addr, cmd_tx, &[play_status_pkg, packs_info_pkg]).await?;
    Ok(())
}

async fn handle_resource_pack_client_response(
    addr: SocketAddr,
    payload: &[u8],
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(resp) = ResourcePackClientResponse::read(payload) {
        log::info!("[{}] ResourcePackClientResponse: status={}", addr, resp.response_status);
        if resp.response_status == 1 || resp.response_status == 3 {
            let stack_pkg = GamePacket {
                id: ID_RESOURCE_PACK_STACK,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: ResourcePackStack {
                    must_accept: false,
                    game_version: "1.26.32".to_string(),
                }.write()?,
            };
            state.send_packets(addr, cmd_tx, &[stack_pkg]).await?;
        }
        if resp.response_status == 0 || resp.response_status == 4 {
            let jigsaw_pkg = GamePacket {
                id: ID_JIGSAW_STRUCTURE_DATA,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: JigsawStructureData::new().write()?,
            };

            let voxel_pkg = GamePacket {
                id: ID_VOXEL_SHAPES,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: VoxelShapes::new().write()?,
            };

            let mut start_game = StartGame::new();
            start_game.entity_id = 1;
            start_game.runtime_entity_id = 1;
            start_game.player_gamemode = 1;
            start_game.player_position = (0.5, -58.38, 0.5);
            start_game.pitch = 0.0;
            start_game.yaw = 0.0;
            start_game.level_name = "Flat World".to_string();

            let start_game_pkg = GamePacket {
                id: ID_START_GAME,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: start_game.write()?,
            };

            let item_registry_pkg = GamePacket {
                id: ID_ITEM_REGISTRY,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: ItemRegistry::load_from_json().write()?,
            };

            state.send_packets(addr, cmd_tx, &[
                jigsaw_pkg,
                voxel_pkg,
                start_game_pkg,
                item_registry_pkg,
            ]).await?;
        }
    } else {
        log::warn!("Failed to parse ResourcePackClientResponse payload");
    }
    Ok(())
}

async fn maybe_update_chunks(
    addr: SocketAddr,
    pos: (f32, f32, f32),
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cx = (pos.0 / 16.0).floor() as i32;
    let cz = (pos.2 / 16.0).floor() as i32;

    let should_update = match (state.last_chunk_x, state.last_chunk_z) {
        (Some(lx), Some(lz)) => (lx - cx).abs() > 0 || (lz - cz).abs() > 0,
        _ => true,
    };

    if should_update {
        state.last_chunk_x = Some(cx);
        state.last_chunk_z = Some(cz);

        let radius = state.chunk_radius;
        let r = radius as i32;

        let mut all_chunks: Vec<ChunkPos> = state.loaded_chunks.iter().map(|&(x, z)| ChunkPos { x, z }).collect();
        for dx in -r..=r {
            for dz in -r..=r {
                let chunk_x = cx + dx;
                let chunk_z = cz + dz;
                if !state.loaded_chunks.contains(&(chunk_x, chunk_z)) {
                    all_chunks.push(ChunkPos { x: chunk_x, z: chunk_z });
                }
            }
        }

        let publisher_payload = NetworkChunkPublisherUpdate {
            position: BlockPos { x: cx * 16, y: pos.1 as i32, z: cz * 16 },
            radius: radius << 4,
            server_built_chunks: all_chunks,
        }.write()?;
        let publisher_pkg = GamePacket {
            id: ID_NETWORK_CHUNK_PUBLISHER_UPDATE,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: publisher_payload,
        };
        state.send_packets(addr, cmd_tx, &[publisher_pkg]).await?;

        let chunk_payload = make_flat_chunk_payload();
        for dx in -r..=r {
            for dz in -r..=r {
                let chunk_x = cx + dx;
                let chunk_z = cz + dz;

                if state.loaded_chunks.contains(&(chunk_x, chunk_z)) {
                    continue;
                }
                state.loaded_chunks.insert((chunk_x, chunk_z));

                let chunk_payload_written = LevelChunk {
                    chunk_x,
                    chunk_z,
                    sub_chunk_count: 24,
                    payload: chunk_payload.clone(),
                }.write()?;
                let chunk_pkg = GamePacket {
                    id: ID_LEVEL_CHUNK,
                    sender_subclient: 0,
                    recipient_subclient: 0,
                    payload: chunk_payload_written,
                };
                state.send_packets(addr, cmd_tx, &[chunk_pkg]).await?;
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            }
        }
    }
    Ok(())
}

async fn handle_move_player(
    addr: SocketAddr,
    payload: &[u8],
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(mp) = MovePlayer::read(payload) {
        maybe_update_chunks(addr, (mp.position.x, mp.position.y, mp.position.z), state, cmd_tx).await?;
    }
    Ok(())
}

async fn handle_player_auth_input(
    addr: SocketAddr,
    payload: &[u8],
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(pai) = PlayerAuthInput::read(payload) {
        if (pai.pitch - state.last_pitch).abs() > 5.0 || (pai.yaw - state.last_yaw).abs() > 5.0 {
            log::info!("[{}] Camera rotation: pitch={:.2}, yaw={:.2}", addr, pai.pitch, pai.yaw);
            state.last_pitch = pai.pitch;
            state.last_yaw = pai.yaw;
        }
        maybe_update_chunks(addr, (pai.position.x, pai.position.y, pai.position.z), state, cmd_tx).await?;

        let prediction_pkg = CorrectPlayerMovePrediction {
            position: (pai.position.x, pai.position.y, pai.position.z),
            pitch: pai.pitch,
            yaw: pai.yaw,
            tick: pai.tick,
        }.write()?;
        let prediction_pkg = GamePacket {
            id: ID_CORRECT_PLAYER_MOVE_PREDICTION,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: prediction_pkg,
        };
        state.send_packets(addr, cmd_tx, &[prediction_pkg]).await?;
    } else {
        log::warn!("Failed to parse PlayerAuthInput!");
    }
    Ok(())
}

async fn handle_request_chunk_radius(
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

        // 2. Biomes + Creative + Actors + Abilities + Commands
        let biomes_base64 = BIOMES_BASE64;
        let uncompressed_biome_payload = general_purpose::STANDARD.decode(biomes_base64).unwrap();
        let compressed_payload = compress_deflate(&uncompressed_biome_payload).unwrap();
        let mut biome_payload = Vec::new();
        crate::protocol::varint::write_varu32(&mut biome_payload, compressed_payload.len() as u32);
        biome_payload.extend_from_slice(&compressed_payload);
        
        let biome_pkg = GamePacket {
            id: ID_COMPRESSED_BIOME_DEFINITIONS,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: biome_payload,
        };
        
        let creative_payload = CreativeContent::new().write();
        let creative_pkg = GamePacket {
            id: ID_CREATIVE_CONTENT,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: creative_payload,
        };

        let actor_id_payload = vec![
            0x0a, 0x00, 0x09, 0x06, 0x69, 0x64, 0x6c, 0x69, 0x73, 0x74, 0x0a, 0x02,
            0x08, 0x02, 0x69, 0x64, 0x10, 0x6d, 0x69, 0x6e, 0x65, 0x63, 0x72, 0x61,
            0x66, 0x74, 0x3a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x00, 0x00
        ]; 
        let actor_id_pkg = GamePacket {
            id: ID_AVAILABLE_ACTOR_IDENTIFIERS,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: actor_id_payload,
        };

        state.send_packets(addr, cmd_tx, &[biome_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        state.send_packets(addr, cmd_tx, &[creative_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        state.send_packets(addr, cmd_tx, &[actor_id_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // 3. Chunks
        let chunk_payload = make_flat_chunk_payload();
        let mut built_chunks = Vec::new();
        let r = radius as i32;
        for dx in -r..=r {
            for dz in -r..=r {
                state.loaded_chunks.insert((dx, dz));
                built_chunks.push(ChunkPos { x: dx, z: dz });

                let chunk_payload_written = LevelChunk {
                    chunk_x: dx,
                    chunk_z: dz,
                    sub_chunk_count: 24,
                    payload: chunk_payload.clone(),
                }.write()?;
                let chunk_pkg = GamePacket {
                    id: ID_LEVEL_CHUNK,
                    sender_subclient: 0,
                    recipient_subclient: 0,
                    payload: chunk_payload_written,
                };
                state.send_packets(addr, cmd_tx, &[chunk_pkg]).await?;
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            }
        }

        // 4. NetworkChunkPublisherUpdate
        log::info!("[{}] Sending NetworkChunkPublisherUpdate...", addr);
        let publisher_payload = NetworkChunkPublisherUpdate {
            position: BlockPos { x: 0, y: -60, z: 0 },
            radius: (radius as u32) << 4,
            server_built_chunks: built_chunks,
        }.write()?;
        let publisher_pkg = GamePacket {
            id: ID_NETWORK_CHUNK_PUBLISHER_UPDATE,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: publisher_payload,
        };
        state.send_packets(addr, cmd_tx, &[publisher_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // 5. PlayStatus PlayerSpawn (status = 3)
        let spawn_payload = PlayStatus { status: 3 }.write()?;
        let spawn_pkg = GamePacket {
            id: ID_PLAY_STATUS,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: spawn_payload,
        };
        state.send_packets(addr, cmd_tx, &[spawn_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // 6. LevelEvent StopWaiting to clear loading screen
        let stop_waiting_pkg_1 = packets::create_level_event_pkg(14, 0);
        let stop_waiting_pkg_2 = packets::create_level_event_pkg(16398, 0); // 0x4000 + 14 (StopWaiting)
        let stop_waiting_pkg_3 = packets::create_level_event_pkg(9501, 0);
        let stop_waiting_pkg_4 = packets::create_level_event_pkg(25885, 0); // 0x4000 + 9501
        state.send_packets(addr, cmd_tx, &[stop_waiting_pkg_1, stop_waiting_pkg_2, stop_waiting_pkg_3, stop_waiting_pkg_4]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    Ok(())
}

pub const BIOMES_BASE64: &str = "CgAKDWJhbWJvb19qdW5nbGUFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoTYmFtYm9vX2p1bmdsZV9oaWxscwUIZG93bmZhbGZmZmY/BQt0ZW1wZXJhdHVyZTMzcz8ACgViZWFjaAUIZG93bmZhbGzNzMw+BQt0ZW1wZXJhdHVyZc3MTD8ACgxiaXJjaF9mb3Jlc3QFCGRvd25mYWxsmpkZPwULdGVtcGVyYXR1cmWamRk/AAoSYmlyY2hfZm9yZXN0X2hpbGxzBQhkb3duZmFsbJqZGT8FC3RlbXBlcmF0dXJlmpkZPwAKGmJpcmNoX2ZvcmVzdF9oaWxsc19tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKFGJpcmNoX2ZvcmVzdF9tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKCmNvbGRfYmVhY2gFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw9AAoKY29sZF9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgpjb2xkX3RhaWdhBQhkb3duZmFsbM3MzD4FC3RlbXBlcmF0dXJlAAAAvwAKEGNvbGRfdGFpZ2FfaGlsbHMFCGRvd25mYWxszczMPgULdGVtcGVyYXR1cmUAAAC/AAoSY29sZF90YWlnYV9tdXRhdGVkBQhkb3duZmFsbM3MzD4FC3RlbXBlcmF0dXJlAAAAvwAKD2RlZXBfY29sZF9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8AChFkZWVwX2Zyb3plbl9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAAChNkZWVwX2x1a2V3YXJtX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKCmRlZXBfb2NlYW4FCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAA/AAoPZGVlcF93YXJtX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKBmRlc2VydAUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAACgxkZXNlcnRfaGlsbHMFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoOZGVzZXJ0X211dGF0ZWQFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoNZXh0cmVtZV9oaWxscwUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4AChJleHRyZW1lX2hpbGxzX2VkZ2UFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw+AAoVZXh0cmVtZV9oaWxsc19tdXRhdGVkBQhkb3duZmFsbJqZmT4FC3RlbXBlcmF0dXJlzcxMPgAKGGV4dHJlbWVfaGlsbHNfcGx1c190cmVlcwUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4ACiBleHRyZW1lX2hpbGxzX3BsdXNfdHJlZXNfbXV0YXRlZAUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4ACg1mbG93ZXJfZm9yZXN0BQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKBmZvcmVzdAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZTMzMz8ACgxmb3Jlc3RfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUzMzM/AAoMZnJvemVuX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAAAAKDGZyb3plbl9yaXZlcgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAACgRoZWxsBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlAAAAQAAKDWljZV9tb3VudGFpbnMFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAAAAAoKaWNlX3BsYWlucwUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAAChFpY2VfcGxhaW5zX3NwaWtlcwUIZG93bmZhbGwAAIA/BQt0ZW1wZXJhdHVyZQAAAAAACgZqdW5nbGUFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoLanVuZ2xlX2VkZ2UFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUzM3M/AAoTanVuZ2xlX2VkZ2UFBGlkZmZmMzM/AAoTbGVnYWN5X2Zyb3plbl9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAACg5sdWtld2FybV9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgptZWdhX3RhaWdhBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlmpmZPgAKEG1lZ2FfdGFpZ2FfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmWamZk+AAoEbWVzYQUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAACgptZXNhX2JyeWNlBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlAAAAQAAKDG1lc2FfcGxhdGVhdQUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAAChRtZXNhX3BsYXRlYXVfbXV0YXRlZAUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAAChJtZXNhX3BsYXRlYXVfc3RvbmUFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoabWVzYV9wbGF0ZWF1X3N0b25lX211dGF0ZWQFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoPbXVzaHJvb21faXNsYW5kBQhkb3duZmFsbAAAgD8FC3RlbXBlcmF0dXJlZmZmPwAKFW11c2hyb29tX2lzbGFuZF9zaG9yZQUIZG93bmZhbGwAAIA/BQt0ZW1wZXJhdHVyZWZmZj8ACgVvY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgZwbGFpbnMFCGRvd25mYWxszczMPgULdGVtcGVyYXR1cmXNzEw/AAobcmVkd29vZF90YWlnYV9oaWxsc19tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlmpmZPgAKFXJlZHdvb2RfdGFpZ2FfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZQAAgD4ACgVyaXZlcgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACg1yb29mZWRfZm9yZXN0BQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKFXJvb2ZlZF9mb3Jlc3RfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZTMzMz8ACgdzYXZhbm5hBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlmpmZPwAKD3NhdmFubmFfbXV0YXRlZAUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZc3MjD8ACg9zYXZhbm5hX3BsYXRlYXUFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAIA/AAoXc2F2YW5uYV9wbGF0ZWF1X211dGF0ZWQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAIA/AAoLc3RvbmVfYmVhY2gFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw+AAoQc3VuZmxvd2VyX3BsYWlucwUIZG93bmZhbGzNzMw+BQt0ZW1wZXJhdHVyZc3MTD8ACglzd2FtcGxhbmQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmXNzEw/AAoRc3dhbXBsYW5kX211dGF0ZWQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmXNzEw/AAoFdGFpZ2EFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUAAIA+AAoLdGFpZ2FfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUAAIA+AAoNdGFpZ2FfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZQAAgD4ACgd0aGVfZW5kBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKCndhcm1fb2NlYW4FCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAA/AAA=";
