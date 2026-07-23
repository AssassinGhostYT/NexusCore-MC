use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use crate::protocol::varint::write_varu32;

// ID_CAMERA_PRESETS is the unique packet ID for CameraPresetsPacket.
pub const ID_CAMERA_PRESETS: u32 = 198;

// Vec2 represents a 2D floating-point vector used for camera rotation limits and offsets.
#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

// Vec3 represents a 3D floating-point vector used for camera positioning offsets.
#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// CameraAimAssistCommandDefinition contains parameters that define aiming behavior toward targets.
#[derive(Debug, Clone)]
pub struct CameraAimAssistCommandDefinition {
    // preset_id is a reference to a registered asset layout in CameraAimAssist.
    pub preset_id: String,
    // target_mode determines the prioritization rule. 0 = angle, 1 = distance.
    pub target_mode: u8,
    // view_angle controls the bounding constraints for target tracking.
    pub view_angle: Vec2,
    // distance is the maximum block radius allowed for target acquisition.
    pub distance: f32,
}

// CameraPreset defines a unique structural template used to drive client-side camera behaviors.
#[derive(Debug, Clone)]
pub struct CameraPreset {
    // name is the unique namespaced string identifier for this preset (e.g., "minecraft:first_person").
    pub name: String,
    // inherit_from references a parent preset identifier to extend base parameters.
    pub inherit_from: String,
    // pos_x is the baseline position constraint on the X axis.
    pub pos_x: f32,
    // pos_y is the baseline position constraint on the Y axis.
    pub pos_y: f32,
    // pos_z is the baseline position constraint on the Z axis.
    pub pos_z: f32,
    // rot_x represents the default pitch angle of the camera lens.
    pub rot_x: f32,
    // rot_y represents the default yaw angle of the camera lens.
    pub rot_y: f32,
    // rotation_speed dictates how rapidly the camera moves toward target adjustments.
    pub rotation_speed: f32,
    // snap_to_target locks the camera view instantly onto the targeted orientation if true.
    pub snap_to_target: bool,
    // horizontal_rotation_limit enforces bounding limits on left-and-right panning tracking.
    pub horizontal_rotation_limit: Vec2,
    // vertical_rotation_limit enforces bounding limits on up-and-down tilting tracking.
    pub vertical_rotation_limit: Vec2,
    // continue_targeting preserves structural alignment tracking across tick cycles.
    pub continue_targeting: bool,
    // block_listening_radius limits game environment audio-listener translation boundaries.
    pub block_listening_radius: f32,
    // view_offset provides custom screen-space positional offsets.
    pub view_offset: Vec2,
    // entity_offset defines the camera's pivot point translation offset relative to the target entity.
    pub entity_offset: Vec3,
    // radius specifies the boom-arm spherical distance boundary from a central entity.
    pub radius: f32,
    // yaw_limit_min sets the minimum angle boundary for horizontal camera panning.
    pub yaw_limit_min: f32,
    // yaw_limit_max sets the maximum angle boundary for horizontal camera panning.
    pub yaw_limit_max: f32,
    // listener defines audio attachment source. 0 = Camera attached, 1 = Player attached.
    pub listener: u8,
    // player_effects toggles environmental screen overlays and post-processing effects.
    pub player_effects: bool,
    // aim_assist configures automated aiming properties associated with the camera view.
    pub aim_assist: CameraAimAssistCommandDefinition,
    // control_scheme dictates input mapping rules. Ranges from 0 to 4 (e.g., camera_relative).
    pub control_scheme: u8,
}

// CameraPresets gives the client a structural list of custom camera presets during setup initialization.
pub struct CameraPresets {
    // presets is a collection of templates. The order matters as client reference indexes point here.
    pub presets: Vec<CameraPreset>,
}

impl CameraPresets {
    // default_presets populates standard vanilla templates used across typical game states.
    pub fn default_presets() -> Self {
        let types = vec![
            ("minecraft:first_person", ""),
            ("minecraft:third_person", ""),
            ("minecraft:third_person_front", ""),
            ("minecraft:free", ""),
        ];

        let mut presets = Vec::new();
        for (id, parent) in types {
            presets.push(CameraPreset {
                name: id.to_string(),
                inherit_from: parent.to_string(),
                pos_x: 0.0,
                pos_y: 0.0,
                pos_z: 0.0,
                rot_x: 0.0,
                rot_y: 0.0,
                rotation_speed: 1.0,
                snap_to_target: false,
                horizontal_rotation_limit: Vec2 { x: 0.0, y: 0.0 },
                vertical_rotation_limit: Vec2 { x: 0.0, y: 0.0 },
                continue_targeting: false,
                block_listening_radius: 0.0,
                view_offset: Vec2 { x: 0.0, y: 0.0 },
                entity_offset: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                radius: 0.0,
                yaw_limit_min: 0.0,
                yaw_limit_max: 0.0,
                listener: 0, 
                player_effects: false,
                aim_assist: CameraAimAssistCommandDefinition {
                    preset_id: "".to_string(),
                    target_mode: 0,
                    view_angle: Vec2 { x: 0.0, y: 0.0 },
                    distance: 0.0,
                },
                control_scheme: 1, // Defaulting to camera_relative
            });
        }

        CameraPresets { presets }
    }

    // write encodes the structural contents of the packet layout directly onto the outbound byte buffer.
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // Encode the size count of the camera presets array layout.
        write_varu32(&mut buf, self.presets.len() as u32);

        for p in &self.presets {
            // Index 0: String identifier token name.
            let name_bytes = p.name.as_bytes();
            write_varu32(&mut buf, name_bytes.len() as u32);
            buf.extend_from_slice(name_bytes);

            // Index 1: Structural inheritance base reference token string.
            let inherit_bytes = p.inherit_from.as_bytes();
            write_varu32(&mut buf, inherit_bytes.len() as u32);
            buf.extend_from_slice(inherit_bytes);

            // Bedrock protocol has 20 optional fields in CameraPreset structure.
            // Each optional field is encoded as: bool present (1 byte), then value if present.
            let eye_height = 1.62;
            let is_third_person = p.name.contains("third_person") || p.name.contains("free");
            let is_front = p.name.contains("front");
            let radius_value = if is_front { -4.0 } else if is_third_person { 4.0 } else { 0.0 };
            let set_radius = is_third_person;

            // 1. pos_x (f32)
            buf.push(0);
            // 2. pos_y (f32)
            buf.push(0);
            // 3. pos_z (f32)
            buf.push(0);
            // 4. rot_x (f32)
            buf.push(0);
            // 5. rot_y (f32)
            buf.push(0);
            // 6. rotation_speed (f32)
            buf.push(0);
            // 7. snap_to_target (bool)
            buf.push(0);
            // 8. horizontal_rotation_limit (Vec2)
            buf.push(0);
            // 9. vertical_rotation_limit (Vec2)
            buf.push(0);
            // 10. continue_targeting (bool)
            buf.push(0);
            // 11. block_listening_radius (f32)
            buf.push(0);
            // 12. view_offset (Vec2)
            buf.push(0);
            // 13. entity_offset (Vec3)
            buf.push(1);
            buf.write_f32::<LittleEndian>(0.0).unwrap();
            buf.write_f32::<LittleEndian>(eye_height).unwrap();
            buf.write_f32::<LittleEndian>(0.0).unwrap();
            // 14. radius (f32)
            if set_radius {
                buf.push(1);
                buf.write_f32::<LittleEndian>(radius_value).unwrap();
            } else {
                buf.push(0);
            }
            // 15. yaw_limit_min (f32)
            buf.push(0);
            // 16. yaw_limit_max (f32)
            buf.push(0);
            // 17. listener (u8)
            buf.push(0);
            // 18. player_effects (bool)
            buf.push(0);
            // 19. aim_assist (CameraAimAssistCommandDefinition)
            buf.push(0);
            // 20. control_scheme (u8)
            buf.push(1);
            buf.push(1);
        }

        buf
    }
}
