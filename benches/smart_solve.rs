use criterion::{black_box, criterion_group, criterion_main, Criterion};

use prison::Boxes;

fn criterion_benchmark(c: &mut Criterion) {
    let mut boxes = Boxes::<1_000>::new_random(0);

    c.bench_function("solve_dumb", |b| {
        b.iter(|| black_box(boxes.solve_dumb()).count_ones() == 1_000)
    });
    c.bench_function("solve_dumb_no_list", |b| {
        b.iter(|| black_box(boxes.solve_dumb_no_list()))
    });
    c.bench_function("solve_dumb_shuffle", |b| {
        b.iter(|| black_box(boxes.solve_dumb_shuffle()))
    });
    c.bench_function("solve_smart", |b| b.iter(|| black_box(boxes.solve_smart())));
    c.bench_function("solve_smart_cycle", |b| {
        b.iter(|| black_box(boxes.solve_smart_cycle_detect()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
