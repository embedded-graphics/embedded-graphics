use criterion::*;
use embedded_graphics::{
    framebuffer::{buffer_size, Framebuffer},
    geometry::Point,
    image::GetPixel,
    pixelcolor::{raw::LittleEndian, BinaryColor, Rgb565},
    prelude::{DrawTarget, Size, WebColors},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
};

fn framebuffer_set_1bpp(c: &mut Criterion) {
    c.bench_function("framebuffer set pixel 1bpp", |b| {
        let mut fb = Framebuffer::<
            BinaryColor,
            _,
            LittleEndian,
            320,
            240,
            { buffer_size::<BinaryColor>(320, 240) },
        >::new();

        b.iter(|| {
            fb.set_pixel(Point::new(1, 1), BinaryColor::On);
            fb.set_pixel(Point::new(300, 200), BinaryColor::On);
        })
    });
}

fn framebuffer_get_1bpp(c: &mut Criterion) {
    c.bench_function("framebuffer get pixel 1bpp", |b| {
        let fb = Framebuffer::<
            BinaryColor,
            _,
            LittleEndian,
            320,
            240,
            { buffer_size::<BinaryColor>(320, 240) },
        >::new();

        b.iter(|| {
            fb.pixel(Point::new(1, 1));
            fb.pixel(Point::new(300, 200));
        })
    });
}

fn framebuffer_1bpp_draw_iter(c: &mut Criterion) {
    c.bench_function("framebuffer 1bpp draw iter", |b| {
        let mut fb = Framebuffer::<
            BinaryColor,
            _,
            LittleEndian,
            320,
            240,
            { buffer_size::<BinaryColor>(320, 240) },
        >::new();

        b.iter(|| {
            let rect = Rectangle::new(Point::new(20, 30), Size::new(40, 50))
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .pixels();

            fb.draw_iter(rect).unwrap();
        })
    });
}

fn framebuffer_set_rgb565(c: &mut Criterion) {
    c.bench_function("framebuffer set pixel rgb565", |b| {
        let mut fb = Framebuffer::<
            Rgb565,
            _,
            LittleEndian,
            320,
            240,
            { buffer_size::<Rgb565>(320, 240) },
        >::new();

        b.iter(|| {
            fb.set_pixel(Point::new(1, 1), Rgb565::CSS_DARK_SALMON);
            fb.set_pixel(Point::new(300, 200), Rgb565::CSS_TEAL);
        })
    });
}

fn framebuffer_get_rgb565(c: &mut Criterion) {
    c.bench_function("framebuffer get pixel rgb565", |b| {
        let fb = Framebuffer::<
            Rgb565,
            _,
            LittleEndian,
            320,
            240,
            { buffer_size::<Rgb565>(320, 240) },
        >::new();

        b.iter(|| {
            fb.pixel(Point::new(1, 1));
            fb.pixel(Point::new(300, 200));
        })
    });
}

criterion_group!(
    framebuffer,
    framebuffer_set_1bpp,
    framebuffer_get_1bpp,
    framebuffer_set_rgb565,
    framebuffer_get_rgb565,
    framebuffer_1bpp_draw_iter
);
criterion_main!(framebuffer);
