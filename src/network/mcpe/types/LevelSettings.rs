use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::{write_vari32};
use crate::macros::helpers;
use crate::protocol::packet::spawn_settings::SpawnSettings;
use crate::protocol::packet::game_rule_legacy::GameRuleLegacyData;
use crate::protocol::packet::experiments::Experiments;

/// Full level-configuration block sent inside the StartGame packet (protocol v1001).
///
/// Field order matches the wire format exactly.
pub struct LevelSettings {
    // ── World seed ──────────────────────────────────────────────────────────
    pub seed: u64,

    // ── Spawn ────────────────────────────────────────────────────────────────
    pub spawn_settings: SpawnSettings,

    // ── World generation ─────────────────────────────────────────────────────
    /// 0=legacy, 1=overworld, 2=flat
    pub generator_type: i32,

    // ── Game mode / difficulty ───────────────────────────────────────────────
    /// 0=survival, 1=creative, 2=adventure
    pub game_type: i32,
    pub is_hardcore_enabled: bool,
    /// 0=peaceful, 1=easy, 2=normal, 3=hard
    pub game_difficulty: i32,

    // ── Default spawn position ───────────────────────────────────────────────
    pub default_spawn_block_position: (i32, i32, i32),

    // ── World flags ──────────────────────────────────────────────────────────
    pub achievements_disabled: bool,
    /// 0=not_editor, 1=editor, 2=export
    pub editor_world_type: i32,
    pub is_created_in_editor: bool,
    pub is_exported_from_editor: bool,
    pub day_cycle_stop_time: i32,

    // ── Education ────────────────────────────────────────────────────────────
    pub education_edition_offer: i32,
    pub education_features_enabled: bool,
    pub education_product_id: String,

    // ── Weather ──────────────────────────────────────────────────────────────
    pub rain_level: f32,
    pub lightning_level: f32,

    // ── Multiplayer ──────────────────────────────────────────────────────────
    pub has_confirmed_platform_locked_content: bool,
    pub multiplayer_enabled: bool,
    pub lan_broadcasting_enabled: bool,
    /// Xbox Live broadcast intent
    pub xbox_live_broadcast_setting: i32,
    /// Platform broadcast intent
    pub platform_broadcast_setting: i32,

    // ── Commands / packs ─────────────────────────────────────────────────────
    pub commands_enabled: bool,
    pub texture_packs_required: bool,

    // ── Game rules & experiments ─────────────────────────────────────────────
    pub rule_data: GameRuleLegacyData,
    pub experiments: Experiments,

    // ── Misc toggles ─────────────────────────────────────────────────────────
    pub bonus_chest_enabled: bool,
    pub starting_map_enabled: bool,
    /// 0=visitor, 1=member, 2=operator
    pub player_permissions: i32,
    /// Fixed i32 LE (not varint)
    pub server_chunk_tick_range: i32,
    pub locked_behaviour_pack: bool,
    pub locked_resource_pack: bool,
    pub from_locked_template: bool,
    pub use_msa_gamer_tags: bool,
    pub from_template: bool,
    pub has_locked_template_settings: bool,
    pub only_spawn_v1_villagers: bool,
    pub persona_disabled: bool,
    pub custom_skins_disabled: bool,
    pub emote_chat_muted: bool,

    // ── Version / world size ─────────────────────────────────────────────────
    /// e.g. "*"
    pub base_game_version: String,
    /// Fixed i32 LE
    pub limited_world_width: i32,
    /// Fixed i32 LE
    pub limited_world_depth: i32,
    pub nether_type: bool,

    // ── Education links ──────────────────────────────────────────────────────
    pub edu_button_name: String,
    pub edu_link_uri: String,

    // ── Experimental gameplay override ────────────────────────────────────────
    /// None → write 0x00; Some(v) → write 0x01 then bool
    pub override_force_experimental_gameplay: Option<bool>,

    // ── Chat / editor policy ─────────────────────────────────────────────────
    /// 0=none, 1=dropped, 2=disabled
    pub chat_restriction_level: i32,
    pub disable_player_interactions: bool,
    pub server_editor_connection_policy: i32,

    // ── v844+ new field ──────────────────────────────────────────────────────
    pub allow_anonymous_block_drops_in_editor_worlds: bool,
}

impl LevelSettings {
    pub fn new() -> Self {
        Self {
            seed: 0,
            spawn_settings: SpawnSettings::new(),
            generator_type: 2, // Flat
            game_type: 0,      // Survival
            is_hardcore_enabled: false,
            game_difficulty: 0,
            default_spawn_block_position: (0, 64, 0),
            achievements_disabled: false,
            editor_world_type: 0,
            is_created_in_editor: false,
            is_exported_from_editor: false,
            day_cycle_stop_time: 0,
            education_edition_offer: 0,
            education_features_enabled: false,
            education_product_id: String::new(),
            rain_level: 0.0,
            lightning_level: 0.0,
            has_confirmed_platform_locked_content: false,
            multiplayer_enabled: true,
            lan_broadcasting_enabled: true,
            xbox_live_broadcast_setting: 4, // Public
            platform_broadcast_setting: 4,  // Public
            commands_enabled: true,
            texture_packs_required: false,
            rule_data: GameRuleLegacyData::new(),
            experiments: Experiments::new(),
            bonus_chest_enabled: false,
            starting_map_enabled: false,
            player_permissions: 1, // Member
            server_chunk_tick_range: 0,
            locked_behaviour_pack: false,
            locked_resource_pack: false,
            from_locked_template: false,
            use_msa_gamer_tags: false,
            from_template: false,
            has_locked_template_settings: false,
            only_spawn_v1_villagers: false,
            persona_disabled: false,
            custom_skins_disabled: false,
            emote_chat_muted: false,
            base_game_version: "*".to_string(),
            limited_world_width: 0,
            limited_world_depth: 0,
            nether_type: false,
            edu_button_name: String::new(),
            edu_link_uri: String::new(),
            override_force_experimental_gameplay: None,
            chat_restriction_level: 0,
            disable_player_interactions: false,
            server_editor_connection_policy: 0,
            allow_anonymous_block_drops_in_editor_worlds: false,
        }
    }

    /// Serialize into bytes and append to `buf`.
    pub fn write_into(&self, buf: &mut Vec<u8>) {
        // 1. seed: u64 LE
        buf.write_u64::<LittleEndian>(self.seed).unwrap();

        // 2. spawn_settings (inline)
        let before = buf.len();
        self.spawn_settings.write_into(buf);
        let written = &buf[before..];
        log::debug!("[LevelSettings] spawn_settings bytes=[{}]", written.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "));

        // 3. generator_type: i32 varint
        write_vari32(buf, self.generator_type);

        // 4. game_type: i32 varint
        write_vari32(buf, self.game_type);

        // 5. is_hardcore_enabled: bool
        buf.push(if self.is_hardcore_enabled { 1 } else { 0 });

        // 6. game_difficulty: i32 varint
        write_vari32(buf, self.game_difficulty);

        // 7. default_spawn_block_position: (i32, i32, i32) varints
        write_vari32(buf, self.default_spawn_block_position.0);
        write_vari32(buf, self.default_spawn_block_position.1);
        write_vari32(buf, self.default_spawn_block_position.2);

        // 8. achievements_disabled: bool
        buf.push(if self.achievements_disabled { 1 } else { 0 });

        // 9. editor_world_type: i32 varint
        write_vari32(buf, self.editor_world_type);

        // 10. is_created_in_editor: bool
        buf.push(if self.is_created_in_editor { 1 } else { 0 });

        // 11. is_exported_from_editor: bool
        buf.push(if self.is_exported_from_editor { 1 } else { 0 });

        // 12. day_cycle_stop_time: i32 varint
        write_vari32(buf, self.day_cycle_stop_time);

        // 13. education_edition_offer: i32 varint
        write_vari32(buf, self.education_edition_offer);

        // 14. education_features_enabled: bool
        buf.push(if self.education_features_enabled { 1 } else { 0 });

        // 15. education_product_id: String
        helpers::write_string(buf, &self.education_product_id);

        // 16. rain_level: f32 LE
        buf.write_f32::<LittleEndian>(self.rain_level).unwrap();

        // 17. lightning_level: f32 LE
        buf.write_f32::<LittleEndian>(self.lightning_level).unwrap();

        // 18. has_confirmed_platform_locked_content: bool
        buf.push(if self.has_confirmed_platform_locked_content { 1 } else { 0 });

        // 19. multiplayer_enabled: bool
        buf.push(if self.multiplayer_enabled { 1 } else { 0 });

        // 20. lan_broadcasting_enabled: bool
        buf.push(if self.lan_broadcasting_enabled { 1 } else { 0 });

        // 21. xbox_live_broadcast_setting: i32 varint
        write_vari32(buf, self.xbox_live_broadcast_setting);

        // 22. platform_broadcast_setting: i32 varint
        write_vari32(buf, self.platform_broadcast_setting);

        // 23. commands_enabled: bool
        buf.push(if self.commands_enabled { 1 } else { 0 });

        // 24. texture_packs_required: bool
        buf.push(if self.texture_packs_required { 1 } else { 0 });

        // 25. rule_data (inline)
        self.rule_data.write_into(buf);

        // 26. experiments (inline)
        self.experiments.write_into(buf);

        // 27. bonus_chest_enabled: bool
        buf.push(if self.bonus_chest_enabled { 1 } else { 0 });

        // 28. starting_map_enabled: bool
        buf.push(if self.starting_map_enabled { 1 } else { 0 });

        // 29. player_permissions: i32 varint
        write_vari32(buf, self.player_permissions);

        // 30. server_chunk_tick_range: i32 LE (fixed, not varint)
        buf.write_i32::<LittleEndian>(self.server_chunk_tick_range).unwrap();

        // 31. locked_behaviour_pack: bool
        buf.push(if self.locked_behaviour_pack { 1 } else { 0 });

        // 32. locked_resource_pack: bool
        buf.push(if self.locked_resource_pack { 1 } else { 0 });

        // 33. from_locked_template: bool
        buf.push(if self.from_locked_template { 1 } else { 0 });

        // 34. use_msa_gamer_tags: bool
        buf.push(if self.use_msa_gamer_tags { 1 } else { 0 });

        // 35. from_template: bool
        buf.push(if self.from_template { 1 } else { 0 });

        // 36. has_locked_template_settings: bool
        buf.push(if self.has_locked_template_settings { 1 } else { 0 });

        // 37. only_spawn_v1_villagers: bool
        buf.push(if self.only_spawn_v1_villagers { 1 } else { 0 });

        // 38. persona_disabled: bool
        buf.push(if self.persona_disabled { 1 } else { 0 });

        // 39. custom_skins_disabled: bool
        buf.push(if self.custom_skins_disabled { 1 } else { 0 });

        // 40. emote_chat_muted: bool
        buf.push(if self.emote_chat_muted { 1 } else { 0 });

        // 41. base_game_version: String
        helpers::write_string(buf, &self.base_game_version);

        // 42. limited_world_width: i32 LE
        buf.write_i32::<LittleEndian>(self.limited_world_width).unwrap();

        // 43. limited_world_depth: i32 LE
        buf.write_i32::<LittleEndian>(self.limited_world_depth).unwrap();

        // 44. nether_type: bool
        buf.push(if self.nether_type { 1 } else { 0 });

        // 45. edu_button_name: String
        helpers::write_string(buf, &self.edu_button_name);

        // 46. edu_link_uri: String
        helpers::write_string(buf, &self.edu_link_uri);

        // 47. override_force_experimental_gameplay: Option<bool>
        //     None → 0x00; Some(v) → 0x01 + bool byte
        match self.override_force_experimental_gameplay {
            Some(v) => {
                buf.push(1);
                buf.push(if v { 1 } else { 0 });
            }
            None => {
                buf.push(0);
            }
        }

        // 48. chat_restriction_level: i32 varint
        write_vari32(buf, self.chat_restriction_level);

        // 49. disable_player_interactions: bool
        buf.push(if self.disable_player_interactions { 1 } else { 0 });

        // 50. server_editor_connection_policy: i32 varint
        write_vari32(buf, self.server_editor_connection_policy);

        // 51. allow_anonymous_block_drops_in_editor_worlds: bool  (NEW v844+)
        buf.push(if self.allow_anonymous_block_drops_in_editor_worlds { 1 } else { 0 });
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        self.write_into(&mut buf);
        Ok(buf)
    }
}
