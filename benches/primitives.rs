use criterion::*;
use embedded_graphics::{geometry::AnchorPoint, pixelcolor::Gray8, prelude::*, primitives::*};

mod common;

use common::Framebuffer;

const BOUNDING_BOX: Rectangle = Rectangle::new(Point::new_equal(32), Size::new_equal(192));

fn rectangle(c: &mut Criterion) {
    closed_shape_benches(c, "rectangle", || BOUNDING_BOX);
}

fn rounded_rectangle(c: &mut Criterion) {
    closed_shape_benches(c, "rounded rectangle", || {
        RoundedRectangle::new(BOUNDING_BOX, CornerRadii::new(Size::new(10, 12)))
    });
}

fn rounded_rectangle_corners(c: &mut Criterion) {
    closed_shape_benches(c, "rounded rectangle corners", || {
        RoundedRectangle::new(
            BOUNDING_BOX,
            CornerRadii {
                top_left: Size::new(10, 12),
                top_right: Size::new(14, 16),
                bottom_right: Size::new(18, 20),
                bottom_left: Size::new(22, 24),
            },
        )
    });
}

fn triangle(c: &mut Criterion) {
    closed_shape_benches(c, "triangle", || {
        Triangle::new(
            BOUNDING_BOX.anchor_point(AnchorPoint::BottomLeft),
            BOUNDING_BOX.anchor_point(AnchorPoint::TopCenter),
            BOUNDING_BOX.anchor_point(AnchorPoint::BottomRight),
        )
    });
}

fn circle(c: &mut Criterion) {
    closed_shape_benches(c, "circle", || {
        Circle::new(BOUNDING_BOX.top_left, BOUNDING_BOX.size.width)
    });
}

fn ellipse(c: &mut Criterion) {
    closed_shape_benches(c, "ellipse", || {
        Ellipse::with_center(BOUNDING_BOX.center(), BOUNDING_BOX.size - Size::new(0, 20))
    });
}

fn sector_150(c: &mut Criterion) {
    closed_shape_benches(c, "sector 150°", || {
        Sector::with_center(
            BOUNDING_BOX.center(),
            BOUNDING_BOX.size.width,
            -30.0.deg(),
            150.0.deg(),
        )
    });
}

fn sector_360(c: &mut Criterion) {
    closed_shape_benches(c, "sector 360°", || {
        Sector::with_center(
            BOUNDING_BOX.center(),
            BOUNDING_BOX.size.width,
            -30.0.deg(),
            150.0.deg(),
        )
    });
}

fn line(c: &mut Criterion) {
    open_shape_benches(c, "line", || {
        Line::new(
            BOUNDING_BOX.anchor_point(AnchorPoint::TopLeft),
            // move point up a bit, because non 45° lines might be slower
            BOUNDING_BOX.anchor_point(AnchorPoint::BottomRight) - Point::new(0, 20),
        )
    });
}

fn polyline(c: &mut Criterion) {
    let points = [
        BOUNDING_BOX.anchor_point(AnchorPoint::BottomLeft),
        BOUNDING_BOX.anchor_point(AnchorPoint::TopCenter) - Point::new(20, 0),
        BOUNDING_BOX.anchor_point(AnchorPoint::BottomCenter),
        BOUNDING_BOX.anchor_point(AnchorPoint::TopRight),
        BOUNDING_BOX.anchor_point(AnchorPoint::BottomRight),
    ];

    open_shape_benches(c, "polyline", || Polyline::new(&points));
}

fn arc_150(c: &mut Criterion) {
    open_shape_benches(c, "arc 150°", || {
        Arc::with_center(
            BOUNDING_BOX.center(),
            BOUNDING_BOX.size.width,
            -30.0.deg(),
            150.0.deg(),
        )
    });
}

fn arc_360(c: &mut Criterion) {
    open_shape_benches(c, "arc 360°", || {
        Arc::with_center(
            BOUNDING_BOX.center(),
            BOUNDING_BOX.size.width,
            0.0.deg(),
            360.0.deg(),
        )
    });
}

criterion_group!(
    primitives,
    rectangle,
    rounded_rectangle,
    rounded_rectangle_corners,
    triangle,
    circle,
    ellipse,
    line,
    polyline,
    sector_150,
    sector_360,
    arc_150,
    arc_360,
);
criterion_main!(primitives);

fn closed_shape_benches<P>(c: &mut Criterion, name: &str, build: impl Fn() -> P)
where
    P: Primitive,
    Styled<P, PrimitiveStyle<Gray8>>: Drawable<Color = Gray8>,
{
    let mut group = c.benchmark_group(name);

    group.bench_function("fill", |b| {
        let style = PrimitiveStyle::with_fill(Gray8::WHITE);
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 1px", |b| {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Gray8::BLACK)
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 10px", |b| {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Gray8::BLACK)
            .stroke_width(10)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 1px and fill", |b| {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Gray8::WHITE)
            .stroke_color(Gray8::BLACK)
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 10px and fill", |b| {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Gray8::WHITE)
            .stroke_color(Gray8::BLACK)
            .stroke_width(10)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.finish()
}

fn open_shape_benches<P>(c: &mut Criterion, name: &str, build: impl Fn() -> P)
where
    P: Primitive,
    Styled<P, PrimitiveStyle<Gray8>>: Drawable<Color = Gray8>,
{
    let mut group = c.benchmark_group(name);

    group.bench_function("stroke 1px", |b| {
        let style = PrimitiveStyle::with_stroke(Gray8::WHITE, 1);
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 10px", |b| {
        let style = PrimitiveStyle::with_stroke(Gray8::WHITE, 10);
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("stroke 50px", |b| {
        let style = PrimitiveStyle::with_stroke(Gray8::WHITE, 50);
        let object = build().into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.finish()
}
