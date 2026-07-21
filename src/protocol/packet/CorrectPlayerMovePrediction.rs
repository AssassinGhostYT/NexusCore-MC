// CorrectPlayerMovePrediction — ID 161
// Sent by the server to correct the client's authoritative movement.

use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::write_varu64;

pub struct CorrectPlayerMovePrediction {
    pub position: (f32, f32, f32),
    pub pitch: f32,
    pub yaw: f32,
    pub tick: u64,
}

impl CorrectPlayerMovePrediction {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        
        // 1. PredictionType (0 = PredictionTypePlayer)
        buf.push(0);
        
        // 2. Position (Vec3)
        buf.write_f32::<LittleEndian>(self.position.0).unwrap();
        buf.write_f32::<LittleEndian>(self.position.1).unwrap();
        buf.write_f32::<LittleEndian>(self.position.2).unwrap();
        
        // 3. Delta (Vec3)
        buf.write_f32::<LittleEndian>(0.0).unwrap();
        buf.write_f32::<LittleEndian>(0.0).unwrap();
        buf.write_f32::<LittleEndian>(0.0).unwrap();
        
        // 4. Rotation (Vec2: pitch, yaw)
        buf.write_f32::<LittleEndian>(self.pitch).unwrap();
        buf.write_f32::<LittleEndian>(self.yaw).unwrap();
        
        // 5. VehicleAngularVelocity (Optional: not present)
        buf.push(0);
        
        // 6. OnGround (bool: true)
        buf.push(1);
        
        // 7. Tick (varuint64)
        write_varu64(&mut buf, self.tick);
        
        Ok(buf)
    }
}
