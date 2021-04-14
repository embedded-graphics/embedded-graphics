use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, PointExt},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        circle::{points::Scanlines, Circle},
        common::{Scanline, StyledScanline},
        rectangle::Rectangle,
        PrimitiveStyle, Styled, StyledPrimitiveAreas,
    },
    Drawable, Pixel, SaturatingCast,
};

/// Pixel iterator for each pixel in the circle border
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    styled_scanlines: StyledScanlines,

    stroke_left: Scanline,
    fill: Scanline,
    stroke_right: Scanline,

    stroke_color: Option<C>,
    fill_color: Option<C>,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    pub(in crate::primitives) fn new(styled: &Styled<Circle, PrimitiveStyle<C>>) -> Self {
        let stroke_area = styled.stroke_area();
        let fill_area = styled.fill_area();

        Self {
            styled_scanlines: StyledScanlines::new(&stroke_area, &fill_area),
            stroke_left: Scanline::new_empty(0),
            fill: Scanline::new_empty(0),
            stroke_right: Scanline::new_empty(0),
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
        match (self.stroke_color, self.fill_color) {
            (Some(stroke_color), None) => loop {
                if let Some(pixel) = self
                    .stroke_left
                    .next()
                    .or_else(|| self.stroke_right.next())
                    .map(|p| Pixel(p, stroke_color))
                {
                    return Some(pixel);
                }

                let scanline = self.styled_scanlines.next()?;
                self.stroke_left = scanline.stroke_left();
                self.stroke_right = scanline.stroke_right();
            },
            (Some(stroke_color), Some(fill_color)) => loop {
                if let Some(pixel) = self
                    .stroke_left
                    .next()
                    .map(|p| Pixel(p, stroke_color))
                    .or_else(|| self.fill.next().map(|p| Pixel(p, fill_color)))
                    .or_else(|| self.stroke_right.next().map(|p| Pixel(p, stroke_color)))
                {
                    return Some(pixel);
                }

                let scanline = self.styled_scanlines.next()?;
                self.stroke_left = scanline.stroke_left();
                self.fill = scanline.fill();
                self.stroke_right = scanline.stroke_right();
            },
            (None, Some(fill_color)) => loop {
                if let Some(pixel) = self.fill.next().map(|p| Pixel(p, fill_color)) {
                    return Some(pixel);
                }

                let scanline = self.styled_scanlines.next()?;
                self.fill = scanline.fill();
            },
            (None, None) => None,
        }
    }
}

impl<C> IntoPixels for &Styled<Circle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<Self::Color>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<C> Drawable for Styled<Circle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        match (self.style.effective_stroke_color(), self.style.fill_color) {
            (Some(stroke_color), None) => {
                for scanline in StyledScanlines::new(&self.stroke_area(), &self.fill_area()) {
                    scanline.draw_stroke(target, stroke_color)?;
                }
            }
            (Some(stroke_color), Some(fill_color)) => {
                for scanline in StyledScanlines::new(&self.stroke_area(), &self.fill_area()) {
                    scanline.draw_stroke_and_fill(target, stroke_color, fill_color)?;
                }
            }
            (None, Some(fill_color)) => {
                for scanline in Scanlines::new(&self.fill_area()) {
                    scanline.draw(target, fill_color)?;
                }
            }
            (None, None) => {}
        }

        Ok(())
    }
}

impl<C> Dimensions for Styled<Circle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        let offset = self.style.outside_stroke_width().saturating_cast();

        self.primitive.bounding_box().offset(offset)
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct StyledScanlines {
    scanlines: Scanlines,
    fill_threshold: u32,
}

impl StyledScanlines {
    pub fn new(stroke_area: &Circle, fill_area: &Circle) -> Self {
        Self {
            scanlines: Scanlines::new(stroke_area),
            fill_threshold: fill_area.threshold(),
        }
    }
}

impl Iterator for StyledScanlines {
    type Item = StyledScanline;

    fn next(&mut self) -> Option<Self::Item> {
        self.scanlines.next().map(|scanline| {
            let fill_range = scanline
                .x
                .clone()
                .find(|x| {
                    let delta = Point::new(*x, scanline.y) * 2 - self.scanlines.center_2x;
                    (delta.length_squared() as u32) < self.fill_threshold
                })
                .map(|x| x..scanline.x.end - (x - scanline.x.start));

            StyledScanline::new(scanline.y, scanline.x, fill_range)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Dimensions, Point},
        iterator::PixelIteratorExt,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{
            OffsetOutline, PointsIter, Primitive, PrimitiveStyleBuilder, StrokeAlignment,
        },
        Drawable,
    };

    /// Draws a styled circle by only using the points iterator.
    fn draw_circle(
        diameter: u32,
        stroke_color: Option<BinaryColor>,
        stroke_width: u32,
        fill_color: Option<BinaryColor>,
    ) -> MockDisplay<BinaryColor> {
        let circle = Circle::with_center(Point::new_equal(10), diameter);

        let mut display = MockDisplay::new();
        display.set_pixels(circle.points(), stroke_color);
        display.set_pixels(
            circle.offset(stroke_width.saturating_cast_neg()).points(),
            fill_color,
        );

        display
    }

    #[test]
    fn fill() {
        for diameter in 5..=6 {
            let expected = draw_circle(diameter, None, 0, Some(BinaryColor::On));

            let circle = Circle::with_center(Point::new_equal(10), diameter)
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

            let mut drawable = MockDisplay::new();
            circle.draw(&mut drawable).unwrap();
            drawable.assert_eq_with_message(&expected, |f| write!(f, "diameter = {}", diameter));

            let mut into_pixels = MockDisplay::new();
            circle.into_pixels().draw(&mut into_pixels).unwrap();
            into_pixels.assert_eq_with_message(&expected, |f| write!(f, "diameter = {}", diameter));
        }
    }

    #[test]
    fn stroke() {
        for (diameter, stroke_width) in [(5, 2), (5, 3), (6, 2), (6, 3)].iter().copied() {
            let expected = draw_circle(diameter, Some(BinaryColor::On), stroke_width, None);

            let style = PrimitiveStyleBuilder::new()
                .stroke_color(BinaryColor::On)
                .stroke_width(stroke_width)
                .stroke_alignment(StrokeAlignment::Inside)
                .build();

            let circle = Circle::with_center(Point::new_equal(10), diameter).into_styled(style);

            let mut drawable = MockDisplay::new();
            circle.draw(&mut drawable).unwrap();
            drawable.assert_eq_with_message(&expected, |f| {
                write!(
                    f,
                    "diameter = {}, stroke_width = {}",
                    diameter, stroke_width
                )
            });

            let mut into_pixels = MockDisplay::new();
            circle.into_pixels().draw(&mut into_pixels).unwrap();
            into_pixels.assert_eq_with_message(&expected, |f| {
                write!(
                    f,
                    "diameter = {}, stroke_width = {}",
                    diameter, stroke_width
                )
            });
        }
    }

    #[test]
    fn stroke_and_fill() {
        for (diameter, stroke_width) in [(5, 2), (5, 3), (6, 2), (6, 3)].iter().copied() {
            let expected = draw_circle(
                diameter,
                Some(BinaryColor::On),
                stroke_width,
                Some(BinaryColor::Off),
            );

            let style = PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::Off)
                .stroke_color(BinaryColor::On)
                .stroke_width(stroke_width)
                .stroke_alignment(StrokeAlignment::Inside)
                .build();

            let circle = Circle::with_center(Point::new_equal(10), diameter).into_styled(style);

            let mut drawable = MockDisplay::new();
            circle.draw(&mut drawable).unwrap();
            drawable.assert_eq_with_message(&expected, |f| {
                write!(
                    f,
                    "diameter = {}, stroke_width = {}",
                    diameter, stroke_width
                )
            });

            let mut into_pixels = MockDisplay::new();
            circle.into_pixels().draw(&mut into_pixels).unwrap();
            into_pixels.assert_eq_with_message(&expected, |f| {
                write!(
                    f,
                    "diameter = {}, stroke_width = {}",
                    diameter, stroke_width
                )
            });
        }
    }

    #[test]
    fn filled_styled_points_matches_points() {
        let circle = Circle::with_center(Point::new(10, 10), 5);

        let styled_points = circle
            .clone()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .into_pixels()
            .map(|Pixel(p, _)| p);

        assert!(circle.points().eq(styled_points));
    }

    // Check that tiny circles render as a "+" shape with a hole in the center
    #[test]
    fn tiny_circle() {
        let mut display = MockDisplay::new();

        Circle::new(Point::new(0, 0), 3)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            " # ", //
            "# #", //
            " # ", //
        ]);
    }

    // Check that tiny filled circle render as a "+" shape with NO hole in the center
    #[test]
    fn tiny_circle_filled() {
        let mut display = MockDisplay::new();

        Circle::new(Point::new(0, 0), 3)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            " # ", //
            "###", //
            " # ", //
        ]);
    }

    #[test]
    fn transparent_border() {
        let circle: Styled<Circle, PrimitiveStyle<BinaryColor>> =
            Circle::new(Point::new(-5, -5), 21)
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert!(circle.into_pixels().count() > 0);
    }

    #[test]
    fn it_handles_negative_coordinates() {
        let positive = Circle::new(Point::new(10, 10), 5)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_pixels();

        let negative = Circle::new(Point::new(-10, -10), 5)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_pixels();

        assert!(negative.eq(positive.map(|Pixel(p, c)| Pixel(p - Point::new(20, 20), c))));
    }

    #[test]
    fn stroke_alignment() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        Circle::with_center(CENTER, SIZE)
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        Circle::with_center(CENTER, SIZE + 2)
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        Circle::with_center(CENTER, SIZE - 4)
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

    /// Test for issue #143
    #[test]
    fn issue_143_stroke_and_fill() {
        for size in 0..10 {
            let circle_no_stroke: Styled<Circle, PrimitiveStyle<BinaryColor>> =
                Circle::new(Point::new(10, 16), size)
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

            let style = PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::On)
                .stroke_color(BinaryColor::On)
                .stroke_width(1)
                .build();
            let circle_stroke: Styled<Circle, PrimitiveStyle<BinaryColor>> =
                Circle::new(Point::new(10, 16), size).into_styled(style);

            assert_eq!(
                circle_stroke.bounding_box(),
                circle_no_stroke.bounding_box(),
                "Filled and unfilled circle bounding boxes are unequal for radius {}",
                size
            );
            assert!(
                circle_no_stroke
                    .into_pixels()
                    .eq(circle_stroke.into_pixels()),
                "Filled and unfilled circle iters are unequal for radius {}",
                size
            );
        }
    }

    #[test]
    fn bounding_boxes() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let center = Circle::with_center(CENTER, SIZE).into_styled(style);

        let inside = Circle::with_center(CENTER, SIZE).into_styled(
            PrimitiveStyleBuilder::from(&style)
                .stroke_alignment(StrokeAlignment::Inside)
                .build(),
        );

        let outside = Circle::with_center(CENTER, SIZE).into_styled(
            PrimitiveStyleBuilder::from(&style)
                .stroke_alignment(StrokeAlignment::Outside)
                .build(),
        );

        let mut display = MockDisplay::new();
        center.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), center.bounding_box());

        let mut display = MockDisplay::new();
        outside.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), outside.bounding_box());

        let mut display = MockDisplay::new();
        inside.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), inside.bounding_box());
    }

    #[test]
    fn bounding_box_is_independent_of_colors() {
        let transparent_circle =
            Circle::new(Point::new(5, 5), 11).into_styled::<BinaryColor>(PrimitiveStyle::new());
        let filled_circle = Circle::new(Point::new(5, 5), 11)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert_eq!(
            transparent_circle.bounding_box(),
            filled_circle.bounding_box(),
        );
    }
}
