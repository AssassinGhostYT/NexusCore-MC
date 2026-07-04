#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl BBox {
    pub fn new(x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self {
        let (x_min, x_max) = if x0 > x1 { (x1, x0) } else { (x0, x1) };
        let (y_min, y_max) = if y0 > y1 { (y1, y0) } else { (y0, y1) };
        let (z_min, z_max) = if z0 > z1 { (z1, z0) } else { (z0, z1) };
        Self {
            min: [x_min, y_min, z_min],
            max: [x_max, y_max, z_max],
        }
    }

    pub fn grow(self, x: f64) -> Self {
        Self {
            min: [self.min[0] - x, self.min[1] - x, self.min[2] - x],
            max: [self.max[0] + x, self.max[1] + x, self.max[2] + x],
        }
    }

    pub fn width(&self) -> f64 {
        self.max[0] - self.min[0]
    }

    pub fn height(&self) -> f64 {
        self.max[1] - self.min[1]
    }

    pub fn length(&self) -> f64 {
        self.max[2] - self.min[2]
    }
}
