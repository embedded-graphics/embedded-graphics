use tinytga::Tga;

#[test]
fn chequerboard_uncompressed_topleft() {
    let data = include_bytes!("./chequerboard-uncompressed-topleft.tga");

    let img = Tga::from_bytes(data).unwrap();

    println!("{:#?}", img.header);
    println!("{:#?}", img.footer);
    println!("Pixel data len {:#?}", img.pixel_data.len());

    let image_data_len =
        img.header.width * img.header.height * (img.header.pixel_depth as u16 / 8u16);

    // Source image is 8x8px, uncompressed, 8BPP color
    assert_eq!(img.header.pixel_depth, 8);
    assert_eq!(img.pixel_data.len(), image_data_len as usize);
}
