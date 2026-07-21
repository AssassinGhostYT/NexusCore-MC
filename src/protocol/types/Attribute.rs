#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub default_min: f32,
    pub default_max: f32,
    pub default: f32,
}
