use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::{write_varu32, write_vari32};
use crate::macros::helpers;

/// Value carried by a legacy game rule.
pub enum GameRuleLegacyType {
    /// discriminant = 1
    Bool(bool),
    /// discriminant = 2, encoded as signed varint
    Int(i32),
    /// discriminant = 3, encoded as f32 LE
    Float(f32),
}

/// A single changed game rule entry.
pub struct GameRuleLegacyChanged {
    pub rule_name: String,
    pub can_be_modified_by_player: bool,
    pub rule_type: GameRuleLegacyType,
}

/// Container holding all changed game rules, sent as part of LevelSettings.
pub struct GameRuleLegacyData {
    pub rules_list: Vec<GameRuleLegacyChanged>,
}

impl GameRuleLegacyData {
    pub fn new() -> Self {
        Self {
            rules_list: vec![],
        }
    }

    /// Serialize into bytes and append to `buf`.
    pub fn write_into(&self, buf: &mut Vec<u8>) {
        // Count as varu32
        write_varu32(buf, self.rules_list.len() as u32);
        for rule in &self.rules_list {
            helpers::write_string(buf, &rule.rule_name);
            buf.push(if rule.can_be_modified_by_player { 1 } else { 0 });
            match &rule.rule_type {
                GameRuleLegacyType::Bool(v) => {
                    write_varu32(buf, 1);
                    buf.push(if *v { 1 } else { 0 });
                }
                GameRuleLegacyType::Int(v) => {
                    write_varu32(buf, 2);
                    write_vari32(buf, *v);
                }
                GameRuleLegacyType::Float(v) => {
                    write_varu32(buf, 3);
                    buf.write_f32::<LittleEndian>(*v).unwrap();
                }
            }
        }
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        self.write_into(&mut buf);
        Ok(buf)
    }
}
