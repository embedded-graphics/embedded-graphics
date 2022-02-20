use criterion::*;
use embedded_graphics::{
    pixelcolor::{Gray8, Rgb555, Rgb565, Rgb888},
    prelude::*,
};

fn impl_bench<CFrom, CTo>(c: &mut Criterion, name: &str)
where
    CFrom: PixelColor + Default + Into<CTo>,
    CTo: PixelColor + Default,
{
    c.bench_function(name, |b| {
        let input = &[CFrom::default(); 320 * 240];
        let mut output = [CTo::default(); 320 * 240];

        b.iter(|| {
            for (i, o) in black_box(input).iter().copied().zip(output.iter_mut()) {
                *o = i.into();
            }
        })
    });
}

fn color_conversions(c: &mut Criterion) {
    impl_bench::<Rgb555, Rgb565>(c, "Rgb555 to Rgb565");
    impl_bench::<Rgb565, Rgb555>(c, "Rgb565 to Rgb555");
    impl_bench::<Rgb565, Rgb888>(c, "Rgb565 to Rgb888");
    impl_bench::<Rgb888, Rgb565>(c, "Rgb888 to Rgb565");
    impl_bench::<Gray8, Rgb888>(c, "Gray8 to Rgb888");
    impl_bench::<Rgb888, Gray8>(c, "Rgb888 to Gray8");
}

criterion_group!(benches, color_conversions);
criterion_main!(benches);
