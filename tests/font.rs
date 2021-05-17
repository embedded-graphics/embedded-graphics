use embedded_graphics::{
    image::ImageRaw,
    mock_display::MockDisplay,
    mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

const DATA: &[u8] = &[0xAA, 0x55];

#[test]
fn custom_font() {
    let mapping = StrGlyphMapping::new("01", 0);

    let font = MonoFont {
        image: ImageRaw::new_binary(DATA, 8),
        character_size: Size::new(2, 2),
        character_spacing: 0,
        baseline: 0,
        strikethrough: DecorationDimensions::default_strikethrough(2),
        underline: DecorationDimensions::default_underline(2),
        glyph_mapping: &mapping,
    };

    let character_style = MonoTextStyle::new(&font, BinaryColor::On);

    let mut display = MockDisplay::new();
    Text::with_baseline("01", Point::zero(), character_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    display.assert_pattern(&[
        "# # ", //
        " # #", //
    ]);
}
