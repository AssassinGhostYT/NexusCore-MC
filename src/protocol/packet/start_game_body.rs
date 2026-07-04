use crate::protocol::varint::{write_varu32, write_vari32, write_vari64, write_varu64};
use super::helpers::write_string;
use crate::protocol::packet::block_palette::get_block_palette;
use super::StartGame;

pub fn write_body(buf: &mut Vec<u8>, game: &StartGame) {
    // 165: AchievementsDisabled
    buf.push(0); // false
    // 166: EditorWorldType
    write_vari32(buf, 0);
    // 167: CreatedInEditor
    buf.push(0); // false
    // 168: ExportedFromEditor
    buf.push(0); // false

    // 169: DayCycleLockTime
    write_vari32(buf, 0);
    // 170: EducationEditionOffer
    write_vari32(buf, 0);
    // 171: EducationFeaturesEnabled
    buf.push(0); // false
    // 172: EducationProductID
    write_string(buf, "");

    // 173: RainLevel & LightningLevel
    buf.write_f32::<LittleEndian>(0.0).unwrap();
    buf.write_f32::<LittleEndian>(0.0).unwrap();

    // 174: ConfirmedPlatformLockedContent
    buf.push(0);
    // 175: MultiPlayerGame
    buf.push(1);
    // 176: LANBroadcastEnabled
    buf.push(1);
    // 177: XBLBroadcastMode
    write_vari32(buf, 4);
    // 178: PlatformBroadcastMode
    write_vari32(buf, 4);
    // 179: CommandsEnabled
    buf.push(1);
    // 180: TexturePackRequired
    buf.push(0);

    // 181: GameRules
    write_varu32(buf, 0);
    // 182: Experiments (slice length)
    buf.write_u32::<LittleEndian>(0).unwrap();
    // 183: ExperimentsPreviouslyToggled
    buf.push(0);
    // 184: BonusChestEnabled
    buf.push(0);
    // 185: StartWithMapEnabled
    buf.push(0);
    // 186: PlayerPermissions (Operator)
    write_vari32(buf, 2);
    // 187: ServerChunkTickRadius
    buf.write_i32::<LittleEndian>(4).unwrap();

    // 188: HasLockedBehaviourPack
    buf.push(0);
    // 189: HasLockedTexturePack
    buf.push(0);
    // 190: FromLockedWorldTemplate
    buf.push(0);
    // 191: MSAGamerTagsOnly
    buf.push(0);
    // 192: FromWorldTemplate
    buf.push(0);
    // 193: WorldTemplateSettingsLocked
    buf.push(0);
    // 194: OnlySpawnV1Villagers
    buf.push(0);
    // 195: PersonaDisabled
    buf.push(0);
    // 196: CustomSkinsDisabled
    buf.push(0);
    // 197: EmoteChatMuted
    buf.push(0);

    // 198: BaseGameVersion
    write_string(buf, "1.26.30");
    // 199: LimitedWorldWidth
    buf.write_i32::<LittleEndian>(0).unwrap();
    // 200: LimitedWorldDepth
    buf.write_i32::<LittleEndian>(0).unwrap();
    // 201: NewNether
    buf.push(0);

    // 202: EducationSharedResourceURI (ButtonName, LinkURI)
    write_string(buf, "");
    write_string(buf, "");

    // 203: ForceExperimentalGameplay (optional bool flag)
    buf.push(0);
    // 204: ChatRestrictionLevel
    buf.push(0);
    // 205: DisablePlayerInteractions
    buf.push(0);

    // 206: ServerID, WorldID, ScenarioID, LevelID
    write_string(buf, "");
    write_string(buf, "");
    write_string(buf, "");
    write_string(buf, "");
    // 207: WorldName
    write_string(buf, &game.level_name);
    // 208: TemplateContentIdentity
    write_string(buf, "");
    // 209: Trial
    buf.push(0);

    // 210: PlayerMovementSettings (MovementType, RewindHistorySize, ServerAuthoritativeBlockBreaking)
    write_vari32(buf, 0);
    write_vari32(buf, 0);
    buf.push(0);

    // 211: Time & EnchantmentSeed
    buf.write_i64::<LittleEndian>(0).unwrap();
    write_vari32(buf, 0);

    // 212: Blocks (palette)
    buf.extend_from_slice(get_block_palette());

    // 213: Items (count = 0)
    write_varu32(buf, 0);

    // 214: MultiPlayerCorrelationID
    write_string(buf, "");
    // 215: ServerAuthoritativeInventory
    buf.push(0);
    // 216: GameVersion
    write_string(buf, "1.26.30");

    // 217: PropertyData NBT Compound (empty)
    buf.extend_from_slice(&[0x0a, 0x00, 0x00]);

    // 218: ServerBlockStateChecksum
    buf.write_u64::<LittleEndian>(0).unwrap();
    // 219: WorldTemplateID (UUID 16 bytes)
    buf.extend_from_slice(&[0u8; 16]);

    // 220: ClientSideGeneration
    buf.push(0);
    // 221: UseBlockNetworkIDHashes
    buf.push(0);
    // 222: ServerAuthoritativeSound
    buf.push(0);
}
