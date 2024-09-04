use criterion::{criterion_group, criterion_main, Criterion};
use crate::algorithms::{
    newhope::NewHope, sphincs::Sphincs, mceliece::McEliece, sike::Sike, new_algorithm::NewAlgorithm,
    new_algorithm_advanced::AdvancedAlgorithm,
};

#[allow(dead_code)]
fn benchmark_newhope(c: &mut Criterion) {
    let newhope = NewHope::new();
    let public_key = vec![1, 2, 3, 4];
    c.bench_function("NewHope Key Exchange", |b| {
        b.iter(|| newhope.exchange(&public_key))
    });
}

#[allow(dead_code)]
fn benchmark_sphincs(c: &mut Criterion) {
    let sphincs = Sphincs::new();
    let message = vec![1, 2, 3, 4];
    c.bench_function("SPHINCS+ Signing", |b| {
        b.iter(|| sphincs.sign(&message))
    });
}

#[allow(dead_code)]
fn benchmark_mceliece(c: &mut Criterion) {
    let mceliece = McEliece::new();
    let plaintext = vec![1, 2, 3, 4];
    c.bench_function("McEliece Encryption", |b| {
        b.iter(|| mceliece.encrypt(&plaintext))
    });
}

#[allow(dead_code)]
fn benchmark_sike(c: &mut Criterion) {
    let sike = Sike::new();
    c.bench_function("SIKE Key Encapsulation", |b| {
        b.iter(|| sike.encapsulate())
    });
}

#[allow(dead_code)]
fn benchmark_new_algorithm(c: &mut Criterion) {
    let algorithm = NewAlgorithm::new();
    let input_data = vec![1, 2, 3, 4];
    c.bench_function("NewAlgorithm Process Data", |b| {
        b.iter(|| algorithm.process_data(&input_data))
    });
}

#[allow(dead_code)]
fn benchmark_advanced_algorithm(c: &mut Criterion) {
    let algorithm = AdvancedAlgorithm::new();
    let plaintext = vec![1, 2, 3, 4];
    c.bench_function("AdvancedAlgorithm Encryption", |b| {
        b.iter(|| algorithm.encrypt(&plaintext))
    });
}

criterion_group!(
    benches,
    benchmark_newhope,
    benchmark_sphincs,
    benchmark_mceliece,
    benchmark_sike,
    benchmark_new_algorithm,
    benchmark_advanced_algorithm
);
criterion_main!(benches);
