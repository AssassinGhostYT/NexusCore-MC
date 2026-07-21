use flate2::read::{ZlibDecoder, DeflateDecoder};
use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::{Read, Write};

use crate::protocol::error::{PacketError, PResult};
use crate::protocol::varint::{read_varu32, write_varu32};

pub struct GamePacket {
    pub id: u32,
    pub sender_subclient: u8,
    pub recipient_subclient: u8,
    pub payload: Vec<u8>,
}

const ID_GAME_PACKET: u8 = 0xFE;
const COMPRESSION_ZLIB: u8 = 0x00;
#[allow(dead_code)]
const COMPRESSION_SNAPPY: u8 = 0x01;
const COMPRESSION_NONE: u8 = 0xFF;

/// Decode a batch of game packets.
/// The `data` includes the 0xFE header byte.
/// If `compressed` is true, data after 0xFE has a compression method byte.
pub fn decode_batch(data: &[u8], compressed: bool) -> PResult<Vec<GamePacket>> {
    if data.is_empty() {
        return Ok(Vec::new());
    }


    if data[0] != ID_GAME_PACKET {
        return Err(PacketError::Format {
            packet: "batch".into(),
            detail: format!("invalid game packet header: 0x{:02x}", data[0]),
        });
    }

    let batch_data = if compressed {
        if data.len() < 2 {
            return Err(PacketError::Format {
                packet: "batch".into(),
                detail: "compressed batch too short".into(),
            });
        }
        let algorithm = data[1];
        match algorithm {
            COMPRESSION_NONE => data[2..].to_vec(),
            COMPRESSION_ZLIB => {
                let compressed_data = &data[2..];
                // Try raw deflate first (client v1001 uses raw deflate, not zlib)
                let mut decoder = DeflateDecoder::new(compressed_data);
                let mut decompressed = Vec::new();
                match decoder.read_to_end(&mut decompressed) {
                    Ok(_) => decompressed,
                    Err(_) => {
                        // Fallback to zlib
                        let mut decoder = ZlibDecoder::new(compressed_data);
                        let mut decompressed = Vec::new();
                        decoder.read_to_end(&mut decompressed).map_err(|e| {
                            PacketError::Io { context: "batch zlib decompression", source: e }
                        })?;
                        decompressed
                    }
                }
            }
            other => {
                return Err(PacketError::Format {
                    packet: "batch".into(),
                    detail: format!("unknown compression method: 0x{:02x}", other),
                });
            }
        }
    } else {
        data[1..].to_vec()
    };

    // Parse sub-packets: [varint: length] [varint: header] [body...]
    let mut reader = &batch_data[..];
    let mut packets = Vec::new();

    while !reader.is_empty() {
        let packet_length = read_varu32(&mut reader).ok_or_else(|| {
            PacketError::Format { packet: "batch".into(), detail: "failed to read packet length".into() }
        })? as usize;

        if reader.len() < packet_length {
            return Err(PacketError::Format {
                packet: "batch".into(),
                detail: format!("packet length {} exceeds remaining bytes {}", packet_length, reader.len()),
            });
        }

        let packet_data = &reader[..packet_length];
        reader = &reader[packet_length..];

        let mut packet_reader = &packet_data[..];
        let header = read_varu32(&mut packet_reader).ok_or_else(|| {
            PacketError::Format { packet: "batch".into(), detail: "failed to read packet header".into() }
        })?;

        // packet_id (10 bits) | sender_sub_client (2 bits) | target_sub_client (2 bits)
        let id = header & 0x3FF;
        let sender_subclient = ((header >> 10) & 0x03) as u8;
        let recipient_subclient = ((header >> 12) & 0x03) as u8;

        let payload = packet_reader.to_vec();

        packets.push(GamePacket { id, sender_subclient, recipient_subclient, payload });
    }

    Ok(packets)
}

pub fn encode_batch(packets: &[GamePacket], compression_enabled: bool) -> Vec<u8> {
    let mut inner = Vec::new();

    for packet in packets {
        let header = (packet.id & 0x3FF)
            | ((packet.sender_subclient as u32 & 0x03) << 10)
            | ((packet.recipient_subclient as u32 & 0x03) << 12);

        let mut packet_buf = Vec::new();
        write_varu32(&mut packet_buf, header);
        packet_buf.extend_from_slice(&packet.payload);

        write_varu32(&mut inner, packet_buf.len() as u32);
        inner.extend_from_slice(&packet_buf);
    }

    let mut result = Vec::new();
    result.push(ID_GAME_PACKET); // 0xFE

    if compression_enabled {
        const COMPRESSION_THRESHOLD: usize = 256;
        if inner.len() >= COMPRESSION_THRESHOLD {
            // Compress with zlib deflate, algorithm = 0x00
            result.push(COMPRESSION_ZLIB);
            match compress_deflate(&inner) {
                Ok(compressed) => result.extend_from_slice(&compressed),
                Err(_) => {
                    // Fallback: send uncompressed if compression fails
                    result[1] = COMPRESSION_NONE;
                    result.extend_from_slice(&inner);
                }
            }
        } else {
            // Below threshold: no compression, algorithm = 0xFF
            result.push(COMPRESSION_NONE);
            result.extend_from_slice(&inner);
        }
    } else {
        result.extend_from_slice(&inner);
    }

    result
}


pub fn compress_deflate(data: &[u8]) -> PResult<Vec<u8>> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).map_err(|e| {
        PacketError::Io { context: "deflate compression", source: e }
    })?;
    encoder.finish().map_err(|e| {
        PacketError::Io { context: "deflate compression finish", source: e }
    })
}
