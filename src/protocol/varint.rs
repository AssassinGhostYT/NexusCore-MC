#![allow(dead_code)]

pub fn read_varu32(buf: &mut &[u8]) -> Option<u32> {
    let mut value: u32 = 0;
    let mut shift: u32 = 0;
    loop {
        if buf.is_empty() {
            return None;
        }
        let byte = buf[0];
        *buf = &buf[1..];
        value |= ((byte & 0x7F) as u32) << shift;
        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;
        if shift >= 32 {
            return None;
        }
    }
    Some(value)
}

pub fn write_varu32(buf: &mut Vec<u8>, mut value: u32) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if value == 0 {
            break;
        }
    }
}

pub fn read_vari32(buf: &mut &[u8]) -> Option<i32> {
    let raw = read_varu32(buf)?;
    Some(((raw >> 1) as i32) ^ -((raw & 1) as i32))
}

pub fn write_vari32(buf: &mut Vec<u8>, value: i32) {
    let raw = ((value << 1) ^ (value >> 31)) as u32;
    write_varu32(buf, raw);
}

pub fn read_varu64(buf: &mut &[u8]) -> Option<u64> {
    let mut value: u64 = 0;
    let mut shift: u32 = 0;
    loop {
        if buf.is_empty() {
            return None;
        }
        let byte = buf[0];
        *buf = &buf[1..];
        value |= ((byte & 0x7F) as u64) << shift;
        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return None;
        }
    }
    Some(value)
}

pub fn write_varu64(buf: &mut Vec<u8>, mut value: u64) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if value == 0 {
            break;
        }
    }
}

pub fn read_vari64(buf: &mut &[u8]) -> Option<i64> {
    let raw = read_varu64(buf)?;
    Some(((raw >> 1) as i64) ^ -((raw & 1) as i64))
}

pub fn write_vari64(buf: &mut Vec<u8>, value: i64) {
    let raw = ((value << 1) ^ (value >> 63)) as u64;
    write_varu64(buf, raw);
}
