use criterion::*;
use embedded_graphics::{
    drawable::Pixel,
    geometry::Point,
    pixelcolor::Gray8,
    primitives::{Circle, Line, Primitive, Rectangle, Triangle},
    style::PrimitiveStyle,
};

fn filled_circle(c: &mut Criterion) {
    c.bench_function("filled circle", |b| {
        let mut style = PrimitiveStyle::default();
        style.fill_color = Some(Gray8::new(1));
        style.stroke_color = Some(Gray8::new(10));

        let object = &Circle::new(Point::new(100, 100), 100).into_styled(style);

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn filled_rect(c: &mut Criterion) {
    c.bench_function("filled rectangle", |b| {
        let mut style = PrimitiveStyle::default();
        style.fill_color = Some(Gray8::new(1));
        style.stroke_color = Some(Gray8::new(10));

        let object = &Rectangle::new(Point::new(100, 100), Point::new(200, 200)).into_styled(style);

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn empty_rect(c: &mut Criterion) {
    c.bench_function("unfilled rectangle", |b| {
        let object = &Rectangle::new(Point::new(100, 100), Point::new(200, 200))
            .into_styled(PrimitiveStyle::stroke(Gray8::new(10), 1));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn line(c: &mut Criterion) {
    c.bench_function("line", |b| {
        let object = &Line::new(Point::new(100, 100), Point::new(200, 200))
            .into_styled(PrimitiveStyle::stroke(Gray8::new(10), 1));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn triangle(c: &mut Criterion) {
    c.bench_function("triangle", |b| {
        let object = &Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(5, 20))
            .into_styled(PrimitiveStyle::stroke(Gray8::new(10), 1));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn filled_triangle(c: &mut Criterion) {
    c.bench_function("filled_triangle", |b| {
        let object = &Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(5, 20))
            .into_styled(PrimitiveStyle::fill(Gray8::new(1)));

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
