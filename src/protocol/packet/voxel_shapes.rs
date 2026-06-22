use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::write_varu32;

pub const ID_VOXEL_SHAPES: u32 = 337;

#[derive(Debug, Clone)]
pub struct VoxelShapes {
    pub custom_shape_count: u16,
}

impl VoxelShapes {
    pub fn new() -> Self {
        VoxelShapes { custom_shape_count: 0 }
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        // shapes: Vec<VoxelShape> length = 0
        write_varu32(&mut buf, 0);
        // names: Vec<VoxelShapeName> length = 0
        write_varu32(&mut buf, 0);
        // custom_shape_count: u16 little-endian
        buf.write_u16::<LittleEndian>(self.custom_shape_count).unwrap();
        buf
    }
}
