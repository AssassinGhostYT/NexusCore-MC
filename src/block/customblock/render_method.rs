#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMethod {
    Opaque,
    AlphaTest,
    Blend,
    DoubleSided,
}

impl RenderMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            RenderMethod::Opaque => "opaque",
            RenderMethod::AlphaTest => "alpha_test",
            RenderMethod::Blend => "blend",
            RenderMethod::DoubleSided => "double_sided",
        }
    }

    pub fn default_ambient_occlusion(&self) -> bool {
        match self {
            RenderMethod::AlphaTest | RenderMethod::Blend => false,
            _ => true,
        }
    }
}
