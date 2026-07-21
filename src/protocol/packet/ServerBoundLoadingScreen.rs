// ServerBoundLoadingScreen — ID 312
// Sent by the client during the loading screen phase.
// The server can use this to know when the client has finished loading.

use crate::protocol::error::PResult;
use crate::protocol::varint::read_varu32;

#[derive(Debug)]
pub enum LoadingScreenEvent {
    Unknown(u32),
}

pub struct ServerBoundLoadingScreen {
    pub event_type: u32,
    pub screen_id: Option<u32>,
}

impl ServerBoundLoadingScreen {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        let hex: Vec<String> = payload.iter().map(|b| format!("{:02x}", b)).collect();
        log::info!("ServerBoundLoadingScreen::read: input payload length={}, bytes=[{}]", payload.len(), hex.join(" "));

        let mut buf = payload;
        let event_type = read_varu32(&mut buf).unwrap_or(0);
        log::info!("ServerBoundLoadingScreen::read: parsed event_type={}, remaining_bytes={}", event_type, buf.len());

        let screen_id = if !buf.is_empty() {
            let parsed = read_varu32(&mut buf);
            log::info!("ServerBoundLoadingScreen::read: parsed screen_id={:?}", parsed);
            parsed
        } else {
            log::info!("ServerBoundLoadingScreen::read: no screen_id present (buffer empty)");
            None
        };

        Ok(Self { event_type, screen_id })
    }
}
