use embedded_graphics::{
    image::{Image, ImageFile},
    mock_display::{ColorMapping, MockDisplay},
    pixelcolor::{Gray8, Rgb555, Rgb565, Rgb888},
    prelude::*,
    primitives::Rectangle,
};
use tinybmp::Bmp;

#[test]
fn negative_top_left() {
    let image: ImageFile<Bmp, Rgb565> =
        ImageFile::from_slice(include_bytes!("./chessboard-4px-color-16bit.bmp")).unwrap();
    let image = Image::new(&image, Point::zero()).translate(Point::new(-1, -1));

    assert_eq!(
        image.bounding_box(),
        Rectangle::new(Point::new(-1, -1), Size::new(4, 4))
    );
}

#[test]
fn dimensions() {
    let image: ImageFile<Bmp, Rgb565> =
        ImageFile::from_slice(include_bytes!("./chessboard-4px-color-16bit.bmp")).unwrap();
    let image = Image::new(&image, Point::zero()).translate(Point::new(100, 200));

    assert_eq!(
        image.bounding_box(),
        Rectangle::new(Point::new(100, 200), Size::new(4, 4))
    );
}

fn test_color_pattern<C>(data: &[u8])
where
    C: PixelColor + From<<C as PixelColor>::Raw> + ColorMapping,
{
    let bmp: ImageFile<Bmp, C> = ImageFile::from_slice(data).unwrap();
    let image = Image::new(&bmp, Point::zero());

    let mut display = MockDisplay::new();
    image.draw(&mut display).unwrap();

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "KRGY", //
            "BMCW", //
        ])
    );
}

#[test]
fn colors_rgb555() {
    test_color_pattern::<Rgb555>(include_bytes!("./colors_rgb555.bmp"));
}

#[test]
fn colors_rgb565() {
    test_color_pattern::<Rgb565>(include_bytes!("./colors_rgb565.bmp"));
}

#[test]
fn colors_rgb888_24bit() {
    test_color_pattern::<Rgb888>(include_bytes!("./colors_rgb888_24bit.bmp"));
}

#[test]
fn colors_rgb888_32bit() {
    test_color_pattern::<Rgb888>(include_bytes!("./colors_rgb888_32bit.bmp"));
}

#[test]
fn colors_grey8() {
    let image: ImageFile<Bmp, Gray8> =
        ImageFile::from_slice(include_bytes!("./colors_grey8.bmp")).unwrap();
    let image = Image::new(&image, Point::zero());

    let mut display = MockDisplay::new();
    image.draw(&mut display).unwrap();

    let mut expected = MockDisplay::new();
    Pixel(Point::new(0, 0), Gray8::new(0x00))
        .draw(&mut expected)
        .unwrap();
    Pixel(Point::new(1, 0), Gray8::new(0x80))
        .draw(&mut expected)
        .unwrap();
    Pixel(Point::new(2, 0), Gray8::new(0xFF))
        .draw(&mut expected)
        .unwrap();

    assert_eq!(display, expected);
}

/// Test for issue #136
#[test]
fn issue_136_row_size_is_multiple_of_4_bytes() {
    let image: ImageFile<Bmp, Rgb565> =
        ImageFile::from_slice(include_bytes!("./issue_136.bmp")).unwrap();
    let image = Image::new(&image, Point::zero());

    let mut display = MockDisplay::new();
    image.draw(&mut display).unwrap();

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "WWWWKWWWW",
            "WKKKKWKKK",
            "WWWWKWKWW",
            "WKKKKWKKW",
            "WWWWKWWWW",
        ])
    );
}
