use criterion::*;
use embedded_graphics::{
    pixelcolor::Gray8,
    prelude::*,
    primitives::{Circle, Line, Rectangle, Triangle},
};

fn filled_circle(c: &mut Criterion) {
    c.bench_function("filled circle", |b| {
        let object: Circle<Gray8> = Circle::new(Point::new(100, 100), 100)
            .fill(Some(Gray8::new(1)))
            .stroke(Some(Gray8::new(10)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn filled_rect(c: &mut Criterion) {
    c.bench_function("filled rectangle", |b| {
        let object: Rectangle<Gray8> = Rectangle::new(Point::new(100, 100), Point::new(200, 200))
            .fill(Some(Gray8::new(1)))
            .stroke(Some(Gray8::new(10)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn empty_rect(c: &mut Criterion) {
    c.bench_function("unfilled rectangle", |b| {
        let object: Rectangle<Gray8> =
            Rectangle::new(Point::new(100, 100), Point::new(200, 200)).stroke(Some(Gray8::new(10)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn line(c: &mut Criterion) {
    c.bench_function("line", |b| {
        let object: Line<Gray8> =
            Line::new(Point::new(100, 100), Point::new(200, 200)).stroke(Some(Gray8::new(10)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn triangle(c: &mut Criterion) {
    c.bench_function("triangle", |b| {
        let object: Triangle<Gray8> =
            Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(5, 20));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn filled_triangle(c: &mut Criterion) {
    c.bench_function("filled_triangle", |b| {
        let object: Triangle<Gray8> =
            Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(5, 20))
                .fill(Some(Gray8::new(1)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

criterion_group!(
    primitives,
    filled_circle,
    filled_rect,
    empty_rect,
    line,
    triangle,
    filled_triangle
);
criterion_main!(primitives);
