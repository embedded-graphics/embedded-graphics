use embedded_graphics::{
    iterator::PixelIteratorExt, mock_display::MockDisplay, pixelcolor::BinaryColor, prelude::Point,
    Pixel,
};

#[test]
fn draw_pixel_iterator() {
    let pixels = [
        Pixel(Point::new(0, 0), BinaryColor::On),
        Pixel(Point::new(1, 0), BinaryColor::Off),
        Pixel(Point::new(2, 0), BinaryColor::On),
        Pixel(Point::new(2, 1), BinaryColor::Off),
    ];

    let mut display = MockDisplay::new();
    pixels.iter().copied().draw(&mut display).unwrap();

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "#.#", //
            "  .", //
        ])
    );
}
