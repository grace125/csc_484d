use criterion::{black_box, criterion_group, criterion_main, Criterion};
use a1::sin::*;

fn quadratic_benchmark(c: &mut Criterion) {
    c.bench_function("quad sin", |b| b.iter(|| {
        sin_quadratic(black_box(10.0));
    }));
}

fn linear_benchmark(c: &mut Criterion) {
    c.bench_function("lin sin", |b| b.iter(|| { 
        sin_linear(black_box(10.0));
    }));
}

fn taylor_benchmark(c: &mut Criterion) {
    c.bench_function("taylor sin",  |b| b.iter(|| {
        sin_taylor(black_box(10.0));
    }));
}

fn std_benchmark(c: &mut Criterion) {
    c.bench_function("std sin",|b| b.iter(|| {
        let _ = black_box(10.0_f32).sin();
    }));
}

criterion_group!(benches, linear_benchmark, quadratic_benchmark, taylor_benchmark, std_benchmark);
criterion_main!(benches);