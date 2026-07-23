// ServerBoundDiagnostics — ID 315 (0x13b)
pub const ID_SERVER_BOUND_DIAGNOSTICS: u32 = 315;
// Sent by the client to report performance and diagnostic information to the server.

use crate::protocol::error::PResult;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct ServerBoundDiagnostics {
    pub avg_fps: f32,
    pub avg_server_tick_time_ms: f32,
    pub avg_client_tick_time_ms: f32,
}

impl ServerBoundDiagnostics {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        let mut reader = payload;
        let avg_fps = reader.read_f32::<LittleEndian>().unwrap_or(0.0);
        let avg_server_tick_time_ms = reader.read_f32::<LittleEndian>().unwrap_or(0.0);
        let avg_client_tick_time_ms = reader.read_f32::<LittleEndian>().unwrap_or(0.0);
        Ok(Self {
            avg_fps,
            avg_server_tick_time_ms,
            avg_client_tick_time_ms,
        })
    }
}
