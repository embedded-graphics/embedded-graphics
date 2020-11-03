use embedded_graphics::{
    image::Image,
    mock_display::{ColorMapping, MockDisplay},
    pixelcolor::{Gray8, Rgb555, Rgb565, Rgb888},
    prelude::*,
    primitives::Rectangle,
};
use tinybmp::{Bmp, DynamicBmp};

#[test]
fn negative_top_left() {
    let image: Bmp<Rgb565> =
        Bmp::from_slice(include_bytes!("./chessboard-4px-color-16bit.bmp")).unwrap();
    let image = Image::new(&image, Point::zero()).translate(Point::new(-1, -1));

    assert_eq!(
        image.bounding_box(),
        Rectangle::new(Point::new(-1, -1), Size::new(4, 4))
    );
}

#[test]
fn dimensions() {
    let image: Bmp<Rgb565> =
        Bmp::from_slice(include_bytes!("./chessboard-4px-color-16bit.bmp")).unwrap();
    let image = Image::new(&image, Point::zero()).translate(Point::new(100, 200));

    assert_eq!(
        image.bounding_box(),
        Rectangle::new(Point::new(100, 200), Size::new(4, 4))
    );
}

fn expected_image_color<C>() -> MockDisplay<C>
where
    C: PixelColor + ColorMapping,
{
    MockDisplay::from_pattern(&[
        "KRGY", //
        "BMCW", //
    ])
}

fn expected_image_gray() -> MockDisplay<Gray8> {
    MockDisplay::from_pattern(&["08F"])
}

fn draw_image<C, T>(image_drawable: T) -> MockDisplay<C>
where
    C: PixelColor,
    T: ImageDrawable<Color = C>,
{
    let image = Image::new(&image_drawable, Point::zero());

    let mut display = MockDisplay::new();
    image.draw(&mut display).unwrap();

    display
}

fn test_color_pattern<C>(data: &[u8])
where
    C: PixelColor + From<<C as PixelColor>::Raw> + ColorMapping,
{
    let bmp = Bmp::<C>::from_slice(data).unwrap();
    assert_eq!(draw_image(bmp), expected_image_color());
}

fn test_color_pattern_dynamic(data: &[u8]) {
    let bmp = DynamicBmp::from_slice(data).unwrap();
    assert_eq!(draw_image(bmp), expected_image_color::<Rgb565>());

    let bmp = DynamicBmp::from_slice(data).unwrap();
    assert_eq!(draw_image(bmp), expected_image_color::<Rgb888>());
}

#[test]
fn colors_rgb555() {
    test_color_pattern::<Rgb555>(include_bytes!("./colors_rgb555.bmp"));
}

#[test]
fn colors_rgb555_dynamic() {
    test_color_pattern_dynamic(include_bytes!("./colors_rgb555.bmp"));
}

#[test]
fn colors_rgb565() {
    test_color_pattern::<Rgb565>(include_bytes!("./colors_rgb565.bmp"));
}

#[test]
fn colors_rgb565_dynamic() {
    test_color_pattern_dynamic(include_bytes!("./colors_rgb565.bmp"));
}

#[test]
fn colors_rgb888_24bit() {
    test_color_pattern::<Rgb888>(include_bytes!("./colors_rgb888_24bit.bmp"));
}

#[test]
fn colors_rgb888_24bit_dynamic() {
    test_color_pattern_dynamic(include_bytes!("./colors_rgb888_24bit.bmp"));
}

#[test]
fn colors_rgb888_32bit() {
    test_color_pattern::<Rgb888>(include_bytes!("./colors_rgb888_32bit.bmp"));
}

#[test]
fn colors_rgb888_32bit_dynamic() {
    test_color_pattern_dynamic(include_bytes!("./colors_rgb888_32bit.bmp"));
}

#[test]
fn colors_grey8() {
    let bmp: Bmp<Gray8> = Bmp::from_slice(include_bytes!("./colors_grey8.bmp")).unwrap();
    assert_eq!(draw_image(bmp), expected_image_gray());
}

#[test]
fn colors_grey8_dynamic() {
    let bmp = DynamicBmp::from_slice(include_bytes!("./colors_grey8.bmp")).unwrap();
    let display = draw_image::<Rgb565, _>(bmp);
    assert_eq!(display, expected_image_gray().map(|c| c.into()));

    let bmp = DynamicBmp::from_slice(include_bytes!("./colors_grey8.bmp")).unwrap();
    let display = draw_image::<Rgb888, _>(bmp);
    assert_eq!(display, expected_image_gray().map(|c| c.into()));
}

/// Test for issue #136
#[test]
fn issue_136_row_size_is_multiple_of_4_bytes() {
    let image: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("./issue_136.bmp")).unwrap();
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
