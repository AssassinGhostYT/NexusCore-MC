use std::net::SocketAddr;
use tokio::sync::mpsc;
use crate::raknet::server::RakNetCommand;
use crate::network::mcpe::protocol::packets::*;
use crate::network::mcpe::types::*;
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
            Ok(body) => {
                let decrypted_body: Vec<u8> = body;
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

    let packets_res = decode_batch(&payload, state.compression_enabled);
    match packets_res {
        Ok(pkt_list) => {
            for packet in pkt_list {
                let hex: Vec<String> = packet.payload.iter().take(16).map(|b| format!("{:02x}", b)).collect();
                log::info!(
                    "[{}] RECV INBOUND GAME PACKET: id={} (0x{:02x}), len={}, hex_prefix=[{}]",
                    addr,
                    packet.id,
                    packet.id,
                    packet.payload.len(),
                    hex.join(" ")
                );
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
                        let hex: Vec<String> = packet.payload.iter().map(|b| format!("{:02x}", b)).collect();
                        log::error!("[{}] 🚨 PACKET VIOLATION WARNING RECEIVED FROM CLIENT! Raw hex=[{}]", addr, hex.join(" "));
                        match PacketViolationWarning::read(&packet.payload) {
                            Ok(violation) => {
                                log::error!(
                                    "[{}] 🚨 VIOLATION DETAILS: packet_id={} (0x{:02x}), severity={}, context='{}'",
                                    addr, violation.packet_id, violation.packet_id, violation.severity, violation.context
                                );
                            }
                            Err(err) => {
                                log::error!("[{}] 🚨 Failed to parse PacketViolationWarning body: {:?}", addr, err);
                            }
                        }
                    }
                    ID_SET_LOCAL_PLAYER_AS_INITIALISED => {
                        if let Ok(init_packet) = SetLocalPlayerAsInitialized::read(&packet.payload) {
                            let rid = init_packet.entity_runtime_id;
                            log::info!(
                                "[{}] Received SetLocalPlayerAsInitialised (ID 113) - Player Runtime ID: {} - PLAYER SPAWN COMPLETED FULLY!", 
                                addr, 
                                rid
                            );
                        }
                    }
                    ID_SERVER_BOUND_LOADING_SCREEN => {
                        if let Ok(loading) = ServerBoundLoadingScreen::read(&packet.payload) {
                            log::info!(
                                "[{}] Received ServerBoundLoadingScreen: type={:?}, screen_id={:?} — SilentDiscard",
                                addr,
                                loading.loading_screen_type,
                                loading.screen_id
                            );
                            // SilentDiscard per PocketMine PreSpawnPacketHandler / SpawnResponsePacketHandler.
                            // The client sends:
                            //   1) StartLoadingScreen (type=1) — server ignores
                            //   2) StopLoadingScreen  (type=2) — server ignores
                            //   3) SetLocalPlayerAsInitialized (ID 113) — server transitions to in-game
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
                        let hex: Vec<String> = packet.payload.iter().map(|b| format!("{:02x}", b)).collect();
                        log::info!("[{}] Received Game Packet ID: {} (0x{:02x}), length={}, payload=[{}]", addr, other, other, packet.payload.len(), hex.join(" "));
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
        state.username = login.username.clone();
        let mut uuid_bytes = [0u8; 16];
        let bytes = login.uuid.as_bytes();
        let len = bytes.len().min(16);
        uuid_bytes[..len].copy_from_slice(&bytes[..len]);
        state.uuid = uuid_bytes;
        log::info!("Player Login Request: Username: {}, UUID: {}", login.username, login.uuid);
        
        if !login.identity_public_key.is_empty() {
            log::info!("[Login] Xbox Live player: {} (UUID: {})", login.username, login.uuid);
        } else {
            log_t!(info, OFFLINE_LOGIN);
        }

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
            let rid = 1u64;

            // StartGame — player spawns at y=64 (all-air world, safe floating position)
            let mut start_game = StartGame::new();
            start_game.entity_id = 1;
            start_game.runtime_entity_id = 1;
            start_game.player_gamemode = 1; // creative
            start_game.player_position = (0.5, 64.0, 0.5);
            start_game.pitch = 0.0;
            start_game.yaw = 0.0;
            start_game.level_name = "NexusCore".to_string();
            start_game.settings.game_type = 1; // creative

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

            let player_list = PlayerListAdd {
                entries: vec![PlayerListAddEntry {
                    uuid: state.uuid,
                    entity_unique_id: 1,
                    username: state.username.clone(),
                }],
            };
            let player_list_pkg = GamePacket {
                id: ID_PLAYER_LIST,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: player_list.write()?,
            };

            let play_status_ok_pkg = GamePacket {
                id: ID_PLAY_STATUS,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: PlayStatus { status: 0 }.write()?,
            };

            // Packet order matches PocketMine's pre-spawn sequence exactly.
            // Removed: JigsawStructureData, VoxelShapes, SetPlayerGameType,
            //          SetTime, SetDifficulty, SetCommandsEnabled, MovePlayer-teleport.
            let abilities_pkg  = packets::create_abilities_pkg(rid as i64);
            let adventure_pkg  = packets::create_adventure_pkg();
            let attr_pkg       = packets::create_update_attributes_pkg(rid);
            let actor_data_pkg = packets::create_set_actor_data_pkg(rid);
            let commands_pkg   = packets::create_available_commands_pkg();
            let crafting_pkg   = packets::create_crafting_data_pkg();

            let biome_pkg = GamePacket {
                id: ID_BIOME_DEFINITION_LIST,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: general_purpose::STANDARD.decode(BIOMES_BASE64).unwrap(),
            };

            let actor_id_payload = vec![
                0x0a, 0x00, 0x09, 0x06, 0x00, b'i', b'd', b'l', b'i', b's', b't',
                0x0a, 0x01, 0x00, 0x00, 0x00, 0x08, 0x02, 0x00, b'i', b'd',
                0x10, 0x00, b'm', b'i', b'n', b'e', b'c', b'r', b'a', b'f', b't', b':', b'p', b'l', b'a', b'y', b'e', b'r',
                0x00, 0x00,
            ];
            let actor_id_pkg = GamePacket {
                id: ID_AVAILABLE_ACTOR_IDENTIFIERS,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: actor_id_payload,
            };

            let creative_pkg = GamePacket {
                id: ID_CREATIVE_CONTENT,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: CreativeContent::new().write(),
            };

            // Exact packet sequence matching PocketMine PreSpawnPacketHandler:
            // 1. StartGame
            // 2. ItemRegistry
            // 3. AvailableActorIdentifiers
            // 4. BiomeDefinitionList
            // 5. UpdateAttributes
            // 6. AvailableCommands
            // 7. UpdateAbilities + UpdateAdventureSettings
            // 8. SetActorData
            // 9. InventoryContent (via inv_setup_pkgs)
            // 10. CreativeContent
            // 11. CraftingData
            // 12. PlayerList
            let mut status_4_batch = vec![
                start_game_pkg,        // StartGame
                item_registry_pkg,     // ItemRegistry (sent right after StartGame in PocketMine)
                actor_id_pkg,          // AvailableActorIdentifiers
                biome_pkg,             // BiomeDefinitionList
                attr_pkg,              // UpdateAttributes
                commands_pkg,          // AvailableCommands
                abilities_pkg,         // UpdateAbilities
                adventure_pkg,         // UpdateAdventureSettings
                actor_data_pkg,        // SetActorData
            ];

            let inv_setup_pkgs = packets::get_inventory_setup_packets();
            status_4_batch.extend(inv_setup_pkgs);

            status_4_batch.push(creative_pkg); // CreativeContent (sent after inventory in PocketMine)
            status_4_batch.push(crafting_pkg); // CraftingData
            status_4_batch.push(player_list_pkg); // PlayerList

            state.send_packets(addr, cmd_tx, &status_4_batch).await?;
            log::info!("[{}] Sent PlayStatus(0) LoginSuccess + full pre-spawn batch!", addr);
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
            server_built_chunks: Vec::new(),
        }.write()?;
        let publisher_pkg = GamePacket {
            id: ID_NETWORK_CHUNK_PUBLISHER_UPDATE,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: publisher_payload,
        };
        state.send_packets(addr, cmd_tx, &[publisher_pkg]).await?;

        let chunk_payload = make_limited_chunk_payload();
        let mut new_chunk_pkgs = Vec::new();
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
                    sub_chunk_count: SUB_CHUNK_REQUEST_MODE_LIMITED,
                    highest_sub_chunk: 0,
                    payload: chunk_payload.clone(),
                }.write()?;
                let chunk_pkg = GamePacket {
                    id: ID_LEVEL_CHUNK,
                    sender_subclient: 0,
                    recipient_subclient: 0,
                    payload: chunk_payload_written,
                };
                new_chunk_pkgs.push(chunk_pkg);
                if new_chunk_pkgs.len() >= 4 {
                    state.send_packets(addr, cmd_tx, &new_chunk_pkgs).await?;
                    new_chunk_pkgs.clear();
                    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                }
            }
        }
        if !new_chunk_pkgs.is_empty() {
            state.send_packets(addr, cmd_tx, &new_chunk_pkgs).await?;
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
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
        log::info!(
            "[{}] PlayerAuthInput: pos=({:.2}, {:.2}, {:.2}), pitch={:.2}, yaw={:.2}",
            addr, pai.position.x, pai.position.y, pai.position.z, pai.pitch, pai.yaw
        );
        state.last_pitch = pai.pitch;
        state.last_yaw = pai.yaw;
        maybe_update_chunks(addr, (pai.position.x, pai.position.y, pai.position.z), state, cmd_tx).await?;
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

        // ── Step 1: ChunkRadiusUpdated ─────────────────────────────────────
        log::info!("[{}] Sending ChunkRadiusUpdated (radius={})...", addr, radius);
        let radius_pkg = GamePacket {
            id: ID_CHUNK_RADIUS_UPDATED,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: ChunkRadiusUpdated { radius }.write()?,
        };
        state.send_packets(addr, cmd_tx, &[radius_pkg]).await?;

        // NOTE: BiomeDefinitionList already sent in the pre-spawn batch with real NBT data.
        // We do NOT send a second (empty/malformed) one here — it would confuse the client.

        // ── Step 3: PlayStatus(PlayerSpawn) ───────────────────────────────
        // GopherTunnel sends PlayStatus BEFORE chunks.
        if !state.world_data_sent {
            state.world_data_sent = true;
            let spawn_pkg = GamePacket {
                id: ID_PLAY_STATUS,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: PlayStatus { status: 3 }.write()?,
            };
            state.send_packets(addr, cmd_tx, &[spawn_pkg]).await?;
            log::info!("[{}] Sent PlayStatus(3) PlayerSpawn!", addr);
        }

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        // ── Step 4: LevelChunks ────────────────────────────────────────────
        state.last_chunk_x = Some(0);
        state.last_chunk_z = Some(0);

        let chunk_payload = make_limited_chunk_payload();
        let mut built_chunks = Vec::new();
        let r = (radius as i32).min(3);
        let mut chunk_pkgs = Vec::new();
        for dx in -r..=r {
            for dz in -r..=r {
                state.loaded_chunks.insert((dx, dz));
                built_chunks.push(ChunkPos { x: dx, z: dz });

                let chunk_pkg = GamePacket {
                    id: ID_LEVEL_CHUNK,
                    sender_subclient: 0,
                    recipient_subclient: 0,
                    payload: LevelChunk {
                        chunk_x: dx,
                        chunk_z: dz,
                        // SubChunkRequestModeLimited: client uses SubChunkRequest for block data.
                        // HighestSubChunk=0 means all-air; client won't request sub-chunks.
                        sub_chunk_count: SUB_CHUNK_REQUEST_MODE_LIMITED,
                        highest_sub_chunk: 0,
                        payload: chunk_payload.clone(),
                    }.write()?,
                };
                chunk_pkgs.push(chunk_pkg);
                if chunk_pkgs.len() >= 16 {
                    state.send_packets(addr, cmd_tx, &chunk_pkgs).await?;
                    chunk_pkgs.clear();
                    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                }
            }
        }
        if !chunk_pkgs.is_empty() {
            state.send_packets(addr, cmd_tx, &chunk_pkgs).await?;
        }
        log::info!("[{}] Sent {} chunks.", addr, built_chunks.len());

        // ── Step 5: NetworkChunkPublisherUpdate ────────────────────────────
        log::info!("[{}] Sending NetworkChunkPublisherUpdate...", addr);
        let publisher_pkg = GamePacket {
            id: ID_NETWORK_CHUNK_PUBLISHER_UPDATE,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: NetworkChunkPublisherUpdate {
                position: BlockPos { x: 0, y: 64, z: 0 },
                radius: (r as u32) << 4,
                server_built_chunks: built_chunks,
            }.write()?,
        };
        state.send_packets(addr, cmd_tx, &[publisher_pkg]).await?;
    }
    Ok(())
}



pub const BIOMES_BASE64: &str = "CgAKDWJhbWJvb19qdW5nbGUFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoTYmFtYm9vX2p1bmdsZV9oaWxscwUIZG93bmZhbGZmZmY/BQt0ZW1wZXJhdHVyZTMzcz8ACgViZWFjaAUIZG93bmZhbGzNzMw+BQt0ZW1wZXJhdHVyZc3MTD8ACgxiaXJjaF9mb3Jlc3QFCGRvd25mYWxsmpkZPwULdGVtcGVyYXR1cmWamRk/AAoSYmlyY2hfZm9yZXN0X2hpbGxzBQhkb3duZmFsbJqZGT8FC3RlbXBlcmF0dXJlmpkZPwAKGmJpcmNoX2ZvcmVzdF9oaWxsc19tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKFGJpcmNoX2ZvcmVzdF9tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKCmNvbGRfYmVhY2gFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw9AAoKY29sZF9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgpjb2xkX3RhaWdhBQhkb3duZmFsbM3MzD4FC3RlbXBlcmF0dXJlAAAAvwAKEGNvbGRfdGFpZ2FfaGlsbHMFCGRvd25mYWxszczMPgULdGVtcGVyYXR1cmUAAAC/AAoSY29sZF90YWlnYV9tdXRhdGVkBQhkb3duZmFsbM3MzD4FC3RlbXBlcmF0dXJlAAAAvwAKD2RlZXBfY29sZF9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8AChFkZWVwX2Zyb3plbl9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAAChNkZWVwX2x1a2V3YXJtX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKCmRlZXBfb2NlYW4FCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAA/AAoPZGVlcF93YXJtX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKBmRlc2VydAUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAACgxkZXNlcnRfaGlsbHMFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoOZGVzZXJ0X211dGF0ZWQFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoNZXh0cmVtZV9oaWxscwUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4AChJleHRyZW1lX2hpbGxzX2VkZ2UFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw+AAoVZXh0cmVtZV9oaWxsc19tdXRhdGVkBQhkb3duZmFsbJqZmT4FC3RlbXBlcmF0dXJlzcxMPgAKGGV4dHJlbWVfaGlsbHNfcGx1c190cmVlcwUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4ACiBleHRyZW1lX2hpbGxzX3BsdXNfdHJlZXNfbXV0YXRlZAUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4ACg1mbG93ZXJfZm9yZXN0BQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKBmZvcmVzdAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZTMzMz8ACgxmb3Jlc3RfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUzMzM/AAoMZnJvemVuX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAAAAKDGZyb3plbl9yaXZlcgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAACgRoZWxsBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlAAAAQAAKDWljZV9tb3VudGFpbnMFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAAAAAoKaWNlX3BsYWlucwUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAAChFpY2VfcGxhaW5zX3NwaWtlcwUIZG93bmZhbGwAAIA/BQt0ZW1wZXJhdHVyZQAAAAAACgZqdW5nbGUFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoLanVuZ2xlX2VkZ2UFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUzM3M/AAoTanVuZ2xlX2VkZ2UFBGlkZmZmMzM/AAoTbGVnYWN5X2Zyb3plbl9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAACg5sdWtld2FybV9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgptZWdhX3RhaWdhBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlmpmZPgAKEG1lZ2FfdGFpZ2FfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmWamZk+AAoEbWVzYQUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAACgptZXNhX2JyeWNlBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlAAAAQAAKDG1lc2FfcGxhdGVhdQUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAAChRtZXNhX3BsYXRlYXVfbXV0YXRlZAUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAAChJtZXNhX3BsYXRlYXVfc3RvbmUFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoabWVzYV9wbGF0ZWF1X3N0b25lX211dGF0ZWQFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoPbXVzaHJvb21faXNsYW5kBQhkb3duZmFsbAAAgD8FC3RlbXBlcmF0dXJlZmZmPwAKFW11c2hyb29tX2lzbGFuZF9zaG9yZQUIZG93bmZhbGwAAIA/BQt0ZW1wZXJhdHVyZWZmZj8ACgVvY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgZwbGFpbnMFCGRvd25mYWxszczMPgULdGVtcGVyYXR1cmXNzEw/AAobcmVkd29vZF90YWlnYV9oaWxsc19tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlmpmZPgAKFXJlZHdvb2RfdGFpZ2FfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZQAAgD4ACgVyaXZlcgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACg1yb29mZWRfZm9yZXN0BQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKFXJvb2ZlZF9mb3Jlc3RfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZTMzMz8ACgdzYXZhbm5hBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlmpmZPwAKD3NhdmFubmFfbXV0YXRlZAUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZc3MjD8ACg9zYXZhbm5hX3BsYXRlYXUFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAIA/AAoXc2F2YW5uYV9wbGF0ZWF1X211dGF0ZWQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAIA/AAoLc3RvbmVfYmVhY2gFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw+AAoQc3VuZmxvd2VyX3BsYWlucwUIZG93bmZhbGzNzMw+BQt0ZW1wZXJhdHVyZc3MTD8ACglzd2FtcGxhbmQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmXNzEw/AAoRc3dhbXBsYW5kX211dGF0ZWQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmXNzEw/AAoFdGFpZ2EFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUAAIA+AAoLdGFpZ2FfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUAAIA+AAoNdGFpZ2FfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZQAAgD4ACgd0aGVfZW5kBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKCndhcm1fb2NlYW4FCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAA/AAA=";
