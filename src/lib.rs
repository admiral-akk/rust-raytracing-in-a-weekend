mod camera;
mod hittable;
mod math;
mod utils;

pub use crate::camera::Camera;

pub use crate::hittable::sphere::Sphere;
pub use crate::math::ray::Ray;
use crate::math::vector;
pub use crate::math::vector::Vec3;
use hittable::Hittable;
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
    camera: Camera,
}

static mut WORLD: Vec<Box<dyn Hittable>> = Vec::new();

#[wasm_bindgen]
impl Display {
    pub fn new(width: u32, height: u32) -> Display {
        utils::set_panic_hook();
        let display = Display {
            width: width,
            height: height,
            pixels: vec![0; (3 * width * height) as usize],
            camera: Camera::new((width as f32) / (height as f32)),
        };
        unsafe {
            WORLD.push(Box::new(Sphere {
                pos: vector::FORWARD * 3.0,
                radius: 1.0,
            }));
            WORLD.push(Box::new(Sphere {
                pos: vector::FORWARD * 5.0 + vector::RIGHT * 3.0,
                radius: 1.0,
            }));
        }
        return display;
    }

    pub fn tick(&mut self, time: u32) -> () {
        let time = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (3
                    * (((x + time) % self.width) + ((y + time) % self.height) * self.width))
                    as usize;
                let color = &mut self.pixels[index..(index + 3)];
                unsafe {
                    (color[0], color[1], color[2]) = self.camera.color(
                        (x as f32) / (self.width as f32),
                        (y as f32) / (self.height as f32),
                        &WORLD,
                    );
                }
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

fn pixel(x: u32, y: u32, width: u32, height: u32) -> (u8, u8, u8) {
    return (((256 * x) / width) as u8, ((256 * y) / height) as u8, 0);
}
