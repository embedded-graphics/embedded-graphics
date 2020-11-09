use criterion::*;
use embedded_graphics::{prelude::*, primitives::*};

fn triangle_contains_inside(c: &mut Criterion) {
    c.bench_function("triangle contains point (inside)", |b| {
        let object = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(5, 20));
        let midpoint = object.bounding_box().center();

        b.iter(|| object.contains(midpoint))
    });
}

// Point outside triangle but still within bounding box. If point is outside bounding box, this
// benchmark doesn't exercise the expensive point-in-triangle code.
fn triangle_contains_outside(c: &mut Criterion) {
    c.bench_function("triangle contains point (outside)", |b| {
        let object = Triangle::new(Point::new(5, 10), Point::new(55, 20), Point::new(30, 40));

        b.iter(|| object.contains(Point::new(50, 13)))
    });
}

criterion_group!(
    contains,
    triangle_contains_inside,
    triangle_contains_outside
);
criterion_main!(contains);
