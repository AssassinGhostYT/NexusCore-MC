use crate::protocol::error::PResult;
use crate::protocol::varint::write_varu32;

pub const ID_CRAFTING_DATA: u32 = 52;

pub struct CraftingData {
    pub clear_recipes: bool,
}

impl CraftingData {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        // 1. crafting_entries count (VarU32 = 0)
        write_varu32(&mut buf, 0);
        // 2. potion_mixes count (VarU32 = 0)
        write_varu32(&mut buf, 0);
        // 3. container_mixes count (VarU32 = 0)
        write_varu32(&mut buf, 0);
        // 4. material_reducers count (VarU32 = 0)
        write_varu32(&mut buf, 0);
        // 5. clear_recipes bool
        buf.push(if self.clear_recipes { 1 } else { 0 });
        Ok(buf)
    }
}
