use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use rust_ray::{init_scene, vector, BoundingBox, Ray, Vec3, World};
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

fn world_benchmark(c: &mut Criterion) {
    c.bench_function("world hit 20", |b| {
        let mut world = World::new();
        init_scene(&mut world, 11);
        let world = black_box(world);
        let mut rays: Vec<Ray> = Vec::new();
        for _ in 0..=100000 {
            rays.push(rand_ray());
        }
        b.iter(|| test_repeated_hit(&world, &rays))
    });
}
criterion_group!(benches, world_benchmark);
criterion_main!(benches);
