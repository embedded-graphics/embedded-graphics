use embedded_graphics::prelude::*;
use tinybmp::{Bpp, Header, RawBmp};

const DATA: &[u8] = include_bytes!("./chessboard-8px-24bit.bmp");

#[test]
fn chessboard_8px_24bit() {
    let bmp = RawBmp::from_slice(DATA).expect("Failed to parse");

    assert_eq!(
        bmp.header(),
        &Header {
            file_size: 314,
            image_data_start: 122,
            bpp: Bpp::Bits24,
            image_size: Size::new(8, 8),
            image_data_len: 192,
            channel_masks: None,
        }
    );

    assert_eq!(bmp.image_data().len(), 314 - 122);
}

#[test]
fn chessboard_8px_24bit_truncated_iter() {
    // corrupt data by removing the last 10 bytes
    let truncated_data = &DATA[..DATA.len() - 10];

    let bmp = RawBmp::from_slice(truncated_data).expect("Failed to parse");

    assert_eq!(
        bmp.header(),
        &Header {
            file_size: 314,
            image_data_start: 122,
            bpp: Bpp::Bits24,
            image_size: Size::new(8, 8),
            image_data_len: 192,
            channel_masks: None,
        }
    );

    let pixels: Vec<u32> = bmp.pixels().map(|pixel| pixel.color).collect();

    assert_eq!(pixels.len(), 8 * 8);

    // 24BPP black/white chessboard.
    // Because BMP files are stored bottom line first the truncated data shows up as
    // zeroes in the top image row.
    let expected = vec![
        0xffffff, 0xffffff, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, //
        0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
        0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, //
        0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
    ];

    assert_eq!(pixels, expected);
}

#[test]
fn chessboard_8px_24bit_iter() {
    let bmp = RawBmp::from_slice(DATA).expect("Failed to parse");

    let pixels: Vec<u32> = bmp.pixels().map(|pixel| pixel.color).collect();

    assert_eq!(pixels.len(), 8 * 8);

    // 24BPP black/white chessboard
    let expected = vec![
        0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, //
        0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
        0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, //
        0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
        0x000000, 0x000000, 0xffffff, 0xffffff, 0x000000, 0x000000, 0xffffff, 0xffffff, //
    ];

    assert_eq!(pixels, expected);
}
