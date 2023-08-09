use criterion::*;
use embedded_graphics::{
    common::GetPixel,
    image::{Image, ImageRaw},
    pixelcolor::{raw::order::Msb0, BinaryColor},
    prelude::*,
};

mod common;

use common::Framebuffer;

fn image_1bpp(c: &mut Criterion) {
    c.bench_function("image 4x4px", |b| {
        let bytes = include_bytes!("../assets/patch_1bpp.raw");

        let image = ImageRaw::<BinaryColor, Msb0>::new(bytes, Size::new(4, 4)).unwrap();
        let object = Image::new(&image, Point::zero());

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

fn image_get_pixel_1bpp_msb0(c: &mut Criterion) {
    c.bench_function("image raw get pixel 1bpp msb0", |b| {
        let bytes = include_bytes!("../assets/patch_1bpp.raw");

        let image = ImageRaw::<BinaryColor, Msb0>::new(bytes, Size::new(4, 4)).unwrap();

        b.iter(|| {
            image.pixel(Point::new(3, 3));
            image.pixel(Point::new(0, 1));
        })
    });
}

fn image_get_pixel_1bpp_lsb0(c: &mut Criterion) {
    c.bench_function("image raw get pixel 1bpp msb0", |b| {
        let bytes = include_bytes!("../assets/patch_1bpp.raw");

        let image = ImageRaw::<BinaryColor, Msb0>::new(bytes, Size::new(4, 4)).unwrap();

        b.iter(|| {
            image.pixel(Point::new(3, 3));
            image.pixel(Point::new(0, 1));
        })
    });
}

criterion_group!(
    images,
    image_1bpp,
    image_get_pixel_1bpp_msb0,
    image_get_pixel_1bpp_lsb0
);
criterion_main!(images);
