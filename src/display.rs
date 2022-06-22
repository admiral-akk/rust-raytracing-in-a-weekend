use std::time::SystemTime;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    material::color, math::vector, scene::init_scene, utils, Camera, Color, Rand, Vec3, World,
};

#[wasm_bindgen]
pub struct Display {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    sample_count: u32,
    camera: Camera,
    world: World,
    rand: Rand,
}

impl Display {
    pub fn rgb(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let index = (3 * ((x % self.width) + (y % self.height) * self.width)) as usize;
        return (
            self.pixels[index],
            self.pixels[index + 1],
            self.pixels[index + 2],
        );
    }
}

struct Samples {
    color: Color,
    sample_count: u32,
}

impl Samples {
    pub fn new() -> Samples {
        Samples {
            color: color::BLACK,
            sample_count: 0,
        }
    }

    pub fn add(&mut self, color: &Color) {
        self.color += *color;
        self.sample_count += 1;
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        return self.color.to_rgb(self.sample_count as f32);
    }
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
            camera: Camera::new(
                (width as f32) / (height as f32),
                fov_angle,
                Vec3 {
                    x: 13.0,
                    y: 2.0,
                    z: 3.0,
                },
                vector::ZERO,
            ),
            world: World::new(),
            rand: Rand::new(7919),
        };
        init_scene::init_scene(&mut display.world, 11);
        return display;
    }

    fn measure_delta(&self, x: i32, y: i32) -> f32 {
        let top = self.rgb_int(x, y - 1);
        let left = self.rgb_int(x - 1, y);
        let bot = self.rgb_int(x, y + 1);
        let right = self.rgb_int(x + 1, y);
        let mid = self.rgb_int(x, y);

        return (i32::abs(bot.0 + top.0 - 2 * mid.0)
            + i32::abs(bot.1 + top.1 - 2 * mid.1)
            + i32::abs(bot.2 + top.2 - 2 * mid.2)
            + i32::abs(left.0 + right.0 - 2 * mid.0)
            + i32::abs(left.1 + right.1 - 2 * mid.1)
            + i32::abs(left.2 + right.2 - 2 * mid.2)) as f32
            / (255.0 * 12.0);
    }

    fn rgb_int(&self, x: i32, y: i32) -> (i32, i32, i32) {
        if x < 0 || x >= (self.width as i32) || y < 0 || y >= (self.height as i32) {
            return (0, 0, 0);
        }
        let index = (y * (self.width as i32) + x) as usize;
        return (
            self.pixels[3 * index] as i32,
            self.pixels[3 * index + 1] as i32,
            self.pixels[3 * index + 2] as i32,
        );
    }

    pub fn tick(&mut self, _time: u32) -> () {
        let mut samples: Vec<Samples> = Vec::new();
        while samples.len() < (self.height * self.width) as usize {
            samples.push(Samples::new());
        }

        for _ in 0..self.sample_count {
            self.rand = Rand::new(7919);
            let time = SystemTime::now();
            let mut sample_count = 0;
            for y in 0..self.height {
                for x in 0..self.width {
                    let index = (y * self.width + x) as usize;
                    let delta = self.measure_delta(x as i32, y as i32);
                    if delta + 1.0 / (1.0 + samples[index].sample_count as f32)
                        > self.rand.random_range(0.0, 0.5)
                    {
                        let mut color = self.camera.color(
                            ((x as f32) + self.rand.rand()) / (self.width as f32),
                            ((y as f32) + self.rand.rand()) / (self.height as f32),
                            &self.world,
                            &mut self.rand,
                        );
                        sample_count = sample_count + 1;
                        while color == color::BLACK {
                            sample_count = sample_count + 1;
                            samples[index].add(&color);
                            color = self.camera.color(
                                ((x as f32) + self.rand.rand()) / (self.width as f32),
                                ((y as f32) + self.rand.rand()) / (self.height as f32),
                                &self.world,
                                &mut self.rand,
                            );
                        }
                        samples[index].add(&color);
                        let color_arr = &mut self.pixels[(3 * index)..(3 * index + 3)];
                        (color_arr[0], color_arr[1], color_arr[2]) = samples[index].rgb();
                    }
                }
            }
            //print!(
            //    "{} samples take\n{}ms passed\n",
            //   sample_count,
            //   SystemTime::now().duration_since(time).unwrap().as_millis()
            //);
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
