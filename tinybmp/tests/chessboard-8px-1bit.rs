use embedded_graphics::prelude::*;
use tinybmp::{Bpp, Header, RawBmp};

#[test]
fn chessboard_8px_1bit() {
    let bmp =
        RawBmp::from_slice(include_bytes!("./chessboard-8px-1bit.bmp")).expect("Failed to parse");

    assert_eq!(
        bmp.header(),
        &Header {
            file_size: 94,
            image_data_start: 62,
            bpp: Bpp::Bits1,
            image_size: Size::new(8, 8),
            image_data_len: 32,
            channel_masks: None,
        }
    );

    assert_eq!(bmp.image_data().len(), 94 - 62);
}

#[test]
fn chessboard_8px_1bit_iter() {
    let bmp =
        RawBmp::from_slice(include_bytes!("./chessboard-8px-1bit.bmp")).expect("Failed to parse");

    let pixels: Vec<u32> = bmp.pixels().map(|pixel| pixel.color).collect();

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
