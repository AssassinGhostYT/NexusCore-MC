pub mod render_method;
pub mod material;
pub mod permutations;

pub use render_method::RenderMethod;
pub use material::Material;
pub use permutations::{Properties, Permutation};

#[derive(Debug, Clone, PartialEq)]
pub struct CustomBlock {
    pub name: String,
    pub properties: Properties,
    pub permutations: Vec<Permutation>,
}
