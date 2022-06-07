mod utils;

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
}

#[wasm_bindgen]
impl Display {
    pub fn new(width: u32, height: u32) -> Display {
        utils::set_panic_hook();
        Display {
            width: width,
            height: height,
            pixels: vec![0; (3 * width * height) as usize],
        }
    }

    pub fn tick(&mut self) -> () {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = (3 * (x * self.width + y)) as usize;
                let color = &mut self.pixels[index..(index + 3)];
                (color[0], color[1], color[2]) = pixel(x, y, self.width, self.height);
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
