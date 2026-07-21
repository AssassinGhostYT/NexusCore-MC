/// BlockPalette: Serializes the block palette for the StartGame packet.
///
/// In protocol v1001+ the client uses a built-in palette, so we send an empty
/// palette (count=0) — client uses built-in palette. The full block_states.nbt
/// is only needed for older protocol versions or custom blocks.
use crate::protocol::varint::write_varu32;

/// Writes the block palette into the StartGame packet.
/// Sends count=0 (empty) — client uses built-in palette.
pub fn write_block_palette(buf: &mut Vec<u8>) {
    log::debug!("[BlockPalette] Sending empty palette (client uses built-in)");
    write_varu32(buf, 0);
}

/// Counts the number of TAG_Compound (0x0a) root entries in a network-NBT stream.
fn count_nbt_entries(data: &[u8]) -> usize {
    let mut pos = 0;
    let mut count = 0;

    while pos < data.len() {
        // Each entry starts with TAG_Compound (0x0a)
        if data[pos] != 0x0a {
            break;
        }
        pos += 1;

        // Root compound name (varuint32 length + bytes)
        let (name_len, n) = read_varu32_at(data, pos);
        pos += n + name_len as usize;

        // Skip fields until TAG_End (0x00)
        match skip_compound(data, pos) {
            Some(new_pos) => pos = new_pos,
            None => break,
        }

        count += 1;
    }

    count
}

/// Skips a compound's fields until TAG_End and returns the new position.
fn skip_compound(data: &[u8], mut pos: usize) -> Option<usize> {
    loop {
        if pos >= data.len() {
            return None;
        }
        let tag = data[pos];
        pos += 1;
        if tag == 0 {
            // TAG_End
            return Some(pos);
        }
        // Skip field name (varuint32 len + bytes)
        let (name_len, n) = read_varu32_at(data, pos);
        pos += n + name_len as usize;
        // Skip field value
        pos = skip_tag(data, tag, pos)?;
    }
}

/// Skips a tag value given its type byte. Returns new position or None on error.
fn skip_tag(data: &[u8], tag: u8, mut pos: usize) -> Option<usize> {
    match tag {
        0 => Some(pos),          // TAG_End (should not appear here)
        1 => { pos += 1; Some(pos) }  // TAG_Byte
        2 => { pos += 2; Some(pos) }  // TAG_Short
        3 => {                         // TAG_Int (zigzag varint in network NBT)
            let (_, n) = read_varu32_at(data, pos);
            Some(pos + n)
        }
        4 => { pos += 8; Some(pos) }  // TAG_Long (fixed 8 bytes LE in network NBT)
        5 => { pos += 4; Some(pos) }  // TAG_Float
        6 => { pos += 8; Some(pos) }  // TAG_Double
        7 => {                         // TAG_ByteArray
            let (len, n) = read_varu32_at(data, pos);
            Some(pos + n + len as usize)
        }
        8 => {                         // TAG_String
            let (len, n) = read_varu32_at(data, pos);
            Some(pos + n + len as usize)
        }
        9 => {                         // TAG_List
            if pos >= data.len() { return None; }
            let elem_type = data[pos]; pos += 1;
            let (count, n) = read_varu32_at(data, pos);
            pos += n;
            // Zigzag decode for signed count
            let count_signed = ((count >> 1) as i32) ^ -((count & 1) as i32);
            let count_u = if count_signed < 0 { 0 } else { count_signed as usize };
            for _ in 0..count_u {
                pos = skip_tag(data, elem_type, pos)?;
            }
            Some(pos)
        }
        10 => skip_compound(data, pos), // TAG_Compound
        11 => {                          // TAG_IntArray
            let (len, n) = read_varu32_at(data, pos);
            Some(pos + n + len as usize * 4)
        }
        12 => {                          // TAG_LongArray
            let (len, n) = read_varu32_at(data, pos);
            Some(pos + n + len as usize * 8)
        }
        _ => {
            log::warn!("[BlockPalette] Unknown NBT tag type 0x{:02x} at pos {}", tag, pos);
            None
        }
    }
}

/// Reads a varuint32 from `data` at `pos`. Returns (value, bytes_consumed).
fn read_varu32_at(data: &[u8], mut pos: usize) -> (u32, usize) {
    let start = pos;
    let mut val: u32 = 0;
    let mut shift = 0u32;
    loop {
        if pos >= data.len() { break; }
        let b = data[pos]; pos += 1;
        val |= ((b & 0x7f) as u32) << shift;
        if (b & 0x80) == 0 { break; }
        shift += 7;
        if shift >= 32 { break; }
    }
    (val, pos - start)
}
