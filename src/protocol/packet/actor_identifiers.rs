pub const ID_AVAILABLE_ACTOR_IDENTIFIERS: u32 = 119;

#[derive(Debug, Clone)]
pub struct AvailableActorIdentifiers {
    pub serialized_entity_identifiers: Vec<u8>,
}

impl AvailableActorIdentifiers {
    pub fn write(&self) -> Vec<u8> {
        self.serialized_entity_identifiers.clone()
    }
}
