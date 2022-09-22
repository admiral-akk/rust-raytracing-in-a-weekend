use std::{fs::File, io::Write, time::SystemTime};

use rust_ray::Display;
fn main() -> std::io::Result<()> {
    println!("Render starting.");
    let time = SystemTime::now();
    let mut display = Display::new(10 * 128, 10 * 72, 5000, 20.0);
    display.tick(0);
    let final_time = SystemTime::now();
    let delta = final_time.duration_since(time);

    println!("Time to render: {}ms", delta.unwrap().as_millis());
    println!("Render complete, outputting!");
    let mut file = File::create("test10.ppm")?;
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
