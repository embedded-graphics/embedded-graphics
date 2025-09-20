use crate::{
    draw_target::DrawTarget,
    geometry::{Point, PointExt, Size},
    pixelcolor::PixelColor,
    primitives::{
        line::{
            dotted_bresenham::DottedLinePoints,
            thick_points::{ThickPoints, HORIZONTAL_LINE},
            Line, Points, StrokeOffset,
        },
        styled::{StyledDimensions, StyledDrawable, StyledPixels},
        Circle, PrimitiveStyle, Rectangle, StrokeStyle,
    },
    transform::Transform,
    Pixel,
};
use az::SaturatingAs;

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
///
/// Rasterization of lines and dots causes positioning errors in the following situations :
/// - when the dot size is even, the rasterized dot is shifted to the bottom right by 0.5px;
/// - when the line has an even number of bresenham lines, the line is shifted by 0.5px
///   to the left.
///
/// When both sources of error add up, the naive dot positioning can be improved
/// (by translating the dots to the top left).
fn start_dot_offset(line: &Line, dot_size: i32) -> Point {
    if dot_size % 2 == 1 {
        // The dot size is odd so the positioning is already ideal.
        return Point::zero();
    }

    // Depending on the line orientation, the number of bresenham lines might be odd or even.
    // The code below supposes that nearly horizontal/vertical lines are thicker on the left and
    // that diagonals are symmetric. This is an approximation (there can be diagonal lines with
    // an even number of bresenham lines).
    //
    // The code also makes the choice that the geometric center of the first dot should lie
    // on the edge of the rasterized line.
    //
    // There's a drawing of the resulting alignment in test `start_dot_offset_matches_drawing`.
    let mut start = Point::zero();
    let delta = if line.start != line.end {
        line.delta()
    } else {
        HORIZONTAL_LINE.delta()
    };
    if is_nearly_horizontal_or_vertical(line) {
        let to_bottom_right = delta.dot_product(Point::new(1, 1));
        let to_top_right = delta.dot_product(Point::new(1, -1));

        if to_bottom_right > 0 {
            start.y -= 1;
        };
        if to_top_right > 0 {
            start.x -= 1;
        };
        start
    } else {
        let to_right = delta.dot_product(Point::new(1, 0));
        let to_bottom = delta.dot_product(Point::new(0, 1));

        if to_right > 0 {
            start.x -= 1;
        };
        if to_bottom > 0 {
            start.y -= 1;
        };
        start
    }
}

fn is_nearly_horizontal_or_vertical(line: &Line) -> bool {
    let mut points = Points::new(line);

    let Some(second) = points.nth(1) else {
        // The orientation of the degenerated line is horizontal.
        return true;
    };

    let diff = second - line.start;
    let sum = diff.abs().dot_product(Point::new(1, 1));

    // If the first two pixels share an edge, the line is nearly horizontal or vertical.
    sum == 1
}

fn extend_line_by_one_unit(line: &Line) -> Line {
    let mut points = Points::new(line);

    let Some(second) = points.nth(1) else {
        // If there is only one point in the iterator, the line is expected to be reduced to a point.
        // In that case we don't extend the line.
        return *line;
    };

    Line::new(line.start, line.end + second - line.start)
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

        // Try hardware acceleration for solid lines only (technically
        // only 1pix wide lines are ok, but the HAL will reject fat lines).
        if style.stroke_style == StrokeStyle::Solid {
            if let Some(result) = target.draw_line_solid(
                self.start.x,
                self.start.y,
                self.end.x,
                self.end.y,
                style.stroke_width,
                stroke_color
            ) {
                return result;
            }
            // Acceleration not available: fall through to software implementation
        }

        if style.stroke_style == StrokeStyle::Dotted && dot_size > 1 {
            let line = if dot_size % 2 == 0 {
                // When drawing a dotted rectangle border, the distance between the endpoint dots
                // is longer by one pixel when the dot size is even.
                // So that the dotted rectangle border matches 4 lines drawn in clockwise order,
                // the line is extended by one pixel when the dot size is even.
                let extended = extend_line_by_one_unit(self);
                // Improve the positioning of the dots.
                extended.translate(start_dot_offset(self, dot_size))
            } else {
                *self
            };

            // Draw dots along the line.
            let dotted_line_points = DottedLinePoints::with_dot_size(&line, dot_size);
            let dot_style = PrimitiveStyle::with_fill(stroke_color);

            if dot_size > 3 {
                draw_dotted_line(
                    &Circle::with_center(Point::zero(), dot_size as u32),
                    &dotted_line_points,
                    &dot_style,
                    target,
                )
            } else {
                draw_dotted_line(
                    &Rectangle::with_center(Point::zero(), Size::new_equal(dot_size as u32)),
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
        if style.stroke_style == StrokeStyle::Dotted {
            Rectangle::with_corners(self.start, self.end).offset((style.stroke_width / 2) as i32)
        } else {
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
                    "{name}, {thickness} px"
                );
            }
        }
    }

    #[test]
    fn bounding_box_for_dotted_line() {
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

        let dotted_style =
            PrimitiveStyleBuilder::from(&PrimitiveStyle::with_stroke(Rgb888::RED, 0))
                .stroke_style(StrokeStyle::Dotted);

        for (line, name) in lines.iter() {
            for thickness in 1..15 {
                let style = dotted_style.stroke_width(thickness).build();
                let styled = line.into_styled(style);
                let bounding_box = styled.bounding_box();

                let mut display = MockDisplay::new();
                styled.draw(&mut display).unwrap();
                let affected_area = display.affected_area();

                // Check that the affected area is contained in the bounding box.
                let intersection = affected_area.intersection(&bounding_box);
                assert_eq!(affected_area, intersection, "{name}, {thickness} px");

                // Check that the bounding box area is only slightly larger than the affected area.
                let intersection = affected_area.offset(1).intersection(&bounding_box);
                assert_eq!(bounding_box, intersection, "{name}, {thickness} px");
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
            Line::new(origin + orientations[0] * 4, origin + orientations[0] * 7),
            Line::new(origin + orientations[1] * 4, origin + orientations[1] * 7),
            Line::new(origin + orientations[2] * 4, origin + orientations[2] * 7),
            Line::new(origin + orientations[3] * 4, origin + orientations[3] * 7),
            Line::new(origin + orientations[4] * 4, origin + orientations[4] * 7),
            Line::new(origin + orientations[5] * 4, origin + orientations[5] * 7),
            Line::new(origin + orientations[6] * 4, origin + orientations[6] * 7),
            Line::new(origin + orientations[7] * 4, origin + orientations[7] * 7),
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
            "22     22      22 ",
            "20     20     102 ",
            " 11    11    11   ",
            "  11   11   11    ",
            "   102 20  20     ",
            "    22 22  22     ",
            "                  ",
            "           221122 ",
            "201102     201102 ",
            "221122            ",
            "                  ",
            "    22  22 22     ",
            "    02  02 201    ",
            "   11   11   11   ",
            "  11    11    11  ",
            "201     02     02 ",
            "22      22     22 ",
        ]);
    }

    #[test]
    /// A dotted rectangle border should exactly match the 4 dotted lines drawn in clockwise order when:
    /// 1. the border width is greater or equal to 4 (when the width is smaller,
    ///    there's not always a dot in each corner)
    ///    AND
    ///    the opposite borders of the rectangle don't overlap (this causes the size of the dots
    ///    to be reduced, so line dots and rectangle dots will have different sizes).
    ///
    /// A dotted rectangle border should exactly match the 2 first dotted lines drawn in clockwise order when:
    /// 2. the rectangle `stroke_area` is flattened to a line and the stroke width is 1.
    fn dotted_lines_match_dotted_rectangle_border() {
        let base_style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_style(StrokeStyle::Dotted);

        // see 1.
        let topleft = [Point::new(5, 6), Point::new(22, 8), Point::new(4, 12)];
        // For the lines to be drawn in clockwise order, `bottomright` coordinates must be greater than `topleft`.
        let bottomright = [Point::new(31, 23), Point::new(46, 36), Point::new(27, 33)];
        let stroke_width = [5, 6, 4];

        for i in 0..3 {
            let rect = Rectangle::with_corners(topleft[i], bottomright[i]);
            let topright = Point::new(bottomright[i].x, topleft[i].y);
            let bottomleft = Point::new(topleft[i].x, bottomright[i].y);
            let lines = [
                Line::new(topleft[i], topright),
                Line::new(topright, bottomright[i]),
                Line::new(bottomright[i], bottomleft),
                Line::new(bottomleft, topleft[i]),
            ];

            let mut lines_display = MockDisplay::new();
            lines_display.set_allow_overdraw(true);
            let mut rect_display = MockDisplay::new();

            rect.into_styled(base_style.stroke_width(stroke_width[i]).build())
                .draw(&mut rect_display)
                .unwrap();
            for line in lines {
                line.into_styled(base_style.stroke_width(stroke_width[i]).build())
                    .draw(&mut lines_display)
                    .unwrap();
            }

            assert_eq!(lines_display, rect_display);
        }

        // see 2.
        let topleft = [Point::new(2, 6), Point::new(2, 6)];
        let bottomright = [Point::new(16, 6), Point::new(2, 19)];
        let stroke_width = [1, 1];

        for i in 0..2 {
            let rect = Rectangle::with_corners(topleft[i], bottomright[i]);
            let topright = Point::new(bottomright[i].x, topleft[i].y);
            let lines = [
                Line::new(topleft[i], topright),
                Line::new(topright, bottomright[i]),
            ];

            let mut lines_display = MockDisplay::new();
            lines_display.set_allow_overdraw(true);
            let mut rect_display = MockDisplay::new();

            rect.into_styled(base_style.stroke_width(stroke_width[i]).build())
                .draw(&mut rect_display)
                .unwrap();
            for line in lines {
                line.into_styled(base_style.stroke_width(stroke_width[i]).build())
                    .draw(&mut lines_display)
                    .unwrap();
            }

            assert_eq!(lines_display, rect_display);
        }
    }
}
