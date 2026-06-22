use std::net::SocketAddr;
use tokio::sync::mpsc;
use base64::{Engine as _, engine::general_purpose};
use crate::raknet::server::RakNetCommand;
use crate::protocol;
use crate::protocol::packet::*;
use crate::server::client::ClientState;

// Protocol constants
const ID_BIOME_DEFINITION_LIST: u32 = 122;
const ID_COMPRESSED_BIOME_DEFINITIONS: u32 = 301;
const ID_CREATIVE_CONTENT: u32 = 145;
const ID_CLIENT_CACHE_STATUS: u32 = 129;
const ID_SET_LOCAL_PLAYER_AS_INITIALISED: u32 = 113;
const ID_SERVER_BOUND_LOADING_SCREEN: u32 = 312;

pub async fn handle_packet(
    addr: SocketAddr,
    mut payload: Vec<u8>,
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    if payload.is_empty() {
        return Ok(());
    }

    if payload[0] == ID_GAME_PACKET {
        if let Some(ref mut crypto) = state.encryption_state {
            match crypto.decrypt_packet(&mut payload[1..]) {
                Ok(decrypted_body) => {
                    payload.truncate(1);
                    payload.extend_from_slice(&decrypted_body);
                }
                Err(e) => {
                    log::error!("Failed to decrypt and verify game packet batch from {}: {}", addr, e);
                    return Ok(());
                }
            }
        }

        let compressed = state.compression_enabled;
        match decode_batch(&payload, compressed) {
            Ok(packets) => {
                for packet in packets {
                    log::info!(
                        "MCPE Game Packet from {}: ID={} (0x{:02x}), Length={}",
                        addr,
                        packet.id,
                        packet.id,
                        packet.payload.len()
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
                            handle_move_player(addr, &packet.payload);
                        }
                        ID_PLAYER_AUTH_INPUT => {
                            handle_player_auth_input(addr, &packet.payload);
                        }
                        ID_REQUEST_CHUNK_RADIUS => {
                            handle_request_chunk_radius(addr, &packet.payload, state, cmd_tx).await?;
                        }
                        ID_CLIENT_CACHE_STATUS => {
                            log::info!("[{}] Received ClientCacheStatus (ID 129), ignoring...", addr);
                        }
                        ID_SET_LOCAL_PLAYER_AS_INITIALISED => {
                            log::info!("[{}] Received SetLocalPlayerAsInitialised (ID 113) - PLAYER SPAWN COMPLETED!", addr);
                        }
                        ID_SERVER_BOUND_LOADING_SCREEN => {
                            let screen_type = if !packet.payload.is_empty() { packet.payload[0] } else { 0 };
                            log::info!("[{}] Received ServerBoundLoadingScreen (ID 312): Type = {}", addr, screen_type);
                        }
                        _ => {
                            log::info!("Unhandled Minecraft Bedrock packet ID: {}", packet.id);
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to decode game packet batch from {}: {:?}. Payload hex: {:02x?}", addr, e, &payload[..std::cmp::min(payload.len(), 32)]);
            }
        }
    } else {
        log::warn!("Received non-game packet ID 0x{:02x} from {}", payload[0], addr);
    }

    Ok(())
}

async fn handle_request_network_settings(
    addr: SocketAddr,
    payload: &[u8],
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(req) = RequestNetworkSettings::read(payload) {
        log::info!(
            "Received RequestNetworkSettings: Protocol Version = {}",
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
            payload: settings.write(),
        };
        
        // The NetworkSettings packet itself is sent UNCOMPRESSED!
        state.send_packets(addr, cmd_tx, &[response_packet]).await?;
        
        // Now enable compression for all future packets
        state.compression_enabled = true;
        log::info!("Compression enabled (Zlib Deflate) for client {}", addr);
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
    if let Some(login) = Login::read(payload) {
        log::info!("Player Login Request:");
        log::info!("  Username: {}", login.username);
        log::info!("  UUID:     {}", login.uuid);
        log::info!("  XUID:     {}", login.xuid);
        log::info!("  Version:  {}", login.game_version);
        log::info!("  DeviceOS: {}", login.device_os);
        
        if !login.identity_public_key.is_empty() {
            log::info!("Client requested Xbox Live authentication. Commencing ECDH handshake...");
            // 1. Parse client public key
            match protocol::encryption::parse_client_public_key(&login.identity_public_key) {
                Ok(client_pub) => {
                    // 2. Generate server ephemeral keypair and salt
                    let server_secret = p384::SecretKey::random(&mut rand::rngs::OsRng);
                    let mut salt = [0u8; 16];
                    rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut salt);
                    
                    // 3. Compute shared secret
                    let shared_secret = protocol::encryption::compute_shared_secret(&server_secret, &client_pub);
                    
                    // 4. Generate signed handshake JWT
                    match protocol::encryption::generate_handshake_jwt(&server_secret, &salt) {
                        Ok(handshake_jwt) => {
                            log::info!("Generated Handshake JWT. Sending ServerToClientHandshake...");
                            
                            // 5. Initialize encryption state
                            let crypto_state = protocol::encryption::EncryptionState::new(&shared_secret, &salt);
                            
                            let handshake_pkg = GamePacket {
                                id: ID_SERVER_TO_CLIENT_HANDSHAKE,
                                sender_subclient: 0,
                                recipient_subclient: 0,
                                payload: ServerToClientHandshake { jwt: handshake_jwt }.write(),
                            };
                            
                            // Send unencrypted handshake
                            let temp_encryption = None;
                            let mut temp_state = ClientState {
                                compression_enabled: state.compression_enabled,
                                encryption_state: temp_encryption,
                            };
                            temp_state.send_packets(addr, cmd_tx, &[handshake_pkg]).await?;
                            
                            // Enable encryption state on our real state
                            state.encryption_state = Some(crypto_state);
                            log::info!("Encryption enabled for client {}", addr);
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
            log::info!("Local/offline login. Bypassing encryption handshake.");
            let play_status_pkg = GamePacket {
                id: ID_PLAY_STATUS,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: PlayStatus { status: 0 }.write(), // LoginSuccess
            };
            let packs_info_pkg = GamePacket {
                id: ID_RESOURCE_PACKS_INFO,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: ResourcePacksInfo {
                    must_accept: false,
                    has_addons: false,
                    has_scripts: false,
                }.write(),
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
    log::info!("[{}] Received ClientToServerHandshake (ID 4). Handshake complete!", addr);
    
    // Now send PlayStatus(LoginSuccess) and ResourcePacksInfo (encrypted)
    log::info!("Sending PlayStatus(LoginSuccess) and ResourcePacksInfo (encrypted)...");
    let play_status_pkg = GamePacket {
        id: ID_PLAY_STATUS,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: PlayStatus { status: 0 }.write(), // LoginSuccess
    };
    let packs_info_pkg = GamePacket {
        id: ID_RESOURCE_PACKS_INFO,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: ResourcePacksInfo {
            must_accept: false,
            has_addons: false,
            has_scripts: false,
        }.write(),
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
    if let Some(resp) = ResourcePackClientResponse::read(payload) {
        log::info!("Received ResourcePackClientResponse: status = {}", resp.response_status);
        if resp.response_status == 1 || resp.response_status == 3 {
            log::info!("Client ready for stack (status={}). Sending ResourcePackStack...", resp.response_status);
            let stack_pkg = GamePacket {
                id: ID_RESOURCE_PACK_STACK,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: ResourcePackStack {
                    must_accept: false,
                    game_version: "".to_string(),
                }.write(),
            };
            
            state.send_packets(addr, cmd_tx, &[stack_pkg]).await?;
        } else if resp.response_status == 4 {
            log::info!("Client resource pack loading completed. Sending VoxelShapes and StartGame...");
            
            let start_game_payload = StartGame {
                entity_id: 1,
                runtime_entity_id: 1,
                player_gamemode: 1, // Creative
                player_position: (0.0, -62.0, 0.0),
                pitch: 0.0,
                yaw: 0.0,
                seed: 12345,
                spawn_position: (0, -63, 0),
                level_name: "Flat World".to_string(),
            }.write();

            let voxel_shapes_payload = VoxelShapes::new().write();
            let voxel_shapes_pkg = GamePacket {
                id: ID_VOXEL_SHAPES,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: voxel_shapes_payload,
            };

            let start_game_pkg = GamePacket {
                id: ID_START_GAME,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: start_game_payload,
            };
            
            log::info!("Sending VoxelShapes, StartGame, ItemRegistry, and AvailableActorIdentifiers in a single batch...");
            let item_component_pkg = GamePacket {
                id: ID_ITEM_REGISTRY,
                sender_subclient: 0,
                recipient_subclient: 0,
                payload: ItemRegistry::new().write()
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
            state.send_packets(
                addr,
                cmd_tx,
                &[voxel_shapes_pkg, start_game_pkg, item_component_pkg, actor_id_pkg]
            ).await?;
        }
    } else {
        log::warn!("Failed to parse ResourcePackClientResponse payload");
    }
    Ok(())
}

fn handle_move_player(addr: SocketAddr, payload: &[u8]) {
    if let Some(mp) = MovePlayer::read(payload) {
        log::info!(
            "[{}] MovePlayer: RuntimeID={}, Position=({:.2}, {:.2}, {:.2}), Pitch={:.2}, Yaw={:.2}",
            addr,
            mp.runtime_entity_id,
            mp.position.0,
            mp.position.1,
            mp.position.2,
            mp.pitch,
            mp.yaw
        );
    }
}

fn handle_player_auth_input(addr: SocketAddr, payload: &[u8]) {
    if let Some(pai) = PlayerAuthInput::read(payload) {
        log::info!(
            "[{}] PlayerAuthInput: Position=({:.2}, {:.2}, {:.2}), Pitch={:.2}, Yaw={:.2}, Flags=0x{:X}",
            addr,
            pai.position.0,
            pai.position.1,
            pai.position.2,
            pai.pitch,
            pai.yaw,
            pai.input_flags
        );
    }
}

async fn handle_request_chunk_radius(
    addr: SocketAddr,
    payload: &[u8],
    state: &mut ClientState,
    cmd_tx: &mpsc::Sender<RakNetCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(req) = RequestChunkRadius::read(payload) {
        log::info!("[{}] Client requested chunk radius: {}", addr, req.radius);
        
        let response_radius = 2;
        
        // 1. Send ChunkRadiusUpdated
        let radius_payload = ChunkRadiusUpdated { radius: response_radius }.write();
        let radius_pkg = GamePacket {
            id: ID_CHUNK_RADIUS_UPDATED,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: radius_payload,
        };
        
        // 2. Send CompressedBiomeDefinitions (decoding and compressing base64 definitions from gophertunnel)
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
        
        // 3. Send CreativeContent (we send a single byte 0 indicating 0 elements for groups and items)
        let creative_payload = vec![0x00, 0x00];
        let creative_pkg = GamePacket {
            id: ID_CREATIVE_CONTENT,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: creative_payload,
        };
        
        // 4. Send NetworkChunkPublisherUpdate
        let publisher_payload = NetworkChunkPublisherUpdate {
            position: (0, 64, 0),
            radius: (response_radius as u32) << 4, // 32 blocks
        }.write();
        let publisher_pkg = GamePacket {
            id: ID_NETWORK_CHUNK_PUBLISHER_UPDATE,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: publisher_payload,
        };
        
        // 5. Send AvailableActorIdentifiers
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
        
        // Send first batch of handshake packets
        state.send_packets(addr, cmd_tx, &[radius_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        state.send_packets(addr, cmd_tx, &[biome_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        state.send_packets(addr, cmd_tx, &[creative_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        state.send_packets(addr, cmd_tx, &[publisher_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        state.send_packets(addr, cmd_tx, &[actor_id_pkg]).await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // Send Chunks individually (radius 2 -> 25 chunks)
        let chunk_payload = make_flat_chunk_payload();
        for cx in -2..=2 {
            for cz in -2..=2 {
                let chunk_payload_written = LevelChunk {
                    chunk_x: cx,
                    chunk_z: cz,
                    sub_chunk_count: 24,
                    payload: chunk_payload.clone(),
                }.write();

                let chunk_pkg = GamePacket {
                    id: ID_LEVEL_CHUNK,
                    sender_subclient: 0,
                    recipient_subclient: 0,
                    payload: chunk_payload_written,
                };
                
                state.send_packets(addr, cmd_tx, &[chunk_pkg]).await?;
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        }

        // Send PlayStatus(PlayerSpawn = 3) AFTER all chunks are sent
        let spawn_payload = PlayStatus { status: 3 }.write();
        let spawn_pkg = GamePacket {
            id: ID_PLAY_STATUS,
            sender_subclient: 0,
            recipient_subclient: 0,
            payload: spawn_payload,
        };
        state.send_packets(addr, cmd_tx, &[spawn_pkg]).await?;
    }
    Ok(())
}

pub const BIOMES_BASE64: &str = "CgAKDWJhbWJvb19qdW5nbGUFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoTYmFtYm9vX2p1bmdsZV9oaWxscwUIZG93bmZhbGxmZmY/BQt0ZW1wZXJhdHVyZTMzcz8ACgViZWFjaAUIZG93bmZhbGzNzMw+BQt0ZW1wZXJhdHVyZc3MTD8ACgxiaXJjaF9mb3Jlc3QFCGRvd25mYWxsmpkZPwULdGVtcGVyYXR1cmWamRk/AAoSYmlyY2hfZm9yZXN0X2hpbGxzBQhkb3duZmFsbJqZGT8FC3RlbXBlcmF0dXJlmpkZPwAKGmJpcmNoX2ZvcmVzdF9oaWxsc19tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKFGJpcmNoX2ZvcmVzdF9tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKCmNvbGRfYmVhY2gFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw9AAoKY29sZF9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgpjb2xkX3RhaWdhBQhkb3duZmFsbM3MzD4FC3RlbXBlcmF0dXJlAAAAvwAKEGNvbGRfdGFpZ2FfaGlsbHMFCGRvd25mYWxszczMPgULdGVtcGVyYXR1cmUAAAC/AAoSY29sZF90YWlnYV9tdXRhdGVkBQhkb3duZmFsbM3MzD4FC3RlbXBlcmF0dXJlAAAAvwAKD2RlZXBfY29sZF9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8AChFkZWVwX2Zyb3plbl9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAAChNkZWVwX2x1a2V3YXJtX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKCmRlZXBfb2NlYW4FCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAA/AAoPZGVlcF93YXJtX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKBmRlc2VydAUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAACgxkZXNlcnRfaGlsbHMFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoOZGVzZXJ0X211dGF0ZWQFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoNZXh0cmVtZV9oaWxscwUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4AChJleHRyZW1lX2hpbGxzX2VkZ2UFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw+AAoVZXh0cmVtZV9oaWxsc19tdXRhdGVkBQhkb3duZmFsbJqZmT4FC3RlbXBlcmF0dXJlzcxMPgAKGGV4dHJlbWVfaGlsbHNfcGx1c190cmVlcwUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4ACiBleHRyZW1lX2hpbGxzX3BsdXNfdHJlZXNfbXV0YXRlZAUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4ACg1mbG93ZXJfZm9yZXN0BQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKBmZvcmVzdAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZTMzMz8ACgxmb3Jlc3RfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUzMzM/AAoMZnJvemVuX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAAAAKDGZyb3plbl9yaXZlcgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAACgRoZWxsBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlAAAAQAAKDWljZV9tb3VudGFpbnMFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAAAAAoKaWNlX3BsYWlucwUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAAChFpY2VfcGxhaW5zX3NwaWtlcwUIZG93bmZhbGwAAIA/BQt0ZW1wZXJhdHVyZQAAAAAACgZqdW5nbGUFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoLanVuZ2xlX2VkZ2UFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUzM3M/AAoTanVuZ2xlX2VkZ2VfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZTMzcz8ACgxqdW5nbGVfaGlsbHMFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoOanVuZ2xlX211dGF0ZWQFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoTbGVnYWN5X2Zyb3plbl9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAACg5sdWtld2FybV9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgptZWdhX3RhaWdhBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlmpmZPgAKEG1lZ2FfdGFpZ2FfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmWamZk+AAoEbWVzYQUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAACgptZXNhX2JyeWNlBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlAAAAQAAKDG1lc2FfcGxhdGVhdQUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAAChRtZXNhX3BsYXRlYXVfbXV0YXRlZAUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAAChJtZXNhX3BsYXRlYXVfc3RvbmUFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoabWVzYV9wbGF0ZWF1X3N0b25lX211dGF0ZWQFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoPbXVzaHJvb21faXNsYW5kBQhkb3duZmFsbAAAgD8FC3RlbXBlcmF0dXJlZmZmPwAKFW11c2hyb29tX2lzbGFuZF9zaG9yZQUIZG93bmZhbGwAAIA/BQt0ZW1wZXJhdHVyZWZmZj8ACgVvY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgZwbGFpbnMFCGRvd25mYWxszczMPgULdGVtcGVyYXR1cmXNzEw/AAobcmVkd29vZF90YWlnYV9oaWxsc19tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlmpmZPgAKFXJlZHdvb2RfdGFpZ2FfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZQAAgD4ACgVyaXZlcgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACg1yb29mZWRfZm9yZXN0BQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKFXJvb2ZlZF9mb3Jlc3RfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZTMzMz8ACgdzYXZhbm5hBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlmpmZPwAKD3NhdmFubmFfbXV0YXRlZAUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZc3MjD8ACg9zYXZhbm5hX3BsYXRlYXUFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAIA/AAoXc2F2YW5uYV9wbGF0ZWF1X211dGF0ZWQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAIA/AAoLc3RvbmVfYmVhY2gFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw+AAoQc3VuZmxvd2VyX3BsYWlucwUIZG93bmZhbGzNzMw+BQt0ZW1wZXJhdHVyZc3MTD8ACglzd2FtcGxhbmQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmXNzEw/AAoRc3dhbXBsYW5kX211dGF0ZWQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmXNzEw/AAoFdGFpZ2EFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUAAIA+AAoLdGFpZ2FfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUAAIA+AAoNdGFpZ2FfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZQAAgD4ACgd0aGVfZW5kBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKCndhcm1fb2NlYW4FCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAA/AAA=";

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine as _, engine::general_purpose};

    #[test]
    fn test_biomes_base64_decode() {
        let res = general_purpose::STANDARD.decode(BIOMES_BASE64);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 3413);
    }
}
