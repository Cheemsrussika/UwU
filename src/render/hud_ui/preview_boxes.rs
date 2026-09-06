use bevy::math::Vec3;
use crate::engine::player_ctrl::steve_uv::*;

pub struct PreviewBox {
    pub origin: Vec3,
    pub size: Vec3,
    pub uvs: FaceUvs,
}

pub fn get_steve_preview_boxes() -> [PreviewBox; 6] {
    [
        // 0: Left Leg
        PreviewBox { origin: Vec3::new(2.0, 6.0, 0.0), size: Vec3::new(4.0, 12.0, 4.0), uvs: left_leg_uvs() },
        // 1: Right Leg
        PreviewBox { origin: Vec3::new(-2.0, 6.0, 0.0), size: Vec3::new(4.0, 12.0, 4.0), uvs: right_leg_uvs() },
        // 2: Torso
        PreviewBox { origin: Vec3::new(0.0, 18.0, 0.0), size: Vec3::new(8.0, 12.0, 4.0), uvs: body_uvs() },
        // 3: Left Arm
        PreviewBox { origin: Vec3::new(6.0, 18.0, 0.0), size: Vec3::new(4.0, 12.0, 4.0), uvs: left_arm_uvs() },
        // 4: Right Arm
        PreviewBox { origin: Vec3::new(-6.0, 18.0, 0.0), size: Vec3::new(4.0, 12.0, 4.0), uvs: right_arm_uvs() },
        // 5: Head
        PreviewBox { origin: Vec3::new(0.0, 28.0, 0.0), size: Vec3::new(8.0, 8.0, 8.0), uvs: head_uvs() },
    ]
}
