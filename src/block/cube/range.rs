#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    pub min: i32,
    pub max: i32,
}

impl Range {
    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }

    pub fn height(&self) -> i32 {
        self.max - self.min
    }
}
