use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::macros::helpers;

/// A single experimental toggle.
pub struct Experiment {
    pub name: String,
    pub enabled: bool,
}

/// List of experimental feature toggles sent as part of LevelSettings.
pub struct Experiments {
    pub experiments: Vec<Experiment>,
    pub ever_toggled: bool,
}

impl Experiments {
    pub fn new() -> Self {
        Self {
            experiments: vec![],
            ever_toggled: false,
        }
    }

    /// Serialize into bytes and append to `buf`.
    pub fn write_into(&self, buf: &mut Vec<u8>) {
        // Count written as u32 LE (not varint)
        buf.write_u32::<LittleEndian>(self.experiments.len() as u32).unwrap();
        for exp in &self.experiments {
            helpers::write_string(buf, &exp.name);
            buf.push(if exp.enabled { 1 } else { 0 });
        }
        buf.push(if self.ever_toggled { 1 } else { 0 });
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        self.write_into(&mut buf);
        Ok(buf)
    }
}
