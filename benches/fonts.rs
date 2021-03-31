use criterion::*;
use embedded_graphics::{
    geometry::Point,
    mono_font::{
        ascii::{FONT_10X20, FONT_6X9},
        MonoTextStyle, MonoTextStyleBuilder,
    },
    pixelcolor::Gray8,
    prelude::*,
    text::Text,
};

mod common;

use common::Framebuffer;

const ONE_LINE: Text = Text::new("Hello world!", Point::new_equal(20));
const THREE_LINES: Text = Text::new("line 1\nl2\nThis is line 3", Point::new_equal(20));

fn font_6x9(c: &mut Criterion) {
    let mut group = c.benchmark_group("font 6x9");

    let style = MonoTextStyle::new(&FONT_6X9, Gray8::WHITE);
    let style_with_bg = MonoTextStyleBuilder::new()
        .font(&FONT_6X9)
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

fn font_10x20(c: &mut Criterion) {
    let mut group = c.benchmark_group("font 10x20");

    let style = MonoTextStyle::new(&FONT_10X20, Gray8::WHITE);
    let style_with_bg = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
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

criterion_group!(fonts, font_6x9, font_10x20);
criterion_main!(fonts);
