#[path = "../../macros/helpers.rs"]
pub mod helpers;
#[path = "Batch.rs"]
pub mod batch;
#[path = "RequestNetworkSettings.rs"]
pub mod request_network_settings;
#[path = "NetworkSettings.rs"]
pub mod network_settings;
#[path = "Login.rs"]
pub mod login;
#[path = "PlayStatus.rs"]
pub mod play_status;
#[path = "ResourcePacksInfo.rs"]
pub mod resource_packs_info;
#[path = "ResourcePackStack.rs"]
pub mod resource_pack_stack;
#[path = "ResourcePackClientResponse.rs"]
pub mod resource_pack_client_response;
#[path = "StartGame.rs"]
pub mod start_game;
#[path = "LevelChunk.rs"]
pub mod level_chunk;
#[path = "RequestChunkRadius.rs"]
pub mod request_chunk_radius;
#[path = "ChunkRadiusUpdated.rs"]
pub mod chunk_radius_updated;
#[path = "MovePlayer.rs"]
pub mod move_player;
#[path = "PlayerAuthInput.rs"]
pub mod player_auth_input;
#[path = "ServerToClientHandshake.rs"]
pub mod server_to_client_handshake;
#[path = "ClientToServerHandshake.rs"]
pub mod client_to_server_handshake;
#[path = "ItemRegistry.rs"]
pub mod item_registry;
#[path = "NetworkChunkPublisherUpdate.rs"]
pub mod network_chunk_publisher_update;
#[path = "AvailableActorIdentifiers.rs"]
pub mod available_actor_identifiers;
#[path = "VoxelShapes.rs"]
pub mod voxel_shapes;
#[path = "SetPlayerGameType.rs"]
pub mod set_player_game_type;
#[path = "UpdateAbilities.rs"]
pub mod update_abilities;
#[path = "BlockPalette.rs"]
pub mod block_palette;
#[path = "Creative.rs"]
pub mod creative;
#[path = "AvailableCommands.rs"]
pub mod available_commands;
#[path = "InventoryTransaction.rs"]
pub mod inventory_transaction;
#[path = "ContainerOpen.rs"]
pub mod container_open;
#[path = "ContainerClose.rs"]
pub mod container_close;
#[path = "Camera.rs"]
pub mod camera;
#[path = "CameraAimAssist.rs"]
pub mod camera_aim_assist;
#[path = "CameraAimAssistActorPriority.rs"]
pub mod camera_aim_assist_actor_priority;
#[path = "CameraAimAssistPresets.rs"]
pub mod camera_aim_assist_presets;
#[path = "CameraShake.rs"]
pub mod camera_shake;
#[path = "CameraSpline.rs"]
pub mod camera_spline;
#[path = "CameraPresets.rs"]
pub mod camera_presets;
#[path = "CameraInstruction.rs"]
pub mod camera_instruction;
#[path = "LevelSettings.rs"]
pub mod level_settings;
#[path = "SyncedPlayerMovementSettings.rs"]
pub mod synced_player_movement_settings;
#[path = "NetworkPermissions.rs"]
pub mod network_permissions;
#[path = "ServerBoundLoadingScreen.rs"]
pub mod server_bound_loading_screen;
#[path = "ItemStackRequest.rs"]
pub mod item_stack_request;
#[path = "SpawnSettings.rs"]
pub mod spawn_settings;
#[path = "GameRuleLegacy.rs"]
pub mod game_rule_legacy;
#[path = "Experiments.rs"]
pub mod experiments;

pub mod ids;
pub use ids::*;

// Re-export constants and types
pub use helpers::{read_string, write_string};
pub use batch::{GamePacket, decode_batch, encode_batch, compress_deflate};
pub use request_network_settings::RequestNetworkSettings;
pub use network_settings::NetworkSettings;
pub use login::Login;
pub use play_status::PlayStatus;
pub use resource_packs_info::ResourcePacksInfo;
pub use resource_pack_stack::ResourcePackStack;
pub use resource_pack_client_response::ResourcePackClientResponse;
pub use start_game::StartGame;
pub use level_chunk::{LevelChunk, make_flat_chunk_payload};
pub use request_chunk_radius::RequestChunkRadius;
pub use chunk_radius_updated::ChunkRadiusUpdated;
pub use move_player::{MovePlayer, MovePlayerPosition};
pub use player_auth_input::PlayerAuthInput;
pub use server_to_client_handshake::ServerToClientHandshake;
pub use client_to_server_handshake::ClientToServerHandshake;
pub use item_registry::ItemRegistry;
pub use network_chunk_publisher_update::NetworkChunkPublisherUpdate;
pub use available_actor_identifiers::AvailableActorIdentifiers;
pub use voxel_shapes::VoxelShapes;
pub use set_player_game_type::SetPlayerGameType;
pub use update_abilities::{UpdateAbilities, SerializedLayer};

#[path = "UpdateAdventureSettings.rs"]
pub mod update_adventure_settings;
pub use update_adventure_settings::UpdateAdventureSettings;
pub use creative::CreativeContent;
pub use available_commands::AvailableCommands;
pub use inventory_transaction::InventoryTransaction;
pub use container_open::ContainerOpen;
pub use container_close::ContainerClose;

#[path = "InventoryContent.rs"]
pub mod inventory_content;
pub use inventory_content::InventoryContent;

#[path = "JigsawStructureData.rs"]
pub mod jigsaw_structure_data;
pub use jigsaw_structure_data::JigsawStructureData;

#[path = "CraftingData.rs"]
pub mod crafting_data;
pub use crafting_data::CraftingData;
pub use camera::Camera;
pub use camera_aim_assist::CameraAimAssist;
pub use camera_aim_assist_actor_priority::{CameraAimAssistActorPriority, CameraAimAssistActorPriorityData};
pub use camera_aim_assist_presets::CameraAimAssistPresets;
pub use camera_shake::CameraShake;
pub use camera_spline::CameraSpline;
pub use camera_presets::{CameraPresets, CameraPreset};
pub use camera_instruction::{CameraInstruction, CameraInstructionSet, CameraInstructionFade, CameraInstructionTarget, CameraInstructionFieldOfView, CameraSplineInstruction};

#[path = "LevelEvent.rs"]
pub mod level_event;
pub use level_event::LevelEvent;

#[path = "SetLocalPlayerAsInitialised.rs"]
pub mod set_local_player_as_initialised;
pub use set_local_player_as_initialised::SetLocalPlayerAsInitialized;

#[path = "SetActorData.rs"]
pub mod set_actor_data;
pub use set_actor_data::SetActorData;

#[path = "UpdateAttributes.rs"]
pub mod update_attributes;
pub use update_attributes::UpdateAttributes;
pub use crate::protocol::types::Attribute;

#[path = "CorrectPlayerMovePrediction.rs"]
pub mod correct_player_move_prediction;
pub use correct_player_move_prediction::CorrectPlayerMovePrediction;

#[path = "PacketViolationWarning.rs"]
pub mod packet_violation_warning;
pub use packet_violation_warning::PacketViolationWarning;

#[path = "ClientCameraAimAssist.rs"]
pub mod client_camera_aim_assist;
pub use client_camera_aim_assist::ClientCameraAimAssist;

#[path = "SubChunk.rs"]
pub mod sub_chunk;
pub use sub_chunk::SubChunk;

#[path = "SubChunkRequest.rs"]
pub mod sub_chunk_request;
pub use sub_chunk_request::SubChunkRequest;

#[path = "Interact.rs"]
pub mod interact;
pub use interact::Interact;

#[path = "EmoteList.rs"]
pub mod emote_list;
pub use emote_list::EmoteList;

#[path = "PlayerSkin.rs"]
pub mod player_skin;
pub use player_skin::PlayerSkin;

#[path = "ServerBoundDiagnostics.rs"]
pub mod server_bound_diagnostics;
pub use server_bound_diagnostics::ServerBoundDiagnostics;
pub use server_bound_loading_screen::ServerBoundLoadingScreen;
pub use item_stack_request::ItemStackRequest;
pub use spawn_settings::SpawnSettings;
pub use game_rule_legacy::GameRuleLegacyData;
pub use experiments::Experiments;

#[path = "PlayerList.rs"]
pub mod player_list;
pub use player_list::{PlayerListAdd, PlayerListAddEntry};
