// CurrentStructureFeature — ID 314 (0x13a)
// Sent by the server to inform the client of the current structure feature.

use crate::protocol::error::PResult;
use crate::macros::helpers;

pub struct CurrentStructureFeature {
    pub current_structure_feature: String,
}

impl CurrentStructureFeature {
    pub fn new(current_structure_feature: String) -> Self {
        Self { current_structure_feature }
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        helpers::write_string(&mut buf, &self.current_structure_feature);
        Ok(buf)
    }
}
