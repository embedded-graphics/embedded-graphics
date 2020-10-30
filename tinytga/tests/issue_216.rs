use tinytga::RawTga;

/// Test for issue #216
///
/// RLE compressed images caused an integer overflow if `number_of_pixels * bytes_per_pixel` in a
/// RLE packet was larger than 255.
#[test]
fn issue_216() {
    let uncompressed = RawTga::from_slice(include_bytes!("issue_216_uncompressed.tga")).unwrap();
    let compressed = RawTga::from_slice(include_bytes!("issue_216_compressed.tga")).unwrap();

    let uncompressed_header = uncompressed.header();
    let compressed_header = uncompressed.header();

    assert_eq!(uncompressed_header.width, compressed_header.width);
    assert_eq!(uncompressed_header.height, compressed_header.height);
    assert_eq!(
        uncompressed_header.pixel_depth,
        compressed_header.pixel_depth
    );

    assert!(uncompressed.pixels().eq(compressed.pixels()));
}
