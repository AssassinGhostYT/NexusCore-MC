use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::{write_vari64, write_varu64, write_vari32};
use crate::macros::helpers;
use crate::protocol::packet::block_palette::write_block_palette;
use crate::protocol::packet::level_settings::LevelSettings;
use crate::protocol::packet::synced_player_movement_settings::SyncedPlayerMovementSettings;
use crate::protocol::packet::network_permissions::NetworkPermissions;

/// StartGame packet (ID = 0x0B / 11).
///
/// Sent by the server immediately after ResourcePackStack is acknowledged (status=4).
/// Field order follows the v1001 wire format exactly.
pub struct StartGame {
    // ── Entity IDs ────────────────────────────────────────────────────────────
    /// Unique entity ID of the local player (zigzag vari64).
    pub entity_id: i64,
    /// Runtime entity ID of the local player (varu64).
    pub runtime_entity_id: u64,

    // ── Player state ─────────────────────────────────────────────────────────
    /// Game mode for this player (vari32): 0=survival, 1=creative, 2=adventure.
    pub player_gamemode: i32,
    /// Player spawn position (x, y, z) as f32 LE.
    pub player_position: (f32, f32, f32),
    /// Pitch and yaw of the player's initial look direction (f32 LE each).
    pub pitch: f32,
    pub yaw: f32,

    // ── Level / world settings ────────────────────────────────────────────────
    pub settings: LevelSettings,

    // ── Level identity ────────────────────────────────────────────────────────
    /// Opaque level ID string (base64 in vanilla, "" is fine for custom servers).
    pub level_id: String,
    /// Human-readable level name shown on the loading screen.
    pub level_name: String,
    /// Template content identity (empty if not a template world).
    pub template_content_identity: String,
    /// Whether the world is a trial world (false = full world).
    pub is_trial: bool,

    // ── Movement settings ─────────────────────────────────────────────────────
    pub movement_settings: SyncedPlayerMovementSettings,

    // ── Time / enchantments ───────────────────────────────────────────────────
    /// Current world time in ticks (u64 LE).
    pub current_level_time: u64,
    /// Seed for the enchantment table (vari32).
    pub enchantment_seed: i32,

    // ── Block palette ─────────────────────────────────────────────────────────
    // (written via write_block_palette — loaded from resources/block_states.nbt)

    // ── Multiplayer / server metadata ─────────────────────────────────────────
    pub multiplayer_correlation_id: String,
    pub enable_item_stack_net_manager: bool,
    /// Bedrock version string, e.g. "1.26.32".
    pub server_version: String,

    // ── Property data (NBT) ───────────────────────────────────────────────────
    // Written as a TAG_Compound NBT with no entries (0x0a 0x00 0x00 ... 0x00).

    // ── Registry checksum / world template ───────────────────────────────────
    pub server_block_type_registry_checksum: u64,
    /// World template UUID (16 bytes, all zero for non-template worlds).
    pub world_template_id: [u8; 16],

    // ── Feature flags ─────────────────────────────────────────────────────────
    pub server_enabled_client_side_generation: bool,
    pub block_network_ids_are_hashes: bool,

    // ── Network permissions ───────────────────────────────────────────────────
    pub network_permissions: NetworkPermissions,

    pub is_logging_chat: bool,

    // ── Server join information (Option — None = not provided) ────────────────
    // Written as a single bool: false = no join info.

    // ── Server / world identity strings ──────────────────────────────────────
    pub server_id: String,
    pub world_id: String,
    pub scenario_id: String,
    pub owner_id: String,
}

impl StartGame {
    pub fn new() -> Self {
        Self {
            entity_id: 609,
            runtime_entity_id: 402,
            player_gamemode: 1, // Creative
            player_position: (0.5, -58.38, 0.5),
            pitch: 0.0,
            yaw: 0.0,
            settings: {
                let mut settings = LevelSettings::new();
                settings.game_type = 1; // Creative
                settings
            },
            level_id: String::new(),
            level_name: String::new(),
            template_content_identity: String::new(),
            is_trial: false,
            movement_settings: SyncedPlayerMovementSettings::new(),
            current_level_time: 0,
            enchantment_seed: 0,
            multiplayer_correlation_id: String::new(),
            enable_item_stack_net_manager: false,
            server_version: "1.26.32".to_string(),
            server_block_type_registry_checksum: 0,
            world_template_id: [0u8; 16],
            server_enabled_client_side_generation: false,
            block_network_ids_are_hashes: true,
            network_permissions: NetworkPermissions::new(),
            is_logging_chat: false,
            server_id: String::new(),
            world_id: String::new(),
            scenario_id: String::new(),
            owner_id: String::new(),
        }
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();

        // Entity IDs
        write_vari64(&mut buf, self.entity_id);
        write_varu64(&mut buf, self.runtime_entity_id);

        // Player state
        write_vari32(&mut buf, self.player_gamemode);
        buf.write_f32::<LittleEndian>(self.player_position.0).unwrap();
        buf.write_f32::<LittleEndian>(self.player_position.1).unwrap();
        buf.write_f32::<LittleEndian>(self.player_position.2).unwrap();
        buf.write_f32::<LittleEndian>(self.pitch).unwrap();
        buf.write_f32::<LittleEndian>(self.yaw).unwrap();

        // Level settings (big embedded struct)
        self.settings.write_into(&mut buf);

        // Level identity
        helpers::write_string(&mut buf, &self.level_id);
        helpers::write_string(&mut buf, &self.level_name);
        helpers::write_string(&mut buf, &self.template_content_identity);
        buf.push(if self.is_trial { 1 } else { 0 });

        // Movement settings
        self.movement_settings.write_into(&mut buf);

        // Time / enchantments
        buf.write_u64::<LittleEndian>(self.current_level_time).unwrap();
        write_vari32(&mut buf, self.enchantment_seed);

        // Block palette: varu32 count + raw network-NBT entries
        write_block_palette(&mut buf);

        // Multiplayer / server metadata
        helpers::write_string(&mut buf, &self.multiplayer_correlation_id);
        buf.push(if self.enable_item_stack_net_manager { 1 } else { 0 });
        helpers::write_string(&mut buf, &self.server_version);

        // Player property data (empty NBT compound in Network NBT format):
        // TAG_Compound (0x0a) + name len varint (0x00) + TAG_End (0x00)
        buf.push(0x0a); // TAG_Compound
        buf.push(0x00); // varint name length = 0
        buf.push(0x00); // TAG_End

        // Registry checksum and world template UUID
        buf.write_u64::<LittleEndian>(self.server_block_type_registry_checksum).unwrap();
        buf.extend_from_slice(&self.world_template_id);

        // Feature flags
        buf.push(if self.server_enabled_client_side_generation { 1 } else { 0 });
        buf.push(if self.block_network_ids_are_hashes { 1 } else { 0 });

        // Network permissions
        self.network_permissions.write_into(&mut buf);

        buf.push(if self.is_logging_chat { 1 } else { 0 });

        // Server join information: Option<ServerJoinInformation>
        // Written as a bool: false = None (no join info provided)
        buf.push(0x00);

        // Server / world identity strings
        helpers::write_string(&mut buf, &self.server_id);
        helpers::write_string(&mut buf, &self.world_id);
        helpers::write_string(&mut buf, &self.scenario_id);
        helpers::write_string(&mut buf, &self.owner_id);

        let hex: Vec<String> = buf.iter().map(|b| format!("{:02x}", b)).collect();
        log::debug!("[StartGame::write] payload size: {} bytes raw=[{}]", buf.len(), hex.join(" "));
        Ok(buf)
    }
}
