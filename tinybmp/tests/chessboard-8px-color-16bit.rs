use tinybmp::{Bmp, FileType, Header, Pixel};

#[test]
fn chessboard_8px_color_16bit() {
    let bmp = Bmp::from_slice(include_bytes!("./chessboard-8px-color-16bit.bmp"))
        .expect("Failed to parse");

    assert_eq!(
        bmp.header,
        Header {
            file_type: FileType::BM,
            file_size: 266,
            reserved_1: 0,
            reserved_2: 0,
            image_data_start: 138,
            bpp: 16,
            image_width: 8,
            image_height: 8,
            image_data_len: 128
        }
    );

    assert_eq!(bmp.image_data().len(), 266 - 138);
}

#[test]
fn chessboard_8px_color_16bit_iter() {
    let bmp = Bmp::from_slice(include_bytes!("./chessboard-8px-color-16bit.bmp"))
        .expect("Failed to parse");

    let pixels: Vec<u32> = bmp.into_iter().map(|Pixel { color, .. }| color).collect();

    // 8px x 8px image. Check that iterator returns all pixels in it
    assert_eq!(pixels.len(), 8 * 8);

    let expected = vec![
        0xffff, 0xffff, 0x0000, 0x0000, 0xffff, 0xffff, 0x0000, 0x0000, //
        0xffff, 0xffff, 0x0000, 0x0000, 0xffff, 0xffff, 0x0000, 0x0000, //
        0x0000, 0x0000, 0xf800, 0xf800, 0x0000, 0x0000, 0x07e0, 0x07e0, //
        0x0000, 0x0000, 0xf800, 0xf800, 0x0000, 0x0000, 0x07e0, 0x07e0, //
        0xffff, 0xffff, 0x0000, 0x0000, 0x001f, 0x001f, 0x0000, 0x0000, //
        0xffff, 0xffff, 0x0000, 0x0000, 0x001f, 0x001f, 0x0000, 0x0000, //
        0x0000, 0x0000, 0xffff, 0xffff, 0x0000, 0x0000, 0xffff, 0xffff, //
        0x0000, 0x0000, 0xffff, 0xffff, 0x0000, 0x0000, 0xffff, 0xffff, //
    ];

    assert_eq!(pixels, expected);
}
