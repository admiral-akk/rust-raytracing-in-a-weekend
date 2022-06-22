use std::arch::x86_64::{__m128, _mm_extract_ps, _mm_hadd_ps, _mm_mul_ps, _mm_set_ps};
use std::ops::Mul;

pub struct Vec3Simd {
    pub xyzw: __m128,
}

impl Vec3Simd {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3Simd {
        unsafe {
            Vec3Simd {
                xyzw: _mm_set_ps(x, y, z, 0.0),
            }
        }
    }
}

impl Mul<&Vec3Simd> for &Vec3Simd {
    type Output = f32;
    fn mul(self, other: &Vec3Simd) -> f32 {
        unsafe {
            let mut v = _mm_mul_ps(self.xyzw, other.xyzw);
            v = _mm_hadd_ps(v, v);
            return _mm_extract_ps(_mm_hadd_ps(v, v), 0) as f32;
        }
    }
}
