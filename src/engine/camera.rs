use bevy::math::{Mat4, Vec3};
use bevy::prelude::Component;

#[derive(Component, Clone, Debug)]
pub struct Camera {
    pub rotation_angle: f32,
    pub pitch_angle: f32,
    pub distance: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            rotation_angle: std::f32::consts::FRAC_PI_4,
            pitch_angle: 35.264_f32.to_radians(),
            distance: 24.0,
        }
    }

    pub fn build_view_projection_matrix(&self, player_pos: Vec3, aspect: f32) -> Mat4 {
        let (cp, sp) = (self.pitch_angle.cos(), self.pitch_angle.sin());
        let (cy, sy) = (self.rotation_angle.cos(), self.rotation_angle.sin());
        let offset = Vec3::new(self.distance * cp * sy, self.distance * sp, self.distance * cp * cy);

        let eye = player_pos + offset;
        let target = player_pos + Vec3::new(0.0, 0.9, 0.0);
        let view = Mat4::look_at_rh(eye, target, Vec3::Y);

        let ortho_size = self.distance * 0.6;
        let (hw, hh) = (ortho_size * aspect * 0.5, ortho_size * 0.5);
        let proj = Mat4::orthographic_rh(-hw, hw, -hh, hh, 0.1, 200.0);
        proj * view
    }

    pub fn ndc_to_ray(&self, p_pos: Vec3, aspect: f32, ndc_x: f32, ndc_y: f32) -> (Vec3, Vec3) {
        let inv_vp = self.build_view_projection_matrix(p_pos, aspect).inverse();
        let near_pt = inv_vp.project_point3(Vec3::new(ndc_x, ndc_y, 0.0));
        let far_pt = inv_vp.project_point3(Vec3::new(ndc_x, ndc_y, 1.0));
        (near_pt, (far_pt - near_pt).normalize())
    }

    pub fn screen_to_ray(&self, p_pos: Vec3, aspect: f32, sx: f32, sy: f32, sw: f32, sh: f32) -> (Vec3, Vec3) {
        self.ndc_to_ray(p_pos, aspect, (sx / sw) * 2.0 - 1.0, 1.0 - (sy / sh) * 2.0)
    }
}