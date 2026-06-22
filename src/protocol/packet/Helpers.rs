use std::io::{Read, Write};
use flate2::Compression;
use flate2::write::DeflateEncoder;
use flate2::read::DeflateDecoder;
use crate::protocol::varint::{read_varu32, write_varu32};

pub fn read_string(buf: &mut &[u8]) -> Option<String> {
    let len = read_varu32(buf)? as usize;
    if buf.len() < len {
        return None;
    }
    let s = std::str::from_utf8(&buf[..len]).ok()?;
    *buf = &buf[len..];
    Some(s.to_string())
}

pub fn write_string(buf: &mut Vec<u8>, s: &str) {
    write_varu32(buf, s.len() as u32);
    buf.extend_from_slice(s.as_bytes());
}

/// Decompresses raw deflate data.
pub fn decompress_deflate(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut decoder = DeflateDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

/// Compresses data to raw deflate format.
pub fn compress_deflate(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

