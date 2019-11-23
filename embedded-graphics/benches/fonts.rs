use criterion::*;
use embedded_graphics::{
    fonts::{Text, Font12x16, Font6x8},
    geometry::Point,
    pixelcolor::Gray8,
    prelude::*,
    style::TextStyle,
};

fn font_6x8(c: &mut Criterion) {
    c.bench_function("font 6x8 Hello world!", |b| {
        let object = Text::new("Hello world!", Point::zero())
            .into_styled(TextStyle::with_text_color(Font6x8, Gray8::new(10)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn font_12x16(c: &mut Criterion) {
    c.bench_function("font 12x16 Hello world!", |b| {
        let object = Text::new("Hello world!", Point::zero())
            .into_styled(TextStyle::with_text_color(Font12x16, Gray8::new(10)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

criterion_group!(fonts, font_6x8, font_12x16);
criterion_main!(fonts);
