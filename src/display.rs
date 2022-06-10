use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    material::{color, dielectric::Dielectric},
    math::vector,
    scene::object::Object,
    utils, Camera, Color, Lambertian, Metal, Sphere, World,
};

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
    pub fn new(width: u32, height: u32, sample_count: u32, fov_angle: f32) -> Display {
        utils::set_panic_hook();
        let mut display = Display {
            width: width,
            height: height,
            pixels: vec![0; (3 * width * height) as usize],
            sample_count: sample_count,
            camera: Camera::new((width as f32) / (height as f32), fov_angle),
            world: World::new(),
        };
        display.world.push(Object::new(
            Box::new(Sphere::new(vector::FORWARD, 0.5)),
            Box::new(Dielectric::new(1.5)),
        ));
        display.world.push(Object::new(
            Box::new(Sphere::new(vector::FORWARD + vector::RIGHT, 0.5)),
            Box::new(Metal::new(color::LIGHT_RED, 1.0)),
        ));
        display.world.push(Object::new(
            Box::new(Sphere::new(vector::FORWARD + vector::LEFT, 0.5)),
            Box::new(Dielectric::new(1.5)),
        ));
        display.world.push(Object::new(
            Box::new(Sphere::new(vector::FORWARD + 100.5 * vector::DOWN, 100.0)),
            Box::new(Lambertian::new(color::GREY)),
        ));
        return display;
    }

    pub fn tick(&mut self, _time: u32) -> () {
        let time = 0;
        let mut rng = thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                let mut color = Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
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