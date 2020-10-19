use tinybmp::{Bmp, Bpp, Header};

#[test]
fn chessboard_8px_24bit() {
    let bmp =
        Bmp::from_slice_raw(include_bytes!("./chessboard-8px-24bit.bmp")).expect("Failed to parse");

    assert_eq!(
        bmp.header,
        Header {
            file_size: 314,
            image_data_start: 122,
            bpp: Bpp::Bits24,
            image_width: 8,
            image_height: 8,
            image_data_len: 192
        }
    );

    assert_eq!(bmp.raw_image_data().len(), 314 - 122);
}

#[test]
fn chessboard_8px_24bit_iter() {
    let bmp =
        Bmp::from_slice_raw(include_bytes!("./chessboard-8px-24bit.bmp")).expect("Failed to parse");

    let pixels: Vec<u32> = bmp.raw_pixels().map(|pixel| pixel.color).collect();

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
