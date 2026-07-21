use crate::protocol::error::PResult;

pub struct UpdateAdventureSettings {
    pub no_pvm: bool,
    pub no_mvp: bool,
    pub immutable_world: bool,
    pub show_name_tags: bool,
    pub auto_jump: bool,
}

impl UpdateAdventureSettings {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        buf.push(if self.no_pvm { 1 } else { 0 });
        buf.push(if self.no_mvp { 1 } else { 0 });
        buf.push(if self.immutable_world { 1 } else { 0 });
        buf.push(if self.show_name_tags { 1 } else { 0 });
        buf.push(if self.auto_jump { 1 } else { 0 });
        Ok(buf)
    }
}
