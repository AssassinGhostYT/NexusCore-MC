use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::{write_varu32, write_vari32, write_vari64, write_varu64};
use super::helpers::{write_string};
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
            // Write block_name as string prefijado con varint (exterior)
            let block_name = format!("minecraft:{}", entry.name);
            write_string(&mut buf, &block_name);
            
            // Build the NBT Compound containing "states" and "version" (unnamed compound)
            let mut nbt_states = HashMap::new();
            for (k, v) in &entry.states {
                let nbt_val = match v.prop_type.as_str() {
                    "byte" => {
                        let val = match &v.value {
                            serde_json::Value::Bool(b) => if *b { 1 } else { 0 },
                            serde_json::Value::Number(num) => num.as_i64().unwrap_or(0) as i8,
                            _ => 0,
                        };
                        Some(nbtx::Value::Byte(val))
                    }
                    "int" => {
                        let val = match &v.value {
                            serde_json::Value::Number(num) => num.as_i64().unwrap_or(0) as i32,
                            _ => 0,
                        };
                        Some(nbtx::Value::Int(val))
                    }
                    "string" => {
                        let val = match &v.value {
                            serde_json::Value::String(s) => s.clone(),
                            _ => "".to_string(),
                        };
                        Some(nbtx::Value::String(val))
                    }
                    _ => None,
                };
                if let Some(val) = nbt_val {
                    nbt_states.insert(k.clone(), val);
                }
            }
            
            let mut block_fields = HashMap::new();
            block_fields.insert("states".to_string(), nbtx::Value::Compound(nbt_states));
            block_fields.insert("version".to_string(), nbtx::Value::Int(entry.version));
            
            let root = nbtx::Value::Compound(block_fields);
            
            let mut nbt_buf = Vec::new();
            nbtx::to_bytes_in::<nbtx::NetworkLittleEndian>(&mut nbt_buf, &root)
                .expect("Failed to serialize block state NBT");
                
            buf.extend_from_slice(&nbt_buf);
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

        // 1. Entity Unique ID: i64 VarInt
        write_vari64(&mut buf, self.entity_id);
        // 2. Entity Runtime ID: u64 VarInt
        write_varu64(&mut buf, self.runtime_entity_id);
        // 3. Player Game Mode: i32 VarInt
        write_vari32(&mut buf, self.player_gamemode);

        // 4. Player Position: 3x f32 little endian
        buf.write_f32::<LittleEndian>(self.player_position.0).unwrap();
        buf.write_f32::<LittleEndian>(self.player_position.1).unwrap();
        buf.write_f32::<LittleEndian>(self.player_position.2).unwrap();

        // 5. Pitch, Yaw: f32 little endian
        buf.write_f32::<LittleEndian>(self.pitch).unwrap();
        buf.write_f32::<LittleEndian>(self.yaw).unwrap();

        // 6. World Seed: i64 little endian
        buf.write_i64::<LittleEndian>(self.seed).unwrap();

        // 7. Spawn biome type: i16 little endian (0 = plains)
        buf.write_i16::<LittleEndian>(0).unwrap();
        // 8. UserDefinedBiomeName: string ("plains")
        write_string(&mut buf, "plains");

        // 9. Dimension: i32 VarInt (0 = Overworld)
        write_vari32(&mut buf, 0);
        // 10. Generator type: i32 VarInt (2 = flat)
        write_vari32(&mut buf, 2);
        // 11. World Game Mode: i32 VarInt
        write_vari32(&mut buf, self.player_gamemode);
        // 12. Hardcore: bool (false = 0)
        buf.push(0);
        // 13. Difficulty: i32 VarInt (1 = normal)
        write_vari32(&mut buf, 1);

        // 14. WorldSpawn: BlockPos (i32 VarInt X, i32 VarInt Y, i32 VarInt Z)
        write_vari32(&mut buf, self.spawn_position.0);
        write_vari32(&mut buf, self.spawn_position.1);
        write_vari32(&mut buf, self.spawn_position.2);

        // 15. Achievements disabled: bool (false = 0)
        buf.push(0);
        // 16. Editor world type: i32 VarInt (0)
        write_vari32(&mut buf, 0);
        // 17. Created in editor: bool (false)
        buf.push(0);
        // 18. Exported from editor: bool (false)
        buf.push(0);

        // 19. Day cycle lock time: i32 VarInt (0)
        write_vari32(&mut buf, 0);
        // 20. Education edition offer: i32 VarInt (0)
        write_vari32(&mut buf, 0);
        // 21. Education features enabled: bool (false)
        buf.push(0);
        // 22. Education product ID: string ("")
        write_string(&mut buf, "");

        // 23. Rain level: f32 (0.0)
        buf.write_f32::<LittleEndian>(0.0).unwrap();
        // 24. Lightning level: f32 (0.0)
        buf.write_f32::<LittleEndian>(0.0).unwrap();

        // 25. Confirmed platform locked content: bool (false)
        buf.push(0);
        // 26. MultiPlayerGame: bool (true)
        buf.push(1);
        // 27. LANBroadcastEnabled: bool (true)
        buf.push(1);
        // 28. XBLBroadcastMode: i32 VarInt (4 = public)
        write_vari32(&mut buf, 4);
        // 29. Platform broadcast mode: i32 VarInt (4)
        write_vari32(&mut buf, 4);
        // 30. CommandsEnabled: bool (true)
        buf.push(1);
        // 31. TexturePackRequired: bool (false)
        buf.push(0);

        // 32. Game rules: VarInt length (0)
        write_varu32(&mut buf, 0);
        // 33. Experiments: u32 little endian length (0)
        buf.write_u32::<LittleEndian>(0).unwrap();
        // 34. Experiments previously toggled: bool (false)
        buf.push(0);

        // 35. Bonus chest: bool (false)
        buf.push(0);
        // 36. Start with map: bool (false)
        buf.push(0);
        // 37. Player permissions: i32 VarInt (2 = Operator)
        write_vari32(&mut buf, 2);
        // 38. Server chunk tick radius: i32 little endian (4)
        buf.write_i32::<LittleEndian>(4).unwrap();

        // 39. Has locked behaviour pack: bool (false)
        buf.push(0);
        // 40. Has locked texture pack: bool (false)
        buf.push(0);
        // 41. From locked world template: bool (false)
        buf.push(0);
        // 42. MSAGamerTagsOnly: bool (false)
        buf.push(0);
        // 43. From world template: bool (false)
        buf.push(0);
        // 44. World template settings locked: bool (false)
        buf.push(0);
        // 45. Only spawn v1 villagers: bool (false)
        buf.push(0);
        // 46. Persona disabled: bool (false)
        buf.push(0);
        // 47. Custom skins disabled: bool (false)
        buf.push(0);
        // 48. Emote chat muted: bool (false)
        buf.push(0);

        // 49. Base game version: string ("1.26.30")
        write_string(&mut buf, "1.26.30");

        // 50. Limited world width, depth: i32 little endian (0, 0 for unlimited)
        buf.write_i32::<LittleEndian>(0).unwrap();
        buf.write_i32::<LittleEndian>(0).unwrap();

        // 51. New nether: bool (false)
        buf.push(0);

        // 52. Education Shared Resource URI: ButtonName string (""), LinkURI string ("")
        write_string(&mut buf, "");
        write_string(&mut buf, "");

        // 53. Force experimental gameplay: Optional bool (false = not present)
        buf.push(0);

        // 54. Chat restriction level: u8 (0)
        buf.push(0);
        // 55. Disable player interactions: bool (false)
        buf.push(0);

        // 56. ServerEditorConnectionPolicy: i32 VarInt (0)
        write_vari32(&mut buf, 0);
        // 57. AllowAnonymousBlockDropsInEditorWorlds: bool (false)
        buf.push(0);

        // 58. Level ID: string ("world")
        write_string(&mut buf, "world");
        // 59. WorldName: string
        write_string(&mut buf, &self.level_name);
        // 60. Template content identity: string ("")
        write_string(&mut buf, "");

        // 61. Is trial: bool (false)
        buf.push(0);

        // 62. Player movement settings: RewindHistorySize (i32 VarInt = 0), ServerAuthoritativeBlockBreaking (bool = false)
        write_vari32(&mut buf, 0);
        buf.push(0);

        // 63. Current ticks: i64 little-endian (0)
        buf.write_i64::<LittleEndian>(0).unwrap();
        // 64. Enchantment seed: i32 VarInt (0)
        write_vari32(&mut buf, 0);

        // 65. Block entries palette: canonical block palette generated from block_states.json
        buf.extend_from_slice(get_block_palette());

        // 66. Multiplayer correlation ID: string ("")
        write_string(&mut buf, "");

        // 67. Server authoritative inventory: bool (false)
        buf.push(0);

        // 68. Game version (telemetry / duplicate): string ("1.26.30")
        write_string(&mut buf, "1.26.30");

        // 69. Property data NBT: empty compound in Network NBT is 3 bytes (0x0a, 0x00, 0x00)
        buf.extend_from_slice(&[0x0a, 0x00, 0x00]);

        // 70. Server block state checksum: u64 little endian (0)
        buf.write_u64::<LittleEndian>(0).unwrap();
        // 71. World template ID: UUID (16 bytes of 0)
        buf.extend_from_slice(&[0u8; 16]);

        // 72. Client side generation: bool (false)
        buf.push(0);
        // 73. Use block network ID hashes: bool (false)
        buf.push(0);
        // 74. Server authoritative sound: bool (false)
        buf.push(0);

        // 75. Is logging chat: bool (false)
        buf.push(0);

        // 76. ServerJoinInformation: Optional (false = not present)
        buf.push(0);

        // 77. Server telemetry data: ServerID (string), ScenarioID (string), WorldID (string), OwnerID (string)
        write_string(&mut buf, ""); // ServerID
        write_string(&mut buf, ""); // ScenarioID
        write_string(&mut buf, ""); // WorldID
        write_string(&mut buf, ""); // OwnerID

        buf
    }
}
