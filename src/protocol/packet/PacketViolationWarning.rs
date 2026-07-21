use crate::protocol::error::PResult;
use crate::protocol::varint::read_vari32;
use crate::macros::helpers;

pub struct PacketViolationWarning {
    pub packet_id: i32,
    pub severity: i32,
    pub context: String,
}

impl PacketViolationWarning {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        let mut buf = &payload[..];
        let packet_id = read_vari32(&mut buf).ok_or_else(|| {
            crate::protocol::error::PacketError::VarintOverflow { kind: "PacketViolationWarning.packet_id" }
        })? as i32;
        let severity = read_vari32(&mut buf).ok_or_else(|| {
            crate::protocol::error::PacketError::VarintOverflow { kind: "PacketViolationWarning.severity" }
        })? as i32;
        let _violation_type = read_vari32(&mut buf).ok_or_else(|| {
            crate::protocol::error::PacketError::VarintOverflow { kind: "PacketViolationWarning.violation_type" }
        })? as i32;
        let context = helpers::read_string(&mut buf).unwrap_or_default();
        Ok(Self { packet_id, severity, context })
    }
}
