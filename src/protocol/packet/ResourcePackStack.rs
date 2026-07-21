use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::macros::helpers;
use crate::protocol::varint::write_varu32;

pub struct ResourcePackStack {
    pub must_accept: bool,
    pub game_version: String,
}

impl ResourcePackStack {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();

        // 1. TexturePackRequired (bool)
        buf.push(if self.must_accept { 1 } else { 0 });

        // 2. TexturePacks: Vec<StackResourcePack>
        //    gophertunnel: protocol.Slice() → varint u32 count
        write_varu32(&mut buf, 0); // 0 entries

        // 3. BaseGameVersion: String → varint length + utf8 bytes
        helpers::write_string(&mut buf, &self.game_version);

        // 4. Experiments: []ExperimentData
        //    gophertunnel: protocol.SliceUint32Length() → u32 LE count (NOT varint!)
        buf.write_u32::<LittleEndian>(0)?; // 0 experiments

        // 5. ExperimentsPreviouslyToggled (bool)
        buf.push(0);

        // 6. IncludeEditorPacks (bool)
        buf.push(0);

        Ok(buf)
    }
}

