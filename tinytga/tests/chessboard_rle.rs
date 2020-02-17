use tinytga::{ImageType, Tga, TgaFooter, TgaHeader};

#[test]
fn chessboard_rle() {
    let data = include_bytes!("./chessboard_rle.tga");

    let img = Tga::from_slice(data).unwrap();

    println!("{:#?}", img.header);
    println!("{:#?}", img.footer);
    println!("Pixel data len {:#?}", img.pixel_data.len());
    println!("Pixel data {:#?}", img.pixel_data);

    assert_eq!(
        img.header,
        TgaHeader {
            id_len: 0,
            has_color_map: false,
            image_type: ImageType::RleTruecolor,
            color_map_start: 0,
            color_map_len: 0,
            color_map_depth: 0,
            x_origin: 0,
            y_origin: 8,
            width: 8,
            height: 8,
            pixel_depth: 24,
            image_descriptor: 32
        }
    );

    assert_eq!(
        img.footer,
        Some(TgaFooter {
            extension_area_offset: 0,
            developer_directory_offset: 0
        })
    );

    let pixels = img.into_iter().map(|p| p.color).collect::<Vec<u32>>();

    dbg!(&pixels);

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
