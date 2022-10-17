use criterion::*;
use embedded_graphics::{
    iterator::raw::RawDataSlice,
    pixelcolor::raw::{
        storage::{LittleEndian, Msb0},
        RawU1, RawU16, RawU2, RawU24, RawU32, RawU4, RawU8,
    },
};

const TEST_DATA: &[u8] = include_bytes!("../fonts/src/10x20.bdf");

macro_rules! impl_bench {
    ($fn:ident, $type:ty) => {
        fn $fn(c: &mut Criterion) {
            c.bench_function(stringify!($type), |b| {
                let slice = RawDataSlice::<$type>::new(TEST_DATA);

                b.iter(|| slice.into_iter().collect::<Vec<_>>())
            });

            c.bench_function(concat!(stringify!($type), " step by 20"), |b| {
                let slice = RawDataSlice::<$type>::new(TEST_DATA);

                b.iter(|| slice.into_iter().step_by(20).collect::<Vec<_>>())
            });
        }
    };
}

impl_bench!(raw_u1, Msb0<RawU1>);
impl_bench!(raw_u2, Msb0<RawU2>);
impl_bench!(raw_u4, Msb0<RawU4>);
impl_bench!(raw_u8, RawU8);
impl_bench!(raw_u16, LittleEndian<RawU16>);
impl_bench!(raw_u24, LittleEndian<RawU24>);
impl_bench!(raw_u32, LittleEndian<RawU32>);

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
