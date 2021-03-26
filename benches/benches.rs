use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

fn test_vectorization(c: &mut Criterion) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let mut rng = rand::thread_rng();

    for _ in 0..1024 {
        left.push(rng.gen::<f32>());
        right.push(rng.gen::<f32>());
    }

    c.bench_function("fully safe", |b| {
        b.iter(|| project::fully_safe(&left, &right))
    });

    c.bench_function("some unsafe", |b| {
        b.iter(|| project::some_unsafe(&left, &right))
    });
}

criterion_group!(benches, test_vectorization);
criterion_main!(benches);
