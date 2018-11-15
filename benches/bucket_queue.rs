#[macro_use]
extern crate bencher;
extern crate bucket_queue;

use bencher::Bencher;
use rand::distributions::{Distribution, Uniform};
use std::collections::VecDeque;

use bucket_queue::*;

type Subject<B> = BucketQueue<B>;

fn benchmark(bencher: &mut Bencher, items: usize, buckets: usize) {
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0..=buckets);

    let data: Vec<(usize, usize)> = (0..items).map(|_| {
        (dist.sample(&mut rng), dist.sample(&mut rng))
    }).collect();

    let mut subject = Subject::<VecDeque<usize>>::new();

    bencher.iter(|| {
        for (value, priority) in &data {
            subject.enqueue(*value, *priority);
        }

        while let Some(_) = subject.dequeue_min() { }
    });
}

fn benchmark_100_items_into_4_buckets(bencher: &mut Bencher) {
    benchmark(bencher, 100, 4);
}

fn benchmark_1_000_items_into_8_buckets(bencher: &mut Bencher) {
    benchmark(bencher, 1_000, 8);
}

fn benchmark_10_000_items_into_16_buckets(bencher: &mut Bencher) {
    benchmark(bencher, 10_000, 16);
}

fn benchmark_100_000_items_into_32_buckets(bencher: &mut Bencher) {
    benchmark(bencher, 100_000, 32);
}

fn benchmark_1_000_000_items_into_64_buckets(bencher: &mut Bencher) {
    benchmark(bencher, 1_000_000, 64);
}

// Benchmarks for nested BucketQueue:

fn benchmark_nested(bencher: &mut Bencher, items: usize, buckets: usize) {
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0..=buckets);

    let data: Vec<(usize, usize, usize)> = (0..items).map(|_| {
        (dist.sample(&mut rng), dist.sample(&mut rng), dist.sample(&mut rng))
    }).collect();

    let mut subject = Subject::<Subject<VecDeque<usize>>>::new();

    bencher.iter(|| {
        for (value, outer_priority, inner_priority) in &data {
            subject.bucket(*outer_priority).enqueue(*value, *inner_priority);
        }

        while let Some(_) = subject.min_bucket().dequeue_min() { }
    });
}

fn benchmark_100_items_into_4x4_nested_buckets(bencher: &mut Bencher) {
    benchmark_nested(bencher, 100, 4);
}

fn benchmark_1_000_items_into_8x8_nested_buckets(bencher: &mut Bencher) {
    benchmark_nested(bencher, 1_000, 8);
}

fn benchmark_10_000_items_into_16x16_nested_buckets(bencher: &mut Bencher) {
    benchmark_nested(bencher, 10_000, 16);
}

fn benchmark_100_000_items_into_32x32_nested_buckets(bencher: &mut Bencher) {
    benchmark_nested(bencher, 100_000, 32);
}

fn benchmark_1_000_000_items_into_64x64_nested_buckets(bencher: &mut Bencher) {
    benchmark_nested(bencher, 1_000_000, 64);
}

benchmark_group!(
    benches,

    benchmark_100_items_into_4_buckets,
    benchmark_1_000_items_into_8_buckets,
    benchmark_10_000_items_into_16_buckets,
    benchmark_100_000_items_into_32_buckets,
    benchmark_1_000_000_items_into_64_buckets,

    benchmark_100_items_into_4x4_nested_buckets,
    benchmark_1_000_items_into_8x8_nested_buckets,
    benchmark_10_000_items_into_16x16_nested_buckets,
    benchmark_100_000_items_into_32x32_nested_buckets,
    benchmark_1_000_000_items_into_64x64_nested_buckets,
);

benchmark_main!(benches);
