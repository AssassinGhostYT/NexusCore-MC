use std::io::{Read, Cursor};
use byteorder::{ReadBytesExt, LittleEndian};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NbtTag {
    Byte(u8),
    Short(i32),
    Int(i32),
    Long(i32),
    Float(u32),
    Double(u64),
    String(String),
    Compound(Vec<(String, NbtTag)>),
}

fn read_varu32<R: Read>(reader: &mut R) -> std::io::Result<u32> {
    let mut val = 0;
    let mut shift = 0;
    loop {
        let b = reader.read_u8()?;
        val |= ((b & 0x7f) as u32) << shift;
        if (b & 0x80) == 0 {
            break;
        }
        shift += 7;
    }
    Ok(val)
}

fn read_vari32<R: Read>(reader: &mut R) -> std::io::Result<i32> {
    let u_val = read_varu32(reader)?;
    let val = ((u_val >> 1) as i32) ^ -((u_val & 1) as i32);
    Ok(val)
}

fn read_string<R: Read>(reader: &mut R) -> std::io::Result<String> {
    let len = read_varu32(reader)? as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

pub fn read_tag<R: Read>(tag_type: u8, reader: &mut R) -> std::io::Result<NbtTag> {
    match tag_type {
        1 => Ok(NbtTag::Byte(reader.read_u8()?)),
        2 => Ok(NbtTag::Short(read_vari32(reader)?)),
        3 => Ok(NbtTag::Int(read_vari32(reader)?)),
        4 => Ok(NbtTag::Long(read_vari32(reader)?)),
        5 => {
            let bits = reader.read_u32::<LittleEndian>()?;
            Ok(NbtTag::Float(bits))
        }
        6 => {
            let bits = reader.read_u64::<LittleEndian>()?;
            Ok(NbtTag::Double(bits))
        }
        8 => Ok(NbtTag::String(read_string(reader)?)),
        10 => {
            let mut comp = Vec::new();
            loop {
                let t = reader.read_u8()?;
                if t == 0 {
                    break;
                }
                let name = read_string(reader)?;
                let val = read_tag(t, reader)?;
                comp.push((name, val));
            }
            Ok(NbtTag::Compound(comp))
        }
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Unsupported tag type {}", tag_type),
        )),
    }
}

#[derive(Debug, Clone)]
pub struct BlockState {
    pub name: String,
    pub properties: Vec<(String, NbtTag)>,
    pub version: i32,
}

pub fn parse_block_states_nbt(data: &[u8]) -> Vec<BlockState> {
    let mut cursor = Cursor::new(data);
    let mut states = Vec::new();
    while (cursor.position() as usize) < data.len() {
        let t = match cursor.read_u8() {
            Ok(t) => t,
            Err(_) => break,
        };
        if t != 10 {
            break;
        }
        let _root_name = read_string(&mut cursor).unwrap_or_default();
        
        let mut block_name = String::new();
        let mut properties = Vec::new();
        let mut version = 0;
        
        loop {
            let tag_t = match cursor.read_u8() {
                Ok(t) => t,
                Err(_) => break,
            };
            if tag_t == 0 {
                break;
            }
            let tag_name = read_string(&mut cursor).unwrap_or_default();
            match tag_name.as_str() {
                "name" => {
                    if let Ok(NbtTag::String(s)) = read_tag(tag_t, &mut cursor) {
                        block_name = s;
                    }
                }
                "states" => {
                    if let Ok(NbtTag::Compound(comp)) = read_tag(tag_t, &mut cursor) {
                        properties = comp;
                    }
                }
                "version" => {
                    if let Ok(NbtTag::Int(v)) = read_tag(tag_t, &mut cursor) {
                        version = v;
                    }
                }
                _ => {
                    let _ = read_tag(tag_t, &mut cursor);
                }
            }
        }
        
        states.push(BlockState {
            name: block_name,
            properties,
            version,
        });
    }
    states
}
