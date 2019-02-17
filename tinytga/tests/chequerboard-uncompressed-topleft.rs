use tinytga::{ImageType, Tga, TgaFooter, TgaHeader};

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
    assert_eq!(
        img.header,
        TgaHeader {
            id_len: 0,
            has_color_map: false,
            image_type: ImageType::Monochrome,
            color_map_start: 0,
            color_map_len: 0,
            color_map_depth: 0,
            x_origin: 0,
            y_origin: 8,
            width: 8,
            height: 8,
            pixel_depth: 8,
            image_descriptor: 32
        }
    );

    // Footer is empty for this image
    assert_eq!(
        img.footer,
        TgaFooter {
            extension_area_offset: 0,
            developer_directory_offset: 0
        }
    );

    assert_eq!(img.pixel_data.len(), image_data_len as usize);
}
