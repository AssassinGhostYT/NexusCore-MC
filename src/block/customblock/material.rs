use crate::block::customblock::render_method::RenderMethod;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Material {
    pub texture: String,
    pub render_method: RenderMethod,
    pub face_dimming: bool,
    pub ambient_occlusion: bool,
}

impl Material {
    pub fn new(texture: String, method: RenderMethod) -> Self {
        Self {
            texture,
            render_method: method,
            face_dimming: true,
            ambient_occlusion: method.default_ambient_occlusion(),
        }
    }
}
