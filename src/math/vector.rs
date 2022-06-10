use std::ops::{Add, AddAssign, Mul, Neg, Sub};

use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const FORWARD: Vec3 = Vec3::new(0.0, 0.0, 1.0);
pub const BACK: Vec3 = Vec3::new(0.0, 0.0, -1.0);
pub const RIGHT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
pub const LEFT: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
pub const DOWN: Vec3 = Vec3::new(0.0, 1.0, 0.0);
pub const UP: Vec3 = Vec3::new(0.0, -1.0, 0.0);

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

    pub fn random_unit() -> Vec3 {
        let mut rng = thread_rng();
        loop {
            let v = Vec3 {
                x: rng.gen_range(-1.0..=1.0),
                y: rng.gen_range(-1.0..=1.0),
                z: rng.gen_range(-1.0..=1.0),
            };
            if v.length_squared() <= 1.0 {
                return v.normalized();
            }
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

impl Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, other: Vec3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}
