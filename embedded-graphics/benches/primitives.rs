use criterion::*;
use embedded_graphics::{prelude::*, primitives::Circle};

fn circle(c: &mut Criterion) {
    c.bench_function("circle", |b| {
        let circle = Circle::new(Coord::new(10, 10), 10);

        b.iter(|| circle.into_iter().collect())
    });
}

criterion_group!(primitives, circle);
criterion_main!(primitives);
