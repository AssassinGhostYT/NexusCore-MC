// AvailableCommands — ID 76
// Sent by the server to send the client list of available commands.

use crate::protocol::error::PResult;
use crate::protocol::varint::write_varu32;
use crate::macros::helpers;
use byteorder::{LittleEndian, WriteBytesExt};

pub struct AvailableCommands {
    pub enum_values: Vec<String>,
    pub chained_subcommand_values: Vec<String>,
    pub post_fixes: Vec<String>,
    pub enum_data: Vec<EnumData>,
    pub chained_subcommand_data: Vec<ChainedSubcommandData>,
    pub commands: Vec<CommandData>,
    pub soft_enums: Vec<SoftEnumData>,
    pub constraints: Vec<ConstrainedValueData>,
}

pub struct EnumData {
    pub name: String,
    pub values: Vec<u32>,
}

pub struct ChainedSubcommandData {
    pub name: String,
    pub subcommand_values: Vec<ChainedSubcommandRelationship>,
}

pub struct ChainedSubcommandRelationship {
    pub first_value: u32,
    pub second_value: u32,
}

pub struct CommandData {
    pub name: String,
    pub description: String,
    pub flags: u16,
    pub permission_level: u8,
    pub alias_enum: i32,
    pub chained_subcommand_indexes: Vec<u32>,
    pub overloads: Vec<OverloadData>,
}

pub struct OverloadData {
    pub is_chaining: bool,
    pub parameters: Vec<ParamData>,
}

pub struct ParamData {
    pub name: String,
    pub parse_symbol: u32,
    pub is_optional: bool,
    pub options: u8,
}

pub struct SoftEnumData {
    pub enum_name: String,
    pub enum_options: Vec<String>,
}

pub struct ConstrainedValueData {
    pub enum_value_symbol: u32,
    pub enum_symbol: u32,
    pub constraint_indices: Vec<u8>,
}

impl AvailableCommands {
    pub fn new() -> Self {
        Self {
            enum_values: Vec::new(),
            chained_subcommand_values: Vec::new(),
            post_fixes: Vec::new(),
            enum_data: Vec::new(),
            chained_subcommand_data: Vec::new(),
            commands: Vec::new(),
            soft_enums: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        
        // Enum Values
        write_varu32(&mut buf, self.enum_values.len() as u32);
        for value in &self.enum_values {
            helpers::write_string(&mut buf, value);
        }
        
        // Chained Subcommand Values
        write_varu32(&mut buf, self.chained_subcommand_values.len() as u32);
        for value in &self.chained_subcommand_values {
            helpers::write_string(&mut buf, value);
        }
        
        // Post Fixes
        write_varu32(&mut buf, self.post_fixes.len() as u32);
        for value in &self.post_fixes {
            helpers::write_string(&mut buf, value);
        }
        
        // Enum Data
        write_varu32(&mut buf, self.enum_data.len() as u32);
        for enum_data in &self.enum_data {
            helpers::write_string(&mut buf, &enum_data.name);
            write_varu32(&mut buf, enum_data.values.len() as u32);
            for val in &enum_data.values {
                buf.write_u32::<LittleEndian>(*val).unwrap();
            }
        }
        
        // Chained Subcommand Data
        write_varu32(&mut buf, self.chained_subcommand_data.len() as u32);
        for data in &self.chained_subcommand_data {
            helpers::write_string(&mut buf, &data.name);
            write_varu32(&mut buf, data.subcommand_values.len() as u32);
            for rel in &data.subcommand_values {
                write_varu32(&mut buf, rel.first_value);
                write_varu32(&mut buf, rel.second_value);
            }
        }
        
        // Commands
        write_varu32(&mut buf, self.commands.len() as u32);
        for cmd in &self.commands {
            helpers::write_string(&mut buf, &cmd.name);
            helpers::write_string(&mut buf, &cmd.description);
            buf.write_u16::<LittleEndian>(cmd.flags).unwrap();
            buf.push(cmd.permission_level);
            buf.write_i32::<LittleEndian>(cmd.alias_enum).unwrap();
            
            write_varu32(&mut buf, cmd.chained_subcommand_indexes.len() as u32);
            for idx in &cmd.chained_subcommand_indexes {
                buf.write_u32::<LittleEndian>(*idx).unwrap();
            }
            
            write_varu32(&mut buf, cmd.overloads.len() as u32);
            for overload in &cmd.overloads {
                buf.push(if overload.is_chaining { 1 } else { 0 });
                
                write_varu32(&mut buf, overload.parameters.len() as u32);
                for param in &overload.parameters {
                    helpers::write_string(&mut buf, &param.name);
                    buf.write_u32::<LittleEndian>(param.parse_symbol).unwrap();
                    buf.push(if param.is_optional { 1 } else { 0 });
                    buf.push(param.options);
                }
            }
        }
        
        // Soft Enums
        write_varu32(&mut buf, self.soft_enums.len() as u32);
        for soft_enum in &self.soft_enums {
            helpers::write_string(&mut buf, &soft_enum.enum_name);
            write_varu32(&mut buf, soft_enum.enum_options.len() as u32);
            for option in &soft_enum.enum_options {
                helpers::write_string(&mut buf, option);
            }
        }
        
        // Constraints
        write_varu32(&mut buf, self.constraints.len() as u32);
        for constraint in &self.constraints {
            buf.write_u32::<LittleEndian>(constraint.enum_value_symbol).unwrap();
            buf.write_u32::<LittleEndian>(constraint.enum_symbol).unwrap();
            write_varu32(&mut buf, constraint.constraint_indices.len() as u32);
            for idx in &constraint.constraint_indices {
                buf.push(*idx);
            }
        }
        
        Ok(buf)
    }
}
