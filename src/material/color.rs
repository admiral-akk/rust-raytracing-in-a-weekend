use std::ops::{Add, AddAssign, Mul};

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn to_rgb(&self, iterations: f32) -> (u8, u8, u8) {
        (
            (256.0 * (self.r / iterations).sqrt()) as u8,
            (256.0 * (self.g / iterations).sqrt()) as u8,
            (256.0 * (self.b / iterations).sqrt()) as u8,
        )
    }
}

pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        return Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        };
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Self {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}
