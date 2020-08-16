use tinytga::{Bpp, ImageOrigin, ImageType, Tga, TgaHeader};

#[test]
fn cbw8() {
    let data = include_bytes!("./cbw8.tga");

    let img = Tga::from_slice_raw(data).unwrap();

    println!("{:#?}", img.raw_header());
    println!("Raw image data len {:#?}", img.raw_image_data().len());

    assert_eq!(
        img.raw_header(),
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
        img.raw_extension_area(),
        Some(&data[8238..data.len() - TGA_FOOTER_LENGTH])
    );
    assert_eq!(img.raw_developer_directory(), None);

    let pixels = img.raw_pixels().collect::<Vec<_>>();

    assert_eq!(pixels.len(), 128 * 128);
}
