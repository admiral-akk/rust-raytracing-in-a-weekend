use crate::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn to_rgb(&self, iterations: f32) -> (u8, u8, u8) {
        (
            (256.0 * (self.x / iterations).sqrt()) as u8,
            (256.0 * (self.y / iterations).sqrt()) as u8,
            (256.0 * (self.z / iterations).sqrt()) as u8,
        )
    }
}

pub const BLACK: Color = Color {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
