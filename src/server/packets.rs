// Helper functions to build outbound game packets
use crate::network::mcpe::protocol::packets::*;
use crate::network::mcpe::types::*;

// Build SetPlayerGameType packet
pub fn create_gametype_pkg(game_type: i32) -> GamePacket {
    GamePacket {
        id: ID_SET_PLAYER_GAME_TYPE,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: SetPlayerGameType { game_type }.write().unwrap(),
    }
}

// Build UpdateAbilities packet with operator permissions and flying enabled
pub fn create_abilities_pkg(target_player_uid: i64) -> GamePacket {
    const ABILITIES: u32 = 0x1 | 0x2 | 0x4 | 0x8 | 0x10 | 0x20 | 0x100 | 0x200 | 0x400 | 0x800;
    let mut payload = Vec::new();
    UpdateAbilities {
        target_player_uid,
        player_permissions: 2,
        command_permissions: 4,
        layers: vec![SerializedLayer {
            layer_type: 1,
            abilities_set: ABILITIES,
            ability_values: ABILITIES,
            fly_speed: 0.05,
            walk_speed: 0.1,
        }],
    }.write(&mut payload);
    GamePacket { id: ID_UPDATE_ABILITIES, sender_subclient: 0, recipient_subclient: 0, payload }
}

// Build UpdateAdventureSettings packet
pub fn create_adventure_pkg() -> GamePacket {
    GamePacket {
        id: ID_UPDATE_ADVENTURE_SETTINGS,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: UpdateAdventureSettings {
            no_pvm: false,
            no_mvp: false,
            immutable_world: false,
            show_name_tags: true,
            auto_jump: true,
        }.write().unwrap(),
    }
}

// Build empty inventory content packets for all containers
pub fn get_inventory_setup_packets() -> Vec<GamePacket> {
    vec![
        GamePacket { id: ID_INVENTORY_CONTENT, sender_subclient: 0, recipient_subclient: 0,
            payload: InventoryContent { window_id: 0, slots: vec![(0, 0, 0); 36] }.write().unwrap() },
        GamePacket { id: ID_INVENTORY_CONTENT, sender_subclient: 0, recipient_subclient: 0,
            payload: InventoryContent { window_id: 120, slots: vec![(0, 0, 0); 4] }.write().unwrap() },
        GamePacket { id: ID_INVENTORY_CONTENT, sender_subclient: 0, recipient_subclient: 0,
            payload: InventoryContent { window_id: 119, slots: vec![(0, 0, 0); 1] }.write().unwrap() },
    ]
}

// Build MovePlayer packet for teleporting the player
pub fn create_move_player_pkg(runtime_entity_id: u64, x: f32, y: f32, z: f32) -> GamePacket {
    GamePacket {
        id: ID_MOVE_PLAYER,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: MovePlayer {
            runtime_entity_id,
            position: MovePlayerPosition { x, y, z },
            pitch: 0.0, yaw: 0.0, head_yaw: 0.0,
            mode: 2, on_ground: true,
            ridden_entity_runtime_id: 0,
            teleport_cause: 0, source_actor_type: 0, tick: 0,
        }.write().unwrap(),
    }
}

// Build LevelEvent packet
pub fn create_level_event_pkg(event_type: i32, data: i32) -> GamePacket {
    GamePacket {
        id: ID_LEVEL_EVENT,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: LevelEvent { event_type, position: (0.0, 0.0, 0.0), data }.write().unwrap(),
    }
}

// Build SetActorData packet
pub fn create_set_actor_data_pkg(runtime_entity_id: u64) -> GamePacket {
    GamePacket {
        id: ID_SET_ACTOR_DATA,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: SetActorData { entity_runtime_id: runtime_entity_id, tick: 0 }.write(),
    }
}

// Build UpdateAttributes packet with health
pub fn create_update_attributes_pkg(runtime_entity_id: u64) -> GamePacket {
    GamePacket {
        id: ID_UPDATE_ATTRIBUTES,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: UpdateAttributes {
            entity_runtime_id: runtime_entity_id,
            attributes: vec![
                Attribute {
                    name: "minecraft:movement".to_string(),
                    min: 0.0, max: 3.402823e38, value: 0.1,
                    default_min: 0.0, default_max: 3.402823e38, default: 0.1,
                },
                Attribute {
                    name: "minecraft:underwater_movement".to_string(),
                    min: 0.0, max: 3.402823e38, value: 0.02,
                    default_min: 0.0, default_max: 3.402823e38, default: 0.02,
                },
                Attribute {
                    name: "minecraft:lava_movement".to_string(),
                    min: 0.0, max: 3.402823e38, value: 0.02,
                    default_min: 0.0, default_max: 3.402823e38, default: 0.02,
                },
                Attribute {
                    name: "minecraft:health".to_string(),
                    min: 0.0, max: 20.0, value: 20.0,
                    default_min: 0.0, default_max: 20.0, default: 20.0,
                },
                Attribute {
                    name: "minecraft:player.hunger".to_string(),
                    min: 0.0, max: 20.0, value: 20.0,
                    default_min: 0.0, default_max: 20.0, default: 20.0,
                },
                Attribute {
                    name: "minecraft:player.saturation".to_string(),
                    min: 0.0, max: 20.0, value: 20.0,
                    default_min: 0.0, default_max: 20.0, default: 20.0,
                },
                Attribute {
                    name: "minecraft:player.level".to_string(),
                    min: 0.0, max: 24791.0, value: 0.0,
                    default_min: 0.0, default_max: 24791.0, default: 0.0,
                },
                Attribute {
                    name: "minecraft:player.experience".to_string(),
                    min: 0.0, max: 1.0, value: 0.0,
                    default_min: 0.0, default_max: 1.0, default: 0.0,
                },
            ],
            tick: 0,
        }.write(),
    }
}

// Build CorrectPlayerMovePrediction packet
pub fn create_correct_player_move_prediction_pkg(
    position: (f32, f32, f32), pitch: f32, yaw: f32, tick: u64,
) -> GamePacket {
    GamePacket {
        id: ID_CORRECT_PLAYER_MOVE_PREDICTION,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: CorrectPlayerMovePrediction { position, pitch, yaw, tick }.write().unwrap(),
    }
}

// Build AvailableCommands packet (empty)
pub fn create_available_commands_pkg() -> GamePacket {
    GamePacket {
        id: ID_AVAILABLE_COMMANDS,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: AvailableCommands::new().write().unwrap(),
    }
}

pub fn create_set_time_pkg(time: i32) -> GamePacket {
    GamePacket {
        id: ID_SET_TIME,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: SetTime { time }.write().unwrap(),
    }
}

pub fn create_set_difficulty_pkg(difficulty: u32) -> GamePacket {
    GamePacket {
        id: ID_SET_DIFFICULTY,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: SetDifficulty { difficulty }.write().unwrap(),
    }
}

pub fn create_crafting_data_pkg() -> GamePacket {
    GamePacket {
        id: ID_CRAFTING_DATA,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: CraftingData { clear_recipes: true }.write().unwrap(),
    }
}

pub fn create_set_commands_enabled_pkg(enabled: bool) -> GamePacket {
    GamePacket {
        id: ID_SET_COMMANDS_ENABLED,
        sender_subclient: 0,
        recipient_subclient: 0,
        payload: SetCommandsEnabled { enabled }.write().unwrap(),
    }
}
