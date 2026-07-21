pub use crate::macros::helpers::read_string;
pub const ID_CONTAINEROPEN: u32 = 0;
pub struct ContainerOpen;
impl ContainerOpen {
    pub fn read(_data: &[u8]) -> Result<Self, crate::protocol::error::PacketError> { Ok(ContainerOpen) }
    pub fn write(&self) -> Result<Vec<u8>, crate::protocol::error::PacketError> { Ok(Vec::new()) }
}
