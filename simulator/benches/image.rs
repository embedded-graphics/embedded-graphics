use criterion::*;
use embedded_graphics::{
    drawable::Pixel,
    image::{ImageBmp, ImageFile},
    pixelcolor::Rgb565,
};

fn image_bmp_4x4(c: &mut Criterion) {
    c.bench_function("image BMP 4x4px", |b| {
        let bytes = include_bytes!("../tests/chessboard-4px-color-16bit.bmp");

        b.iter(|| {
            ImageBmp::new(bytes)
                .unwrap()
                .into_iter()
                .collect::<Vec<Pixel<Rgb565>>>()
        })
    });
}

criterion_group!(fonts, image_bmp_4x4);
criterion_main!(fonts);
