use criterion::*;
use embedded_graphics::{iterator::raw::RawDataIter, pixelcolor::raw::*};

const TEST_DATA: &[u8] = include_bytes!("../fonts/src/10x20.bdf");

macro_rules! impl_bench {
    ($fn:ident, $type:ident) => {
        fn $fn(c: &mut Criterion) {
            c.bench_function(stringify!($type), |b| {
                let iter = RawDataIter::<$type, LittleEndian>::new(TEST_DATA);

                b.iter(|| iter.clone().collect::<Vec<_>>())
            });

            c.bench_function(concat!(stringify!($type), " step by 20"), |b| {
                let iter = RawDataIter::<$type, LittleEndian>::new(TEST_DATA);

                b.iter(|| iter.clone().step_by(20).collect::<Vec<_>>())
            });
        }
    };
}

impl_bench!(raw_u1, RawU1);
impl_bench!(raw_u2, RawU2);
impl_bench!(raw_u4, RawU4);
impl_bench!(raw_u8, RawU8);
impl_bench!(raw_u16, RawU16);
impl_bench!(raw_u24, RawU24);
impl_bench!(raw_u32, RawU32);

criterion_group!(
    raw_data_iter,
    raw_u1,
    raw_u2,
    raw_u4,
    raw_u8,
    raw_u16,
    raw_u24,
    raw_u32
);
criterion_main!(raw_data_iter);
