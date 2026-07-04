use crate::block::cube::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Self { yaw, pitch }
    }

    pub fn direction(&self) -> Direction {
        let mut y = self.yaw % 360.0;
        if y > 180.0 {
            y -= 360.0;
        } else if y <= -180.0 {
            y += 360.0;
        }
        
        if y > 45.0 && y <= 135.0 {
            Direction::West
        } else if y > -45.0 && y <= 45.0 {
            Direction::South
        } else if y > -135.0 && y <= -45.0 {
            Direction::East
        } else {
            Direction::North
        }
    }
}
