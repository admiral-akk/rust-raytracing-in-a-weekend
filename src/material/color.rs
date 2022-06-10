use std::ops::{Add, AddAssign, Mul};

use crate::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn to_rgb(&self, iterations: f32) -> (u8, u8, u8) {
        (
            (256.0 * (self.r / iterations).sqrt()) as u8,
            (256.0 * (self.g / iterations).sqrt()) as u8,
            (256.0 * (self.b / iterations).sqrt()) as u8,
        )
    }

    pub fn sky_color(dir: &Vec3) -> Color {
        let t = 0.5 * (dir.y + 1.0);
        return t * WHITE + (1.0 - t) * LIGHT_BLUE;
    }
}

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const LIGHT_BLUE: Color = Color::new(0.5, 0.7, 1.0);
pub const LIGHT_RED: Color = Color::new(1.0, 0.7, 0.5);
pub const GREY: Color = Color::new(0.5, 0.5, 0.5);

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
impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self * other.r,
            g: self * other.g,
            b: self * other.b,
        }
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
