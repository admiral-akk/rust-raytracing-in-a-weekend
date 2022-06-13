use std::ops::{Add, AddAssign, Mul, Neg, Sub};

use crate::Rand;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const ONE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
pub const DOWN: Vec3 = Vec3::new(0.0, -1.0, 0.0);
pub const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
pub const LEFT: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
pub const RIGHT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
pub const FORWARD: Vec3 = Vec3::new(0.0, 0.0, -1.0);
pub const BACK: Vec3 = Vec3::new(0.0, 0.0, 1.0);

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn normalized(&self) -> Vec3 {
        let norm = self.length();
        Vec3 {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn random_unit(rand: &mut Rand) -> Vec3 {
        return rand.random_unit();
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn min(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: f32::min(self.x, other.x),
            y: f32::min(self.y, other.y),
            z: f32::min(self.z, other.z),
        }
    }

    pub fn max(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: f32::max(self.x, other.x),
            y: f32::max(self.y, other.y),
            z: f32::max(self.z, other.z),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Self {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = f32;
    fn mul(self, other: &Vec3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}
