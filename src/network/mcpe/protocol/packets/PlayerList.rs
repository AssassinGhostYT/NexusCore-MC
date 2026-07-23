// PlayerList — ID 63
// Sent by the server to add/remove players in the client tab/player list.
// Required by the Bedrock client when spawning.

use crate::protocol::error::PResult;
use crate::protocol::varint::{write_varu32, write_vari64};
use crate::macros::helpers::write_string;
use byteorder::{LittleEndian, WriteBytesExt};

pub const ID_PLAYER_LIST: u32 = 63;

pub struct PlayerListAddEntry {
    pub uuid: [u8; 16],
    pub entity_unique_id: i64,
    pub username: String,
}

pub struct PlayerListAdd {
    pub entries: Vec<PlayerListAddEntry>,
}

impl PlayerListAdd {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        
        // Action: 0 (Add)
        buf.push(0);
        
        // Entries count as VarU32
        write_varu32(&mut buf, self.entries.len() as u32);
        
        for entry in &self.entries {
            // 1. UUID (16 raw bytes)
            buf.extend_from_slice(&entry.uuid);
            // 2. Entity Unique ID (VarI64)
            write_vari64(&mut buf, entry.entity_unique_id);
            // 3. Username (String)
            write_string(&mut buf, &entry.username);
            // 4. Xbox XUID (String)
            write_string(&mut buf, "");
            // 5. Platform Chat ID (String)
            write_string(&mut buf, "");
            // 6. Build Platform (i32 LE)
            buf.write_i32::<LittleEndian>(0).unwrap();
            
            // 7. SerializedSkin
            write_string(&mut buf, ""); // skin_id
            write_string(&mut buf, ""); // play_fab_id
            write_string(&mut buf, ""); // skin_resource_patch
            buf.write_u32::<LittleEndian>(0).unwrap(); // skin_image_width
            buf.write_u32::<LittleEndian>(0).unwrap(); // skin_image_height
            write_varu32(&mut buf, 0); // skin_image_bytes
            buf.write_u32::<LittleEndian>(0).unwrap(); // animations count (u32 LE)
            buf.write_u32::<LittleEndian>(0).unwrap(); // cape_image_width
            buf.write_u32::<LittleEndian>(0).unwrap(); // cape_image_height
            write_varu32(&mut buf, 0); // cape_image_bytes
            write_string(&mut buf, ""); // geometry_data
            write_string(&mut buf, ""); // geometry_data_engine_version
            write_string(&mut buf, ""); // animation_data
            write_string(&mut buf, ""); // cape_id
            write_string(&mut buf, ""); // full_id
            write_string(&mut buf, ""); // arm_size
            write_string(&mut buf, ""); // skin_color
            buf.write_u32::<LittleEndian>(0).unwrap(); // persona_pieces count (u32 LE)
            buf.write_u32::<LittleEndian>(0).unwrap(); // piece_tint_colors count (u32 LE)
            buf.push(0); // is_premium_skin
            buf.push(0); // is_persona_skin
            buf.push(0); // is_persona_cape_on_classic_skin
            buf.push(1); // is_primary_user
            buf.push(0); // overrides_player_appearance

            // 8. is_teacher (bool)
            buf.push(0);
            // 9. is_host (bool)
            buf.push(1);
            // 10. is_sub_client (bool)
            buf.push(0);
            // 11. color (u32 LE)
            buf.write_u32::<LittleEndian>(0).unwrap();
        }

        // is_trusted_skin for each entry
        for _ in &self.entries {
            buf.push(1); // true
        }

        Ok(buf)
    }
}
