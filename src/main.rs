use std::{
    fs::{self, File},
    io::Write,
};

use rust_ray::Display;
fn main() -> std::io::Result<()> {
    let mut display = Display::new(10 * 128, 10 * 72, 50000, 20.0);
    display.tick(0);
    let mut file = File::create("test8.ppm")?;
    let s = format!("P3\n{} {} 255\n", display.width(), display.height());
    file.write(s.as_bytes())?;
    for y in 0..display.height() {
        for x in 0..display.width() {
            let (r, g, b) = display.rgb(x, y);
            let s = format!("{} {} {}\n", r, g, b);
            file.write(s.as_bytes())?;
        }
    }
    Ok(())
}
