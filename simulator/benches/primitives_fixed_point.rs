use criterion::*;
use embedded_graphics::{
    drawable::Pixel,
    geometry::{AngleUnit, Point},
    pixelcolor::Gray8,
    primitives::*,
    style::PrimitiveStyle,
};

fn arc(c: &mut Criterion) {
    c.bench_function("arc", |b| {
        let object = &Arc::new(Point::new(100, 100), 100, -30.0.deg(), 150.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 1));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn sector(c: &mut Criterion) {
    c.bench_function("sector", |b| {
        let object = &Sector::new(Point::new(100, 100), 100, -30.0.deg(), 150.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(Gray8::new(1), 1));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

fn filled_sector(c: &mut Criterion) {
    c.bench_function("filled_sector", |b| {
        let object = &Sector::new(Point::new(100, 100), 100, -30.0.deg(), 150.0.deg())
            .into_styled(PrimitiveStyle::with_fill(Gray8::new(1)));

        b.iter(|| object.into_iter().collect::<Vec<Pixel<Gray8>>>())
    });
}

criterion_group!(primitives_fixed_point, arc, sector, filled_sector);
criterion_main!(primitives_fixed_point);
