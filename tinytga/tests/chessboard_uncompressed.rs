use tinytga::{Bpp, ImageOrigin, ImageType, Tga, TgaHeader};

#[test]
fn chessboard_uncompressed() {
    let data = include_bytes!("./chessboard_uncompressed.tga");

    let img = Tga::from_slice_raw(data).unwrap();

    println!("{:#?}", img.raw_header());
    println!("Raw image data len {:#?}", img.raw_image_data().len());
    println!("Raw image data {:#?}", img.raw_image_data());

    assert_eq!(
        img.raw_header(),
        TgaHeader {
            id_len: 0,
            has_color_map: false,
            image_type: ImageType::Truecolor,
            color_map_start: 0,
            color_map_len: 0,
            color_map_depth: None,
            x_origin: 0,
            y_origin: 8,
            width: 8,
            height: 8,
            pixel_depth: Bpp::Bits24,
            image_origin: ImageOrigin::TopLeft,
            alpha_channel_depth: 0,
        }
    );

    assert_eq!(img.raw_extension_area(), None);
    assert_eq!(img.raw_developer_directory(), None);

    let pixels = img.raw_pixels().map(|p| p.color).collect::<Vec<u32>>();

    assert_eq!(pixels.len(), 8 * 8);
    assert_eq!(
        pixels,
        vec![
            0xffffffu32,
            0xffffff,
            0x000000,
            0x000000,
            0xffffff,
            0xffffff,
            0x000000,
            0x000000,
            0xffffff,
            0xffffff,
            0x000000,
            0x000000,
            0xffffff,
            0xffffff,
            0x000000,
            0x000000,
            0x000000,
            0x000000,
            0xff0000,
            0xff0000,
            0x000000,
            0x000000,
            0x00ff00,
            0x00ff00,
            0x000000,
            0x000000,
            0xff0000,
            0xff0000,
            0x000000,
            0x000000,
            0x00ff00,
            0x00ff00,
            0xffffff,
            0xffffff,
            0x000000,
            0x000000,
            0x0000ff,
            0x0000ff,
            0x000000,
            0x000000,
            0xffffff,
            0xffffff,
            0x000000,
            0x000000,
            0x0000ff,
            0x0000ff,
            0x000000,
            0x000000,
            0x000000,
            0x000000,
            0xffffff,
            0xffffff,
            0x000000,
            0x000000,
            0xffffff,
            0xffffff,
            0x000000,
            0x000000,
            0xffffff,
            0xffffff,
            0x000000,
            0x000000,
            0xffffff,
            0xffffff,
        ]
    );
}
