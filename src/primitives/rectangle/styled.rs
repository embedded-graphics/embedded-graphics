use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Real, Size},
    pixelcolor::PixelColor,
    primitives::{
        primitive_style::StrokeStyle,
        rectangle::{Points, Rectangle},
        styled::{StyledDimensions, StyledDrawable, StyledPixels},
        Circle, PointsIter, PrimitiveStyle,
    },
    transform::Transform,
    Pixel,
};
use az::SaturatingAs;

/// Pixel iterator for each pixel in the rect border
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct StyledPixelsIterator<C> {
    iter: Points,

    stroke_color: Option<C>,

    fill_area: Rectangle,
    fill_color: Option<C>,
}

impl<C: PixelColor> StyledPixelsIterator<C> {
    pub(in crate::primitives) fn new(primitive: &Rectangle, style: &PrimitiveStyle<C>) -> Self {
        let iter = if !style.is_transparent() {
            style.stroke_area(primitive).points()
        } else {
            Points::empty()
        };

        Self {
            iter,
            fill_area: style.fill_area(primitive),
            stroke_color: style.stroke_color,
            fill_color: style.fill_color,
        }
    }
}

impl<C: PixelColor> Iterator for StyledPixelsIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        for point in &mut self.iter {
            let color = if self.fill_area.contains(point) {
                self.fill_color
            } else {
                self.stroke_color
            };

            if let Some(color) = color {
                return Some(Pixel(point, color));
            }
        }

        None
    }
}

impl<C: PixelColor> StyledPixels<PrimitiveStyle<C>> for Rectangle {
    type Iter = StyledPixelsIterator<C>;

    fn pixels(&self, style: &PrimitiveStyle<C>) -> Self::Iter {
        Self::Iter::new(self, style)
    }
}

/// Compute dot positions from a `length` and `dot_size`.
///
/// A dot will be positioned at each endpoint (except in cases described below).
/// These 2 endpoints can be either included or excluded from the resulting iterator.
///
/// If `dot_size` is 0 or greater than `length`:
/// - and `include_corners` is true, an iterator containing 0 is returned;
/// - and `include_corners` is false, an empty iterator is returned.
fn dot_positions_with_dotted_corners(
    length: u32,
    dot_size: u32,
    include_corners: bool,
) -> impl Iterator<Item = i32> {
    // gaps can have negative or positive error
    let nb_dots = (length + dot_size)
        .checked_div(2 * dot_size)
        .unwrap_or_default();
    let dot_offset = if nb_dots != 0 {
        Real::from(length) / Real::from(nb_dots)
    } else {
        Real::from(0)
    };

    let idx_iter = if include_corners {
        0..=nb_dots
    } else {
        1..=nb_dots.saturating_sub(1)
    };

    idx_iter.map(move |idx| (dot_offset * Real::from(idx)).round().into())
}

/// Compute dot and gap positions from a `length` and `dot_size`.
///
/// A dot or a gap can be positioned at each endpoint. The starting endpoint
/// is included in the resulting iterator but not the ending endpoint.
///
/// If `dot_size` is 0 or greater than `length`, an empty iterator is returned.
fn unit_positions_in_clockwise_order(length: u32, dot_size: u32) -> impl Iterator<Item = i32> {
    // units can only have positive error
    let nb_units = length.checked_div(dot_size).unwrap_or_default();
    let unit_offset = if nb_units != 0 {
        Real::from(length) / Real::from(nb_units)
    } else {
        Real::from(0) // this value won't be used
    };

    let idx_iter = 0..nb_units;

    idx_iter.map(move |idx| (unit_offset * Real::from(idx)).round().into())
}

/// Draw a dotted rectangular border with dots in the 4 corners.
///
/// The gaps between dots ideally have the same size as the dots.
/// The gaps can be smaller or larger than ideal.
/// Opposite borders are identical (horizontal and vertical sides are independent).
fn draw_dotted_rectangle_border_with_dotted_corners<D>(
    top_left: &Point,
    border_size: &Size,
    dot_size: u32,
    style: &PrimitiveStyle<D::Color>,
    target: &mut D,
) -> Result<(), D::Error>
where
    D: DrawTarget,
{
    let top_left_dot = Circle::new(*top_left, dot_size);

    // Draw horizontal sides (including corner dots)
    for x in dot_positions_with_dotted_corners(border_size.width, dot_size, true) {
        top_left_dot
            .translate(Point::new(x, 0))
            .draw_styled(style, target)?;
        top_left_dot
            .translate(Point::new(x, 0) + border_size.y_axis())
            .draw_styled(style, target)?;
    }

    // Draw vertical sides (without corner dots)
    for y in dot_positions_with_dotted_corners(border_size.height, dot_size, false) {
        top_left_dot
            .translate(Point::new(0, y))
            .draw_styled(style, target)?;
        top_left_dot
            .translate(Point::new(0, y) + border_size.x_axis())
            .draw_styled(style, target)?;
    }

    Ok(())
}

/// Draw a dotted rectangular border.
///
/// The dot type is [`Rectangle`] (this method is meant to be used with smaller values of `dot_size`).
/// The gaps between dots ideally have the same size as the dots.
/// The gaps can be larger than ideal, but not smaller.
/// A corner can be filled either by a dot or a gap (sides are drawn in clockwise order).
fn draw_dotted_rectangle_border_in_clockwise_order<D>(
    top_left: &Point,
    border_sides: &[Point],
    dot_size: u32,
    style: &PrimitiveStyle<D::Color>,
    target: &mut D,
) -> Result<(), D::Error>
where
    D: DrawTarget,
{
    let mut corner_dot = Rectangle::new(*top_left, Size::new_equal(dot_size));
    let mut unit_is_dot = true;

    for (side_idx, side) in border_sides.iter().enumerate() {
        let length = side[side_idx % 2].unsigned_abs();

        for offset in unit_positions_in_clockwise_order(length, dot_size) {
            if unit_is_dot {
                let translation = Point::new(side.x.signum(), side.y.signum()) * offset;
                corner_dot
                    .translate(translation)
                    .draw_styled(style, target)?;
            };
            unit_is_dot = !unit_is_dot; // alternating dots and gaps
        }
        corner_dot.translate_mut(*side);
    }

    Ok(())
}

impl<C: PixelColor> StyledDrawable<PrimitiveStyle<C>> for Rectangle {
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
        let fill_area = style.fill_area(self);

        // Fill rectangle
        if let Some(fill_color) = style.fill_color {
            target.fill_solid(&fill_area, fill_color)?;
        }

        // Draw stroke
        let Some(stroke_color) = style.effective_stroke_color() else {
            return Ok(());
        };
        let stroke_width = style.stroke_width;
        let stroke_area = style.stroke_area(self);

        if style.stroke_style == StrokeStyle::Dotted {
            let dot_size = stroke_width
                .min(stroke_area.size.height / 2)
                .min(stroke_area.size.width / 2);
            if dot_size == 0 {
                return Ok(());
            }

            let border_size = stroke_area.size.saturating_sub(Size::new_equal(dot_size));
            let dot_style = PrimitiveStyle::with_fill(stroke_color);

            if dot_size < 4 {
                let mut border_sides: [Point; 4] = [Point::zero(); 4];
                border_sides[0] += border_size.x_axis();
                border_sides[1] += border_size.y_axis();
                border_sides[2] -= border_size.x_axis();
                border_sides[3] -= border_size.y_axis();

                draw_dotted_rectangle_border_in_clockwise_order(
                    &stroke_area.top_left,
                    &border_sides,
                    dot_size,
                    &dot_style,
                    target,
                )?
            } else {
                draw_dotted_rectangle_border_with_dotted_corners(
                    &stroke_area.top_left,
                    &border_size,
                    dot_size,
                    &dot_style,
                    target,
                )?
            }
        } else {
            let top_border = Rectangle::new(
                stroke_area.top_left,
                Size::new(
                    stroke_area.size.width,
                    stroke_width.min(stroke_area.size.height / 2),
                ),
            );

            let bottom_stroke_width =
                stroke_width.min(stroke_area.size.height - top_border.size.height);

            let bottom_border = Rectangle::new(
                top_border.top_left
                    + Size::new(
                        0,
                        stroke_area.size.height.saturating_sub(bottom_stroke_width),
                    ),
                Size::new(stroke_area.size.width, bottom_stroke_width),
            );

            target.fill_solid(&top_border, stroke_color)?;
            target.fill_solid(&bottom_border, stroke_color)?;

            if fill_area.size.height > 0 {
                let left_border = Rectangle::new(
                    stroke_area.top_left + top_border.size.y_axis(),
                    Size::new(
                        (stroke_width * 2).min(stroke_area.size.width + 1) / 2,
                        fill_area.size.height,
                    ),
                );

                let right_border = left_border.translate(Point::new(
                    stroke_area
                        .size
                        .width
                        .saturating_sub(left_border.size.width) as i32,
                    0,
                ));

                target.fill_solid(&left_border, stroke_color)?;
                target.fill_solid(&right_border, stroke_color)?;
            }
        }

        Ok(())
    }
}

impl<C: PixelColor> StyledDimensions<PrimitiveStyle<C>> for Rectangle {
    fn styled_bounding_box(&self, style: &PrimitiveStyle<C>) -> Rectangle {
        let offset = style.outside_stroke_width().saturating_as();

        self.bounding_box().offset(offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Point, Size},
        iterator::PixelIteratorExt,
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb565, RgbColor},
        primitives::{Primitive, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
        Drawable,
    };

    #[test]
    fn it_draws_unfilled_rect() {
        let mut rect = Rectangle::new(Point::new(2, 2), Size::new(3, 3))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
            .pixels();

        assert_eq!(rect.next(), Some(Pixel(Point::new(2, 2), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(3, 2), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(4, 2), Rgb565::RED)));

        assert_eq!(rect.next(), Some(Pixel(Point::new(2, 3), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(4, 3), Rgb565::RED)));

        assert_eq!(rect.next(), Some(Pixel(Point::new(2, 4), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(3, 4), Rgb565::RED)));
        assert_eq!(rect.next(), Some(Pixel(Point::new(4, 4), Rgb565::RED)));
    }

    #[test]
    fn points_iter_matches_filled_styled() {
        let rectangle = Rectangle::new(Point::new(10, 10), Size::new(20, 30));

        let styled_points = rectangle
            .clone()
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .pixels()
            .map(|Pixel(p, _)| p);

        assert!(rectangle.points().eq(styled_points));
    }

    #[test]
    fn stroke_alignment() {
        const TOP_LEFT: Point = Point::new(5, 6);
        const SIZE: Size = Size::new(10, 5);

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        Rectangle::new(TOP_LEFT, SIZE)
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        Rectangle::new(TOP_LEFT - Point::new(1, 1), SIZE + Size::new(2, 2))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        Rectangle::new(TOP_LEFT + Point::new(2, 2), SIZE - Size::new(4, 4))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Outside)
                    .build(),
            )
            .draw(&mut display_outside)
            .unwrap();

        display_inside.assert_eq(&display_center);
        display_outside.assert_eq(&display_center);
    }

    #[test]
    fn stroke_iter_vs_draw() {
        const TOP_LEFT: Point = Point::new(5, 6);
        const SIZE: Size = Size::new(10, 5);

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let rectangle_center = Rectangle::new(TOP_LEFT, SIZE).into_styled(style);

        let mut drawn_center = MockDisplay::new();
        let mut iter_center = MockDisplay::new();
        rectangle_center.draw(&mut drawn_center).unwrap();
        rectangle_center.pixels().draw(&mut iter_center).unwrap();
        drawn_center.assert_eq(&iter_center);

        let rectangle_inside = Rectangle::new(TOP_LEFT - Point::new(1, 1), SIZE + Size::new(2, 2))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            );

        let mut drawn_inside = MockDisplay::new();
        let mut iter_inside = MockDisplay::new();
        rectangle_inside.draw(&mut drawn_inside).unwrap();
        rectangle_inside.pixels().draw(&mut iter_inside).unwrap();
        drawn_inside.assert_eq(&iter_inside);

        let rectangle_outside = Rectangle::new(TOP_LEFT + Point::new(2, 2), SIZE - Size::new(4, 4))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Outside)
                    .build(),
            );

        let mut drawn_outside = MockDisplay::new();
        let mut iter_outside = MockDisplay::new();
        rectangle_outside.draw(&mut drawn_outside).unwrap();
        rectangle_outside.pixels().draw(&mut iter_outside).unwrap();
        drawn_outside.assert_eq(&iter_outside);
    }

    #[test]
    fn fill_iter_vs_draw() {
        const TOP_LEFT: Point = Point::new(5, 6);
        const SIZE: Size = Size::new(10, 5);

        let style = PrimitiveStyle::with_fill(BinaryColor::On);

        let rectangle = Rectangle::new(TOP_LEFT, SIZE).into_styled(style);

        let mut drawn = MockDisplay::new();
        let mut iter = MockDisplay::new();
        rectangle.draw(&mut drawn).unwrap();
        rectangle.pixels().draw(&mut iter).unwrap();
        drawn.assert_eq(&iter);
    }

    /// Compare the output of the draw() call vs iterators across multiple styles and stroke
    /// alignments.
    fn compare_drawable_iter(rect: Rectangle) {
        let thin_stroke = PrimitiveStyle::with_stroke(Rgb565::RED, 1);
        let stroke = PrimitiveStyle::with_stroke(Rgb565::RED, 5);
        let stroke_fill = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::RED)
            .stroke_width(5)
            .fill_color(Rgb565::GREEN)
            .build();
        let fill = PrimitiveStyle::with_fill(Rgb565::BLUE);

        for (name, style) in [
            ("thin_stroke", thin_stroke),
            ("stroke", stroke),
            ("stroke_fill", stroke_fill),
            ("fill", fill),
        ]
        .iter()
        {
            for alignment in [
                StrokeAlignment::Center,
                StrokeAlignment::Inside,
                StrokeAlignment::Outside,
            ]
            .iter()
            {
                let style = PrimitiveStyleBuilder::from(style)
                    .stroke_alignment(*alignment)
                    .build();

                let mut display_drawable = MockDisplay::new();
                let mut display_iter = MockDisplay::new();

                // Calls draw() impl above using fill_solid()
                rect.into_styled(style).draw(&mut display_drawable).unwrap();

                // Calls draw_iter()
                rect.into_styled(style)
                    .pixels()
                    .draw(&mut display_iter)
                    .unwrap();

                display_drawable.assert_eq_with_message(
                    &display_iter,
                    |f| write!(f,
                        "{} x {} rectangle with style '{}' and alignment {:?} does not match iterator",
                        rect.size.width, rect.size.height, name, alignment
                    )
                );
            }
        }
    }

    #[test]
    fn drawable_vs_iterator() {
        compare_drawable_iter(Rectangle::new(Point::new(10, 20), Size::new(20, 30)))
    }

    #[test]
    fn drawable_vs_iterator_squares() {
        for i in 0..20 {
            compare_drawable_iter(Rectangle::new(Point::new(7, 7), Size::new_equal(i)))
        }
    }

    #[test]
    fn reuse() {
        let rectangle = Rectangle::new(Point::zero(), Size::new_equal(10));

        let styled = rectangle.into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        let _pixels = styled.pixels();

        let moved = rectangle.translate(Point::new(1, 2));

        assert_eq!(moved, Rectangle::new(Point::new(1, 2), Size::new_equal(10)));
    }

    #[test]
    fn bounding_box() {
        let rectangle = Rectangle::new(Point::new(10, 10), Size::new(15, 20));

        let base = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(5);

        let center = rectangle.into_styled(base.stroke_alignment(StrokeAlignment::Center).build());
        let inside = rectangle.into_styled(base.stroke_alignment(StrokeAlignment::Inside).build());
        let outside =
            rectangle.into_styled(base.stroke_alignment(StrokeAlignment::Outside).build());

        let mut display = MockDisplay::new();
        center.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), center.bounding_box());
        let mut display = MockDisplay::new();
        inside.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), inside.bounding_box());
        let mut display = MockDisplay::new();
        outside.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), outside.bounding_box());
    }

    #[test]
    fn bounding_box_is_independent_of_colors() {
        let rect = Rectangle::new(Point::new(5, 5), Size::new(11, 14));

        let transparent_rect = rect.into_styled(PrimitiveStyle::<BinaryColor>::new());
        let filled_rect = rect.into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert_eq!(transparent_rect.bounding_box(), filled_rect.bounding_box(),);
    }

    #[test]
    fn dotted_border_is_inside_the_regular_border() {
        let base = PrimitiveStyleBuilder::from(&PrimitiveStyle::with_stroke(BinaryColor::On, 5));

        let rectangles = [
            Rectangle::new(Point::new(5, 6), Size::new(40, 3)),
            Rectangle::new(Point::new(4, 5), Size::new(40, 39)),
        ];

        for rect in &rectangles {
            let mut regular = MockDisplay::new();
            let mut dotted = MockDisplay::new();

            rect.into_styled(base.build()).draw(&mut regular).unwrap();
            rect.into_styled(base.stroke_style(StrokeStyle::Dotted).build())
                .draw(&mut dotted)
                .unwrap();

            for p in regular.bounding_box().points() {
                if dotted.get_pixel(p) == Some(BinaryColor::On) {
                    assert_eq!(regular.get_pixel(p), Some(BinaryColor::On));
                }
            }
        }
    }

    #[test]
    fn dotted_border_dots_have_correct_shape_and_size() {
        // The diameter of the dots should be the border width, except in the case
        // where this causes corner dots to overlap.
        // Rectangles are used for dots with size <= 3.
        let base = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(5)
            .stroke_alignment(StrokeAlignment::Inside);

        let rectangle1 = Rectangle::new(Point::new(5, 6), Size::new(40, 7));
        let rectangle2 = Rectangle::new(Point::new(4, 5), Size::new(40, 39));

        let top_left_dot1 = Rectangle::new(Point::new(5, 6), Size::new_equal(3)); // because the height is 7
        let top_left_dot2 = Circle::new(Point::new(4, 5), 5);

        let mut dot = MockDisplay::new();
        let mut dotted_border = MockDisplay::new();

        top_left_dot1
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut dot)
            .unwrap();
        rectangle1
            .into_styled(base.stroke_style(StrokeStyle::Dotted).build())
            .draw(&mut dotted_border)
            .unwrap();

        for p in top_left_dot1.bounding_box().points() {
            assert_eq!(dot.get_pixel(p), dotted_border.get_pixel(p));
        }

        let mut dot = MockDisplay::new();
        let mut dotted_border = MockDisplay::new();

        top_left_dot2
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut dot)
            .unwrap();
        rectangle2
            .into_styled(base.stroke_style(StrokeStyle::Dotted).build())
            .draw(&mut dotted_border)
            .unwrap();

        for p in top_left_dot2.bounding_box().points() {
            assert_eq!(dot.get_pixel(p), dotted_border.get_pixel(p));
        }
    }

    #[test]
    fn dotted_border_fill_is_independent_of_stroke_alignment() {
        let rect = Rectangle::new(Point::new(3, 4), Size::new(9, 13));
        let base = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::On)
            .stroke_width(5)
            .stroke_style(StrokeStyle::Dotted);

        let mut inside = MockDisplay::new();
        let mut outside = MockDisplay::new();

        rect.into_styled(base.stroke_alignment(StrokeAlignment::Inside).build())
            .draw(&mut inside)
            .unwrap();
        rect.into_styled(base.stroke_alignment(StrokeAlignment::Outside).build())
            .draw(&mut outside)
            .unwrap();

        for p in rect.bounding_box().points() {
            assert_eq!(inside.get_pixel(p), outside.get_pixel(p));
        }
    }

    #[test]
    fn thin_dotted_border_matches_prediction() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Rectangle::new(Point::new(4, 3), Size::new(4, 5))
            .into_styled(
                PrimitiveStyleBuilder::from(&PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                    .stroke_style(StrokeStyle::Dotted)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "              ",
            "              ",
            "              ",
            "    # #       ",
            "       #      ",
            "    #         ",
            "       #      ",
            "    # #       ",
        ]);

        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Rectangle::new(Point::new(4, 3), Size::new(4, 5))
            .into_styled(
                PrimitiveStyleBuilder::from(&PrimitiveStyle::with_stroke(BinaryColor::On, 2))
                    .stroke_style(StrokeStyle::Dotted)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "              ",
            "              ",
            "   ##  ##     ",
            "   ##  ##     ",
            "              ",
            "              ",
            "              ",
            "   ##  ##     ",
            "   ##  ##     ",
        ]);

        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Rectangle::new(Point::new(4, 3), Size::new(4, 5))
            .into_styled(
                PrimitiveStyleBuilder::from(&PrimitiveStyle::with_stroke(BinaryColor::On, 3))
                    .stroke_style(StrokeStyle::Dotted)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "              ",
            "              ",
            "   ###        ",
            "   ###        ",
            "   ###        ",
            "              ",
            "      ###     ",
            "      ###     ",
            "      ###     ",
        ]);
    }

    #[test]
    fn dot_positions_edge_cases() {
        // Test `dot_positions_with_dotted_corners` and `unit_positions_in_clockwise_order`
        // when `dot_size` is 0 or greater than `length`.

        let mut positions = dot_positions_with_dotted_corners(10, 0, false);
        assert_eq!(positions.next(), None);

        let mut positions = dot_positions_with_dotted_corners(0, 6, false);
        assert_eq!(positions.next(), None);

        let mut positions = dot_positions_with_dotted_corners(12, 0, true);
        assert_eq!(positions.next(), Some(0));
        assert_eq!(positions.next(), None);

        let mut positions = dot_positions_with_dotted_corners(9, 11, true);
        assert_eq!(positions.next(), Some(0));
        assert_eq!(positions.next(), None);

        let mut positions = unit_positions_in_clockwise_order(8, 0);
        assert_eq!(positions.next(), None);

        let mut positions = unit_positions_in_clockwise_order(7, 10);
        assert_eq!(positions.next(), None);
    }
}
