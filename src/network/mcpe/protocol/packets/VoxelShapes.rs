// VoxelShapesPacket — ID 337 (0x151)
// Sent BEFORE StartGame. Defines custom voxel collision shapes for blocks.
// Empty by default (no custom behavior packs with voxel shapes).
//
// Wire format (v1001 / v944):
//   shapes:    u32 LE count, then each VoxelShape
//   names:     u32 LE count, then each VoxelShapeName
//   custom_shape_count: u16 LE
//
// VoxelShape:
//   cells:
//     (x_size, y_size, z_size): u8 u8 u8
//     storage: u32 LE count, then bytes
//   x_coordinates: u32 LE count, then each f32 LE
//   y_coordinates: u32 LE count, then each f32 LE
//   z_coordinates: u32 LE count, then each f32 LE
//
// VoxelShapeName:
//   name:  String (varu32 length + UTF-8 bytes)
//   index: u16 LE

use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::macros::helpers;
use crate::protocol::varint::write_varu32;

/// Cell data for a single voxel shape.
pub struct VoxelShapeCells {
    pub x_size: u8,
    pub y_size: u8,
    pub z_size: u8,
    pub storage: Vec<u8>,
}

impl VoxelShapeCells {
    pub fn write_into(&self, buf: &mut Vec<u8>) {
        buf.push(self.x_size);
        buf.push(self.y_size);
        buf.push(self.z_size);
        write_varu32(buf, self.storage.len() as u32);
        buf.extend_from_slice(&self.storage);
    }
}

/// A single voxel shape definition.
pub struct VoxelShape {
    pub cells: VoxelShapeCells,
    pub x_coordinates: Vec<f32>,
    pub y_coordinates: Vec<f32>,
    pub z_coordinates: Vec<f32>,
}

impl VoxelShape {
    pub fn write_into(&self, buf: &mut Vec<u8>) {
        self.cells.write_into(buf);
        write_varu32(buf, self.x_coordinates.len() as u32);
        for &v in &self.x_coordinates { buf.write_f32::<LittleEndian>(v).unwrap(); }
        write_varu32(buf, self.y_coordinates.len() as u32);
        for &v in &self.y_coordinates { buf.write_f32::<LittleEndian>(v).unwrap(); }
        write_varu32(buf, self.z_coordinates.len() as u32);
        for &v in &self.z_coordinates { buf.write_f32::<LittleEndian>(v).unwrap(); }
    }
}

/// A name-to-index mapping entry in the shape name map.
pub struct VoxelShapeName {
    pub name: String,
    pub index: u16,
}

impl VoxelShapeName {
    pub fn write_into(&self, buf: &mut Vec<u8>) {
        helpers::write_string(buf, &self.name);
        buf.write_u16::<LittleEndian>(self.index).unwrap();
    }
}

/// VoxelShapes packet — ID 337 (0x151).
pub struct VoxelShapes {
    pub shapes: Vec<VoxelShape>,
    pub names: Vec<VoxelShapeName>,
    pub custom_shape_count: u16,
}

impl VoxelShapes {
    pub fn new() -> Self {
        Self {
            shapes: vec![],
            names: vec![],
            custom_shape_count: 0,
        }
    }

    pub fn write_into(&self, buf: &mut Vec<u8>) {
        write_varu32(buf, self.shapes.len() as u32);
        for shape in &self.shapes { shape.write_into(buf); }
        write_varu32(buf, self.names.len() as u32);
        for entry in &self.names { entry.write_into(buf); }
        buf.write_u16::<LittleEndian>(self.custom_shape_count).unwrap();
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        self.write_into(&mut buf);
        log::info!("VoxelShapes::write: bytes count={}, hex={:02x?}", buf.len(), buf);
        Ok(buf)
    }
}
