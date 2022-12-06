use criterion::{criterion_group, criterion_main, Criterion};
use elo_multiplayer::EloRank;

fn calculate_elo(vec: Vec<f64>, base: f64) -> Vec<f64> {
    let elo = EloRank {
        players: vec,
        score_base: base,
        ..Default::default()
    };
    elo.calculate()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Elo calc");
    group.bench_function("4 players", |b| {
        b.iter(|| calculate_elo(vec![1000f64; 4], 1f64))
    });
    group.bench_function("5 players", |b| {
        b.iter(|| calculate_elo(vec![1000f64; 5], 1f64))
    });
    group.bench_function("4 players, 1.5 base", |b| {
        b.iter(|| calculate_elo(vec![1000f64; 4], 1.5))
    });
    group.bench_function("5 players, 1.5 base", |b| {
        b.iter(|| calculate_elo(vec![1000f64; 5], 1.5))
    });
    group.bench_function("100 players", |b| {
        b.iter(|| calculate_elo(vec![1000f64; 100], 1.0))
    });
    group.bench_function("100 players, 1.5 base", |b| {
        b.iter(|| calculate_elo(vec![1000f64; 100], 1.5))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
