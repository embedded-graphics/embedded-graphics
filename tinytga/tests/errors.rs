use embedded_graphics::{
    pixelcolor::{Gray8, Rgb888},
    prelude::*,
    primitives::Rectangle,
};
use std::iter::repeat;
use tinytga::{ParseError, RawPixel, RawTga, Tga};

#[test]
fn color_map() {
    // The color map in "error_color_map.tga" has too many entries and is larger than the file
    assert_eq!(
        RawTga::from_slice(include_bytes!("../tests/error_color_map.tga")),
        Err(ParseError::ColorMap)
    );
}

#[test]
fn image_data_missing() {
    // The image data in "error_no_image_data.tga" is missing
    let tga = RawTga::from_slice(include_bytes!("../tests/error_no_image_data.tga")).unwrap();

    assert!(tga.image_data().is_empty());

    let expected: Vec<_> = Rectangle::new(Point::zero(), tga.size())
        .points()
        .map(|p| RawPixel::new(p, 0))
        .collect();

    let pixels: Vec<_> = tga.pixels().collect();

    assert_eq!(pixels, expected);
}

#[test]
fn image_data_truncated() {
    // The image data in "error_truncated_image_data.tga" is truncated.
    let tga =
        RawTga::from_slice(include_bytes!("../tests/error_truncated_image_data.tga")).unwrap();

    assert_eq!(tga.image_data(), &[1, 2, 3, 4, 5, 6, 7, 8]);

    let expected: Vec<_> = Rectangle::new(Point::zero(), tga.size())
        .points()
        .zip((1..=8).chain(repeat(0)))
        .map(|(p, c)| RawPixel::new(p, c))
        .collect();

    let pixels: Vec<_> = tga.pixels().collect();

    assert_eq!(pixels, expected);
}

#[test]
fn mismatched_bpp() {
    // type2_tl_24bpp.tga is a 24 BPP image
    assert_eq!(
        Tga::<Gray8>::from_slice(include_bytes!("../tests/type2_24bpp_tl.tga")),
        Err(ParseError::MismatchedBpp(24))
    );

    // type3_tl.tga is a 8 BPP image
    assert_eq!(
        Tga::<Rgb888>::from_slice(include_bytes!("../tests/type3_tl.tga")),
        Err(ParseError::MismatchedBpp(8))
    );
}
