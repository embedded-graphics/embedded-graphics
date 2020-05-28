use criterion::*;
use tinytga::{Pixel, Tga};

fn uncompressed(c: &mut Criterion) {
    let mut group = c.benchmark_group("uncompressed");

    let file = include_bytes!("../tests/type1_bl.tga");
    group.throughput(Throughput::Bytes(file.len() as u64));
    group.bench_function("colormap", |b| {
        b.iter(|| {
            Tga::from_slice(file)
                .unwrap()
                .into_iter()
                .collect::<Vec<Pixel>>()
        })
    });

    let file = include_bytes!("../tests/type2_bl.tga");
    group.throughput(Throughput::Bytes(file.len() as u64));
    group.bench_function("truecolor", |b| {
        b.iter(|| {
            Tga::from_slice(file)
                .unwrap()
                .into_iter()
                .collect::<Vec<Pixel>>()
        })
    });

    let file = include_bytes!("../tests/type3_bl.tga");
    group.throughput(Throughput::Bytes(file.len() as u64));
    group.bench_function("monochrome", |b| {
        b.iter(|| {
            Tga::from_slice(file)
                .unwrap()
                .into_iter()
                .collect::<Vec<Pixel>>()
        })
    });

    group.finish();
}

fn rle(c: &mut Criterion) {
    let mut group = c.benchmark_group("rle");

    let file = include_bytes!("../tests/type9_bl.tga");
    group.throughput(Throughput::Bytes(file.len() as u64));
    group.bench_function("colormap", |b| {
        b.iter(|| {
            Tga::from_slice(file)
                .unwrap()
                .into_iter()
                .collect::<Vec<Pixel>>()
        })
    });

    let file = include_bytes!("../tests/type10_bl.tga");
    group.throughput(Throughput::Bytes(file.len() as u64));
    group.bench_function("truecolor", |b| {
        b.iter(|| {
            Tga::from_slice(file)
                .unwrap()
                .into_iter()
                .collect::<Vec<Pixel>>()
        })
    });

    let file = include_bytes!("../tests/type11_bl.tga");
    group.throughput(Throughput::Bytes(file.len() as u64));
    group.bench_function("monochrome", |b| {
        b.iter(|| {
            Tga::from_slice(file)
                .unwrap()
                .into_iter()
                .collect::<Vec<Pixel>>()
        })
    });

    group.finish();
}

criterion_group!(tga, uncompressed, rle);
criterion_main!(tga);
