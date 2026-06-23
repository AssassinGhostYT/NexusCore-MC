use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::{write_varu32, write_vari32, write_vari64, write_varu64};
use super::helpers::write_string;
use std::sync::OnceLock;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct BlockStateProperty {
    #[serde(rename = "type")]
    prop_type: String,
    value: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct BlockStateEntry {
    name: String,
    states: HashMap<String, BlockStateProperty>,
    version: i32,
}

static BLOCK_PALETTE: OnceLock<Vec<u8>> = OnceLock::new();

fn get_block_palette() -> &'static [u8] {
    BLOCK_PALETTE.get_or_init(|| {
        let json_str = include_str!("../../../block_states.json");
        let entries: Vec<BlockStateEntry> = serde_json::from_str(json_str)
            .expect("Failed to parse block_states.json");
            
        let mut buf = Vec::new();
        // 1. Write the count of blocks as a varint (VarU32)
        write_varu32(&mut buf, entries.len() as u32);
        
        // 2. Write each block entry
        for entry in &entries {
            let block_name = format!("minecraft:{}", entry.name);
            write_string(&mut buf, &block_name);
            
            // Build the NBT Compound using our deterministic serializer
            // 1. Root Compound Start (type 0x0a, empty name length 0x00)
            buf.push(0x0a);
            buf.push(0x00);
            
            // 2. "name" property: type 0x08 (TAG_String), name "name", value "minecraft:..."
            buf.push(0x08);
            write_string(&mut buf, "name");
            write_string(&mut buf, &block_name);
            
            // 3. "states" property: type 0x0a (TAG_Compound), name "states"
            buf.push(0x0a);
            write_string(&mut buf, "states");
            
            // Sort state properties alphabetically for determinism
            let mut sorted_states: Vec<(&String, &BlockStateProperty)> = entry.states.iter().collect();
            sorted_states.sort_by_key(|&(k, _)| k);
            
            for (k, v) in sorted_states {
                match v.prop_type.as_str() {
                    "byte" => {
                        buf.push(0x01); // TAG_Byte
                        write_string(&mut buf, k);
                        let byte_val = match &v.value {
                            serde_json::Value::Bool(b) => if *b { 1 } else { 0 },
                            serde_json::Value::Number(num) => num.as_i64().unwrap_or(0) as u8,
                            _ => 0,
                        };
                        buf.push(byte_val);
                    }
                    "int" => {
                        buf.push(0x03); // TAG_Int
                        write_string(&mut buf, k);
                        let int_val = match &v.value {
                            serde_json::Value::Number(num) => num.as_i64().unwrap_or(0) as i32,
                            _ => 0,
                        };
                        write_vari32(&mut buf, int_val);
                    }
                    "string" => {
                        buf.push(0x08); // TAG_String
                        write_string(&mut buf, k);
                        let str_val = match &v.value {
                            serde_json::Value::String(s) => s.clone(),
                            _ => "".to_string(),
                        };
                        write_string(&mut buf, &str_val);
                    }
                    _ => {}
                }
            }
            buf.push(0x00); // TAG_End for states compound
            
            // 4. "version" property: type 0x03 (TAG_Int), name "version", value entry.version
            buf.push(0x03);
            write_string(&mut buf, "version");
            write_vari32(&mut buf, entry.version);
            
            // 5. Root Compound End (0x00)
            buf.push(0x00);
        }
        
        buf
    })
}

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

        // 264: EntityUniqueID
        write_vari64(&mut buf, self.entity_id);
        // 265: EntityRuntimeID
        write_varu64(&mut buf, self.runtime_entity_id);
        // 266: PlayerGameMode
        write_vari32(&mut buf, self.player_gamemode);

        // 267: PlayerPosition
        buf.write_f32::<LittleEndian>(self.player_position.0).unwrap();
        buf.write_f32::<LittleEndian>(self.player_position.1).unwrap();
        buf.write_f32::<LittleEndian>(self.player_position.2).unwrap();

        // 268: Pitch
        buf.write_f32::<LittleEndian>(self.pitch).unwrap();
        // 269: Yaw
        buf.write_f32::<LittleEndian>(self.yaw).unwrap();

        // 270: WorldSeed
        buf.write_i64::<LittleEndian>(self.seed).unwrap();

        // 271: SpawnBiomeType
        buf.write_i16::<LittleEndian>(0).unwrap();
        // 272: UserDefinedBiomeName
        write_string(&mut buf, "plains");

        // 273: Dimension
        write_vari32(&mut buf, 0); // Overworld
        // 274: Generator
        write_vari32(&mut buf, 2); // Flat
        // 275: WorldGameMode
        write_vari32(&mut buf, self.player_gamemode);
        // 276: Hardcore
        buf.push(0); // false
        // 277: Difficulty
        write_vari32(&mut buf, 1); // Easy

        // 278: WorldSpawn (BlockPos: Varint32, Varint32, Varint32)
        write_vari32(&mut buf, self.spawn_position.0);
        write_vari32(&mut buf, self.spawn_position.1);
        write_vari32(&mut buf, self.spawn_position.2);

        // 279: AchievementsDisabled
        buf.push(0); // false
        // 280: EditorWorldType
        write_vari32(&mut buf, 0);
        // 281: CreatedInEditor
        buf.push(0); // false
        // 282: ExportedFromEditor
        buf.push(0); // false

        // 283: DayCycleLockTime
        write_vari32(&mut buf, 0);
        // 284: EducationEditionOffer
        write_vari32(&mut buf, 0);
        // 285: EducationFeaturesEnabled
        buf.push(0); // false
        // 286: EducationProductID
        write_string(&mut buf, "");

        // 287: RainLevel
        buf.write_f32::<LittleEndian>(0.0).unwrap();
        // 288: LightningLevel
        buf.write_f32::<LittleEndian>(0.0).unwrap();

        // 289: ConfirmedPlatformLockedContent
        buf.push(0); // false
        // 290: MultiPlayerGame
        buf.push(1); // true
        // 291: LANBroadcastEnabled
        buf.push(1); // true
        // 292: XBLBroadcastMode
        write_vari32(&mut buf, 4);
        // 293: PlatformBroadcastMode
        write_vari32(&mut buf, 4);
        // 294: CommandsEnabled
        buf.push(1); // true
        // 295: TexturePackRequired
        buf.push(0); // false

        // 296: GameRules (FuncSlice: Varuint32 count)
        write_varu32(&mut buf, 0); // 0 rules
        // 297: Experiments (SliceUint32Length: LittleEndian u32 count)
        buf.write_u32::<LittleEndian>(0).unwrap(); // 0 experiments
        // 298: ExperimentsPreviouslyToggled
        buf.push(0); // false
        // 299: BonusChestEnabled
        buf.push(0); // false
        // 300: StartWithMapEnabled
        buf.push(0); // false
        // 301: PlayerPermissions
        write_vari32(&mut buf, 2); // Operator
        // 302: ServerChunkTickRadius
        buf.write_i32::<LittleEndian>(4).unwrap();

        // 303: HasLockedBehaviourPack
        buf.push(0); // false
        // 304: HasLockedTexturePack
        buf.push(0); // false
        // 305: FromLockedWorldTemplate
        buf.push(0); // false
        // 306: MSAGamerTagsOnly
        buf.push(0); // false
        // 307: FromWorldTemplate
        buf.push(0); // false
        // 308: WorldTemplateSettingsLocked
        buf.push(0); // false
        // 309: OnlySpawnV1Villagers
        buf.push(0); // false
        // 310: PersonaDisabled
        buf.push(0); // false
        // 311: CustomSkinsDisabled
        buf.push(0); // false
        // 312: EmoteChatMuted
        buf.push(0); // false

        // 313: BaseGameVersion
        write_string(&mut buf, "1.26.30");
        // 314: LimitedWorldWidth
        buf.write_i32::<LittleEndian>(0).unwrap();
        // 315: LimitedWorldDepth
        buf.write_i32::<LittleEndian>(0).unwrap();
        // 316: NewNether
        buf.push(0); // false

        // 317: EducationSharedResourceURI (ButtonName, LinkURI)
        write_string(&mut buf, "");
        write_string(&mut buf, "");

        // 318: ForceExperimentalGameplay (Optional[bool]: presence flag 0 = not present)
        buf.push(0); // false
        // 319: ChatRestrictionLevel
        buf.push(0);
        // 320: DisablePlayerInteractions
        buf.push(0); // false

        // 321: ServerID
        write_string(&mut buf, "");
        // 322: WorldID
        write_string(&mut buf, "");
        // 323: ScenarioID
        write_string(&mut buf, "");
        // 324: LevelID
        write_string(&mut buf, "");
        // 325: WorldName
        write_string(&mut buf, &self.level_name);
        // 326: TemplateContentIdentity
        write_string(&mut buf, "");
        // 327: Trial
        buf.push(0); // false

        // 328: PlayerMovementSettings (MovementType, RewindHistorySize, ServerAuthoritativeBlockBreaking)
        write_vari32(&mut buf, 0); // MovementType: PlayerMovementModeClient (0)
        write_vari32(&mut buf, 0); // RewindHistorySize: 0
        buf.push(0); // ServerAuthoritativeBlockBreaking: false

        // 329: Time
        buf.write_i64::<LittleEndian>(0).unwrap();
        // 330: EnchantmentSeed
        write_vari32(&mut buf, 0);

        // 331: Blocks (Slice of BlockEntry)
        buf.extend_from_slice(get_block_palette());

        // 332: Items (Slice of ItemEntry: Varuint32 count = 0)
        write_varu32(&mut buf, 0);

        // 333: MultiPlayerCorrelationID
        write_string(&mut buf, "");
        // 334: ServerAuthoritativeInventory
        buf.push(0); // false
        // 335: GameVersion
        write_string(&mut buf, "1.26.30");

        // 336: PropertyData NBT Compound (empty compound: [0x0a, 0x00, 0x00])
        buf.extend_from_slice(&[0x0a, 0x00, 0x00]);

        // 337: ServerBlockStateChecksum
        buf.write_u64::<LittleEndian>(0).unwrap();
        // 338: WorldTemplateID (UUID: 16 bytes)
        buf.extend_from_slice(&[0u8; 16]);

        // 339: ClientSideGeneration
        buf.push(0); // false
        // 340: UseBlockNetworkIDHashes
        buf.push(0); // false
        // 341: ServerAuthoritativeSound
        buf.push(0); // false

        buf
    }
}