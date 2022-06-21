use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use rust_ray::{vector, BoundingBox, Hittable, Ray, Sphere, Vec3};
fn sphere_new(v1: Vec3, rad: f32) -> Sphere {
    Sphere::new(v1, rad)
}

fn sphere_hit(sphere: &Sphere, ray: &Ray) -> f32 {
    sphere.hit(ray)
}
fn sphere_normal(sphere: &Sphere, ray: &Ray, hit_point: &Vec3) -> Vec3 {
    sphere.hit_normal(ray, hit_point)
}
fn sphere_bounds(sphere: &Sphere) -> BoundingBox {
    sphere.bounds()
}
fn rand_vec3() -> Vec3 {
    let mut rng = thread_rng();
    black_box(Vec3::new(rng.gen(), rng.gen(), rng.gen()))
}

fn rand() -> f32 {
    let mut rng = thread_rng();
    black_box(rng.gen())
}

fn rand_rang(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    black_box(rng.gen_range(min..=max))
}

fn rand_sphere() -> Sphere {
    black_box(Sphere::new(rand_vec3(), f32::abs(rand_rang(0.1, 1000.0))))
}

pub fn sphere_benchmark(c: &mut Criterion) {
    c.bench_function("sphere new 20", |b| {
        let v1 = rand_vec3();
        let rad = rand();
        b.iter(|| sphere_new(v1, rad))
    });
    c.bench_function("sphere hit, ray outside", |b| {
        let sphere = rand_sphere();

        let ray = black_box(Ray::new(
            sphere.pos - 4.0 * sphere.radius_sq.sqrt() * vector::ONE,
            vector::ONE.normalized(),
        ));
        b.iter(|| sphere_hit(&sphere, &ray))
    });
    c.bench_function("sphere hit, ray inside", |b| {
        let sphere = rand_sphere();

        let ray = black_box(Ray::new(
            sphere.pos - 0.2 * sphere.radius_sq.sqrt() * vector::ONE,
            vector::ONE.normalized(),
        ));
        b.iter(|| sphere_hit(&sphere, &ray))
    });
    c.bench_function("sphere miss", |b| {
        let sphere = rand_sphere();

        let ray = black_box(Ray::new(
            sphere.pos - 4.0 * sphere.radius_sq.sqrt() * vector::ONE,
            -vector::ONE.normalized(),
        ));
        b.iter(|| sphere_hit(&sphere, &ray))
    });
    c.bench_function("sphere normal", |b| {
        let sphere = rand_sphere();

        let ray = black_box(Ray::new(
            sphere.pos - 4.0 * sphere.radius_sq.sqrt() * vector::ONE,
            vector::ONE.normalized(),
        ));

        let t = sphere.hit(&ray);
        let hit_point = black_box(ray.project(t));

        b.iter(|| sphere_normal(&sphere, &ray, &hit_point))
    });
    c.bench_function("sphere bounds", |b| {
        let sphere = rand_sphere();

        b.iter(|| sphere_bounds(&sphere))
    });
}

criterion_group!(benches, sphere_benchmark);
criterion_main!(benches);
