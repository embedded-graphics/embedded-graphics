use tinybmp::{BmpRaw, FileType, Header, Pixel};

#[test]
fn chessboard_8px_1bit() {
    let bmp =
        BmpRaw::from_slice(include_bytes!("./chessboard-8px-1bit.bmp")).expect("Failed to parse");

    assert_eq!(
        bmp.header,
        Header {
            file_type: FileType::BM,
            file_size: 94,
            reserved_1: 0,
            reserved_2: 0,
            image_data_start: 62,
            bpp: 1,
            image_width: 8,
            image_height: 8,
            image_data_len: 32
        }
    );

    assert_eq!(bmp.image_data().len(), 94 - 62);
}

#[test]
fn chessboard_8px_1bit_iter() {
    let bmp =
        BmpRaw::from_slice(include_bytes!("./chessboard-8px-1bit.bmp")).expect("Failed to parse");

    let pixels: Vec<u32> = bmp.into_iter().map(|Pixel { color, .. }| color).collect();

    // 8px x 8px image. Check that iterator returns all pixels in it
    assert_eq!(pixels.len(), 8 * 8);

    let expected = vec![
        1, 1, 0, 0, 1, 1, 0, 0, //
        1, 1, 0, 0, 1, 1, 0, 0, //
        0, 0, 1, 1, 0, 0, 1, 1, //
        0, 0, 1, 1, 0, 0, 1, 1, //
        1, 1, 0, 0, 1, 1, 0, 0, //
        1, 1, 0, 0, 1, 1, 0, 0, //
        0, 0, 1, 1, 0, 0, 1, 1, //
        0, 0, 1, 1, 0, 0, 1, 1, //
    ];

    assert_eq!(pixels, expected);
}
