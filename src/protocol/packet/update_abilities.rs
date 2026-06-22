use byteorder::{LittleEndian, WriteBytesExt};

pub const ID_UPDATE_ABILITIES: u32 = 187;

#[derive(Debug, Clone)]
pub struct AbilityLayer {
    pub layer_type: u16,
    pub abilities: u32,
    pub values: u32,
    pub fly_speed: f32,
    pub walk_speed: f32,
}

#[derive(Debug, Clone)]
pub struct UpdateAbilities {
    pub entity_unique_id: i64,
    pub player_permissions: u8,
    pub command_permissions: u8,
    pub layers: Vec<AbilityLayer>,
}

impl UpdateAbilities {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.write_i64::<LittleEndian>(self.entity_unique_id).unwrap();
        buf.push(self.player_permissions);
        buf.push(self.command_permissions);
        buf.push(self.layers.len() as u8);
        for layer in &self.layers {
            buf.write_u16::<LittleEndian>(layer.layer_type).unwrap();
            buf.write_u32::<LittleEndian>(layer.abilities).unwrap();
            buf.write_u32::<LittleEndian>(layer.values).unwrap();
            buf.write_f32::<LittleEndian>(layer.fly_speed).unwrap();
            buf.write_f32::<LittleEndian>(layer.walk_speed).unwrap();
        }
        buf
    }
}
