use criterion::*;
use embedded_graphics::{
    fonts::{Font12x16, Font6x8, Text},
    geometry::Point,
    pixelcolor::Gray8,
    prelude::*,
    style::TextStyle,
};

mod common;

use common::Framebuffer;

fn font_6x8(c: &mut Criterion) {
    c.bench_function("font 6x8 Hello world!", |b| {
        let object = Text::new("Hello world!", Point::zero())
            .into_styled(TextStyle::new(Font6x8, Gray8::new(10)));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn font_12x16(c: &mut Criterion) {
    c.bench_function("font 12x16 Hello world!", |b| {
        let object = Text::new("Hello world!", Point::zero())
            .into_styled(TextStyle::new(Font12x16, Gray8::new(10)));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

criterion_group!(fonts, font_6x8, font_12x16);
criterion_main!(fonts);
