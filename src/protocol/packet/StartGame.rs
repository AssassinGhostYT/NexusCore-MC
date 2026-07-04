use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::{write_varu32, write_vari32, write_vari64, write_varu64};
use super::helpers::write_string;
use crate::protocol::packet::block_palette::get_block_palette;

pub const ID_START_GAME: u32 = 11;

#[derive(Debug, Clone)]
pub struct StartGame {
    pub entity_id: i64,
    pub runtime_entity_id: u64,
    pub player_gamemode: i32,
    pub player_position: (f32, f32, f32),
    pub pitch: f32,
    pub yaw: f32,
    pub seed: i64,
    pub spawn_position: (i32, i32, i32),
    pub level_name: String,
}

impl StartGame {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // == ORDEN EXACTO DEL Marshal() de gophertunnel ==

        // 1: EntityUniqueID (Varint64)
        write_vari64(&mut buf, self.entity_id);
        // 2: EntityRuntimeID (Varuint64)
        write_varu64(&mut buf, self.runtime_entity_id);
        // 3: PlayerGameMode (Varint32)
        write_vari32(&mut buf, self.player_gamemode);

        // 4: PlayerPosition (Vec3 = 3x Float32 LE)
        buf.write_f32::<LittleEndian>(self.player_position.0).unwrap();
        buf.write_f32::<LittleEndian>(self.player_position.1).unwrap();
        buf.write_f32::<LittleEndian>(self.player_position.2).unwrap();

        // 5: Pitch (Float32 LE)
        buf.write_f32::<LittleEndian>(self.pitch).unwrap();
        // 6: Yaw (Float32 LE)
        buf.write_f32::<LittleEndian>(self.yaw).unwrap();

        // 7: WorldSeed (Int64 LE)
        buf.write_i64::<LittleEndian>(self.seed).unwrap();

        // 8: SpawnBiomeType (Int16 LE) = 0 (default)
        buf.write_i16::<LittleEndian>(0).unwrap();
        // 9: UserDefinedBiomeName (String)
        write_string(&mut buf, "plains");

        // 10: Dimension (Varint32) = 0 (Overworld)
        write_vari32(&mut buf, 0);
        // 11: Generator (Varint32) = 2 (Flat)
        write_vari32(&mut buf, 2);
        // 12: WorldGameMode (Varint32)
        write_vari32(&mut buf, self.player_gamemode);
        // 13: Hardcore (Bool)
        buf.push(0);
        // 14: Difficulty (Varint32) = 1 (Easy)
        write_vari32(&mut buf, 1);

        // 15: WorldSpawn (BlockPos: Varint32 x, Varint32 y, Varint32 z)
        write_vari32(&mut buf, self.spawn_position.0);
        write_vari32(&mut buf, self.spawn_position.1);
        write_vari32(&mut buf, self.spawn_position.2);

        // 16: AchievementsDisabled (Bool)
        buf.push(0);
        // 17: EditorWorldType (Varint32) = 0
        write_vari32(&mut buf, 0);
        // 18: CreatedInEditor (Bool)
        buf.push(0);
        // 19: ExportedFromEditor (Bool)
        buf.push(0);

        // 20: DayCycleLockTime (Varint32) — ServerEditorConnectionPolicy va MÁS ADELANTE
        write_vari32(&mut buf, 0);
        // 21: EducationEditionOffer (Varint32)
        write_vari32(&mut buf, 0);
        // 22: EducationFeaturesEnabled (Bool)
        buf.push(0);
        // 23: EducationProductID (String)
        write_string(&mut buf, "");

        // 24: RainLevel (Float32 LE)
        buf.write_f32::<LittleEndian>(0.0).unwrap();
        // 25: LightningLevel (Float32 LE)
        buf.write_f32::<LittleEndian>(0.0).unwrap();

        // 26: ConfirmedPlatformLockedContent (Bool)
        buf.push(0);
        // 27: MultiPlayerGame (Bool)
        buf.push(1);
        // 28: LANBroadcastEnabled (Bool)
        buf.push(1);
        // 29: XBLBroadcastMode (Varint32) = 4
        write_vari32(&mut buf, 4);
        // 30: PlatformBroadcastMode (Varint32) = 4
        write_vari32(&mut buf, 4);
        // 31: CommandsEnabled (Bool)
        buf.push(1);
        // 32: TexturePackRequired (Bool)
        buf.push(0);

        // 33: GameRules (FuncSlice: Varuint32 count = 0)
        write_varu32(&mut buf, 0);
        // 34: Experiments (SliceUint32Length: u32 LE count = 0)
        buf.write_u32::<LittleEndian>(0).unwrap();
        // 35: ExperimentsPreviouslyToggled (Bool)
        buf.push(0);
        // 36: BonusChestEnabled (Bool)
        buf.push(0);
        // 37: StartWithMapEnabled (Bool)
        buf.push(0);
        // 38: PlayerPermissions (Varint32) = 2 (Operator)
        write_vari32(&mut buf, 2);
        // 39: ServerChunkTickRadius (Int32 LE)
        buf.write_i32::<LittleEndian>(4).unwrap();

        // 40: HasLockedBehaviourPack (Bool)
        buf.push(0);
        // 41: HasLockedTexturePack (Bool)
        buf.push(0);
        // 42: FromLockedWorldTemplate (Bool)
        buf.push(0);
        // 43: MSAGamerTagsOnly (Bool)
        buf.push(0);
        // 44: FromWorldTemplate (Bool)
        buf.push(0);
        // 45: WorldTemplateSettingsLocked (Bool)
        buf.push(0);
        // 46: OnlySpawnV1Villagers (Bool)
        buf.push(0);
        // 47: PersonaDisabled (Bool)
        buf.push(0);
        // 48: CustomSkinsDisabled (Bool)
        buf.push(0);
        // 49: EmoteChatMuted (Bool)
        buf.push(0);

        // 50: BaseGameVersion (String)
        write_string(&mut buf, "1.26.32");
        // 51: LimitedWorldWidth (Int32 LE)
        buf.write_i32::<LittleEndian>(0).unwrap();
        // 52: LimitedWorldDepth (Int32 LE)
        buf.write_i32::<LittleEndian>(0).unwrap();
        // 53: NewNether (Bool)
        buf.push(0);

        // 54: EducationSharedResourceURI (Single = ButtonName + LinkURI)
        write_string(&mut buf, ""); // ButtonName
        write_string(&mut buf, ""); // LinkURI

        // 55: ForceExperimentalGameplay (OptionalFunc: 0 = not present)
        buf.push(0);
        // 56: ChatRestrictionLevel (Uint8)
        buf.push(0);
        // 57: DisablePlayerInteractions (Bool)
        buf.push(0);

        // === Campos que estaban mal posicionados — ahora en posición CORRECTA ===
        // 58: ServerEditorConnectionPolicy (Varint32)
        write_vari32(&mut buf, 0);
        // 59: AllowAnonymousBlockDropsInEditorWorlds (Bool)
        buf.push(0);

        // 60: LevelID (String) — faltaba completamente
        write_string(&mut buf, "");
        // 61: WorldName (String) — faltaba completamente
        write_string(&mut buf, &self.level_name);
        // 62: TemplateContentIdentity (String) — faltaba completamente
        write_string(&mut buf, "");
        // 63: Trial (Bool) — faltaba completamente
        buf.push(0);

        // 64: PlayerMovementSettings
        write_vari32(&mut buf, 0); // RewindHistorySize
        buf.push(0);               // ServerAuthoritativeBlockBreaking

        // 65: Time (Int64 LE)
        buf.write_i64::<LittleEndian>(0).unwrap();
        // 66: EnchantmentSeed (Varint32)
        write_vari32(&mut buf, 0);

        // 67: Blocks (Slice of BlockEntry)
        buf.extend_from_slice(get_block_palette());

        // 68: MultiPlayerCorrelationID (String) — SIN Items antes de este campo
        write_string(&mut buf, "");
        // 69: ServerAuthoritativeInventory (Bool)
        buf.push(0);
        // 70: GameVersion (String)
        write_string(&mut buf, "1.26.32");

        // 71: PropertyData (NBT NetworkLittleEndian - compound vacío)
        buf.push(0x0a);                               // TAG_Compound
        buf.push(0x00);                               // nombre vacío (varint 0)
        buf.push(0x00);                               // TAG_End

        // 72: ServerBlockStateChecksum (Uint64 LE)
        buf.write_u64::<LittleEndian>(0).unwrap();
        // 73: WorldTemplateID (UUID: 16 bytes cero)
        buf.extend_from_slice(&[0u8; 16]);

        // 74: ClientSideGeneration (Bool)
        buf.push(0);
        // 75: UseBlockNetworkIDHashes (Bool)
        buf.push(0);
        // 76: ServerAuthoritativeSound (Bool)
        buf.push(0);
        // 77: IsLoggingChat (Bool) — faltaba
        buf.push(0);

        // 78: ServerJoinInformation (OptionalMarshaler: 0 = nil/not present)
        buf.push(0);
        // 79: ServerID (String) — faltaba
        write_string(&mut buf, "");
        // 80: ScenarioID (String) — faltaba
        write_string(&mut buf, "");
        // 81: WorldID (String) — faltaba
        write_string(&mut buf, "");
        // 82: OwnerID (String) — faltaba
        write_string(&mut buf, "");

        buf
    }
}