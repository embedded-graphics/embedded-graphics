use tinytga::{Bpp, ImageOrigin, ImageType, RawTga, TgaHeader};

#[test]
fn chessboard_4px_raw() {
    let data = include_bytes!("./chessboard_4px_raw.tga");

    let img = RawTga::from_slice(data).unwrap();

    println!("{:#?}", img.header());
    println!("Raw image data len {:#?}", img.image_data().len());
    println!("Raw image data {:#?}", img.image_data());

    assert_eq!(
        img.header(),
        TgaHeader {
            id_len: 0,
            has_color_map: false,
            image_type: ImageType::Truecolor,
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

    assert_eq!(img.extension_area(), None);
    assert_eq!(img.developer_directory(), None);

    let pixels = img.pixels().map(|p| p.color).collect::<Vec<u32>>();

    dbg!(&pixels);

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
