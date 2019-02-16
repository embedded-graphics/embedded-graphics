use tinytga::Tga;

#[test]
fn chequerboard_uncompressed_topleft() {
    let data = include_bytes!("./chequerboard-uncompressed-topleft.tga");

    let img = Tga::from_bytes(data).unwrap();

    println!("{:#?}", img.header);
    println!("{:#?}", img.footer);
    println!("{:#?}", img.pixel_data.len());

    // Source image is 8x8px, uncompressed, 8BPP color
    assert_eq!(img.header.pixel_depth, 8);
    assert_eq!(img.pixel_data.len(), 64);
}
