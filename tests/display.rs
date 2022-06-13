#[cfg(test)]

mod tests {
    use rust_ray::Display;

    #[test]
    fn tiny() {
        let mut display = Display::new(10, 10, 1, 90.0);
        display.tick(0);
    }
}
