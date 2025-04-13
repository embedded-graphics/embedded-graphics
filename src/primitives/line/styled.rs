use crate::{
    draw_target::DrawTarget,
    geometry::{PointExt, Size},
    pixelcolor::PixelColor,
    primitives::{
        line::{dotted_bresenham::DottedLinePoints, thick_points::ThickPoints, Line, StrokeOffset},
        styled::{StyledDimensions, StyledDrawable, StyledPixels},
        Circle, PrimitiveStyle, Rectangle, StrokeStyle,
    },
    transform::Transform,
    Pixel,
};
use az::SaturatingAs;
use integer_sqrt::IntegerSquareRoot;

/// Styled line iterator.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct StyledPixelsIterator<C> {
    stroke_color: Option<C>,
    line_iter: ThickPoints,
}

impl<C: PixelColor> StyledPixelsIterator<C> {
    pub(in crate::primitives::line) fn new(primitive: &Line, style: &PrimitiveStyle<C>) -> Self {
        // Note: stroke color will be None if stroke width is 0
        let stroke_color = style.effective_stroke_color();
        let stroke_width = style.stroke_width.saturating_as();

        Self {
            stroke_color,
            line_iter: ThickPoints::new(primitive, stroke_width),
        }
    }
}

impl<C: PixelColor> Iterator for StyledPixelsIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Return none if stroke color is none
        let stroke_color = self.stroke_color?;

        self.line_iter
            .next()
            .map(|point| Pixel(point, stroke_color))
    }
}

impl<C: PixelColor> StyledPixels<PrimitiveStyle<C>> for Line {
    type Iter = StyledPixelsIterator<C>;

    fn pixels(&self, style: &PrimitiveStyle<C>) -> Self::Iter {
        StyledPixelsIterator::new(self, style)
    }
}

fn draw_dotted_line<TS, D>(
    dot: &TS,
    dotted_line_points: &DottedLinePoints,
    dot_style: &PrimitiveStyle<D::Color>,
    target: &mut D,
) -> Result<(), D::Error>
where
    TS: Transform + StyledDrawable<PrimitiveStyle<D::Color>, Color = D::Color, Output = ()>,
    D: DrawTarget,
{
    for position in *dotted_line_points {
        dot.translate(position).draw_styled(dot_style, target)?;
    }
    Ok(())
}

impl<C: PixelColor> StyledDrawable<PrimitiveStyle<C>> for Line {
    type Color = C;
    type Output = ();

    fn draw_styled<D>(
        &self,
        style: &PrimitiveStyle<C>,
        target: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let dot_size = style.stroke_width as i32;
        if dot_size <= 0 {
            return Ok(());
        }

        let Some(stroke_color) = style.effective_stroke_color() else {
            return Ok(());
        };

        if style.stroke_style == StrokeStyle::Dotted && dot_size > 1 {
            // Draw dots along the line.
            let mut length = self.delta().length_squared().integer_sqrt();
            // The gaps between dots ideally have the same size as the dots
            // If `dot_size <= 3`, only positive error is allowed,
            // otherwise both positive and negative error are allowed.
            if dot_size > 3 {
                length += dot_size;
            }
            // The 2 endpoint dots take half the space of a regular dot.
            let nb_dots_desired = length / (2 * dot_size) + 1;
            let dotted_line_points =
                DottedLinePoints::new(&self.translate(-self.start), nb_dots_desired);
            let dot_style = PrimitiveStyle::with_fill(stroke_color);

            if dot_size > 3 {
                draw_dotted_line(
                    &Circle::with_center(self.start, dot_size as u32),
                    &dotted_line_points,
                    &dot_style,
                    target,
                )
            } else {
                draw_dotted_line(
                    &Rectangle::with_center(self.start, Size::new_equal(dot_size as u32)),
                    &dotted_line_points,
                    &dot_style,
                    target,
                )
            }
        } else if style.stroke_style == StrokeStyle::Dotted && style.stroke_width == 1 {
            target.draw_iter(StyledPixelsIterator::new(self, style).step_by(2))
        } else {
            target.draw_iter(StyledPixelsIterator::new(self, style))
        }
    }
}

impl<C: PixelColor> StyledDimensions<PrimitiveStyle<C>> for Line {
    fn styled_bounding_box(&self, style: &PrimitiveStyle<C>) -> Rectangle {
        let (l, r) = self.extents(style.stroke_width, StrokeOffset::None);

        let min = l
            .start
            .component_min(l.end)
            .component_min(r.start)
            .component_min(r.end);
        let max = l
            .start
            .component_max(l.end)
            .component_max(r.start)
            .component_max(r.end);

        Rectangle::with_corners(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Dimensions, Point},
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        primitives::{Primitive, PrimitiveStyleBuilder},
        Drawable,
    };

    #[test]
    fn bounding_box() {
        let lines = [
            (
                Line::new(Point::new(10, 20), Point::new(10, 50)),
                "vertical",
            ),
            (
                Line::new(Point::new(20, 20), Point::new(50, 20)),
                "horizontal",
            ),
            (
                Line::new(Point::new(20, 20), Point::new(55, 55)),
                "diagonal",
            ),
            (Line::new(Point::new(20, 20), Point::new(55, 55)), "thin"),
            (
                Line::new(Point::new(40, 40), Point::new(13, 14)),
                "random angle 1",
            ),
            (
                Line::new(Point::new(30, 30), Point::new(12, 53)),
                "random angle 2",
            ),
        ];

        for (line, name) in lines.iter() {
            for thickness in 1..15 {
                let style = PrimitiveStyle::with_stroke(Rgb888::RED, thickness);
                let styled = line.into_styled(style);

                let mut display = MockDisplay::new();
                styled.draw(&mut display).unwrap();
                assert_eq!(
                    display.affected_area(),
                    styled.bounding_box(),
                    "{}, {} px",
                    name,
                    thickness
                );
            }
        }
    }

    #[test]
    fn bounding_box_is_independent_of_colors() {
        let line = Line::new(Point::new(5, 5), Point::new(11, 14));

        let transparent_line = line.into_styled(
            PrimitiveStyleBuilder::<Rgb888>::new()
                .stroke_width(10)
                .build(),
        );
        let stroked_line = line.into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 10));

        assert_eq!(transparent_line.bounding_box(), stroked_line.bounding_box(),);
    }

    #[test]
    /// Dotted lines of width 1 are drawn by skipping one out of two pixels.
    /// The first pixel is always drawn (the last pixel is skipped if
    /// and only if the line has an even number of pixels).
    fn dotted_line_by_skipping_pixels() {
        let solid_style = PrimitiveStyle::with_stroke(BinaryColor::Off, 1);
        let dotted_style = PrimitiveStyleBuilder::from(&solid_style)
            .stroke_style(StrokeStyle::Dotted)
            .stroke_color(BinaryColor::On)
            .build();

        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);

        Line::new(Point::new(2, 2), Point::new(20, 8))
            .into_styled(solid_style)
            .draw(&mut display)
            .unwrap();
        Line::new(Point::new(2, 2), Point::new(20, 8))
            .into_styled(dotted_style)
            .draw(&mut display)
            .unwrap();

        // Drawing a 1px line to check the pixel is drawn
        Line::new(Point::new(8, 8), Point::new(8, 8))
            .into_styled(dotted_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                       ",
            "                       ",
            "  #.                   ",
            "    #.#                ",
            "       .#.             ",
            "          #.#          ",
            "             .#.       ",
            "                #.#    ",
            "        #          .#  ",
            "                       ",
            "                       ",
        ]);

        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);

        Line::new(Point::new(2, 2), Point::new(19, 8))
            .into_styled(solid_style)
            .draw(&mut display)
            .unwrap();
        Line::new(Point::new(2, 2), Point::new(19, 8))
            .into_styled(dotted_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                       ",
            "                       ",
            "  #.                   ",
            "    #.#                ",
            "       .#.             ",
            "          #.           ",
            "            #.#        ",
            "               .#.     ",
            "                  #.   ",
            "                       ",
            "                       ",
        ]);
    }

    #[test]
    fn dotted_line_using_dotted_bresenham() {
        let solid_style = PrimitiveStyle::with_stroke(BinaryColor::Off, 3);
        let dotted_style = PrimitiveStyleBuilder::from(&solid_style)
            .stroke_style(StrokeStyle::Dotted)
            .stroke_color(BinaryColor::On)
            .build();

        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);

        Line::new(Point::new(2, 2), Point::new(20, 8))
            .into_styled(solid_style)
            .draw(&mut display)
            .unwrap();
        Line::new(Point::new(2, 2), Point::new(20, 8))
            .into_styled(dotted_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                       ",
            " ###                   ",
            " ###...                ",
            " ###...###             ",
            "    ...###...          ",
            "       ###...###       ",
            "          ...###...    ",
            "             ###...### ",
            "                ...### ",
            "                   ### ",
        ]);
    }
}
