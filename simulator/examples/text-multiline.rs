use embedded_graphics::{
    egtext,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::{AlignH, AlignV, TextStyleBuilder},
    text_style,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(256, 512));

    // Show multiline text example
    let style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();

    Text::new("This is a\nmultiline\nHello World!", Point::new(15, 15))
        .into_styled(style)
        .draw(&mut display)?;

    // Show multiline text example using a macro
    egtext!(
        text = "This is a\nmultiline\nHello World!\nwith macro",
        top_left = (15, 64),
        style = text_style!(
            font = Font6x8,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off
        )
    )
    .draw(&mut display)?;

    // Show centered multiline text example
    let style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .horizontal_alignment(AlignH::CENTER)
        .build();

    Text::new(
        "This is a\ncentered\nmultiline\nHello World!",
        Point::new(15, 128),
    )
    .sized(Size::new(226, 64))
    .into_styled(style)
    .draw(&mut display)?;

    // Show centered multiline text example using a macro
    egtext!(
        text = "This is a\ncentered\nmultiline\nHello World!\nwith macro",
        top_left = (15, 192),
        size = (226, 64),
        style = text_style!(
            font = Font6x8,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off,
            horizontal_alignment = AlignH::CENTER
        )
    )
    .draw(&mut display)?;

    // Show right aligned multiline text example
    let style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .horizontal_alignment(AlignH::RIGHT)
        .build();

    Text::new(
        "This is a\nright aligned\nmultiline\nHello World!",
        Point::new(15, 256),
    )
    .sized(Size::new(226, 64))
    .into_styled(style)
    .draw(&mut display)?;

    // Show right aligned multiline text example using a macro
    egtext!(
        text = "This is a\nright aligned\nmultiline\nHello World!\nwith macro",
        top_left = (15, 320),
        size = (226, 64),
        style = text_style!(
            font = Font6x8,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off,
            horizontal_alignment = AlignH::RIGHT
        )
    )
    .draw(&mut display)?;

    // Show top aligned multiline text example
    let style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .vertical_alignment(AlignV::TOP)
        .build();

    Text::new(
        "This is a\ntop aligned\nmultiline\nHello World!",
        Point::new(15, 384),
    )
    .sized(Size::new(75, 64))
    .into_styled(style)
    .draw(&mut display)?;

    // Show top aligned multiline text example using a macro
    egtext!(
        text = "This is a\ntop aligned\nmultiline\nHello World!\nwith macro",
        top_left = (15, 448),
        size = (75, 64),
        style = text_style!(
            font = Font6x8,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off,
            vertical_alignment = AlignV::TOP
        )
    )
    .draw(&mut display)?;

    // Show centered multiline text example
    let style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .vertical_alignment(AlignV::CENTER)
        .build();

    Text::new(
        "This is a\ncentered\nmultiline\nHello World!",
        Point::new(90, 384),
    )
    .sized(Size::new(75, 64))
    .into_styled(style)
    .draw(&mut display)?;

    // Show centered multiline text example using a macro
    egtext!(
        text = "This is a\ncentered\nmultiline\nHello World!\nwith macro",
        top_left = (90, 448),
        size = (75, 64),
        style = text_style!(
            font = Font6x8,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off,
            vertical_alignment = AlignV::CENTER
        )
    )
    .draw(&mut display)?;

    // Show bottom aligned multiline text example
    let style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .vertical_alignment(AlignV::BOTTOM)
        .build();

    Text::new(
        "This is a\nbottom aligned\nmultiline\nHello World!",
        Point::new(165, 384),
    )
    .sized(Size::new(75, 64))
    .into_styled(style)
    .draw(&mut display)?;

    // Show bottom aligned multiline text example using a macro
    egtext!(
        text = "This is a\nbottom aligned\nmultiline\nHello World!\nwith macro",
        top_left = (165, 448),
        size = (75, 64),
        style = text_style!(
            font = Font6x8,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off,
            vertical_alignment = AlignV::BOTTOM
        )
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}
