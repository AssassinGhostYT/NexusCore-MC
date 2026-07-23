// ServerBoundLoadingScreen — ID 312
// Sent by the client during the loading screen phase.
//
// Wire format (BedrockProtocol / v1001):
//   field 0: loadingScreenType  — VarInt::readSignedInt (zigzag vari32)
//               0 = UNKNOWN
//               1 = START_LOADING_SCREEN
//               2 = STOP_LOADING_SCREEN
//   field 1: loadingScreenId    — Optional<u32 LE>
//               First byte is a boolean: 0x00 = None, 0x01 = Some
//               If Some: next 4 bytes = u32 LE (the loading screen ID)

use crate::protocol::error::PResult;
use crate::protocol::varint::read_vari32;
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingScreenType {
    Unknown,
    StartLoadingScreen,
    StopLoadingScreen,
}

impl LoadingScreenType {
    pub fn from_i32(v: i32) -> Self {
        match v {
            1 => Self::StartLoadingScreen,
            2 => Self::StopLoadingScreen,
            _ => Self::Unknown,
        }
    }
}

pub struct ServerBoundLoadingScreen {
    pub loading_screen_type: LoadingScreenType,
    pub screen_id: Option<u32>,
}

impl ServerBoundLoadingScreen {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        let hex: Vec<String> = payload.iter().map(|b| format!("{:02x}", b)).collect();
        log::info!(
            "ServerBoundLoadingScreen::read: payload length={}, bytes=[{}]",
            payload.len(),
            hex.join(" ")
        );

        let mut buf = payload;

        // field 0: signed zigzag varint
        let type_raw = read_vari32(&mut buf).unwrap_or(0);
        let loading_screen_type = LoadingScreenType::from_i32(type_raw);
        log::info!(
            "ServerBoundLoadingScreen::read: type_raw={} -> {:?}, remaining={}",
            type_raw,
            loading_screen_type,
            buf.len()
        );

        // field 1: Optional<u32 LE> — prefixed with a boolean byte
        let screen_id = if !buf.is_empty() {
            let has_value = buf[0] != 0;
            buf = &buf[1..];
            if has_value && buf.len() >= 4 {
            let id = (&buf[..4]).read_u32::<LittleEndian>().ok();
                log::info!("ServerBoundLoadingScreen::read: screen_id=Some({:?})", id);
                id
            } else {
                log::info!("ServerBoundLoadingScreen::read: screen_id=None (has_value={})", has_value);
                None
            }
        } else {
            log::info!("ServerBoundLoadingScreen::read: screen_id=None (buffer empty)");
            None
        };

        Ok(Self { loading_screen_type, screen_id })
    }
}
