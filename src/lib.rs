pub mod display;
mod hittable;
mod material;
mod math;
mod scene;
mod utils;

pub use crate::display::Display;
pub use crate::hittable::sphere::Sphere;
pub use crate::material::color::Color;
pub use crate::material::lambertian::Lambertian;
pub use crate::material::metal::Metal;
pub use crate::math::ray::Ray;
pub use crate::math::vector::Vec3;
pub use crate::scene::camera::Camera;
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
