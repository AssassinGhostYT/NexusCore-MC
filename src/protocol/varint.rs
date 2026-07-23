pub fn read_varu32(buf: &mut &[u8]) -> Option<u32> {
    let mut value = 0u32;
    let mut shift = 0;
    while !buf.is_empty() {
        let b = buf[0];
        *buf = &buf[1..];
        value |= ((b & 0x7f) as u32) << shift;
        if (b & 0x80) == 0 {
            return Some(value);
        }
        shift += 7;
        if shift >= 35 {
            return None;
        }
    }
    None
}

pub fn write_varu32(buf: &mut Vec<u8>, mut val: u32) {
    while val >= 0x80 {
        buf.push((val as u8 & 0x7f) | 0x80);
        val >>= 7;
    }
    buf.push(val as u8);
}

pub fn read_vari32(buf: &mut &[u8]) -> Option<i32> {
    let raw = read_varu32(buf)?;
    Some(((raw >> 1) as i32) ^ (-((raw & 1) as i32)))
}

pub fn write_vari32(buf: &mut Vec<u8>, val: i32) {
    let raw = ((val << 1) ^ (val >> 31)) as u32;
    write_varu32(buf, raw);
}

pub fn read_varu64(buf: &mut &[u8]) -> Option<u64> {
    let mut value = 0u64;
    let mut shift = 0;
    while !buf.is_empty() {
        let b = buf[0];
        *buf = &buf[1..];
        value |= ((b & 0x7f) as u64) << shift;
        if (b & 0x80) == 0 {
            return Some(value);
        }
        shift += 7;
        if shift >= 70 {
            return None;
        }
    }
    None
}

pub fn write_varu64(buf: &mut Vec<u8>, mut val: u64) {
    while val >= 0x80 {
        buf.push((val as u8 & 0x7f) | 0x80);
        val >>= 7;
    }
    buf.push(val as u8);
}

pub fn read_vari64(buf: &mut &[u8]) -> Option<i64> {
    let raw = read_varu64(buf)?;
    Some(((raw >> 1) as i64) ^ (-((raw & 1) as i64)))
}

pub fn write_vari64(buf: &mut Vec<u8>, val: i64) {
    let raw = ((val << 1) ^ (val >> 63)) as u64;
    write_varu64(buf, raw);
}
