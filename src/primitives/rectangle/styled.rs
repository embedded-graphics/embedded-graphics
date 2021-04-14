use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        rectangle::{Points, Rectangle},
        styled::{Styled, StyledDimensions, StyledDrawable},
        PointsIter, PrimitiveStyle,
    },
    transform::Transform,
    Pixel, SaturatingCast,
};

/// Pixel iterator for each pixel in the rect border
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    iter: Points,

    stroke_color: Option<C>,

    fill_area: Rectangle,
    fill_color: Option<C>,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    pub(in crate::primitives) fn new(styled: &Styled<Rectangle, PrimitiveStyle<C>>) -> Self {
        let iter = if !styled.style.is_transparent() {
            styled.stroke_area().points()
        } else {
            Points::empty()
        };

        Self {
            iter,
            fill_area: styled.fill_area(),
            stroke_color: styled.style.stroke_color,
            fill_color: styled.style.fill_color,
        }
    }
}

impl<C> Iterator for StyledPixels<C>
where
    C: PixelColor,
{
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

impl<C> IntoPixels for &Styled<Rectangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<C>;

    fn into_pixels(self) -> Self::Iter {
        Self::Iter::new(self)
    }
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
        if let Some(stroke_color) = style.effective_stroke_color() {
            let stroke_width = style.stroke_width;

            let stroke_area = style.stroke_area(self);

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
        let offset = style.outside_stroke_width().saturating_cast();

        self.bounding_box().offset(offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Point, Size},
        iterator::{IntoPixels, PixelIteratorExt},
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb565, RgbColor},
        primitives::{Primitive, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
        Drawable,
    };

    #[test]
    fn it_draws_unfilled_rect() {
        let mut rect = Rectangle::new(Point::new(2, 2), Size::new(3, 3))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
            .into_pixels();

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
            .into_pixels()
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
        rectangle_center
            .into_pixels()
            .draw(&mut iter_center)
            .unwrap();
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
        rectangle_inside
            .into_pixels()
            .draw(&mut iter_inside)
            .unwrap();
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
        rectangle_outside
            .into_pixels()
            .draw(&mut iter_outside)
            .unwrap();
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
        rectangle.into_pixels().draw(&mut iter).unwrap();
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
                    .into_pixels()
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

        let _pixels = styled.into_pixels();

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
}
