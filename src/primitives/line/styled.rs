use crate::{
    draw_target::DrawTarget,
    geometry::{Point, PointExt, Size},
    pixelcolor::PixelColor,
    primitives::{
        line::{
            dotted_bresenham::DottedLinePoints,
            thick_points::{ThickPoints, HORIZONTAL_LINE},
            Line, StrokeOffset,
        },
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

/// Compute the translation needed so that the dotted line fits the line as well as possible.
/// The naive positioning of the dots might not be ideal for the following reasons :
///
/// 1. Considering that a dot is either a `Circle` or a squared `Rectangle`, the geometric
///    center of a dot can either be on a pixel center (when the dot size is odd),
///    or on a 4-pixel intersection (when the dot size is even).
///    In the second case, the "center point" used in e-g is actually the neighboring top-left
///    pixel, which causes the dot to be moved slightly to the bottom right.
///
/// 2. A line with an odd number of bresenham lines is symmetric. However when there is an
///    even number of bresenham lines, the line is slightly thicker on the left.
fn start_dot_offset(line: &Line, dot_size: i32) -> Point {
    if dot_size % 2 == 1 {
        // No translation is applied for dots of odd diameter
        // (their geometric center coincides with their "center point" as computed in e-g).
        return line.start;
    }

    // Dots of even diameter (with a geometric center on a 4-pixel intersection).
    //
    // A translation is applied to get the following result:
    // - on horizontal, vertical and other lines using the minimal (odd) number of bresenham lines,
    // the geometric center of the starting dot will lie on the left starting corner of the
    // bresenham line;
    // If possible, the translation as a function of the line orientation should change
    // when the number of bresenham lines changes.
    let mut start = line.start;
    let delta = if start != line.end {
        line.delta()
    } else {
        HORIZONTAL_LINE.delta()
    };
    if !ThickPoints::has_more_lines_than_expected(line, dot_size) {
        // Horizontal, vertical, and other lines that use the minimal (odd) number of bresenham lines.
        let to_bottom_right = delta.dot_product(Point::new(1, 1));
        let to_top_right = delta.dot_product(Point::new(1, -1));

        if to_bottom_right > 0 || to_bottom_right == 0 && to_top_right < 0 {
            start.y -= 1;
        };
        if to_top_right > 0 || to_top_right == 0 && to_bottom_right > 0 {
            start.x -= 1;
        };
        start
    } else {
        // Lines that have more bresenham lines than the minimum.
        // The same offset is used as on the previous horizontal/vertical line in clockwise order.
        let to_right = delta.dot_product(Point::new(1, 0));
        let to_bottom = delta.dot_product(Point::new(0, 1));

        if to_right > 0 || to_right == 0 && to_bottom < 0 {
            start.x -= 1;
        };
        if to_bottom > 0 || to_bottom == 0 && to_right > 0 {
            start.y -= 1;
        };
        start
    }
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
            // Improve the positioning of the dots.
            let start = start_dot_offset(self, dot_size);

            if dot_size > 3 {
                draw_dotted_line(
                    &Circle::with_center(start, dot_size as u32),
                    &dotted_line_points,
                    &dot_style,
                    target,
                )
            } else {
                draw_dotted_line(
                    &Rectangle::with_center(start, Size::new_equal(dot_size as u32)),
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
        pixelcolor::{BinaryColor, Gray2, Rgb888, RgbColor},
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

    #[test]
    /// When the dotted line is reduced to a point (it has the same `start` and `end`), then only
    /// one dot should be drawn, and it should fit the corresponding solid line (which is horizontal).
    fn null_line_is_correct() {
        let point = Point::new_equal(3);
        let null_line = Line::new(point, point);

        let expected_pattern_width_1 = [
            "         ",
            "         ",
            "         ",
            "   0     ",
            "         ",
            "         ",
        ];

        let expected_pattern_width_2 = [
            "         ",
            "         ",
            "  21     ",
            "  20     ",
            "         ",
            "         ",
        ];

        let expected_pattern_width_3 = [
            "         ",
            "         ",
            "  212    ",
            "  202    ",
            "  212    ",
            "         ",
        ];

        let expected_pattern_width_4 = [
            "         ",
            "  21     ",
            " 2212    ",
            " 2202    ",
            "  21     ",
            "         ",
        ];

        let width_and_result = [
            expected_pattern_width_1,
            expected_pattern_width_2,
            expected_pattern_width_3,
            expected_pattern_width_4,
        ];

        let solid_1px_style = PrimitiveStyle::with_stroke(Gray2::new(0x0), 1);

        for (width, expected_pattern) in width_and_result.iter().enumerate() {
            let solid_style = PrimitiveStyle::with_stroke(Gray2::new(0x1), (width + 1) as u32);
            let dotted_style = PrimitiveStyleBuilder::from(&solid_style)
                .stroke_color(Gray2::new(0x2))
                .stroke_style(StrokeStyle::Dotted)
                .build();

            let mut display = MockDisplay::new();

            null_line
                .into_styled(dotted_style)
                .draw(&mut display)
                .unwrap();
            // overdraw wasn't allowed up to this point, this proves only one dot was drawn
            display.set_allow_overdraw(true);
            null_line
                .into_styled(solid_style)
                .draw(&mut display)
                .unwrap();
            null_line
                .into_styled(solid_1px_style)
                .draw(&mut display)
                .unwrap();

            display.assert_pattern(expected_pattern);
        }
    }

    #[test]
    /// Lines with even width are thicker on the left, and dots with even diameter have
    /// an offset to the bottom right. The output of `start_dot_offset` should match the
    /// array `expected_corrections` on 8 major orientations.
    fn start_dot_offset_matches_expectations() {
        let origin = Point::zero();
        // 8 orientations in clockwise order (the first one is to the right).
        let orientations = [
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(0, 1),
            Point::new(-1, 1),
            Point::new(-1, 0),
            Point::new(-1, -1),
            Point::new(0, -1),
            Point::new(1, -1),
        ];
        let expected_corrections = [
            Point::new(-1, -1),
            Point::new(-1, -1),
            Point::new(0, -1),
            Point::new(0, -1),
            Point::new(0, 0),
            Point::new(0, 0),
            Point::new(-1, 0),
            Point::new(-1, 0),
        ];

        let lines = [
            Line::new(origin, orientations[0]),
            Line::new(origin, orientations[1]),
            Line::new(origin, orientations[2]),
            Line::new(origin, orientations[3]),
            Line::new(origin, orientations[4]),
            Line::new(origin, orientations[5]),
            Line::new(origin, orientations[6]),
            Line::new(origin, orientations[7]),
        ];

        for width in [2, 4] {
            assert_eq!(start_dot_offset(&lines[0], width), expected_corrections[0]);
            assert_eq!(start_dot_offset(&lines[1], width), expected_corrections[1]);
            assert_eq!(start_dot_offset(&lines[2], width), expected_corrections[2]);
            assert_eq!(start_dot_offset(&lines[3], width), expected_corrections[3]);
            assert_eq!(start_dot_offset(&lines[4], width), expected_corrections[4]);
            assert_eq!(start_dot_offset(&lines[5], width), expected_corrections[5]);
            assert_eq!(start_dot_offset(&lines[6], width), expected_corrections[6]);
            assert_eq!(start_dot_offset(&lines[7], width), expected_corrections[7]);
        }

        for width in [1, 3] {
            assert_eq!(start_dot_offset(&lines[0], width), Point::zero());
            assert_eq!(start_dot_offset(&lines[1], width), Point::zero());
            assert_eq!(start_dot_offset(&lines[2], width), Point::zero());
            assert_eq!(start_dot_offset(&lines[3], width), Point::zero());
            assert_eq!(start_dot_offset(&lines[4], width), Point::zero());
            assert_eq!(start_dot_offset(&lines[5], width), Point::zero());
            assert_eq!(start_dot_offset(&lines[6], width), Point::zero());
            assert_eq!(start_dot_offset(&lines[7], width), Point::zero());
        }
    }

    #[test]
    /// Lines with even width are thicker on the left, and dots with even diameter have
    /// an offset to the bottom right. start_dot_offset` should translate the dotted line
    /// so that it fits the solid line as well as possible.
    fn start_dot_offset_matches_drawing() {
        let origin = Point::new_equal(8);
        // 8 orientations in clockwise order (the first direction is to the right).
        let orientations = [
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(0, 1),
            Point::new(-1, 1),
            Point::new(-1, 0),
            Point::new(-1, -1),
            Point::new(0, -1),
            Point::new(1, -1),
        ];

        let lines = [
            Line::new(origin + orientations[0] * 4, origin + orientations[0] * 8),
            Line::new(origin + orientations[1] * 4, origin + orientations[1] * 8),
            Line::new(origin + orientations[2] * 4, origin + orientations[2] * 8),
            Line::new(origin + orientations[3] * 4, origin + orientations[3] * 8),
            Line::new(origin + orientations[4] * 4, origin + orientations[4] * 8),
            Line::new(origin + orientations[5] * 4, origin + orientations[5] * 8),
            Line::new(origin + orientations[6] * 4, origin + orientations[6] * 8),
            Line::new(origin + orientations[7] * 4, origin + orientations[7] * 8),
        ];

        let solid_fill_style = PrimitiveStyle::with_fill(Gray2::new(0x0));
        let solid_style = PrimitiveStyle::with_stroke(Gray2::new(0x1), 2);
        let dotted_style = PrimitiveStyleBuilder::from(&solid_style)
            .stroke_color(Gray2::new(0x2))
            .stroke_style(StrokeStyle::Dotted)
            .build();

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        for line in lines {
            line.into_styled(solid_style).draw(&mut display).unwrap();
            line.into_styled(dotted_style).draw(&mut display).unwrap();
            Rectangle::new(line.start, Size::new_equal(1))
                .into_styled(solid_fill_style)
                .draw(&mut display)
                .unwrap();

            Rectangle::new(line.end, Size::new_equal(1))
                .into_styled(solid_fill_style)
                .draw(&mut display)
                .unwrap();
        }

        display.assert_pattern(&[
            "02     20      20 ",
            "22     22     122 ",
            " 11    11    11   ",
            "  11   11   11    ",
            "   102 20  20     ",
            "    22 22  22     ",
            "                  ",
            "           221122 ",
            "021102     201120 ",
            "221122            ",
            "                  ",
            "    22  22 22     ",
            "    02  02 201    ",
            "   11   11   11   ",
            "  11    11    11  ",
            "221     22     22 ",
            "02      02     20 ",
        ]);
    }
}
