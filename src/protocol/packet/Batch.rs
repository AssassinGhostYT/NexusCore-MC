use crate::protocol::varint::{read_varu32, write_varu32};
use super::helpers::{compress_deflate, decompress_deflate};

pub const ID_GAME_PACKET: u8 = 0xfe;

#[derive(Debug, Clone)]
pub struct GamePacket {
    pub id: u32,
    pub sender_subclient: u8,
    pub recipient_subclient: u8,
    pub payload: Vec<u8>,
}

/// Decodes a game packet batch.
/// If `compressed` is true, it will decompress the batch first.
pub fn decode_batch(mut data: &[u8], compressed: bool) -> std::io::Result<Vec<GamePacket>> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    // Verify game packet header (0xfe)
    if data[0] != ID_GAME_PACKET {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid game packet header: 0x{:02x}", data[0]),
        ));
    }
    data = &data[1..];

    let batch_data = if compressed {
        if data.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Compressed batch missing compression algorithm byte",
            ));
        }
        let algorithm = data[0];
        if algorithm == 0xff {
            data[1..].to_vec()
        } else if algorithm != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unsupported compression algorithm: {}", algorithm),
            ));
        } else {
            decompress_deflate(&data[1..])?
        }
    } else {
        data.to_vec()
    };

    let mut reader = &batch_data[..];
    let mut packets = Vec::new();

    while !reader.is_empty() {
        // Each packet inside the batch is prefixed by its length as a VarInt
        let length = read_varu32(&mut reader).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to read packet length VarInt")
        })? as usize;

        if reader.len() < length {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Incomplete packet inside batch",
            ));
        }

        let mut packet_data = &reader[..length];
        reader = &reader[length..];

        // Parse packet header: VarInt containing packet ID and subclient flags
        let header = read_varu32(&mut packet_data).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to read packet header VarInt")
        })?;

        let id = header & 0x3ff;
        let sender_subclient = ((header >> 10) & 0x03) as u8;
        let recipient_subclient = ((header >> 12) & 0x03) as u8;

        packets.push(GamePacket {
            id,
            sender_subclient,
            recipient_subclient,
            payload: packet_data.to_vec(),
        });
    }

    Ok(packets)
}

/// Encodes a list of game packets into a batch.
/// If `compressed` is true, the batch will be compressed.
pub fn encode_batch(packets: &[GamePacket], compressed: bool) -> std::io::Result<Vec<u8>> {
    let mut batch_data = Vec::new();

    for packet in packets {
        let mut packet_buf = Vec::new();

        // Encode header VarInt
        let header = (packet.id & 0x3ff)
            | ((packet.sender_subclient as u32 & 0x03) << 10)
            | ((packet.recipient_subclient as u32 & 0x03) << 12);
        
        write_varu32(&mut packet_buf, header);
        packet_buf.extend_from_slice(&packet.payload);

        // Prefix length as VarInt
        write_varu32(&mut batch_data, packet_buf.len() as u32);
        batch_data.extend_from_slice(&packet_buf);
    }

    let mut result = Vec::new();
    result.push(ID_GAME_PACKET);

    if compressed {
        let compressed_data = compress_deflate(&batch_data)?;
        result.push(0x00); // Compression algorithm byte: Zlib/Deflate (0x00)
        result.extend_from_slice(&compressed_data);
    } else {
        result.extend_from_slice(&batch_data);
    }

    Ok(result)
}
