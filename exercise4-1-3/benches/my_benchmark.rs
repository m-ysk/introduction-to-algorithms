use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use exercise4_1_3::{divide_and_conquer, divide_and_conquer_improved, search_all};

fn gen_random_array(n: usize) -> Vec<i64> {
    use rand::Rng;

    let mut arr = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        arr.push(rng.gen::<i64>() % 1000);
    }
    arr
}

fn bench_max_subarray(c: &mut Criterion) {
    let mut group = c.benchmark_group("max_subarray");
    for i in [1, 2, 4, 8, 16, 24, 25, 26, 27, 28, 29, 30, 31, 32].iter() {
        let arr: Vec<i64> = gen_random_array(*i);
        group.bench_with_input(BenchmarkId::new("search_all", i), &i, |b, _| {
            b.iter(|| search_all(&arr, 0, arr.len()))
        });
        group.bench_with_input(BenchmarkId::new("divide_and_conquer", i), &i, |b, _| {
            b.iter(|| divide_and_conquer(&arr, 0, arr.len()))
        });
        group.bench_with_input(
            BenchmarkId::new("divide_and_conquer_improved", i),
            &i,
            |b, _| b.iter(|| divide_and_conquer_improved(&arr, 0, arr.len())),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_max_subarray);
criterion_main!(benches);
