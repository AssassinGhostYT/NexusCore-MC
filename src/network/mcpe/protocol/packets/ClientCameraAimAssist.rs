pub const ID_CLIENT_CAMERA_AIM_ASSIST: u32 = 321;

use crate::protocol::error::{PResult, PacketError};
use crate::macros::helpers;
use byteorder::ReadBytesExt;

pub struct ClientCameraAimAssist {
    pub preset_id: String,
    pub action: u8,
    pub allow_aim_assist: bool,
}

impl ClientCameraAimAssist {
    pub fn read(buf: &mut &[u8]) -> PResult<Self> {
        let preset_id = helpers::read_string(buf).ok_or(PacketError::Format {
            packet: "ClientCameraAimAssist",
            detail: "failed to read preset_id".to_string(),
        })?;
        let action = buf.read_u8()?;
        let allow_aim_assist = buf.read_u8()? != 0;
        Ok(Self {
            preset_id,
            action,
            allow_aim_assist,
        })
    }
}
