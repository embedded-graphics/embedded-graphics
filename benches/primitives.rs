use criterion::*;
use embedded_graphics::{pixelcolor::Gray8, prelude::*, primitives::*};

mod common;

use common::Framebuffer;

fn filled_circle(c: &mut Criterion) {
    c.bench_function("filled circle", |b| {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Gray8::new(1))
            .stroke_color(Gray8::new(10))
            .stroke_width(1)
            .build();

        let object = Circle::new(Point::new(100, 100), 100).into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn filled_rect(c: &mut Criterion) {
    c.bench_function("filled rectangle", |b| {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Gray8::new(1))
            .stroke_color(Gray8::new(10))
            .stroke_width(1)
            .build();

        let object = Rectangle::new(Point::new(100, 100), Size::new(100, 100)).into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn empty_rect(c: &mut Criterion) {
    c.bench_function("unfilled rectangle", |b| {
        let object = Rectangle::new(Point::new(100, 100), Size::new(100, 100))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(10), 1));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn line(c: &mut Criterion) {
    c.bench_function("line", |b| {
        let object = Line::new(Point::new(100, 100), Point::new(200, 200))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(10), 1));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn thick_line(c: &mut Criterion) {
    c.bench_function("thick line 10px wide", |b| {
        let object = Line::new(Point::new(100, 100), Point::new(150, 200))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(10), 10));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn thicker_line(c: &mut Criterion) {
    c.bench_function("thick line 50px wide", |b| {
        let object = Line::new(Point::new(50, 50), Point::new(150, 200))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(10), 50));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn triangle(c: &mut Criterion) {
    c.bench_function("triangle", |b| {
        let object = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(5, 20))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(10), 1));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn filled_triangle(c: &mut Criterion) {
    c.bench_function("filled_triangle", |b| {
        let object = Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(5, 20))
            .into_styled(PrimitiveStyle::with_fill(Gray8::new(1)));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn ellipse(c: &mut Criterion) {
    c.bench_function("ellipse", |b| {
        let object = Ellipse::new(Point::new(10, 10), Size::new(50, 30))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 1));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn filled_ellipse(c: &mut Criterion) {
    c.bench_function("filled_ellipse", |b| {
        let object = Ellipse::new(Point::new(10, 10), Size::new(50, 30))
            .into_styled(PrimitiveStyle::with_fill(Gray8::new(1)));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn arc(c: &mut Criterion) {
    c.bench_function("arc", |b| {
        let object = Arc::new(Point::new(100, 100), 100, -30.0.deg(), 150.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 1));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn sector(c: &mut Criterion) {
    let mut group = c.benchmark_group("sector");

    let sector = Sector::with_center(Point::new_equal(32), 30, -30.0.deg(), 150.0.deg());

    group.bench_function("1px stroke", |b| {
        let style = PrimitiveStyle::with_stroke(Gray8::WHITE, 1);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| sector.into_styled(style).draw(&mut framebuffer))
    });

    group.bench_function("10px stroke", |b| {
        let style = PrimitiveStyle::with_stroke(Gray8::WHITE, 10);

        // Reduce sector radius by half the stoke width to make the bounding box
        // equal to the other benches.
        let sector = sector.offset(-(style.stroke_width as i32 / 2));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| sector.into_styled(style).draw(&mut framebuffer))
    });

    group.bench_function("1px stroke and fill", |b| {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Gray8::WHITE)
            .stroke_width(1)
            .fill_color(Gray8::new(128))
            .build();

        let mut framebuffer = Framebuffer::new();
        b.iter(|| sector.into_styled(style).draw(&mut framebuffer))
    });

    group.bench_function("10px stroke and fill", |b| {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Gray8::WHITE)
            .stroke_width(10)
            .fill_color(Gray8::new(128))
            .build();

        // Reduce sector radius by half the stoke width to make the bounding box
        // equal to the other benches.
        let sector = sector.offset(-(style.stroke_width as i32 / 2));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| sector.into_styled(style).draw(&mut framebuffer))
    });

    group.bench_function("fill", |b| {
        let style = PrimitiveStyle::with_fill(Gray8::WHITE);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| sector.into_styled(style).draw(&mut framebuffer))
    });

    group.finish();
}

fn sector_360(c: &mut Criterion) {
    let mut group = c.benchmark_group("360° sector");

    let sector = Sector::with_center(Point::new_equal(32), 30, 0.0.deg(), 360.0.deg());

    group.bench_function("1px stroke", |b| {
        let style = PrimitiveStyle::with_stroke(Gray8::WHITE, 1);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| sector.into_styled(style).draw(&mut framebuffer))
    });

    group.bench_function("10px stroke", |b| {
        let style = PrimitiveStyle::with_stroke(Gray8::WHITE, 10);

        // Reduce sector radius by half the stoke width to make the bounding box
        // equal to the other benches.
        let sector = sector.offset(-(style.stroke_width as i32 / 2));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| sector.into_styled(style).draw(&mut framebuffer))
    });

    group.bench_function("1px stroke and fill", |b| {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Gray8::WHITE)
            .stroke_width(1)
            .fill_color(Gray8::new(128))
            .build();

        let mut framebuffer = Framebuffer::new();
        b.iter(|| sector.into_styled(style).draw(&mut framebuffer))
    });

    group.bench_function("10px stroke and fill", |b| {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Gray8::WHITE)
            .stroke_width(10)
            .fill_color(Gray8::new(128))
            .build();

        // Reduce sector radius by half the stoke width to make the bounding box
        // equal to the other benches.
        let sector = sector.offset(-(style.stroke_width as i32 / 2));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| sector.into_styled(style).draw(&mut framebuffer))
    });

    group.bench_function("fill", |b| {
        let style = PrimitiveStyle::with_fill(Gray8::WHITE);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| sector.into_styled(style).draw(&mut framebuffer))
    });

    group.finish();
}

fn polyline(c: &mut Criterion) {
    c.bench_function("polyline", |b| {
        let points = [
            Point::new(105, 110),
            Point::new(115, 120),
            Point::new(105, 120),
            Point::new(130, 150),
            Point::new(200, 200),
        ];

        let object =
            Polyline::new(&points).into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 1));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn thick_polyline(c: &mut Criterion) {
    c.bench_function("thick polyline", |b| {
        let points = [
            Point::new(105, 110),
            Point::new(115, 120),
            Point::new(105, 120),
            Point::new(130, 150),
            Point::new(200, 200),
        ];

        let object =
            Polyline::new(&points).into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 10));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn rounded_rectangle(c: &mut Criterion) {
    c.bench_function("rounded_rectangle", |b| {
        let object = RoundedRectangle::new(
            Rectangle::new(Point::new(10, 10), Size::new(50, 40)),
            CornerRadii::new(Size::new(10, 12)),
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(5)
                .fill_color(Gray8::new(10))
                .stroke_color(Gray8::new(60))
                .build(),
        );

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn rounded_rectangle_corners(c: &mut Criterion) {
    c.bench_function("rounded_rectangle_corners", |b| {
        let object = &RoundedRectangle::new(
            Rectangle::new(Point::new(10, 10), Size::new(50, 40)),
            CornerRadii {
                top_left: Size::new(10, 12),
                top_right: Size::new(14, 16),
                bottom_right: Size::new(18, 20),
                bottom_left: Size::new(22, 24),
            },
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(5)
                .fill_color(Gray8::new(10))
                .stroke_color(Gray8::new(60))
                .build(),
        );

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

criterion_group!(
    primitives,
    filled_circle,
    filled_rect,
    empty_rect,
    line,
    thick_line,
    thicker_line,
    triangle,
    filled_triangle,
    ellipse,
    filled_ellipse,
    polyline,
    thick_polyline,
    rounded_rectangle,
    rounded_rectangle_corners,
    arc,
    sector,
    sector_360,
);
criterion_main!(primitives);
