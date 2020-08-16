use tinytga::{Bpp, ImageOrigin, ImageType, Tga, TgaHeader};

#[test]
fn chessboard_4px_rle() {
    let data = include_bytes!("./chessboard_4px_rle.tga");

    let img = Tga::from_slice_raw(data).unwrap();

    println!("{:#?}", img.raw_header());
    println!("Raw image data len {:#?}", img.raw_image_data().len());
    println!("Raw image data {:#?}", img.raw_image_data());

    assert_eq!(
        img.raw_header(),
        TgaHeader {
            id_len: 0,
            has_color_map: false,
            image_type: ImageType::RleTruecolor,
            color_map_start: 0,
            color_map_len: 0,
            color_map_depth: None,
            x_origin: 0,
            y_origin: 4,
            width: 4,
            height: 4,
            pixel_depth: Bpp::Bits24,
            image_origin: ImageOrigin::TopLeft,
            alpha_channel_depth: 0,
        }
    );

    assert_eq!(img.raw_extension_area(), None);
    assert_eq!(img.raw_developer_directory(), None);

    let pixels = img.raw_pixels().map(|p| p.color).collect::<Vec<u32>>();

    // dbg!(&pixels);

    assert_eq!(pixels.len(), 4 * 4);
    assert_eq!(
        pixels,
        vec![
            0x00ffffffu32,
            0x00000000,
            0x00ffffff,
            0x00000000,
            0x00000000,
            0x00ff0000,
            0x00000000,
            0x0000ff00,
            0x00ffffff,
            0x00000000,
            0x000000ff,
            0x00000000,
            0x00000000,
            0x00ffffff,
            0x00000000,
            0x00ffffff,
        ]
    );
}
