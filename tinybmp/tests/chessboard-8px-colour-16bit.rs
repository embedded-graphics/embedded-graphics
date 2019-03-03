use tinybmp::{Bmp, FileType, Header};

#[test]
fn chessboard_8px_colour_16bit() {
    let bmp = Bmp::from_slice(include_bytes!("./chessboard-8px-colour-16bit.bmp"))
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
