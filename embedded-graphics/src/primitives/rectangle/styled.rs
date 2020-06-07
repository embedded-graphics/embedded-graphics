use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Point, Size},
    pixelcolor::PixelColor,
    primitives::{
        rectangle::{Points, Rectangle},
        ContainsPoint, Primitive,
    },
    style::{PrimitiveStyle, Styled},
    transform::Transform,
};

/// Pixel iterator for each pixel in the rect border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledRectangleIterator<C: PixelColor>
where
    C: PixelColor,
{
    iter: Points,

    stroke_color: Option<C>,

    fill_area: Rectangle,
    fill_color: Option<C>,
}

impl<C> StyledRectangleIterator<C>
where
    C: PixelColor,
{
    pub(crate) fn new(styled: &Styled<Rectangle, PrimitiveStyle<C>>) -> Self {
        let Styled { style, primitive } = styled;

        let iter = if !style.is_transparent() {
            let stroke_area = primitive.expand(style.outside_stroke_width());
            stroke_area.points()
        } else {
            Points::empty()
        };

        let fill_area = primitive.shrink(style.inside_stroke_width());

        Self {
            iter,
            stroke_color: style.stroke_color,
            fill_area,
            fill_color: style.fill_color,
        }
    }
}

impl<C> Iterator for StyledRectangleIterator<C>
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

impl<C> IntoIterator for &Styled<Rectangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledRectangleIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledRectangleIterator::new(self)
    }
}

impl<C> Drawable<C> for &Styled<Rectangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<Color = C>>(self, display: &mut D) -> Result<(), D::Error> {
        let fill_area = self.primitive.shrink(self.style.inside_stroke_width());

        // Fill rectangle
        if let Some(fill_color) = self.style.fill_color {
            display.fill_solid(&fill_area, fill_color)?;
        }

        // Draw stroke
        if let Some(stroke_color) = self.style.effective_stroke_color() {
            let stroke_width = self.style.stroke_width;

            let stroke_area = self.primitive.expand(self.style.outside_stroke_width());

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

            display.fill_solid(&top_border, stroke_color)?;
            display.fill_solid(&bottom_border, stroke_color)?;

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

                display.fill_solid(&left_border, stroke_color)?;
                display.fill_solid(&right_border, stroke_color)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Drawable,
        geometry::{Point, Size},
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb565, RgbColor},
        primitives::Primitive,
        style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    };

    #[test]
    fn it_draws_unfilled_rect() {
        let mut rect = Rectangle::new(Point::new(2, 2), Size::new(3, 3))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
            .into_iter();

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

        assert_eq!(display_center, display_inside);
        assert_eq!(display_center, display_outside);
    }
}
