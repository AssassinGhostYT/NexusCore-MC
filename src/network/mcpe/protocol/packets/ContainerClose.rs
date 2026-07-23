use byteorder::ReadBytesExt;
use crate::protocol::error::PResult;

pub struct ContainerClose {
    pub window_id: i8,
    pub server_side: bool,
}

impl ContainerClose {
    pub fn read(buf: &mut &[u8]) -> PResult<Self> {
        let window_id = buf.read_i8().unwrap_or(0);
        let server_side = buf.read_u8().unwrap_or(0) != 0;
        Ok(Self { window_id, server_side })
    }
}
