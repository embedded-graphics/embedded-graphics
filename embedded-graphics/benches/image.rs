use criterion::*;
use embedded_graphics::{image::ImageBmp, pixelcolor::PixelColorU16, prelude::*};

fn image_bmp_4x4(c: &mut Criterion) {
    c.bench_function("image BMP 4x4px", |b| {
        let bytes = include_bytes!("../tests/chessboard-4px-colour-16bit.bmp");

        b.iter(|| {
            ImageBmp::new(bytes)
                .unwrap()
                .into_iter()
                .collect::<Vec<Pixel<PixelColorU16>>>()
        })
    });
}

criterion_group!(fonts, image_bmp_4x4);
criterion_main!(fonts);
