use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};
use rand::{distributions::Alphanumeric, rngs::ThreadRng, Rng};

pub fn generate_random_message(message_size: usize, rng: &mut ThreadRng) -> String {
    rng.sample_iter(Alphanumeric)
        .take(message_size)
        .map(char::from)
        .collect()
}

pub fn benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let mut group = c.benchmark_group("Lzss Transposition Cycle");
    group.sampling_mode(SamplingMode::Flat);

    for sliding_window_size in (20..=30).step_by(5) {
        for lookahead_buffer_size in (5..=10).step_by(2) {
            let dataset = generate_random_message(1 + rng.gen::<usize>() % 100, &mut rng);
            let encrypted_dataset = lzsrs::lzss_encode_dataset(
                &mut dataset.chars(),
                sliding_window_size,
                lookahead_buffer_size,
            )
            .unwrap()
            .collect::<Vec<_>>();

            group.bench_with_input(
                BenchmarkId::new(
                    "lzss-encryption",
                    format!("({sliding_window_size}, {lookahead_buffer_size})"),
                ),
                &dataset,
                |b, input| {
                    b.iter_with_large_drop(|| {
                        let mut itr = input.chars();
                        let _res = lzsrs::lzss_encode_dataset(
                            &mut itr,
                            sliding_window_size,
                            lookahead_buffer_size,
                        );
                    });
                },
            );

            group.bench_with_input(
                BenchmarkId::new(
                    "lzss-decryption",
                    format!("({sliding_window_size}, {lookahead_buffer_size})"),
                ),
                &encrypted_dataset,
                |b, input| {
                    b.iter(|| lzsrs::lzss_decode_dataset(input.iter().cloned()));
                },
            );
        }
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .significance_level(0.1)
        .sample_size(300)
        .confidence_level(0.85);
    targets = benchmark
}
criterion_main!(benches);
