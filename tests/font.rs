use embedded_graphics::{
    image::ImageRaw,
    mock_display::MockDisplay,
    mono_font::{GlyphIndices, GlyphRange, MonoFontBuilder, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

const DATA: &[u8] = &[0xAA, 0x55];

#[test]
fn custom_font() {
    let ranges = [GlyphRange::new('0', '1', 0)];

    let font = MonoFontBuilder::new()
        .image(ImageRaw::new_binary(DATA, 8))
        .character_size(Size::new(2, 2))
        .glyph_indices(GlyphIndices::new(&ranges, 0))
        .build();

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
