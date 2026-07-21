
pub const ID_CAMERA_AIM_ASSIST_ACTOR_PRIORITY: u32 = 339;

/// Individual priority rules mapped inside the actor priority packet.
#[derive(Clone, Debug, Default)]
pub struct CameraAimAssistActorPriorityData {
    /// Associated index linking back to the predefined tracking preset.
    pub preset_index: i32,
    /// Associated group classification category index.
    pub category_index: i32,
    /// Targeted entity type identifier mapping index.
    pub actor_index: i32,
    /// Core ranking hierarchy score given to this specific target.
    pub priority_value: i32,
}

/// Sent by the server to set or update priority hierarchies for target acquisition.
#[derive(Clone, Debug, Default)]
pub struct CameraAimAssistActorPriority {
    /// Dynamic collection of configuration rules defining entity priority behaviors.
    pub priority_data: Vec<CameraAimAssistActorPriorityData>,
}

impl CameraAimAssistActorPriority {
    /// Serializes the CameraAimAssistActorPriority packet payload into the network buffer.
    pub fn write(&self) -> Vec<u8> {
        let mut w = Vec::new();

        // Write the total count of element entries inside the slice.
        crate::protocol::varint::write_varu32(&mut w, self.priority_data.len() as u32);

        // Serialize each data item block in order.
        for data in &self.priority_data {
            crate::protocol::varint::write_vari32(&mut w, data.preset_index);
            crate::protocol::varint::write_vari32(&mut w, data.category_index);
            crate::protocol::varint::write_vari32(&mut w, data.actor_index);
            crate::protocol::varint::write_vari32(&mut w, data.priority_value);
        }

        w
    }
}
