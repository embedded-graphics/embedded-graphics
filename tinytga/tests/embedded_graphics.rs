use embedded_graphics::{
    image::Image,
    mock_display::MockDisplay,
    pixelcolor::{Gray8, Rgb888},
    prelude::*,
};
use tinytga::EgTga;

const CHESSBOARD_PATTERN: &[&str] = &[
    "WKWK", //
    "KRKG", //
    "WKBK", //
    "KWKW", //
];

#[test]
fn chessboard_compressed() {
    let tga: EgTga<Rgb888> = EgTga::from_slice(include_bytes!("./chessboard_4px_rle.tga")).unwrap();
    let image = Image::new(&tga, Point::zero());

    let mut display = MockDisplay::new();
    image.draw(&mut display).unwrap();

    assert_eq!(display, MockDisplay::from_pattern(CHESSBOARD_PATTERN));
}

#[test]
fn chessboard_uncompressed() {
    let tga: EgTga<Rgb888> = EgTga::from_slice(include_bytes!("./chessboard_raw.tga")).unwrap();
    let image = Image::new(&tga, Point::zero());

    let mut display = MockDisplay::new();
    image.draw(&mut display).unwrap();

    assert_eq!(display, MockDisplay::from_pattern(CHESSBOARD_PATTERN));
}

fn test_color_tga(data: &[u8]) {
    let im: EgTga<Rgb888> = EgTga::from_slice(data).unwrap();
    let image = Image::new(&im, Point::zero());

    let mut display = MockDisplay::new();
    image.draw(&mut display).unwrap();

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "WKRGBYMCW",
            "KKRGBYMCW",
            "WKRGBYMCW",
            "KKKKKKKKK",
            "WKWCMYBGR",
        ])
    );
}

fn test_gray_tga(data: &[u8]) {
    let im: EgTga<Gray8> = EgTga::from_slice(data).unwrap();
    let image = Image::new(&im, Point::zero());

    let mut display = MockDisplay::new();
    image.draw(&mut display).unwrap();

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "0F0F0F0F0",
            "00FF00FF0",
            "0000FFFF0",
            "012345670",
            "89ABCDEF0",
        ])
    );
}

/// Tests color mapped, uncompressed, bottom left origin TGA file.
#[test]
fn type1_bl() {
    test_color_tga(include_bytes!("./type1_bl.tga"));
}

/// Tests color mapped, uncompressed, top left origin TGA file.
#[test]
fn type1_tl() {
    test_color_tga(include_bytes!("./type1_tl.tga"));
}

/// Tests true color, uncompressed, bottom left origin TGA file.
#[test]
fn type2_bl() {
    test_color_tga(include_bytes!("./type2_bl.tga"));
}

/// Tests true color, uncompressed, top left origin TGA file.
#[test]
fn type2_tl() {
    test_color_tga(include_bytes!("./type2_tl.tga"));
}

/// Tests grayscale, uncompressed, bottom left origin TGA file.
#[test]
fn type3_bl() {
    test_gray_tga(include_bytes!("./type3_bl.tga"));
}

/// Tests grayscale, uncompressed, top left origin TGA file.
#[test]
fn type3_tl() {
    test_gray_tga(include_bytes!("./type3_tl.tga"));
}

/// Tests color mapped, RLE compressed, bottom left origin TGA file.
#[test]
fn type9_bl() {
    test_color_tga(include_bytes!("./type9_bl.tga"));
}

/// Tests color mapped, RLE compressed, top left origin TGA file.
#[test]
fn type9_tl() {
    test_color_tga(include_bytes!("./type9_tl.tga"));
}

/// Tests true color, RLE compressed, bottom left origin TGA file.
#[test]
fn type10_bl() {
    test_color_tga(include_bytes!("./type10_bl.tga"));
}

/// Tests true color, RLE compressed, top left origin TGA file.
#[test]
fn type10_tl() {
    test_color_tga(include_bytes!("./type10_tl.tga"));
}

/// Tests grayscale, RLE compressed, bottom left origin TGA file.
#[test]
fn type11_bl() {
    test_gray_tga(include_bytes!("./type11_bl.tga"));
}

/// Tests grayscale, RLE compressed, top left origin TGA file.
#[test]
fn type11_tl() {
    test_gray_tga(include_bytes!("./type11_tl.tga"));
}
