use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use rust_ray::{Ray, Vec3};
fn ray_new(v1: Vec3, v2: Vec3) -> Ray {
    Ray::new(v1, v2)
}

fn ray_project(ray: &Ray, t: f32) -> Vec3 {
    ray.project(t)
}

fn rand_vec3() -> Vec3 {
    let mut rng = thread_rng();
    black_box(Vec3::new(rng.gen(), rng.gen(), rng.gen()))
}

fn rand() -> f32 {
    let mut rng = thread_rng();
    black_box(rng.gen())
}

fn rand_ray() -> Ray {
    black_box(Ray::new(rand_vec3(), rand_vec3()))
}

pub fn ray_benchmark(c: &mut Criterion) {
    c.bench_function("ray new 20", |b| {
        let v1 = rand_vec3();
        let v2 = rand_vec3();
        b.iter(|| ray_new(v1, v2))
    });
    c.bench_function("ray project 20", |b| {
        let ray = rand_ray();
        let t = rand();
        b.iter(|| ray_project(&ray, t))
    });
}

criterion_group!(benches, ray_benchmark);
criterion_main!(benches);
