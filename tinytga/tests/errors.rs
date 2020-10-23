use embedded_graphics::{
    pixelcolor::{Gray8, Rgb888},
    prelude::*,
};
use std::iter::repeat;
use tinytga::{ParseError, RawPixel, Tga};

#[test]
fn color_map() {
    // The color map in "error_color_map.tga" has too many entries and is larger than the file
    assert_eq!(
        Tga::from_slice_raw(include_bytes!("../tests/error_color_map.tga")),
        Err(ParseError::ColorMap)
    );
}

#[test]
fn image_data_missing() {
    // The image data in "error_no_image_data.tga" is missing
    let tga = Tga::from_slice_raw(include_bytes!("../tests/error_no_image_data.tga")).unwrap();

    assert!(tga.raw_image_data().is_empty());

    let expected: Vec<_> = tga
        .bounding_box()
        .points()
        .map(|p| RawPixel::new(p, 0))
        .collect();

    let pixels: Vec<_> = tga.raw_pixels().collect();

    assert_eq!(pixels, expected);
}

#[test]
fn image_data_truncated() {
    // The image data in "error_truncated_image_data.tga" is truncated.
    let tga =
        Tga::from_slice_raw(include_bytes!("../tests/error_truncated_image_data.tga")).unwrap();

    assert_eq!(tga.raw_image_data(), &[1, 2, 3, 4, 5, 6, 7, 8]);

    let expected: Vec<_> = tga
        .bounding_box()
        .points()
        .zip((1..=8).chain(repeat(0)))
        .map(|(p, c)| RawPixel::new(p, c))
        .collect();

    let pixels: Vec<_> = tga.raw_pixels().collect();

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
