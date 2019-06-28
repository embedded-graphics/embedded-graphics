use criterion::*;
use embedded_graphics::{
    pixelcolor::Y8,
    prelude::*,
    primitives::{Circle, Line, Rectangle, Triangle},
};

fn filled_circle(c: &mut Criterion) {
    c.bench_function("filled circle", |b| {
        let object: Circle<Y8> = Circle::new(Coord::new(100, 100), 100)
            .fill(Some(Y8::new(1)))
            .stroke(Some(Y8::new(10)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Y8>>>())
    });
}

fn filled_rect(c: &mut Criterion) {
    c.bench_function("filled rectangle", |b| {
        let object: Rectangle<Y8> = Rectangle::new(Coord::new(100, 100), Coord::new(200, 200))
            .fill(Some(Y8::new(1)))
            .stroke(Some(Y8::new(10)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Y8>>>())
    });
}

fn empty_rect(c: &mut Criterion) {
    c.bench_function("unfilled rectangle", |b| {
        let object: Rectangle<Y8> =
            Rectangle::new(Coord::new(100, 100), Coord::new(200, 200)).stroke(Some(Y8::new(10)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Y8>>>())
    });
}

fn line(c: &mut Criterion) {
    c.bench_function("line", |b| {
        let object: Line<Y8> =
            Line::new(Coord::new(100, 100), Coord::new(200, 200)).stroke(Some(Y8::new(10)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Y8>>>())
    });
}

fn triangle(c: &mut Criterion) {
    c.bench_function("triangle", |b| {
        let object: Triangle<Y8> =
            Triangle::new(Coord::new(5, 10), Coord::new(15, 20), Coord::new(5, 20));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Y8>>>())
    });
}

fn filled_triangle(c: &mut Criterion) {
    c.bench_function("filled_triangle", |b| {
        let object: Triangle<Y8> =
            Triangle::new(Coord::new(5, 10), Coord::new(15, 20), Coord::new(5, 20))
                .fill(Some(Y8::new(1)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Y8>>>())
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
