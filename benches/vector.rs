use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use rust_ray::{Vec3, Vec3Simd};

fn vector_add_assign(v1: &mut Vec3, v2: Vec3) {
    *v1 += v2;
}

fn vector_normalize(v: Vec3) -> Vec3 {
    v.normalized()
}

fn vector_length(v: Vec3) -> f32 {
    v.length()
}

fn vector_length_squared(v: Vec3) -> f32 {
    v.length_squared()
}

fn vector_cross(v1: Vec3, v2: Vec3) -> Vec3 {
    v1.cross(&v2)
}

fn vector_min(v1: Vec3, v2: Vec3) -> Vec3 {
    v1.min(&v2)
}
fn vector_max(v1: Vec3, v2: Vec3) -> Vec3 {
    v1.max(&v2)
}

fn vector_add(v1: Vec3, v2: Vec3) -> Vec3 {
    v1 + v2
}

fn vector_mul(v: Vec3, val: f32) -> Vec3 {
    v * val
}
fn vector_neg(v: Vec3) -> Vec3 {
    -v
}
fn vector_mul_f32(val: f32, v: Vec3) -> Vec3 {
    val * v
}

fn vector_sub_addr(v1: Vec3, v2: Vec3) -> Vec3 {
    &v1 - &v2
}

fn vector_sub(v1: Vec3, v2: Vec3) -> Vec3 {
    v1 - v2
}
fn vector_dot(v1: Vec3, v2: Vec3) -> f32 {
    &v1 * &v2
}
fn vector_simd_dot(n: u64) {
    let v: Vec3Simd = Vec3Simd::new(4.0, 5.0, 6.0);
    let w: Vec3Simd = Vec3Simd::new(1.0, 2.0, 3.0);
    let x = &v * &w;
}
fn vector_new(n: u64) {
    let v: Vec3 = Vec3::new(4.0, 5.0, 6.0);
}
fn vector_simd_new(n: u64) {
    let v: Vec3Simd = Vec3Simd::new(4.0, 5.0, 6.0);
}

fn rand_vec3() -> Vec3 {
    let mut rng = thread_rng();
    black_box(Vec3::new(rng.gen(), rng.gen(), rng.gen()))
}

fn rand() -> f32 {
    let mut rng = thread_rng();
    black_box(rng.gen())
}

pub fn vector_benchmark(c: &mut Criterion) {
    c.bench_function("vector_new 20", |b| b.iter(|| vector_new(black_box(20))));

    c.bench_function("vector_simd_new 20", |b| {
        b.iter(|| vector_simd_new(black_box(20)))
    });

    c.bench_function("vector_dot 20", |b| {
        let v1 = rand_vec3();
        let v2 = rand_vec3();
        b.iter(|| vector_dot(v1, v2))
    });

    c.bench_function("vector_simd_dot 20", |b| {
        b.iter(|| vector_simd_dot(black_box(20)))
    });

    c.bench_function("vector add assign 20", |b| {
        let mut v1 = rand_vec3();
        let v2 = rand_vec3();
        b.iter(|| vector_add_assign(&mut v1, v2))
    });
    c.bench_function("vector normalize 20", |b| {
        let v1 = rand_vec3();
        b.iter(|| vector_normalize(v1))
    });
    c.bench_function("vector length 20", |b| {
        let v1 = rand_vec3();
        b.iter(|| vector_length(v1))
    });

    c.bench_function("vector length squared 20", |b| {
        let v1 = rand_vec3();
        b.iter(|| vector_length_squared(v1))
    });
    c.bench_function("vector cross 20", |b| {
        let v1 = rand_vec3();
        let v2 = rand_vec3();
        b.iter(|| vector_cross(v1, v2))
    });

    c.bench_function("vector max 20", |b| {
        let v1 = rand_vec3();
        let v2 = rand_vec3();
        b.iter(|| vector_max(v1, v2))
    });
    c.bench_function("vector min 20", |b| {
        let v1 = rand_vec3();
        let v2 = rand_vec3();
        b.iter(|| vector_min(v1, v2))
    });

    c.bench_function("vector_add 20", |b| {
        let v1 = rand_vec3();
        let v2 = rand_vec3();
        b.iter(|| vector_add(v1, v2))
    });
    c.bench_function("vector_mul 20", |b| {
        let v1 = rand_vec3();
        let val = rand();
        b.iter(|| vector_mul(v1, val))
    });
    c.bench_function("vector_neg 20", |b| {
        let v1 = rand_vec3();
        b.iter(|| vector_neg(v1))
    });
    c.bench_function("vector_mul_f32 20", |b| {
        let v1 = rand_vec3();
        let val = rand();
        b.iter(|| vector_mul_f32(val, v1))
    });
    c.bench_function("vector_sub_addr 20", |b| {
        let v1 = rand_vec3();
        let v2 = rand_vec3();
        b.iter(|| vector_sub_addr(v1, v2))
    });
    c.bench_function("vector_sub 20", |b| {
        let v1 = rand_vec3();
        let v2 = rand_vec3();
        b.iter(|| vector_sub(v1, v2))
    });
}

criterion_group!(benches, vector_benchmark);
criterion_main!(benches);
