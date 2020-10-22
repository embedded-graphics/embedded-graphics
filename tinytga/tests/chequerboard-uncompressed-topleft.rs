use tinytga::{Bpp, ImageOrigin, ImageType, RawTga, TgaHeader};

#[test]
fn chequerboard_uncompressed_topleft() {
    let data = include_bytes!("./chequerboard-uncompressed-topleft.tga");

    let img = RawTga::from_slice(data).unwrap();

    println!("{:#?}", img.header());
    println!("Raw image data len {:#?}", img.image_data().len());

    let header = img.header();
    let image_data_len = header.width * header.height * header.pixel_depth.bytes() as u16;

    // Source image is 8x8px, uncompressed, 8BPP color
    assert_eq!(
        img.header(),
        TgaHeader {
            id_len: 0,
            has_color_map: false,
            image_type: ImageType::Monochrome,
            color_map_start: 0,
            color_map_len: 0,
            color_map_depth: None,
            x_origin: 0,
            y_origin: 8,
            width: 8,
            height: 8,
            pixel_depth: Bpp::Bits8,
            image_origin: ImageOrigin::TopLeft,
            alpha_channel_depth: 0,
        }
    );

    // Footer is empty for this image
    assert_eq!(img.extension_area(), None);
    assert_eq!(img.developer_directory(), None);

    assert_eq!(img.image_data().len(), image_data_len as usize);
}
