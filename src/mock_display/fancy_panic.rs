use crate::{
    geometry::Point,
    mock_display::{ColorMapping, MockDisplay},
    pixelcolor::{PixelColor, RgbColor},
    primitives::Rectangle,
};
use core::fmt::{self, Write};

pub struct FancyPanic<'a, C>
where
    C: PixelColor,
{
    display: &'a MockDisplay<C>,
    expected: &'a MockDisplay<C>,
}

impl<'a, C> FancyPanic<'a, C>
where
    C: PixelColor,
{
    pub fn new(display: &'a MockDisplay<C>, expected: &'a MockDisplay<C>) -> Self {
        Self { display, expected }
    }
}

fn write_display<C>(
    f: &mut fmt::Formatter<'_>,
    display: &MockDisplay<C>,
    bounding_box: &Rectangle,
) -> fmt::Result
where
    C: PixelColor + ColorMapping,
{
    // Skip all odd y coordinates, because `write_row` outputs two rows of pixels.
    for y in bounding_box.rows().step_by(2) {
        write_row(f, display, bounding_box, y, 0)?;
        f.write_char('\n')?
    }

    Ok(())
}

fn write_row<C>(
    f: &mut fmt::Formatter<'_>,
    display: &MockDisplay<C>,
    bounding_box: &Rectangle,
    y: i32,
    column_width: usize,
) -> fmt::Result
where
    C: PixelColor + ColorMapping,
{
    for x in bounding_box.columns() {
        let point_top = Point::new(x, y);
        let point_bottom = Point::new(x, y + 1);

        // Set foreground color.
        if bounding_box.contains(point_top) {
            let color = display
                .get_pixel(point_top)
                .map(|c| c.into())
                .unwrap_or(C::NONE_COLOR);

            write!(f, "\x1b[38;2;{};{};{}m", color.r(), color.g(), color.b())?;
        } else {
            write!(f, "\x1b[39m")?;
        };

        // Set background color.
        if bounding_box.contains(point_bottom) {
            let color = display
                .get_pixel(point_bottom)
                .map(|c| c.into())
                .unwrap_or(C::NONE_COLOR);

            write!(f, "\x1b[48;2;{};{};{}m", color.r(), color.g(), color.b())?;
        } else {
            write!(f, "\x1b[49m")?;
        };

        // Write "upper half block" character.
        f.write_char('\u{2580}')?;
    }

    // Reset colors.
    f.write_str("\x1b[0;m")?;

    // Pad output with spaces if column width is larger than the width of the bounding box.
    for _ in bounding_box.size.width as usize..column_width {
        f.write_char(' ')?
    }

    Ok(())
}

fn write_vertical_border(f: &mut fmt::Formatter<'_>, column_width: usize) -> fmt::Result {
    write!(
        f,
        "+-{:-<width$}-+-{:-<width$}-+-{:-<width$}-+\n",
        "",
        "",
        "",
        width = column_width
    )
}

fn write_header(f: &mut fmt::Formatter<'_>, column_width: usize) -> fmt::Result {
    write_vertical_border(f, column_width)?;
    write!(
        f,
        "| {:^width$} | {:^width$} | {:^width$} |\n",
        "display",
        "expected",
        "diff",
        width = column_width
    )?;
    write_vertical_border(f, column_width)
}

impl<C> fmt::Display for FancyPanic<'_, C>
where
    C: PixelColor + ColorMapping,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let diff = self.display.diff(self.expected);

        let bounding_box_display = self.display.affected_area_origin();
        let bounding_box_expected = self.expected.affected_area_origin();

        let bounding_box = Rectangle::new(
            Point::zero(),
            bounding_box_display
                .size
                .component_max(bounding_box_expected.size),
        );

        f.write_char('\n')?;

        // Output the 3 displays in columns if they are less than 30 pixels wide.
        if bounding_box.size.width <= 30 {
            // Set the width of the output columns to the width of the bounding box,
            // but at least 10 characters to ensure the column labels fit.
            let column_width = (bounding_box.size.width as usize).max(10);

            write_header(f, column_width)?;

            // Skip all odd y coordinates, because `write_row` outputs two rows of pixels.
            for y in bounding_box.rows().step_by(2) {
                f.write_str("| ")?;
                write_row(f, self.display, &bounding_box, y, column_width)?;
                f.write_str(" | ")?;
                write_row(f, self.expected, &bounding_box, y, column_width)?;
                f.write_str(" | ")?;
                write_row(f, &diff, &bounding_box, y, column_width)?;
                f.write_str(" |\n")?;
            }

            write_vertical_border(f, column_width)?;
        } else {
            let width = bounding_box.size.width as usize;

            write!(f, "display\n{:-<width$}\n", "", width = width)?;
            write_display(f, self.display, &bounding_box)?;

            write!(f, "\nexpected\n{:-<width$}\n", "", width = width)?;
            write_display(f, &self.expected, &bounding_box)?;

            write!(f, "\ndiff\n{:-<width$}\n", "", width = width)?;
            write_display(f, &diff, &bounding_box)?;
        }

        Ok(())
    }
}
