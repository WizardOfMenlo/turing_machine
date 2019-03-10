use criterion::{black_box, criterion_group, criterion_main, Criterion};
use turing_machine::common::*;
use turing_machine::utils::apply_action;

fn action_benchmark(c: &mut Criterion) {
    c.bench_function("action_right", |b| {
        let mut v = Vec::new();
        let mut s = 0;
        let mut pos = 0;
        let motion = Action::new(0, '1', Motion::Right);
        b.iter(|| {
            apply_action(motion.clone(), &mut v, &mut s, &mut pos);
        });
        black_box(v);
        black_box(s);
        black_box(pos);
    });

    c.bench_function("action_left", |b| {
        let mut v = Vec::new();
        let mut s = 0;
        let mut pos = 0;
        let motion = Action::new(0, '1', Motion::Left);
        b.iter(|| {
            apply_action(motion.clone(), &mut v, &mut s, &mut pos);
        });
        black_box(v);
        black_box(s);
        black_box(pos);
    });
}

criterion_group!(benches, action_benchmark);
criterion_main!(benches);
