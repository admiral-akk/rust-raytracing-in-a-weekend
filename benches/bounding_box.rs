use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use rust_ray::{vector, BoundingBox, Ray, Vec3};
fn rand_vec3() -> Vec3 {
    let mut rng = thread_rng();
    black_box(Vec3::new(rng.gen(), rng.gen(), rng.gen()))
}
fn rand_bounding_box() -> BoundingBox {
    let min = rand_vec3();
    black_box(BoundingBox::new(min, min + vector::ONE * 10.0))
}

fn box_hit(b: &BoundingBox, ray: &Ray) -> bool {
    let mut min = f32::NEG_INFINITY;
    let mut max = f32::INFINITY;
    b.is_hit(&ray, &mut min, &mut max)
}

fn bounding_box_benchmark(c: &mut Criterion) {
    c.bench_function("bounding box hit 20", |b| {
        let bb = rand_bounding_box();
        let ray = Ray::new(bb.min - vector::ONE, vector::ONE.normalized());
        b.iter(|| box_hit(&bb, &ray))
    });
    c.bench_function("bounding box miss 20", |b| {
        let bb = rand_bounding_box();
        let ray = Ray::new(bb.min - vector::ONE, -vector::ONE.normalized());
        b.iter(|| box_hit(&bb, &ray))
    });
}
criterion_group!(benches, bounding_box_benchmark);
criterion_main!(benches);
