use tinytga::{Bpp, ImageOrigin, ImageType, RawTga, TgaHeader};

#[test]
fn cbw8() {
    let data = include_bytes!("./cbw8.tga");

    let img = RawTga::from_slice(data).unwrap();

    println!("{:#?}", img.header());
    println!("Raw image data len {:#?}", img.image_data().len());

    assert_eq!(
        img.header(),
        TgaHeader {
            id_len: 26,
            has_color_map: false,
            image_type: ImageType::RleMonochrome,
            color_map_start: 0,
            color_map_len: 0,
            color_map_depth: None,
            x_origin: 0,
            y_origin: 0,
            width: 128,
            height: 128,
            pixel_depth: Bpp::Bits8,
            image_origin: ImageOrigin::BottomLeft,
            alpha_channel_depth: 0,
        }
    );

    const TGA_FOOTER_LENGTH: usize = 26;
    assert_eq!(
        img.extension_area(),
        Some(&data[8238..data.len() - TGA_FOOTER_LENGTH])
    );
    assert_eq!(img.developer_directory(), None);

    let pixels = img.pixels().collect::<Vec<_>>();

    assert_eq!(pixels.len(), 128 * 128);
}
