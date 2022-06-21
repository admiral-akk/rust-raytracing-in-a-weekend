pub mod display;
mod hittable;
mod material;
mod math;
mod rand;
mod scene;
mod utils;

pub use crate::display::Display;
pub use crate::hittable::bounding_box::BoundingBox;
pub use crate::hittable::hittable::Hittable;
pub use crate::hittable::sphere::Sphere;
pub use crate::material::color::Color;
pub use crate::material::dielectric::Dielectric;
pub use crate::material::lambertian::Lambertian;
pub use crate::material::metal::Metal;
pub use crate::math::ray::Ray;
pub use crate::math::simd_vector::Vec3Simd;
pub use crate::math::vector;
pub use crate::math::vector::Vec3;
pub use crate::rand::rand::Rand;
pub use crate::scene::camera::Camera;
pub use crate::scene::init_scene::init_scene;
pub use crate::scene::object::Object;
pub use crate::scene::world::World;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
