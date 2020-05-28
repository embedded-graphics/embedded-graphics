use criterion::*;
use embedded_graphics::{drawable, image::IntoPixelIter, pixelcolor::Rgb565};
use tinytga::Tga;

fn embedded_graphics(c: &mut Criterion) {
    let file = include_bytes!("../tests/type2_bl.tga");

    c.bench_function("embedded-graphics", |b| {
        b.iter(|| {
            Tga::from_slice(file)
                .unwrap()
                .pixel_iter()
                .collect::<Vec<drawable::Pixel<Rgb565>>>()
        })
    });
}

criterion_group!(e_g, embedded_graphics);
criterion_main!(e_g);
