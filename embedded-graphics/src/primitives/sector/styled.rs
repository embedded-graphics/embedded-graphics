use crate::{
    drawable::{Drawable, Pixel},
    geometry::Point,
    pixel_iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        arc::PlaneSectorIterator, circle, circle::DistanceIterator, line::ThickPoints, Sector,
        Styled,
    },
    style::PrimitiveStyle,
    DrawTarget,
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum IterState {
    Arc,
    Lines,
    Done,
}

/// Pixel iterator for each pixel in the sector border
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    iter: DistanceIterator<PlaneSectorIterator>,

    outer_threshold: u32,
    outer_color: Option<C>,

    inner_threshold: u32,
    inner_color: Option<C>,

    line_a_iter: ThickPoints,
    line_b_iter: ThickPoints,

    state: IterState,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    fn new(styled: &Styled<Sector, PrimitiveStyle<C>>) -> Self {
        let Styled { primitive, style } = styled;

        let stroke_area = primitive.expand(style.outside_stroke_width());
        let fill_area = primitive.shrink(style.inside_stroke_width());

        let inner_threshold = circle::diameter_to_threshold(fill_area.diameter);
        let outer_threshold = circle::diameter_to_threshold(stroke_area.diameter);

        let line_a = stroke_area.line_from_angle(styled.primitive.angle_start);
        let line_b = stroke_area.line_from_angle(styled.primitive.angle_end());

        let line_a_iter = ThickPoints::new(&line_a, styled.style.stroke_width_i32());
        let line_b_iter = ThickPoints::new(&line_b, styled.style.stroke_width_i32());

        let iter = if !styled.style.is_transparent() {
            DistanceIterator::new(
                stroke_area.center_2x(),
                PlaneSectorIterator::new(
                    &stroke_area,
                    stroke_area.center(),
                    stroke_area.angle_start,
                    stroke_area.angle_sweep,
                ),
            )
        } else {
            DistanceIterator::new(Point::zero(), PlaneSectorIterator::empty())
        };

        Self {
            iter,
            outer_threshold,
            outer_color: styled.style.stroke_color,
            inner_threshold,
            inner_color: styled.style.fill_color,
            line_a_iter,
            line_b_iter,
            state: IterState::Arc,
        }
    }
}

impl<C> Iterator for StyledPixels<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                IterState::Arc => {
                    if let Some((point, distance)) = self.iter.next() {
                        let color = if distance < self.inner_threshold {
                            self.inner_color
                        } else if distance < self.outer_threshold {
                            self.outer_color
                        } else {
                            None
                        };

                        if let Some(color) = color {
                            return Some(Pixel(point, color));
                        }
                    } else {
                        self.state = IterState::Lines;
                    }
                }
                IterState::Lines => {
                    if let Some(color) = self.outer_color {
                        if let Some(point) =
                            self.line_a_iter.next().or_else(|| self.line_b_iter.next())
                        {
                            break Some(Pixel(point, color));
                        }
                    }
                    self.state = IterState::Done;
                }
                IterState::Done => {
                    break None;
                }
            }
        }
    }
}

impl<'a, C> IntoPixels for &'a Styled<Sector, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<Self::Color>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<C> Drawable<C> for Styled<Sector, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        display.draw_iter(self.into_pixels())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::AngleUnit,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::Primitive,
        style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    };

    #[test]
    fn stroke_width_doesnt_affect_fill() -> Result<(), core::convert::Infallible> {
        let mut expected = MockDisplay::new();
        let mut style = PrimitiveStyle::with_fill(BinaryColor::On);
        Sector::new(Point::new(5, 5), 4, 30.0.deg(), 120.0.deg())
            .into_styled(style)
            .draw(&mut expected)?;

        let mut with_stroke_width = MockDisplay::new();
        style.stroke_width = 1;
        Sector::new(Point::new(5, 5), 4, 30.0.deg(), 120.0.deg())
            .into_styled(style)
            .draw(&mut with_stroke_width)?;

        assert_eq!(expected, with_stroke_width);

        Ok(())
    }

    // Check the rendering of a simple sector
    #[test]
    fn tiny_sector() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        Sector::new(Point::zero(), 9, 30.0.deg(), 120.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)?;

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "  #####  ",
                " ##   ## ",
                " #     # ",
                "  ## ##  ",
                "    #    ",
            ])
        );

        Ok(())
    }

    // Check the rendering of a filled sector with negative sweep
    #[test]
    fn tiny_sector_filled() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();

        Sector::new(Point::zero(), 7, -30.0.deg(), -300.0.deg())
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)?;

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "  ###  ",
                " ##### ",
                "#####  ",
                "####   ",
                "#####  ",
                " ##### ",
                "  ###  ",
            ])
        );

        Ok(())
    }

    #[test]
    fn transparent_border() {
        let sector: Styled<Sector, PrimitiveStyle<BinaryColor>> =
            Sector::new(Point::new(-5, -5), 21, 0.0.deg(), 90.0.deg())
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert!(sector.into_pixels().count() > 0);
    }

    #[test]
    fn stroke_alignment() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        display_center.set_allow_overdraw(true);
        Sector::with_center(CENTER, SIZE, 0.0.deg(), 90.0.deg())
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        display_inside.set_allow_overdraw(true);
        Sector::with_center(CENTER, SIZE + 2, 0.0.deg(), 90.0.deg())
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        display_outside.set_allow_overdraw(true);
        Sector::with_center(CENTER, SIZE - 4, 0.0.deg(), 90.0.deg())
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
