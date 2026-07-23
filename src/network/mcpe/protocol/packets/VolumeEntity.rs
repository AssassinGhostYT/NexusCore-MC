use crate::protocol::error::PResult;
use crate::protocol::varint::{write_vari32, write_varu32};
use crate::macros::helpers::write_string;

pub const ID_ADD_VOLUME_ENTITY: u32 = 166; // 0xa6
pub const ID_REMOVE_VOLUME_ENTITY: u32 = 167; // 0xa7

/// AddVolumeEntity packet (ID = 166 / 0xa6).
/// Used to define sound/fog volumes in Bedrock.
pub struct AddVolumeEntity {
    pub entity_net_id: u32,
    pub data_nbt: Vec<u8>,
    pub json_identifier: String,
    pub instance_name: String,
    pub min_bound: (i32, i32, i32),
    pub max_bound: (i32, i32, i32),
    pub dimension: i32,
    pub engine_version: String,
}

impl AddVolumeEntity {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();

        write_varu32(&mut buf, self.entity_net_id);
        buf.extend_from_slice(&self.data_nbt);
        write_string(&mut buf, &self.json_identifier);
        write_string(&mut buf, &self.instance_name);

        write_vari32(&mut buf, self.min_bound.0);
        write_vari32(&mut buf, self.min_bound.1);
        write_vari32(&mut buf, self.min_bound.2);

        write_vari32(&mut buf, self.max_bound.0);
        write_vari32(&mut buf, self.max_bound.1);
        write_vari32(&mut buf, self.max_bound.2);

        write_vari32(&mut buf, self.dimension);
        write_string(&mut buf, &self.engine_version);

        Ok(buf)
    }
}

/// RemoveVolumeEntity packet (ID = 167 / 0xa7).
pub struct RemoveVolumeEntity {
    pub entity_net_id: u32,
    pub dimension: i32,
}

impl RemoveVolumeEntity {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_varu32(&mut buf, self.entity_net_id);
        write_vari32(&mut buf, self.dimension);
        Ok(buf)
    }
}
