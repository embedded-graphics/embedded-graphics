use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, PointExt},
    pixelcolor::PixelColor,
    primitives::{
        circle::{points::Scanlines, Circle},
        common::{Scanline, StyledScanline},
        rectangle::Rectangle,
        styled::{StyledDimensions, StyledDrawable, StyledPixels},
        PrimitiveStyle,
    },
    Pixel,
};
use az::SaturatingAs;

#[cfg(feature = "async_draw")]
use crate::draw_target::AsyncDrawTarget;
#[cfg(feature = "async_draw")]
use crate::primitives::styled::AsyncStyledDrawable;
#[cfg(feature = "async_draw")]
impl<C: PixelColor> AsyncStyledDrawable<PrimitiveStyle<C>> for Circle {
    type Color = C;
    type Output = ();
    async fn draw_styled_async<D>(
        &self,
        style: &PrimitiveStyle<C>,
        target: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: AsyncDrawTarget<Color = C>,
    {
        match (style.effective_stroke_color(), style.fill_color) {
            (Some(stroke_color), None) => {
                for scanline in
                    StyledScanlines::new(&style.stroke_area(self), &style.fill_area(self))
                {
                    scanline.draw_stroke_async(target, stroke_color).await?;
                }
            }
            (Some(stroke_color), Some(fill_color)) => {
                for scanline in
                    StyledScanlines::new(&style.stroke_area(self), &style.fill_area(self))
                {
                    scanline
                        .draw_stroke_and_fill_async(target, stroke_color, fill_color)
                        .await?;
                }
            }
            (None, Some(fill_color)) => {
                for scanline in Scanlines::new(&style.fill_area(self)) {
                    scanline.draw_async(target, fill_color).await?;
                }
            }
            (None, None) => {}
        }

        Ok(())
    }
}

/// Pixel iterator for each pixel in the circle border
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct StyledPixelsIterator<C> {
    styled_scanlines: StyledScanlines,

    stroke_left: Scanline,
    fill: Scanline,
    stroke_right: Scanline,

    stroke_color: Option<C>,
    fill_color: Option<C>,
}

impl<C: PixelColor> StyledPixelsIterator<C> {
    pub(in crate::primitives) fn new(primitive: &Circle, style: &PrimitiveStyle<C>) -> Self {
        let stroke_area = style.stroke_area(primitive);
        let fill_area = style.fill_area(primitive);

        Self {
            styled_scanlines: StyledScanlines::new(&stroke_area, &fill_area),
            stroke_left: Scanline::new_empty(0),
            fill: Scanline::new_empty(0),
            stroke_right: Scanline::new_empty(0),
            stroke_color: style.stroke_color,
            fill_color: style.fill_color,
        }
    }
}

impl<C> Iterator for StyledPixelsIterator<C>
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

impl<C: PixelColor> StyledPixels<PrimitiveStyle<C>> for Circle {
    type Iter = StyledPixelsIterator<C>;

    fn pixels(&self, style: &PrimitiveStyle<C>) -> Self::Iter {
        StyledPixelsIterator::new(self, style)
    }
}

impl<C: PixelColor> StyledDrawable<PrimitiveStyle<C>> for Circle {
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
        match (style.effective_stroke_color(), style.fill_color) {
            (Some(stroke_color), None) => {
                for scanline in
                    StyledScanlines::new(&style.stroke_area(self), &style.fill_area(self))
                {
                    scanline.draw_stroke(target, stroke_color)?;
                }
            }
            (Some(stroke_color), Some(fill_color)) => {
                for scanline in
                    StyledScanlines::new(&style.stroke_area(self), &style.fill_area(self))
                {
                    scanline.draw_stroke_and_fill(target, stroke_color, fill_color)?;
                }
            }
            (None, Some(fill_color)) => {
                for scanline in Scanlines::new(&style.fill_area(self)) {
                    scanline.draw(target, fill_color)?;
                }
            }
            (None, None) => {}
        }

        Ok(())
    }
}

impl<C: PixelColor> StyledDimensions<PrimitiveStyle<C>> for Circle {
    fn styled_bounding_box(&self, style: &PrimitiveStyle<C>) -> Rectangle {
        let offset = style.outside_stroke_width().saturating_as();

        self.bounding_box().offset(offset)
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
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
            OffsetOutline, PointsIter, Primitive, PrimitiveStyleBuilder, StrokeAlignment, Styled,
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
            circle.offset(-stroke_width.saturating_as::<i32>()).points(),
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

            let mut pixels = MockDisplay::new();
            circle.pixels().draw(&mut pixels).unwrap();
            pixels.assert_eq_with_message(&expected, |f| write!(f, "diameter = {}", diameter));
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

            let mut pixels = MockDisplay::new();
            circle.pixels().draw(&mut pixels).unwrap();
            pixels.assert_eq_with_message(&expected, |f| {
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

            let mut pixels = MockDisplay::new();
            circle.pixels().draw(&mut pixels).unwrap();
            pixels.assert_eq_with_message(&expected, |f| {
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
            .pixels()
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

        assert!(circle.pixels().count() > 0);
    }

    #[test]
    fn it_handles_negative_coordinates() {
        let positive = Circle::new(Point::new(10, 10), 5)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .pixels();

        let negative = Circle::new(Point::new(-10, -10), 5)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .pixels();

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
                circle_no_stroke.pixels().eq(circle_stroke.pixels()),
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
            Circle::new(Point::new(5, 5), 11).into_styled(PrimitiveStyle::<BinaryColor>::new());
        let filled_circle = Circle::new(Point::new(5, 5), 11)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert_eq!(
            transparent_circle.bounding_box(),
            filled_circle.bounding_box(),
        );
    }
}
