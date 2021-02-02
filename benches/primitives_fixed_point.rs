use criterion::*;
use embedded_graphics::{geometry::AngleUnit, pixelcolor::Gray8, prelude::*, primitives::*};

mod common;

use common::Framebuffer;

fn arc(c: &mut Criterion) {
    c.bench_function("arc", |b| {
        let object = &Arc::new(Point::new(100, 100), 100, -30.0.deg(), 150.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 1));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn sector(c: &mut Criterion) {
    c.bench_function("sector", |b| {
        let object = &Sector::new(Point::new(100, 100), 100, -30.0.deg(), 150.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 1));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn filled_sector(c: &mut Criterion) {
    c.bench_function("filled_sector", |b| {
        let object = &Sector::new(Point::new(100, 100), 100, -30.0.deg(), 150.0.deg())
            .into_styled(PrimitiveStyle::with_fill(Gray8::new(1)));

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

criterion_group!(primitives_fixed_point, arc, sector, filled_sector);
criterion_main!(primitives_fixed_point);
