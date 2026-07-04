pub const ID_UPDATE_ADVENTURE_SETTINGS: u32 = 188;

#[derive(Debug, Clone)]
pub struct UpdateAdventureSettings {
    /// NoPvM: player cannot fight mobs
    pub no_pvm: bool,
    /// NoMvP: mobs cannot fight player
    pub no_mvp: bool,
    /// ImmutableWorld: player cannot modify world
    pub immutable_world: bool,
    /// ShowNameTags: player name tags shown
    pub show_name_tags: bool,
    /// AutoJump: player auto-jumps
    pub auto_jump: bool,
}

impl UpdateAdventureSettings {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.no_pvm as u8);
        buf.push(self.no_mvp as u8);
        buf.push(self.immutable_world as u8);
        buf.push(self.show_name_tags as u8);
        buf.push(self.auto_jump as u8);
        buf
    }
}
