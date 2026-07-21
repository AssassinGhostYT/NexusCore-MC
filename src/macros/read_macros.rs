// Macros for reading/writing packet primitives with proper error types.
// Usage: r_i32be!(buf, "field_name") instead of buf.read_i32::<BigEndian>().ok()?
#[macro_export]
macro_rules! r_i32be {
    ($buf:expr, $field:expr) => {{
        use byteorder::ReadBytesExt;
        use byteorder::BigEndian;
        $buf.read_i32::<BigEndian>().map_err(|e| {
            log::error!("read {} failed: {}", $field, e);
            $crate::protocol::error::PacketError::Io { context: $field, source: e }
        })?
    }};
}

#[macro_export]
macro_rules! r_i32le {
    ($buf:expr, $field:expr) => {{
        use byteorder::ReadBytesExt;
        use byteorder::LittleEndian;
        $buf.read_i32::<LittleEndian>().map_err(|e| {
            log::error!("read {} failed: {}", $field, e);
            $crate::protocol::error::PacketError::Io { context: $field, source: e }
        })?
    }};
}

#[macro_export]
macro_rules! r_u32le {
    ($buf:expr, $field:expr) => {{
        use byteorder::ReadBytesExt;
        use byteorder::LittleEndian;
        $buf.read_u32::<LittleEndian>().map_err(|e| {
            log::error!("read {} failed: {}", $field, e);
            $crate::protocol::error::PacketError::Io { context: $field, source: e }
        })?
    }};
}

#[macro_export]
macro_rules! r_u16le {
    ($buf:expr, $field:expr) => {{
        use byteorder::ReadBytesExt;
        use byteorder::LittleEndian;
        $buf.read_u16::<LittleEndian>().map_err(|e| {
            log::error!("read {} failed: {}", $field, e);
            $crate::protocol::error::PacketError::Io { context: $field, source: e }
        })?
    }};
}

#[macro_export]
macro_rules! r_f32le {
    ($buf:expr, $field:expr) => {{
        use byteorder::ReadBytesExt;
        use byteorder::LittleEndian;
        $buf.read_f32::<LittleEndian>().map_err(|e| {
            log::error!("read {} failed: {}", $field, e);
            $crate::protocol::error::PacketError::Io { context: $field, source: e }
        })?
    }};
}

#[macro_export]
macro_rules! r_i8 {
    ($buf:expr, $field:expr) => {{
        use byteorder::ReadBytesExt;
        $buf.read_i8().map_err(|e| {
            log::error!("read {} failed: {}", $field, e);
            $crate::protocol::error::PacketError::Io { context: $field, source: e }
        })?
    }};
}

#[macro_export]
macro_rules! r_u8 {
    ($buf:expr, $field:expr) => {{
        use byteorder::ReadBytesExt;
        $buf.read_u8().map_err(|e| {
            log::error!("read {} failed: {}", $field, e);
            $crate::protocol::error::PacketError::Io { context: $field, source: e }
        })?
    }};
}

// Read a varint-prefixed string
#[macro_export]
macro_rules! r_string {
    ($buf:expr, $field:expr) => {{
        $crate::macros::helpers::read_string($buf).ok_or_else(|| {
            let msg = format!("failed to read string field '{}'", $field);
            log::error!("{}", msg);
            $crate::protocol::error::PacketError::Format { packet: "packet", detail: msg }
        })?
    }};
}

// Read varint-prefixed bytes
#[macro_export]
macro_rules! r_bytes {
    ($buf:expr, $field:expr) => {{
        $crate::macros::helpers::read_bytes($buf).ok_or_else(|| {
            let msg = format!("failed to read bytes field '{}'", $field);
            log::error!("{}", msg);
            $crate::protocol::error::PacketError::Format { packet: "packet", detail: msg }
        })?
    }};
}

// Read u32 varint
#[macro_export]
macro_rules! r_varu32 {
    ($buf:expr, $field:expr) => {{
        $crate::protocol::varint::read_varu32($buf).ok_or_else(|| {
            log::error!("read varu32 '{}' failed: buffer exhausted", $field);
            $crate::protocol::error::PacketError::VarintOverflow { kind: "u32" }
        })?
    }};
}

// Read i32 varint
#[macro_export]
macro_rules! r_vari32 {
    ($buf:expr, $field:expr) => {{
        $crate::protocol::varint::read_vari32($buf).ok_or_else(|| {
            log::error!("read vari32 '{}' failed: buffer exhausted", $field);
            $crate::protocol::error::PacketError::VarintOverflow { kind: "i32" }
        })?
    }};
}

// Read u64 varint
#[macro_export]
macro_rules! r_varu64 {
    ($buf:expr, $field:expr) => {{
        $crate::protocol::varint::read_varu64($buf).ok_or_else(|| {
            log::error!("read varu64 '{}' failed: buffer exhausted", $field);
            $crate::protocol::error::PacketError::VarintOverflow { kind: "u64" }
        })?
    }};
}

// Read i64 varint
#[macro_export]
macro_rules! r_vari64 {
    ($buf:expr, $field:expr) => {{
        $crate::protocol::varint::read_vari64($buf).ok_or_else(|| {
            log::error!("read vari64 '{}' failed: buffer exhausted", $field);
            $crate::protocol::error::PacketError::VarintOverflow { kind: "i64" }
        })?
    }};
}

// Read raw u32 LE
#[macro_export]
macro_rules! r_u32le_raw {
    ($buf:expr, $field:expr) => {{
        use byteorder::ReadBytesExt;
        use byteorder::LittleEndian;
        $buf.read_u32::<LittleEndian>().map_err(|e| {
            log::error!("read {} failed: {}", $field, e);
            $crate::protocol::error::PacketError::Io { context: $field, source: e }
        })?
    }};
}

// Check buffer has enough bytes, return error if not
#[macro_export]
macro_rules! r_check {
    ($buf:expr, $need:expr, $field:expr) => {{
        if $buf.len() < $need {
            let msg = format!("not enough bytes for '{}': need {}, have {}", $field, $need, $buf.len());
            log::error!("{}", msg);
            return Err($crate::protocol::error::PacketError::Underflow {
                field: $field,
                need: $need,
                have: $buf.len(),
            });
        }
    }};
}

// ─── Write macros ──────────────────────────────────────────────────────────────

#[macro_export]
macro_rules! w_i32le {
    ($buf:expr, $val:expr) => {{
        use byteorder::WriteBytesExt;
        use byteorder::LittleEndian;
        $buf.write_i32::<LittleEndian>($val).unwrap();
    }};
}

#[macro_export]
macro_rules! w_u32le {
    ($buf:expr, $val:expr) => {{
        use byteorder::WriteBytesExt;
        use byteorder::LittleEndian;
        $buf.write_u32::<LittleEndian>($val).unwrap();
    }};
}

#[macro_export]
macro_rules! w_u16le {
    ($buf:expr, $val:expr) => {{
        use byteorder::WriteBytesExt;
        use byteorder::LittleEndian;
        $buf.write_u16::<LittleEndian>($val).unwrap();
    }};
}

#[macro_export]
macro_rules! w_f32le {
    ($buf:expr, $val:expr) => {{
        use byteorder::WriteBytesExt;
        use byteorder::LittleEndian;
        $buf.write_f32::<LittleEndian>($val).unwrap();
    }};
}

#[macro_export]
macro_rules! w_u64le {
    ($buf:expr, $val:expr) => {{
        use byteorder::WriteBytesExt;
        use byteorder::LittleEndian;
        $buf.write_u64::<LittleEndian>($val).unwrap();
    }};
}

#[macro_export]
macro_rules! w_i64le {
    ($buf:expr, $val:expr) => {{
        use byteorder::WriteBytesExt;
        use byteorder::LittleEndian;
        $buf.write_i64::<LittleEndian>($val).unwrap();
    }};
}

#[macro_export]
macro_rules! w_varu32 {
    ($buf:expr, $val:expr) => {{
        $crate::protocol::varint::write_varu32($buf, $val);
    }};
}

#[macro_export]
macro_rules! w_vari32 {
    ($buf:expr, $val:expr) => {{
        $crate::protocol::varint::write_vari32($buf, $val);
    }};
}

#[macro_export]
macro_rules! w_varu64 {
    ($buf:expr, $val:expr) => {{
        $crate::protocol::varint::write_varu64($buf, $val);
    }};
}

#[macro_export]
macro_rules! w_vari64 {
    ($buf:expr, $val:expr) => {{
        $crate::protocol::varint::write_vari64($buf, $val);
    }};
}

#[macro_export]
macro_rules! w_string {
    ($buf:expr, $val:expr) => {{
        $crate::macros::helpers::write_string($buf, $val);
    }};
}

#[macro_export]
macro_rules! w_bool {
    ($buf:expr, $val:expr) => {
        $buf.push(if $val { 1 } else { 0 });
    };
}
