use crate::{
    geometry::Point,
    mock_display::{ColorMapping, MockDisplay},
    pixelcolor::{PixelColor, Rgb888, RgbColor},
    primitives::Rectangle,
};
use core::fmt::{self, Display, Write};

pub struct FancyPanic<'a, C>
where
    C: PixelColor + ColorMapping,
{
    display: FancyDisplay<'a, C>,
    expected: FancyDisplay<'a, C>,
}

impl<'a, C> FancyPanic<'a, C>
where
    C: PixelColor + ColorMapping,
{
    pub fn new(
        display: &'a MockDisplay<C>,
        expected: &'a MockDisplay<C>,
        max_column_width: usize,
    ) -> Self {
        let bounding_box_display = display.affected_area_origin();
        let bounding_box_expected = expected.affected_area_origin();

        let bounding_box = Rectangle::new(
            Point::zero(),
            bounding_box_display
                .size
                .component_max(bounding_box_expected.size),
        );

        // Output the 3 displays in columns if they are less than max_column_width pixels wide.
        let column_width = if bounding_box.size.width as usize <= max_column_width {
            // Set the width of the output columns to the width of the bounding box,
            // but at least 10 characters to ensure the column labels fit.
            (bounding_box.size.width as usize).max(10)
        } else {
            0
        };

        Self {
            display: FancyDisplay::new(display, bounding_box, column_width),
            expected: FancyDisplay::new(expected, bounding_box, column_width),
        }
    }

    fn write_vertical_border(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "+-{:-<width$}-+-{:-<width$}-+-{:-<width$}-+",
            "",
            "",
            "",
            width = self.display.column_width
        )
    }

    fn write_header(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "| {:^width$} | {:^width$} | {:^width$} |",
            "display",
            "expected",
            "diff",
            width = self.display.column_width
        )
    }

    fn write_footer(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "diff colors: {}\u{25FC}{} additional pixel",
            Ansi::Foreground(Some(Rgb888::GREEN)),
            Ansi::Foreground(None)
        )?;

        write!(
            f,
            ", {}\u{25FC}{} missing pixel",
            Ansi::Foreground(Some(Rgb888::RED)),
            Ansi::Foreground(None)
        )?;

        writeln!(
            f,
            ", {}\u{25FC}{} wrong color",
            Ansi::Foreground(Some(Rgb888::BLUE)),
            Ansi::Foreground(None)
        )
    }
}

impl<C> Display for FancyPanic<'_, C>
where
    C: PixelColor + ColorMapping,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let diff = self.display.display.diff(self.expected.display);
        let diff = FancyDisplay::new(&diff, self.display.bounding_box, self.display.column_width);

        // Output the 3 displays in columns if they are less than 30 pixels wide.
        if self.display.column_width > 0 {
            self.write_vertical_border(f)?;
            self.write_header(f)?;
            self.write_vertical_border(f)?;

            // Skip all odd y coordinates, because `write_row` outputs two rows of pixels.
            for y in self.display.bounding_box.rows().step_by(2) {
                f.write_str("| ")?;
                self.display.write_row(f, y)?;
                f.write_str(" | ")?;
                self.expected.write_row(f, y)?;
                f.write_str(" | ")?;
                diff.write_row(f, y)?;
                f.write_str(" |\n")?;
            }

            self.write_vertical_border(f)?;
        } else {
            let width = self.display.bounding_box.size.width as usize;

            write!(f, "display\n{:-<w$}\n{}", "", self.display, w = width)?;
            write!(f, "\nexpected\n{:-<w$}\n{}", "", self.expected, w = width)?;
            write!(f, "\ndiff\n{:-<width$}\n{}", "", diff, width = width)?;
        }
        self.write_footer(f)?;

        Ok(())
    }
}

struct FancyDisplay<'a, C>
where
    C: PixelColor + ColorMapping,
{
    display: &'a MockDisplay<C>,
    bounding_box: Rectangle,
    column_width: usize,
}

impl<'a, C> FancyDisplay<'a, C>
where
    C: PixelColor + ColorMapping,
{
    fn new(display: &'a MockDisplay<C>, bounding_box: Rectangle, column_width: usize) -> Self {
        Self {
            display,
            bounding_box,
            column_width,
        }
    }

    fn write_row(&self, f: &mut fmt::Formatter<'_>, y: i32) -> fmt::Result {
        for x in self.bounding_box.columns() {
            let point_top = Point::new(x, y);
            let point_bottom = Point::new(x, y + 1);

            let foreground = if self.bounding_box.contains(point_top) {
                self.display
                    .get_pixel(point_top)
                    .map(|c| Some(c.into()))
                    .unwrap_or(Some(C::NONE_COLOR))
            } else {
                None
            };

            let background = if self.bounding_box.contains(point_bottom) {
                self.display
                    .get_pixel(point_bottom)
                    .map(|c| Some(c.into()))
                    .unwrap_or(Some(C::NONE_COLOR))
            } else {
                None
            };

            // Write "upper half block" character.
            write!(
                f,
                "{}{}\u{2580}",
                Ansi::Foreground(foreground),
                Ansi::Background(background)
            )?;
        }

        // Reset colors.
        Ansi::Reset.fmt(f)?;

        // Pad output with spaces if column width is larger than the width of the bounding box.
        for _ in self.bounding_box.size.width as usize..self.column_width {
            f.write_char(' ')?
        }

        Ok(())
    }
}

impl<C> Display for FancyDisplay<'_, C>
where
    C: PixelColor + ColorMapping,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Skip all odd y coordinates, because `write_row` outputs two rows of pixels.
        for y in self.bounding_box.rows().step_by(2) {
            self.write_row(f, y)?;
            f.write_char('\n')?
        }

        Ok(())
    }
}

enum Ansi {
    Foreground(Option<Rgb888>),
    Background(Option<Rgb888>),
    Reset,
}

impl Display for Ansi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Foreground(Some(color)) => {
                write!(f, "\x1b[38;2;{};{};{}m", color.r(), color.g(), color.b())
            }
            Self::Foreground(None) => write!(f, "\x1b[39m"),
            Self::Background(Some(color)) => {
                write!(f, "\x1b[48;2;{};{};{}m", color.r(), color.g(), color.b())
            }
            Self::Background(None) => write!(f, "\x1b[49m"),
            Self::Reset => write!(f, "\x1b[0m"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::BinaryColor;

    #[test]
    fn fancy_panic_columns() {
        let display = MockDisplay::<BinaryColor>::from_pattern(&[
            "   ", //
            ".##", //
        ]);

        let expected = MockDisplay::<BinaryColor>::from_pattern(&[
            ".# ", //
            "  #", //
        ]);

        let mut out = arrayvec::ArrayString::<[_; 1024]>::new();
        write!(&mut out, "{}", FancyPanic::new(&display, &expected, 30)).unwrap();

        assert_eq!(&out, concat!(
            "+------------+------------+------------+\n",
            "|  display   |  expected  |    diff    |\n",
            "+------------+------------+------------+\n",
            "| \x1b[38;2;128;128;128m\x1b[48;2;0;0;0m▀\x1b[38;2;128;128;128m\x1b[48;2;255;255;255m▀\x1b[38;2;128;128;128m\x1b[48;2;255;255;255m▀\x1b[0m        ",
            "| \x1b[38;2;0;0;0m\x1b[48;2;128;128;128m▀\x1b[38;2;255;255;255m\x1b[48;2;128;128;128m▀\x1b[38;2;128;128;128m\x1b[48;2;255;255;255m▀\x1b[0m        ",
            "| \x1b[38;2;255;0;0m\x1b[48;2;0;255;0m▀\x1b[38;2;255;0;0m\x1b[48;2;0;255;0m▀\x1b[38;2;128;128;128m\x1b[48;2;128;128;128m▀\x1b[0m        |\n",
            "+------------+------------+------------+\n",
            "diff colors: \x1b[38;2;0;255;0m◼\x1b[39m additional pixel, \x1b[38;2;255;0;0m◼\x1b[39m missing pixel, \x1b[38;2;0;0;255m◼\x1b[39m wrong color\n",
        ));
    }

    #[test]
    fn fancy_panic_no_columns() {
        let display = MockDisplay::<BinaryColor>::from_pattern(&[
            "   ", //
            ".##", //
        ]);

        let expected = MockDisplay::<BinaryColor>::from_pattern(&[
            ".# ", //
            "  #", //
        ]);

        let mut out = arrayvec::ArrayString::<[_; 1024]>::new();
        write!(&mut out, "{}", FancyPanic::new(&display, &expected, 0)).unwrap();

        assert_eq!(&out, concat!(
            "display\n",
            "---\n",
            "\x1b[38;2;128;128;128m\x1b[48;2;0;0;0m▀\x1b[38;2;128;128;128m\x1b[48;2;255;255;255m▀\x1b[38;2;128;128;128m\x1b[48;2;255;255;255m▀\x1b[0m\n",
            "\n",
            "expected\n",
            "---\n",
            "\x1b[38;2;0;0;0m\x1b[48;2;128;128;128m▀\x1b[38;2;255;255;255m\x1b[48;2;128;128;128m▀\x1b[38;2;128;128;128m\x1b[48;2;255;255;255m▀\x1b[0m\n",
            "\n",
            "diff\n",
            "---\n",
            "\x1b[38;2;255;0;0m\x1b[48;2;0;255;0m▀\x1b[38;2;255;0;0m\x1b[48;2;0;255;0m▀\x1b[38;2;128;128;128m\x1b[48;2;128;128;128m▀\x1b[0m\n",
            "diff colors: \x1b[38;2;0;255;0m◼\x1b[39m additional pixel, \x1b[38;2;255;0;0m◼\x1b[39m missing pixel, \x1b[38;2;0;0;255m◼\x1b[39m wrong color\n",
        ));
    }
}
