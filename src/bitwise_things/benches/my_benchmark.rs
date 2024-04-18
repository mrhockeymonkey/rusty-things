use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use bitwise_things::{is_odd_bitwise_and, is_odd_modulo, is_pow_2, is_pow_2_bitwise};

pub fn bench_odd(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_odd");
    for input in [4, 23567, 111110056, u32::MAX]{
        group.bench_with_input(BenchmarkId::new("odd modulo", input), &input, |b, &i| b.iter(|| is_odd_modulo(i)));
        group.bench_with_input(BenchmarkId::new("odd bitwise", input), &input, |b, &i| b.iter(|| is_odd_bitwise_and(i)));
    }
}

pub fn bench_pow_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("pow_2");
    let input: u64 = 2^58;
    group.bench_with_input(BenchmarkId::new("division", input), &input, |b,  &i| b.iter(|| is_pow_2(i)));
    group.bench_with_input(BenchmarkId::new("bitwise", input), &input, |b,  &i| b.iter(|| is_pow_2_bitwise(i)));

}

criterion_group!(benches, bench_pow_2);
criterion_main!(benches);