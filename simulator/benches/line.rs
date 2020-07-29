use core::cmp::Ordering;
use criterion::*;
use embedded_graphics::{geometry::Point, prelude::*, primitives::*, style::StrokeAlignment};

fn line_extents(c: &mut Criterion) {
    c.bench_function("line extents", |b| {
        let object = Line::new(Point::new(100, 100), Point::new(85, 35));

        b.iter(|| object.extents(30, StrokeAlignment::Center))
    });
}

criterion_group!(line, line_extents);
criterion_main!(line);
