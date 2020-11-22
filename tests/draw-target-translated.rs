use embedded_graphics::{
    draw_target::{DrawTarget, DrawTargetExt},
    geometry::{Dimensions, Point, Size},
    mock_display::MockDisplay,
    pixelcolor::BinaryColor,
    primitives::Rectangle,
    transform::Transform,
    Pixel,
};

#[test]
fn draw_iter() {
    let mut display = MockDisplay::new();

    let mut translated = display.translated(Point::new(2, 3));

    let pixels = [
        Pixel(Point::new(0, 0), BinaryColor::On),
        Pixel(Point::new(1, 2), BinaryColor::Off),
    ];
    translated.draw_iter(pixels.iter().copied()).unwrap();

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "    ", //
            "    ", //
            "    ", //
            "  # ", //
            "    ", //
            "   .", //
        ])
    );
}

#[test]
fn fill_contiguous() {
    let mut display = MockDisplay::new();

    let mut translated = display.translated(Point::new(3, 2));

    let colors = [
        1, 1, 1, 1, 1, //
        0, 0, 0, 0, 1, //
        0, 1, 0, 1, 1, //
        1, 0, 1, 0, 1, //
    ];
    let area = Rectangle::new(Point::new(1, 2), Size::new(5, 4));
    translated
        .fill_contiguous(&area, colors.iter().map(|c| BinaryColor::from(*c != 0)))
        .unwrap();

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "         ", //
            "         ", //
            "         ", //
            "         ", //
            "    #####", //
            "    ....#", //
            "    .#.##", //
            "    #.#.#", //
        ])
    );
}

#[test]
fn fill_solid() {
    let mut display = MockDisplay::new();

    let mut translated = display.translated(Point::new(1, 3));

    let area = Rectangle::new(Point::new(2, 1), Size::new(3, 4));
    translated.fill_solid(&area, BinaryColor::On).unwrap();

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "      ", //
            "      ", //
            "      ", //
            "      ", //
            "   ###", //
            "   ###", //
            "   ###", //
            "   ###", //
        ])
    );
}

#[test]
fn clear() {
    let mut display = MockDisplay::new();
    let mut translated = display.translated(Point::new(1, 3));
    translated.clear(BinaryColor::On).unwrap();

    let mut expected = MockDisplay::new();
    expected.clear(BinaryColor::On).unwrap();

    assert_eq!(display, expected);
}

#[test]
fn bounding_box() {
    let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
    let display_bb = display.bounding_box();

    let translated = display.translated(Point::new(1, 3));

    assert_eq!(
        display_bb.translate(-Point::new(1, 3)),
        translated.bounding_box()
    );
}
