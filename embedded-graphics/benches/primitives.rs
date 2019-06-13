use criterion::*;
use embedded_graphics::{
    pixelcolor::PixelColorU8,
    prelude::*,
    primitives::{Circle, Line, Rect, Triangle},
};

fn filled_circle(c: &mut Criterion) {
    c.bench_function("filled circle", |b| {
        let object: Circle<PixelColorU8> = Circle::new(Coord::new(100, 100), 100)
            .fill(Some(1u8.into()))
            .stroke(Some(10u8.into()));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<PixelColorU8>>>())
    });
}

fn filled_rect(c: &mut Criterion) {
    c.bench_function("filled rectangle", |b| {
        let object: Rect<PixelColorU8> = Rect::new(Coord::new(100, 100), Coord::new(200, 200))
            .fill(Some(1u8.into()))
            .stroke(Some(10u8.into()));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<PixelColorU8>>>())
    });
}

fn empty_rect(c: &mut Criterion) {
    c.bench_function("unfilled rectangle", |b| {
        let object: Rect<PixelColorU8> =
            Rect::new(Coord::new(100, 100), Coord::new(200, 200)).stroke(Some(10u8.into()));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<PixelColorU8>>>())
    });
}

fn line(c: &mut Criterion) {
    c.bench_function("line", |b| {
        let object: Line<PixelColorU8> =
            Line::new(Coord::new(100, 100), Coord::new(200, 200)).stroke(Some(10u8.into()));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<PixelColorU8>>>())
    });
}

fn triangle(c: &mut Criterion) {
    c.bench_function("triangle", |b| {
        let object: Triangle<PixelColorU8> =
            Triangle::new(Coord::new(5, 10), Coord::new(15, 20), Coord::new(5, 20));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<PixelColorU8>>>())
    });
}

fn filled_triangle(c: &mut Criterion) {
    c.bench_function("filled_triangle", |b| {
        let object: Triangle<PixelColorU8> =
            Triangle::new(Coord::new(5, 10), Coord::new(15, 20), Coord::new(5, 20))
                .fill(Some(1u8.into()));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<PixelColorU8>>>())
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
