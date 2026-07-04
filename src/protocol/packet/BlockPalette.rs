use crate::protocol::varint::{write_varu32, write_vari32};
use std::sync::OnceLock;
use crate::block::registry::get_all_states;
use crate::block::nbt::NbtTag;

static BLOCK_PALETTE: OnceLock<Vec<u8>> = OnceLock::new();

fn write_net_nbt_string(buf: &mut Vec<u8>, s: &str) {
    write_varu32(buf, s.len() as u32);
    buf.extend_from_slice(s.as_bytes());
}

pub fn get_block_palette() -> &'static [u8] {
    BLOCK_PALETTE.get_or_init(|| {
        let entries = get_all_states();
        
        let mut buf = Vec::new();
        // 1. Escribir el conteo de bloques como VarU32
        write_varu32(&mut buf, entries.len() as u32);
        
        // 2. Escribir cada bloque
        for entry in entries {
            let block_name = &entry.name;
            
            // Nombre del bloque (string del paquete: varint length)
            write_varu32(&mut buf, block_name.len() as u32);
            buf.extend_from_slice(block_name.as_bytes());
            
            // NBT Compound en formato little-endian network NBT de Bedrock
            
            // Root Compound tag: tipo 0x0a, nombre vacío (varint length = 0)
            buf.push(0x0a); // TAG_Compound
            buf.push(0x00); // nombre vacío (varint 0)
            
            // "name" property: TAG_String (0x08)
            buf.push(0x08); // TAG_String
            write_net_nbt_string(&mut buf, "name");
            write_net_nbt_string(&mut buf, block_name);
            
            // "states" compound: TAG_Compound (0x0a)
            buf.push(0x0a); // TAG_Compound
            write_net_nbt_string(&mut buf, "states");
            
            // Ordenar estados alfabéticamente para determinismo
            let mut sorted_states: Vec<&(String, NbtTag)> = entry.properties.iter().collect();
            sorted_states.sort_by_key(|a| &a.0);
            
            for &(ref k, ref v) in &sorted_states {
                match v {
                    NbtTag::Byte(b) => {
                        buf.push(0x01); // TAG_Byte
                        write_net_nbt_string(&mut buf, k);
                        buf.push(*b);
                    }
                    NbtTag::Short(s) => {
                        buf.push(0x02); // TAG_Short
                        write_net_nbt_string(&mut buf, k);
                        write_vari32(&mut buf, *s);
                    }
                    NbtTag::Int(i) => {
                        buf.push(0x03); // TAG_Int
                        write_net_nbt_string(&mut buf, k);
                        write_vari32(&mut buf, *i);
                    }
                    NbtTag::Long(l) => {
                        buf.push(0x04); // TAG_Long
                        write_net_nbt_string(&mut buf, k);
                        write_vari32(&mut buf, *l);
                    }
                    NbtTag::Float(f) => {
                        buf.push(0x05); // TAG_Float
                        write_net_nbt_string(&mut buf, k);
                        buf.extend_from_slice(&f.to_le_bytes());
                    }
                    NbtTag::Double(d) => {
                        buf.push(0x06); // TAG_Double
                        write_net_nbt_string(&mut buf, k);
                        buf.extend_from_slice(&d.to_le_bytes());
                    }
                    NbtTag::String(s) => {
                        buf.push(0x08); // TAG_String
                        write_net_nbt_string(&mut buf, k);
                        write_net_nbt_string(&mut buf, s);
                    }
                    _ => {}
                }
            }
            buf.push(0x00); // TAG_End para "states"
            
            // "version" property: TAG_Int (0x03)
            buf.push(0x03); // TAG_Int
            write_net_nbt_string(&mut buf, "version");
            write_vari32(&mut buf, entry.version);
            
            // TAG_End del root compound
            buf.push(0x00);
        }
        
        buf
    })
}
