use criterion::*;
use embedded_graphics::{pixelcolor::Rgb565, Pixel};
use tinybmp::Bmp;

fn image_bmp_4x4(c: &mut Criterion) {
    c.bench_function("image BMP 4x4px", |b| {
        let bytes = include_bytes!("../../tinybmp/tests/chessboard-4px-color-16bit.bmp");

        b.iter(|| {
            Bmp::from_slice(bytes)
                .unwrap()
                .pixels()
                .collect::<Vec<Pixel<Rgb565>>>()
        })
    });
}

criterion_group!(fonts, image_bmp_4x4);
criterion_main!(fonts);
