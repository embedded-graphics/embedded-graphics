use tinytga::Tga;

#[test]
fn issue_216() {
    let uncompressed = Tga::from_slice(include_bytes!("issue_216_uncompressed.tga")).unwrap();
    let compressed = Tga::from_slice(include_bytes!("issue_216_compressed.tga")).unwrap();

    assert_eq!(uncompressed.header.width, compressed.header.width);
    assert_eq!(uncompressed.header.height, compressed.header.height);
    assert_eq!(
        uncompressed.header.pixel_depth,
        compressed.header.pixel_depth
    );

    assert!(uncompressed.into_iter().eq(compressed.into_iter()));
}
