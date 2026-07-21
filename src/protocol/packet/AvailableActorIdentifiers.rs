// AvailableActorIdentifiers — ID 119
// Envia un NBT compuesto vacío (cliente usa biomas por defecto)

use crate::protocol::error::PResult;

pub struct AvailableActorIdentifiers;

impl AvailableActorIdentifiers {
    pub fn write() -> PResult<Vec<u8>> {
        // NBT vacío: solo byte 0x00 (TAG_Compound con 0 elementos)
        Ok(vec![0x00])
    }
}
