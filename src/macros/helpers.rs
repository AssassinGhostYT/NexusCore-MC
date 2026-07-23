use crate::network::mcpe::protocol::packets::varint::{read_varu32, write_varu32};

pub fn read_string(buf: &mut &[u8]) -> Option<String> {
    let len = read_varu32(buf)? as usize;
    if len > buf.len() { return None; }
    let s = std::str::from_utf8(&buf[..len]).ok()?.to_string();
    *buf = &buf[len..];
    Some(s)
}

pub fn write_string(buf: &mut Vec<u8>, s: &str) {
    write_varu32(buf, s.len() as u32);
    buf.extend_from_slice(s.as_bytes());
}

pub fn read_bytes(buf: &mut &[u8]) -> Option<Vec<u8>> {
    let len = read_varu32(buf)? as usize;
    if len > buf.len() { return None; }
    let data = buf[..len].to_vec();
    *buf = &buf[len..];
    Some(data)
}

pub fn write_bytes(buf: &mut Vec<u8>, data: &[u8]) {
    write_varu32(buf, data.len() as u32);
    buf.extend_from_slice(data);
}

pub fn read_varu32_le(buf: &mut &[u8]) -> Option<u32> {
    use byteorder::ReadBytesExt;
    use byteorder::LittleEndian;
    let val = buf.read_u32::<LittleEndian>().ok()?;
    Some(val)
}

pub fn read_vari32_le(buf: &mut &[u8]) -> Option<i32> {
    use byteorder::ReadBytesExt;
    use byteorder::LittleEndian;
    let val = buf.read_i32::<LittleEndian>().ok()?;
    Some(val)
}
