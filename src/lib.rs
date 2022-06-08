mod camera;
mod color;
mod hittable;
mod math;
mod utils;
mod world;

pub use crate::camera::Camera;

pub use crate::color::Color;
pub use crate::hittable::sphere::Sphere;
pub use crate::math::ray::Ray;
use crate::math::vector;
pub use crate::math::vector::Vec3;
pub use crate::world::World;
use rand::{thread_rng, Rng};
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

#[wasm_bindgen]
pub struct Display {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    sample_count: u32,
    camera: Camera,
    world: World,
}

#[wasm_bindgen]
impl Display {
    pub fn new(width: u32, height: u32, sample_count: u32) -> Display {
        utils::set_panic_hook();
        let mut display = Display {
            width: width,
            height: height,
            pixels: vec![0; (3 * width * height) as usize],
            sample_count: sample_count,
            camera: Camera::new((width as f32) / (height as f32)),
            world: World::new(),
        };
        display.world.push(Box::new(Sphere {
            pos: vector::FORWARD,
            radius: 0.5,
        }));
        display.world.push(Box::new(Sphere {
            pos: vector::FORWARD + (vector::DOWN * 100.5),
            radius: 100.0,
        }));
        return display;
    }

    pub fn tick(&mut self, _time: u32) -> () {
        let time = 0;
        let mut rng = thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                let mut color = Color {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                for _ in 0..self.sample_count {
                    color += self.camera.color(
                        ((x as f32) + rng.gen_range(0.0..=1.0)) / (self.width as f32),
                        ((y as f32) + rng.gen_range(0.0..=1.0)) / (self.height as f32),
                        &self.world,
                    );
                }

                let index = (3
                    * (((x + time) % self.width) + ((y + time) % self.height) * self.width))
                    as usize;
                let color_arr = &mut self.pixels[index..(index + 3)];
                (color_arr[0], color_arr[1], color_arr[2]) = color.to_rgb(self.sample_count as f32);
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn pixels(&self) -> *const u8 {
        self.pixels.as_ptr()
    }
}
