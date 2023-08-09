use criterion::*;
use embedded_graphics::{
    common::{buffer_size, GetPixel, Horizontal, SetPixel, Vertical},
    framebuffer::ArrayFramebuffer,
    geometry::Point,
    pixelcolor::{
        raw::order::{LittleEndian, Lsb0, Msb0},
        BinaryColor, Rgb565,
    },
    prelude::*,
    primitives::{Primitive, PrimitiveStyle, Rectangle},
};

const SIZE: Size = Size::new(320, 240);

fn framebuffer_set_1bpp(c: &mut Criterion) {
    c.bench_function("framebuffer set pixel 1bpp", |b| {
        let mut fb = ArrayFramebuffer::<
            { buffer_size::<BinaryColor, Horizontal>(SIZE) },
            BinaryColor,
            Msb0,
            Horizontal,
        >::new(SIZE);

        b.iter(|| {
            fb.set_pixel(Point::new(1, 1), BinaryColor::On);
            fb.set_pixel(Point::new(300, 200), BinaryColor::On);
        })
    });
}

fn framebuffer_set_1bpp_lsb0(c: &mut Criterion) {
    c.bench_function("framebuffer set pixel 1bpp lsb0", |b| {
        let mut fb = ArrayFramebuffer::<
            { buffer_size::<BinaryColor, Horizontal>(SIZE) },
            BinaryColor,
            Lsb0,
            Horizontal,
        >::new(SIZE);

        b.iter(|| {
            fb.set_pixel(Point::new(1, 1), BinaryColor::On);
            fb.set_pixel(Point::new(300, 200), BinaryColor::On);
        })
    });
}

fn framebuffer_set_1bpp_lsb0_vertical(c: &mut Criterion) {
    c.bench_function("framebuffer set pixel 1bpp lsb0 vertical", |b| {
        let mut fb = ArrayFramebuffer::<
            { buffer_size::<BinaryColor, Horizontal>(SIZE) },
            BinaryColor,
            Lsb0,
            Vertical,
        >::new(SIZE);

        b.iter(|| {
            fb.set_pixel(Point::new(1, 1), BinaryColor::On);
            fb.set_pixel(Point::new(300, 200), BinaryColor::On);
        })
    });
}

fn framebuffer_get_1bpp(c: &mut Criterion) {
    c.bench_function("framebuffer get pixel 1bpp", |b| {
        let fb = ArrayFramebuffer::<
            { buffer_size::<BinaryColor, Horizontal>(SIZE) },
            BinaryColor,
            Msb0,
            Horizontal,
        >::new(SIZE);

        b.iter(|| {
            fb.pixel(Point::new(1, 1));
            fb.pixel(Point::new(300, 200));
        })
    });
}

fn framebuffer_1bpp_draw_iter(c: &mut Criterion) {
    c.bench_function("framebuffer 1bpp draw iter", |b| {
        let mut fb = ArrayFramebuffer::<
            { buffer_size::<BinaryColor, Horizontal>(SIZE) },
            BinaryColor,
            Lsb0,
            Horizontal,
        >::new(SIZE);

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
        let mut fb = ArrayFramebuffer::<
            { buffer_size::<Rgb565, Horizontal>(SIZE) },
            Rgb565,
            LittleEndian,
            Horizontal,
        >::new(SIZE);

        b.iter(|| {
            fb.set_pixel(Point::new(1, 1), Rgb565::CSS_DARK_SALMON);
            fb.set_pixel(Point::new(300, 200), Rgb565::CSS_TEAL);
        })
    });
}

fn framebuffer_get_rgb565(c: &mut Criterion) {
    c.bench_function("framebuffer get pixel rgb565", |b| {
        let fb = ArrayFramebuffer::<
            { buffer_size::<Rgb565, Horizontal>(SIZE) },
            Rgb565,
            LittleEndian,
            Horizontal,
        >::new(SIZE);

        b.iter(|| {
            fb.pixel(Point::new(1, 1));
            fb.pixel(Point::new(300, 200));
        })
    });
}

criterion_group!(
    framebuffer,
    framebuffer_set_1bpp,
    framebuffer_set_1bpp_lsb0,
    framebuffer_get_1bpp,
    framebuffer_set_rgb565,
    framebuffer_get_rgb565,
    framebuffer_1bpp_draw_iter,
    framebuffer_set_1bpp_lsb0_vertical,
);
criterion_main!(framebuffer);
