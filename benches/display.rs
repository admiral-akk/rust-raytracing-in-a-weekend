use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use rust_ray::{init_scene, vector, BoundingBox, Display, Ray, Vec3, World};
fn rand_vec3() -> Vec3 {
    let mut rng = thread_rng();
    black_box(Vec3::new(rng.gen(), rng.gen(), rng.gen()))
}

fn rand_ray() -> Ray {
    Ray::new(rand_vec3(), rand_vec3().normalized())
}
fn test_repeated_hit(world: &World, rays: &Vec<Ray>) {
    for ray in rays {
        world.hit(ray);
    }
}

fn new_display(width: u32, height: u32, sample_count: u32, fov_angle: f32) -> Display {
    Display::new(width, height, sample_count, fov_angle)
}

fn display_benchmark(c: &mut Criterion) {
    c.bench_function("display tiny", |b| {
        let mut display = black_box(Display::new(10, 10, 1, 20.0));
        b.iter(|| display.tick(0))
    });
    c.bench_function("display tiny high sample", |b| {
        let mut display = black_box(Display::new(10, 10, 10, 20.0));
        b.iter(|| display.tick(0))
    });
    c.bench_function("display medium", |b| {
        let mut display = black_box(Display::new(100, 100, 1, 20.0));
        b.iter(|| display.tick(0))
    });
    c.bench_function("display medium high sample", |b| {
        let mut display = black_box(Display::new(100, 100, 10, 20.0));
        b.iter(|| display.tick(0))
    });
}
criterion_group!(benches, display_benchmark);
criterion_main!(benches);
