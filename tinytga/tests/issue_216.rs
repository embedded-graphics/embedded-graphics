use tinytga::Tga;

#[test]
fn issue_216() {
    let uncompressed = Tga::from_slice_raw(include_bytes!("issue_216_uncompressed.tga")).unwrap();
    let compressed = Tga::from_slice_raw(include_bytes!("issue_216_compressed.tga")).unwrap();

    let uncompressed_header = uncompressed.raw_header();
    let compressed_header = uncompressed.raw_header();

    assert_eq!(uncompressed_header.width, compressed_header.width);
    assert_eq!(uncompressed_header.height, compressed_header.height);
    assert_eq!(
        uncompressed_header.pixel_depth,
        compressed_header.pixel_depth
    );

    assert!(uncompressed.raw_pixels().eq(compressed.raw_pixels()));
}
