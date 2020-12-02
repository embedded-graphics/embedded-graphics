use criterion::*;
use embedded_graphics::{
    fonts::{Font12x16, Font6x8, Text},
    geometry::Point,
    pixelcolor::Gray8,
    prelude::*,
    style::{MonoTextStyle, MonoTextStyleBuilder},
};

mod common;

use common::Framebuffer;

const ONE_LINE: Text = Text::new("Hello world!", Point::new_equal(20));
const THREE_LINES: Text = Text::new("line 1\nl2\nThis is line 3", Point::new_equal(20));

fn font_6x8(c: &mut Criterion) {
    let mut group = c.benchmark_group("font 6x8");

    let style = MonoTextStyle::new(Font6x8, Gray8::WHITE);
    let style_with_bg = MonoTextStyleBuilder::new(Font6x8)
        .text_color(Gray8::WHITE)
        .background_color(Gray8::BLACK)
        .build();

    group.bench_function("one line", |b| {
        let object = ONE_LINE.into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("one line with background", |b| {
        let object = ONE_LINE.into_styled(style_with_bg);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("three lines", |b| {
        let object = THREE_LINES.into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("three lines with background)", |b| {
        let object = THREE_LINES.into_styled(style_with_bg);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.finish();
}

fn font_12x16(c: &mut Criterion) {
    let mut group = c.benchmark_group("font 12x16");

    let style = MonoTextStyle::new(Font12x16, Gray8::WHITE);
    let style_with_bg = MonoTextStyleBuilder::new(Font12x16)
        .text_color(Gray8::WHITE)
        .background_color(Gray8::BLACK)
        .build();

    group.bench_function("one line", |b| {
        let object = ONE_LINE.into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("one line with background", |b| {
        let object = ONE_LINE.into_styled(style_with_bg);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("three lines", |b| {
        let object = THREE_LINES.into_styled(style);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.bench_function("three lines with background)", |b| {
        let object = THREE_LINES.into_styled(style_with_bg);

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });

    group.finish();
}

criterion_group!(fonts, font_6x8, font_12x16);
criterion_main!(fonts);
