pub use crate::macros::helpers::read_string;
pub const ID_CRAFTINGDATA: u32 = 0;
pub struct CraftingData;
impl CraftingData {
    pub fn read(_data: &[u8]) -> Result<Self, crate::protocol::error::PacketError> { Ok(CraftingData) }
    pub fn write(&self) -> Result<Vec<u8>, crate::protocol::error::PacketError> { Ok(Vec::new()) }
}
