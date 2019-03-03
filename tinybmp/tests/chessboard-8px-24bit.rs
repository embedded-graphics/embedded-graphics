use tinybmp::{Bmp, FileType, Header};

#[test]
fn chessboard_8px_24bit() {
    let bmp =
        Bmp::from_slice(include_bytes!("./chessboard-8px-24bit.bmp")).expect("Failed to parse");

    assert_eq!(
        bmp.header,
        Header {
            file_type: FileType::BM,
            file_size: 314,
            reserved_1: 0,
            reserved_2: 0,
            image_data_start: 122,
            bpp: 24,
            image_width: 8,
            image_height: 8,
            image_data_len: 192
        }
    );

    assert_eq!(bmp.image_data().len(), 314 - 122);
}
