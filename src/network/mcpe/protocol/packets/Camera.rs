// Camera — ID 73
use crate::protocol::error::PResult;
use crate::protocol::varint::write_vari64;

pub struct Camera {
    pub camera_entity_unique_id: i64,
    pub target_player_unique_id: i64,
}

impl Camera {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_vari64(&mut buf, self.camera_entity_unique_id);
        write_vari64(&mut buf, self.target_player_unique_id);
        Ok(buf)
    }
}
