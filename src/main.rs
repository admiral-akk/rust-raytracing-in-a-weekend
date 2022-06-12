use rust_ray::Display;
fn main() {
    let mut display = Display::new(1280, 720, 1, 20.0);
    display.tick(0);
}
