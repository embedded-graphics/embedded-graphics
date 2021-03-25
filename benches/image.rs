use criterion::*;
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};

mod common;

use common::Framebuffer;

fn image_1bpp(c: &mut Criterion) {
    c.bench_function("image 4x4px", |b| {
        let bytes = include_bytes!("../assets/patch_1bpp.raw");

        let image: ImageRaw<BinaryColor> = ImageRaw::new(bytes, 4);
        let object = Image::new(&image, Point::zero());

        let mut framebuffer = Framebuffer::new();
        b.iter(|| object.draw(&mut framebuffer))
    });
}

criterion_group!(images, image_1bpp);
criterion_main!(images);
