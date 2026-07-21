use byteorder::{LittleEndian, WriteBytesExt};

pub struct SerializedLayer {
    pub layer_type: u16, // u16 LE (enum Repr)
    pub abilities_set: u32,
    pub ability_values: u32,
    pub fly_speed: f32,
    pub vertical_fly_speed: f32,
    pub walk_speed: f32,
}

pub struct UpdateAbilities {
    pub target_player_uid: i64,
    pub player_permissions: u8, // u8
    pub command_permissions: i8, // i8
    pub layers: Vec<SerializedLayer>,
}

impl UpdateAbilities {
    pub fn write(&self, buf: &mut Vec<u8>) {
        buf.write_i64::<LittleEndian>(self.target_player_uid).unwrap();
        buf.write_u8(self.player_permissions).unwrap();
        buf.write_i8(self.command_permissions).unwrap();
        
        // Vec length is serialized as u8
        buf.write_u8(self.layers.len() as u8).unwrap();
        
        for layer in &self.layers {
            buf.write_u16::<LittleEndian>(layer.layer_type).unwrap();
            buf.write_u32::<LittleEndian>(layer.abilities_set).unwrap();
            buf.write_u32::<LittleEndian>(layer.ability_values).unwrap();
            buf.write_f32::<LittleEndian>(layer.fly_speed).unwrap();
            buf.write_f32::<LittleEndian>(layer.vertical_fly_speed).unwrap();
            buf.write_f32::<LittleEndian>(layer.walk_speed).unwrap();
        }
    }
}
