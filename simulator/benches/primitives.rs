use criterion::*;
use embedded_graphics::{
    pixelcolor::Gray8,
    prelude::*,
    primitives::*,
    style::{PrimitiveStyle, PrimitiveStyleBuilder},
};

fn filled_circle(c: &mut Criterion) {
    c.bench_function("filled circle", |b| {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Gray8::new(1))
            .stroke_color(Gray8::new(10))
            .stroke_width(1)
            .build();

        let object = &Circle::new(Point::new(100, 100), 100).into_styled(style);

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn filled_rect(c: &mut Criterion) {
    c.bench_function("filled rectangle", |b| {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Gray8::new(1))
            .stroke_color(Gray8::new(10))
            .stroke_width(1)
            .build();

        let object = &Rectangle::new(Point::new(100, 100), Size::new(100, 100)).into_styled(style);

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn empty_rect(c: &mut Criterion) {
    c.bench_function("unfilled rectangle", |b| {
        let object = &Rectangle::new(Point::new(100, 100), Size::new(100, 100))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(10), 1));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn line(c: &mut Criterion) {
    c.bench_function("line", |b| {
        let object = &Line::new(Point::new(100, 100), Point::new(200, 200))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(10), 1));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn thick_line(c: &mut Criterion) {
    c.bench_function("thick line 10px wide", |b| {
        let object = &Line::new(Point::new(100, 100), Point::new(150, 200))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(10), 10));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn thicker_line(c: &mut Criterion) {
    c.bench_function("thick line 50px wide", |b| {
        let object = &Line::new(Point::new(20, 20), Point::new(150, 200))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(10), 50));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn triangle(c: &mut Criterion) {
    c.bench_function("triangle", |b| {
        let object = &Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(5, 20))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(10), 1));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn filled_triangle(c: &mut Criterion) {
    c.bench_function("filled_triangle", |b| {
        let object = &Triangle::new(Point::new(5, 10), Point::new(15, 20), Point::new(5, 20))
            .into_styled(PrimitiveStyle::with_fill(Gray8::new(1)));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn ellipse(c: &mut Criterion) {
    c.bench_function("ellipse", |b| {
        let object = &Ellipse::new(Point::new(10, 10), Size::new(50, 30))
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 1));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn filled_ellipse(c: &mut Criterion) {
    c.bench_function("filled_ellipse", |b| {
        let object = &Ellipse::new(Point::new(10, 10), Size::new(50, 30))
            .into_styled(PrimitiveStyle::with_fill(Gray8::new(1)));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn arc(c: &mut Criterion) {
    c.bench_function("arc", |b| {
        let object = &Arc::new(Point::new(100, 100), 100, -30.0.deg(), 150.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 1));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn sector(c: &mut Criterion) {
    c.bench_function("sector", |b| {
        let object = &Sector::new(Point::new(100, 100), 100, -30.0.deg(), 150.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 1));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn filled_sector(c: &mut Criterion) {
    c.bench_function("filled_sector", |b| {
        let object = &Sector::new(Point::new(100, 100), 100, -30.0.deg(), 150.0.deg())
            .into_styled(PrimitiveStyle::with_fill(Gray8::new(1)));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn polyline(c: &mut Criterion) {
    c.bench_function("polyline", |b| {
        let points = [
            Point::new(5, 10),
            Point::new(15, 20),
            Point::new(5, 20),
            Point::new(30, 50),
            Point::new(100, 100),
        ];

        let object =
            &Polyline::new(&points).into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 1));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn thick_polyline(c: &mut Criterion) {
    c.bench_function("thick polyline", |b| {
        let points = [
            Point::new(5, 10),
            Point::new(15, 20),
            Point::new(5, 20),
            Point::new(30, 50),
            Point::new(100, 100),
        ];

        let object =
            &Polyline::new(&points).into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 10));

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn rounded_rectangle(c: &mut Criterion) {
    c.bench_function("rounded_rectangle", |b| {
        let object = &RoundedRectangle::new(
            Rectangle::new(Point::zero(), Size::new(50, 40)),
            CornerRadii::new(Size::new(10, 12)),
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(5)
                .fill_color(Gray8::new(10))
                .stroke_color(Gray8::new(60))
                .build(),
        );

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn rounded_rectangle_corners(c: &mut Criterion) {
    c.bench_function("rounded_rectangle_corners", |b| {
        let object = &RoundedRectangle::new(
            Rectangle::new(Point::zero(), Size::new(50, 40)),
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

        b.iter(|| object.into_pixels().collect::<Vec<Pixel<Gray8>>>())
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
    filled_sector
);
criterion_main!(primitives);
