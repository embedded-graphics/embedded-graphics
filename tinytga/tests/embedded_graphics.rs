use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    image::Image,
    mock_display::MockDisplay,
    pixelcolor::{raw::RawData, Gray8, PixelColor, Rgb888, RgbColor},
    transform::Transform,
    DrawTarget,
};
use tinytga::Tga;

const PIXEL_COLORS: [(i32, i32, Rgb888); 16] = [
    (0, 0, Rgb888::WHITE),
    (1, 0, Rgb888::BLACK),
    (2, 0, Rgb888::WHITE),
    (3, 0, Rgb888::BLACK),
    (0, 1, Rgb888::BLACK),
    (1, 1, Rgb888::RED),
    (2, 1, Rgb888::BLACK),
    (3, 1, Rgb888::GREEN),
    (0, 2, Rgb888::WHITE),
    (1, 2, Rgb888::BLACK),
    (2, 2, Rgb888::BLUE),
    (3, 2, Rgb888::BLACK),
    (0, 3, Rgb888::BLACK),
    (1, 3, Rgb888::WHITE),
    (2, 3, Rgb888::BLACK),
    (3, 3, Rgb888::WHITE),
];

#[test]
fn chessboard_compressed() -> Result<(), ()> {
    let im = Tga::from_slice(include_bytes!("../../tests/chessboard_rle.tga"))?;
    let im: ImageTga<Rgb888> = Image::new(&im);

    let mut pixels = im.into_iter();

    for (i, (x, y, color)) in PIXEL_COLORS.iter().enumerate() {
        assert_eq!(
            pixels.next(),
            Some(Pixel(Point::new(*x, *y), *color)),
            "Pixel color at index {} does not match",
            i
        );
    }

    // 17th iteration should have no pixels from 4x4px image
    assert_eq!(pixels.next(), None);

    Ok(())
}

#[test]
fn chessboard_uncompressed() -> Result<(), ()> {
    let im = Tga::from_slice(include_bytes!("../../tests/chessboard_raw.tga"))?;
    let im: ImageTga<Rgb888> = Image::new(&im);

    let mut pixels = im.into_iter();

    for (i, (x, y, color)) in PIXEL_COLORS.iter().enumerate() {
        assert_eq!(
            pixels.next(),
            Some(Pixel(Point::new(*x, *y), *color)),
            "Pixel color at index {} does not match",
            i
        );
    }

    // 17th iteration should have no pixels from 4x4px image
    assert_eq!(pixels.next(), None);

    Ok(())
}

fn test_color_tga(data: &[u8]) {
    let im = Tga::from_slice(data).unwrap();
    let image: ImageTga<Rgb888> = Image::new(&im);

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
    let im = Tga::from_slice(data).unwrap();
    let image: ImageTga<Gray8> = Image::new(&im);

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
#[ignore]
fn type1_bl() {
    test_color_tga(include_bytes!("../../../tinytga/tests/type1_bl.tga"));
}

/// Tests color mapped, uncompressed, top left origin TGA file.
#[test]
fn type1_tl() {
    test_color_tga(include_bytes!("../../../tinytga/tests/type1_tl.tga"));
}

/// Tests true color, uncompressed, bottom left origin TGA file.
#[test]
#[ignore]
fn type2_bl() {
    test_color_tga(include_bytes!("../../../tinytga/tests/type2_bl.tga"));
}

/// Tests true color, uncompressed, top left origin TGA file.
#[test]
fn type2_tl() {
    test_color_tga(include_bytes!("../../../tinytga/tests/type2_tl.tga"));
}

/// Tests grayscale, uncompressed, bottom left origin TGA file.
#[test]
#[ignore]
fn type3_bl() {
    test_gray_tga(include_bytes!("../../../tinytga/tests/type3_bl.tga"));
}

/// Tests grayscale, uncompressed, top left origin TGA file.
#[test]
fn type3_tl() {
    test_gray_tga(include_bytes!("../../../tinytga/tests/type3_tl.tga"));
}

/// Tests color mapped, RLE compressed, bottom left origin TGA file.
#[test]
#[ignore]
fn type9_bl() {
    test_color_tga(include_bytes!("../../../tinytga/tests/type9_bl.tga"));
}

/// Tests color mapped, RLE compressed, top left origin TGA file.
#[test]
fn type9_tl() {
    test_color_tga(include_bytes!("../../../tinytga/tests/type9_tl.tga"));
}

/// Tests true color, RLE compressed, bottom left origin TGA file.
#[test]
#[ignore]
fn type10_bl() {
    test_color_tga(include_bytes!("../../../tinytga/tests/type10_bl.tga"));
}

/// Tests true color, RLE compressed, top left origin TGA file.
#[test]
fn type10_tl() {
    test_color_tga(include_bytes!("../../../tinytga/tests/type10_tl.tga"));
}

/// Tests grayscale, RLE compressed, bottom left origin TGA file.
#[test]
#[ignore]
fn type11_bl() {
    test_gray_tga(include_bytes!("../../../tinytga/tests/type11_bl.tga"));
}

/// Tests grayscale, RLE compressed, top left origin TGA file.
#[test]
fn type11_tl() {
    test_gray_tga(include_bytes!("../../../tinytga/tests/type11_tl.tga"));
}
