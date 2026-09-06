// Steve 64x64 skin UV coordinates for 6 faces: [Top, Bottom, Front, Back, Left, Right]
// Each face has [u0, v0, u1, v1] in 0.0..1.0 normalized coordinates.

pub type FaceUvs = [[f32; 4]; 6];

const fn f(u: f32, v: f32, w: f32, h: f32) -> [f32; 4] {
    [u / 64.0, v / 64.0, (u + w) / 64.0, (v + h) / 64.0]
}

pub fn head_uvs() -> FaceUvs {
    [
        f(8.0, 0.0, 8.0, 8.0),   // Top
        f(16.0, 0.0, 8.0, 8.0),  // Bottom
        f(8.0, 8.0, 8.0, 8.0),   // Front (+Z)
        f(24.0, 8.0, 8.0, 8.0),  // Back (-Z)
        f(16.0, 8.0, 8.0, 8.0),  // Left (+X)
        f(0.0, 8.0, 8.0, 8.0),   // Right (-X)
    ]
}

pub fn body_uvs() -> FaceUvs {
    [
        f(20.0, 16.0, 8.0, 4.0), // Top
        f(28.0, 16.0, 8.0, 4.0), // Bottom
        f(20.0, 20.0, 8.0, 12.0),// Front (+Z)
        f(32.0, 20.0, 8.0, 12.0),// Back (-Z)
        f(28.0, 20.0, 4.0, 12.0),// Left (+X)
        f(16.0, 20.0, 4.0, 12.0),// Right (-X)
    ]
}

pub fn right_arm_uvs() -> FaceUvs {
    [
        f(44.0, 16.0, 4.0, 4.0), // Top
        f(48.0, 16.0, 4.0, 4.0), // Bottom
        f(44.0, 20.0, 4.0, 12.0),// Front
        f(52.0, 20.0, 4.0, 12.0),// Back
        f(48.0, 20.0, 4.0, 12.0),// Left (inner)
        f(40.0, 20.0, 4.0, 12.0),// Right (outer)
    ]
}

pub fn left_arm_uvs() -> FaceUvs {
    [
        f(36.0, 48.0, 4.0, 4.0), // Top
        f(40.0, 48.0, 4.0, 4.0), // Bottom
        f(36.0, 52.0, 4.0, 12.0),// Front
        f(44.0, 52.0, 4.0, 12.0),// Back
        f(32.0, 52.0, 4.0, 12.0),// Left (outer)
        f(40.0, 52.0, 4.0, 12.0),// Right (inner)
    ]
}

pub fn right_leg_uvs() -> FaceUvs {
    [
        f(4.0, 16.0, 4.0, 4.0),  // Top
        f(8.0, 16.0, 4.0, 4.0),  // Bottom
        f(4.0, 20.0, 4.0, 12.0), // Front
        f(12.0, 20.0, 4.0, 12.0),// Back
        f(8.0, 20.0, 4.0, 12.0), // Left (inner)
        f(0.0, 20.0, 4.0, 12.0), // Right (outer)
    ]
}

pub fn left_leg_uvs() -> FaceUvs {
    [
        f(20.0, 48.0, 4.0, 4.0), // Top
        f(24.0, 48.0, 4.0, 4.0), // Bottom
        f(20.0, 52.0, 4.0, 12.0),// Front
        f(28.0, 52.0, 4.0, 12.0),// Back
        f(16.0, 52.0, 4.0, 12.0),// Left (outer)
        f(24.0, 52.0, 4.0, 12.0),// Right (inner)
    ]
}
