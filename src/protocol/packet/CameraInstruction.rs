use std::io::Write;

pub const ID_CAMERA_INSTRUCTION: u32 = 300;

#[derive(Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, Default)]
pub struct CameraInstructionSet {
    pub preset: u32,
    pub ease: Option<CameraEaseOption>,
    pub pos: Option<Vec3>,
    pub rot: Option<(f32, f32)>, 
    pub facing: Option<Vec3>,
    pub view_offset: Option<(f32, f32)>,
    pub entity_offset: Option<Vec3>,
    pub default_preset: bool,
    pub remove_ignore_starting_values: bool,
}

#[derive(Clone, Debug)]
pub struct CameraEaseOption {
    pub ease_type: u8,
    pub time: f32,
}

#[derive(Clone, Debug)]
pub struct CameraInstructionFade {
    pub time: Option<CameraFadeTimeOption>,
    pub color: Option<Vec3>, 
}

#[derive(Clone, Debug)]
pub struct CameraFadeTimeOption {
    pub fade_in_time: f32,
    pub hold_time: f32,
    pub fade_out_time: f32,
}

#[derive(Clone, Debug)]
pub struct CameraInstructionTarget {
    pub center_offset: Vec3,
    pub target_actor_id: i64,
}

#[derive(Clone, Debug)]
pub struct CameraInstructionFieldOfView {
    pub fov: f32,
    pub ease_time: f32,
    pub ease_type: i32,
    pub fov_clear: bool,
}

#[derive(Clone, Debug)]
pub struct CameraSplineInstruction {
    pub total_time: f32,
    pub spline_type: u8,
    pub curve: Vec<Vec3>,
    pub progress_key_frames: Vec<SplineProgressOption>,
    pub rotation_option: Vec<SplineRotationOption>,
    pub spline_identifier: String,
    pub load_from_json: bool,
}

#[derive(Clone, Debug)]
pub struct SplineProgressOption {
    pub key_frame_value: f32,
    pub key_frame_time: f32,
    pub key_frame_easing_func: i32,
}

#[derive(Clone, Debug)]
pub struct SplineRotationOption {
    pub key_frame_value: Vec3,
    pub key_frame_time: f32,
    pub key_frame_easing_func: i32,
}

#[derive(Clone, Debug, Default)]
pub struct CameraInstruction {
    pub set: Option<CameraInstructionSet>,
    pub clear: Option<bool>,
    pub fade: Option<CameraInstructionFade>,
    pub target: Option<CameraInstructionTarget>,
    pub remove_target: Option<bool>,
    pub field_of_view: Option<CameraInstructionFieldOfView>,
    pub spline: Option<CameraSplineInstruction>,
    pub attach_to_entity: Option<i64>,
    pub detach_from_entity: Option<bool>,
}

impl CameraInstruction {
    pub fn write(&self) -> Vec<u8> {
        let mut w = Vec::new();

        if let Some(ref set) = self.set {
            w.push(1);
            crate::protocol::varint::write_varu32(&mut w, set.preset);
            
            if let Some(ref ease) = set.ease {
                w.push(1);
                w.push(ease.ease_type);
                w.extend_from_slice(&ease.time.to_le_bytes());
            } else {
                w.push(0);
            }

            if let Some(ref pos) = set.pos {
                w.push(1);
                w.extend_from_slice(&pos.x.to_le_bytes());
                w.extend_from_slice(&pos.y.to_le_bytes());
                w.extend_from_slice(&pos.z.to_le_bytes());
            } else {
                w.push(0);
            }

            if let Some(ref rot) = set.rot {
                w.push(1);
                w.extend_from_slice(&rot.0.to_le_bytes());
                w.extend_from_slice(&rot.1.to_le_bytes());
            } else {
                w.push(0);
            }

            if let Some(ref facing) = set.facing {
                w.push(1);
                w.extend_from_slice(&facing.x.to_le_bytes());
                w.extend_from_slice(&facing.y.to_le_bytes());
                w.extend_from_slice(&facing.z.to_le_bytes());
            } else {
                w.push(0);
            }

            if let Some(ref offset) = set.view_offset {
                w.push(1);
                w.extend_from_slice(&offset.0.to_le_bytes());
                w.extend_from_slice(&offset.1.to_le_bytes());
            } else {
                w.push(0);
            }

            if let Some(ref ent_offset) = set.entity_offset {
                w.push(1);
                w.extend_from_slice(&ent_offset.x.to_le_bytes());
                w.extend_from_slice(&ent_offset.y.to_le_bytes());
                w.extend_from_slice(&ent_offset.z.to_le_bytes());
            } else {
                w.push(0);
            }

            w.push(if set.default_preset { 1 } else { 0 });
            w.push(if set.remove_ignore_starting_values { 1 } else { 0 });
        } else {
            w.push(0);
        }

        if let Some(clear) = self.clear {
            w.push(1);
            w.push(if clear { 1 } else { 0 });
        } else {
            w.push(0);
        }

        if let Some(ref fade) = self.fade {
            w.push(1);
            if let Some(ref time) = fade.time {
                w.push(1);
                w.extend_from_slice(&time.fade_in_time.to_le_bytes());
                w.extend_from_slice(&time.hold_time.to_le_bytes());
                w.extend_from_slice(&time.fade_out_time.to_le_bytes());
            } else {
                w.push(0);
            }
            if let Some(ref color) = fade.color {
                w.push(1);
                w.extend_from_slice(&color.x.to_le_bytes()); 
                w.extend_from_slice(&color.y.to_le_bytes()); 
                w.extend_from_slice(&color.z.to_le_bytes()); 
            } else {
                w.push(0);
            }
        } else {
            w.push(0);
        }

        if let Some(ref target) = self.target {
            w.push(1);
            w.extend_from_slice(&target.center_offset.x.to_le_bytes());
            w.extend_from_slice(&target.center_offset.y.to_le_bytes());
            w.extend_from_slice(&target.center_offset.z.to_le_bytes());
            crate::protocol::varint::write_vari64(&mut w, target.target_actor_id);
        } else {
            w.push(0);
        }

        if let Some(remove) = self.remove_target {
            w.push(1);
            w.push(if remove { 1 } else { 0 });
        } else {
            w.push(0);
        }

        if let Some(ref fov) = self.field_of_view {
            w.push(1);
            w.extend_from_slice(&fov.fov.to_le_bytes());
            w.extend_from_slice(&fov.ease_time.to_le_bytes());
            crate::protocol::varint::write_vari32(&mut w, fov.ease_type);
            w.push(if fov.fov_clear { 1 } else { 0 });
        } else {
            w.push(0);
        }

        if let Some(ref spline) = self.spline {
            w.push(1);
            w.extend_from_slice(&spline.total_time.to_le_bytes());
            w.push(spline.spline_type);
            
            crate::protocol::varint::write_varu32(&mut w, spline.curve.len() as u32);
            for p in &spline.curve {
                w.extend_from_slice(&p.x.to_le_bytes());
                w.extend_from_slice(&p.y.to_le_bytes());
                w.extend_from_slice(&p.z.to_le_bytes());
            }

            crate::protocol::varint::write_varu32(&mut w, spline.progress_key_frames.len() as u32);
            for kf in &spline.progress_key_frames {
                w.extend_from_slice(&kf.key_frame_value.to_le_bytes());
                w.extend_from_slice(&kf.key_frame_time.to_le_bytes());
                crate::protocol::varint::write_vari32(&mut w, kf.key_frame_easing_func);
            }

            crate::protocol::varint::write_varu32(&mut w, spline.rotation_option.len() as u32);
            for rot in &spline.rotation_option {
                w.extend_from_slice(&rot.key_frame_value.x.to_le_bytes());
                w.extend_from_slice(&rot.key_frame_value.y.to_le_bytes());
                w.extend_from_slice(&rot.key_frame_value.z.to_le_bytes());
                w.extend_from_slice(&rot.key_frame_time.to_le_bytes());
                crate::protocol::varint::write_vari32(&mut w, rot.key_frame_easing_func);
            }

            crate::protocol::varint::write_varu32(&mut w, spline.spline_identifier.len() as u32);
            w.write_all(spline.spline_identifier.as_bytes()).unwrap();
            w.push(if spline.load_from_json { 1 } else { 0 });
        } else {
            w.push(0);
        }

        if let Some(entity_id) = self.attach_to_entity {
            w.push(1);
            crate::protocol::varint::write_vari64(&mut w, entity_id);
        } else {
            w.push(0);
        }

        if let Some(detach) = self.detach_from_entity {
            w.push(1);
            w.push(if detach { 1 } else { 0 });
        } else {
            w.push(0);
        }

        w
    }
}
