use crate::block::cube::BBox;
use crate::block::cube::Pos;
use crate::block::customblock::material::Material;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Properties {
    pub collision_box: Option<BBox>,
    pub selection_box: Option<BBox>,
    pub cube: bool,
    pub geometry: String,
    pub map_colour: String,
    pub rotation: Pos,
    pub scale: [f64; 3],
    pub textures: HashMap<String, Material>,
    pub translation: [f64; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Permutation {
    pub properties: Properties,
    pub condition: String,
}
