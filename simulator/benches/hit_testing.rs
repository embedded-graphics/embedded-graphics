use criterion::*;
use embedded_graphics::{geometry::Point, primitives::*};

fn triangle_contains_point(c: &mut Criterion) {
    c.bench_function("triangle contains point", |b| {
        let object = &Triangle::new(Point::new(10, 10), Point::new(70, 10), Point::new(40, 70));

        let point_inside_near_top = Point::new(30, 15);
        let point_inside_near_bottom = Point::new(40, 50);
        let point_outside = Point::new(2, 2);

        b.iter(|| {
            object.contains(point_inside_near_top);
            object.contains(point_inside_near_bottom);
            object.contains(point_outside);
        })
    });
}

criterion_group!(hit_testing, triangle_contains_point);
criterion_main!(hit_testing);
