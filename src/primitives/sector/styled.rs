use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Size},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        circle::DistanceIterator, common::PlaneSectorIterator, line::ThickPoints, OffsetOutline,
        Rectangle, Sector, Styled,
    },
    style::{PrimitiveStyle, StyledPrimitiveAreas},
    SaturatingCast,
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
        let stroke_area = styled.stroke_area();
        let fill_area = styled.fill_area();

        let line_a = stroke_area.line_from_angle(styled.primitive.angle_start);
        let line_b = stroke_area.line_from_angle(styled.primitive.angle_end());

        let line_a_iter = ThickPoints::new(&line_a, styled.style.stroke_width.saturating_cast());
        let line_b_iter = ThickPoints::new(&line_b, styled.style.stroke_width.saturating_cast());

        let points = if !styled.style.is_transparent() {
            PlaneSectorIterator::new(
                &stroke_area,
                stroke_area.center_2x(),
                stroke_area.angle_start,
                stroke_area.angle_sweep,
            )
        } else {
            PlaneSectorIterator::empty()
        };

        let stroke_area_circle = stroke_area.to_circle();

        Self {
            iter: stroke_area_circle.distances(points),
            outer_threshold: stroke_area_circle.threshold(),
            outer_color: styled.style.stroke_color,
            inner_threshold: fill_area.to_circle().threshold(),
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

impl<C> IntoPixels for &Styled<Sector, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<Self::Color>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<C> Drawable for Styled<Sector, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        display.draw_iter(self.into_pixels())
    }
}

impl<C> Dimensions for Styled<Sector, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    // FIXME: This doesn't take into account start/end angles. This should be fixed to close #405.
    fn bounding_box(&self) -> Rectangle {
        if !self.style.is_transparent() {
            let offset = self.style.outside_stroke_width().saturating_cast();

            self.primitive.bounding_box().offset(offset)
        } else {
            Rectangle::new(self.primitive.bounding_box().center(), Size::zero())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{AngleUnit, Point},
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        primitives::Circle,
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

    #[test]
    fn bounding_boxes() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let center = Sector::with_center(CENTER, SIZE, 0.0.deg(), 90.0.deg()).into_styled(style);
        let inside = Sector::with_center(CENTER, SIZE + 2, 0.0.deg(), 90.0.deg()).into_styled(
            PrimitiveStyleBuilder::from(&style)
                .stroke_alignment(StrokeAlignment::Inside)
                .build(),
        );
        let outside = Sector::with_center(CENTER, SIZE - 4, 0.0.deg(), 90.0.deg()).into_styled(
            PrimitiveStyleBuilder::from(&style)
                .stroke_alignment(StrokeAlignment::Outside)
                .build(),
        );
        let empty = Sector::with_center(CENTER, SIZE - 4, 0.0.deg(), 90.0.deg())
            .into_styled::<BinaryColor>(PrimitiveStyle::new());

        // TODO: Uncomment when arc bounding box is fixed in #405
        // let mut display = MockDisplay::new();
        // center.draw(&mut display).unwrap();
        // assert_eq!(display.affected_area(), center.bounding_box());

        assert_eq!(empty.bounding_box(), Rectangle::new(CENTER, Size::zero()));

        assert_eq!(center.bounding_box(), inside.bounding_box());
        assert_eq!(outside.bounding_box(), inside.bounding_box());
    }

    /// The radial lines should be connected using a line join.
    #[test]
    #[ignore]
    fn issue_484_line_join_90_deg() {
        let mut display = MockDisplay::<Rgb888>::new();
        // TODO: sectors shouldn't overdraw
        display.set_allow_overdraw(true);

        Sector::new(Point::new(-6, 1), 15, 0.0.deg(), 90.0.deg())
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_color(Rgb888::RED)
                    .stroke_width(3)
                    .fill_color(Rgb888::GREEN)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "RRRR      ",
            "RRRRRR    ",
            "RRRRRRRR  ",
            "RRRGRRRR  ",
            "RRRGGRRRR ",
            "RRRGGGRRR ",
            "RRRGGGGRRR",
            "RRRRRRRRRR",
            "RRRRRRRRRR",
            "RRRRRRRRRR",
        ]);
    }

    // TODO: add tests for other angles with mitre and bevel joins

    /// The stroke for the radial lines shouldn't overlap the outer edge of the stroke on the
    /// circular part of the sector.
    #[test]
    #[ignore]
    fn issue_484_stroke_should_not_overlap_outer_edge() {
        let mut display = MockDisplay::<Rgb888>::new();
        // TODO: sectors shouldn't overdraw
        display.set_allow_overdraw(true);

        Sector::with_center(Point::new(10, 15), 11, 0.0.deg(), 90.0.deg())
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_color(Rgb888::RED)
                    .stroke_width(21)
                    .fill_color(Rgb888::GREEN)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "RRRRRRRRRRRRRR            ",
            "RRRRRRRRRRRRRRRRR         ",
            "RRRRRRRRRRRRRRRRRRR       ",
            "RRRRRRRRRRRRRRRRRRRR      ",
            "RRRRRRRRRRRRRRRRRRRRR     ",
            "RRRRRRRRRRRRRRRRRRRRRR    ",
            "RRRRRRRRRRRRRRRRRRRRRRR   ",
            "RRRRRRRRRRRRRRRRRRRRRRRR  ",
            "RRRRRRRRRRRRRRRRRRRRRRRR  ",
            "RRRRRRRRRRRRRRRRRRRRRRRRR ",
            "RRRRRRRRRRRRRRRRRRRRRRRRR ",
            "RRRRRRRRRRRRRRRRRRRRRRRRR ",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
            "RRRRRRRRRRRRRRRRRRRRRRRRRR",
        ]);
    }

    /// Both radial lines should be perfectly aligned for 180° sweep angle.
    #[test]
    #[ignore]
    fn issue_484_stroke_center_semicircle() {
        let mut display = MockDisplay::new();
        // TODO: sectors shouldn't overdraw
        display.set_allow_overdraw(true);

        Sector::new(Point::new_equal(1), 15, 0.0.deg(), 180.0.deg())
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(BinaryColor::On)
                    .stroke_color(BinaryColor::Off)
                    .stroke_width(2)
                    .stroke_alignment(StrokeAlignment::Center)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "      .....      ",
            "    .........    ",
            "  ....#####....  ",
            "  ..#########..  ",
            " ..###########.. ",
            " ..###########.. ",
            "..#############..",
            ".................",
            ".................",
        ]);
    }

    /// Both radial lines should be perfectly aligned for 180° sweep angle.
    #[test]
    #[ignore]
    fn issue_484_stroke_center_semicircle_vertical() {
        let mut display = MockDisplay::new();
        // TODO: sectors shouldn't overdraw
        display.set_allow_overdraw(true);

        Sector::new(Point::new_equal(1), 15, 90.0.deg(), 180.0.deg())
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(BinaryColor::On)
                    .stroke_color(BinaryColor::Off)
                    .stroke_width(2)
                    .stroke_alignment(StrokeAlignment::Center)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "      ...",
            "    .....",
            "  ....#..",
            "  ..###..",
            " ..####..",
            " ..####..",
            "..#####..",
            "..#####..",
            "..#####..",
            "..#####..",
            "..#####..",
            " ..####..",
            " ..####..",
            "  ..###..",
            "  ....#..",
            "    .....",
            "      ...",
        ]);
    }

    /// The fill shouldn't overlap the stroke and there should be no gaps between stroke and fill.
    #[test]
    #[ignore]
    fn issue_484_gaps_and_overlap() {
        let mut display = MockDisplay::new();
        // TODO: sectors shouldn't overdraw
        display.set_allow_overdraw(true);

        Sector::with_center(Point::new(2, 20), 40, -14.0.deg(), 90.0.deg())
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb888::GREEN)
                    .stroke_color(Rgb888::RED)
                    .stroke_width(2)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            // TODO: Update expected pattern
        ]);
    }

    /// No radial lines should be drawn if the sweep angle is 360°.
    #[test]
    #[ignore]
    fn issue_484_no_radial_lines_for_360_degree_sweep_angle() {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::GREEN)
            .stroke_color(Rgb888::RED)
            .stroke_width(1)
            .build();

        let circle = Circle::new(Point::new_equal(1), 11);

        let mut expected = MockDisplay::new();
        circle.into_styled(style).draw(&mut expected).unwrap();

        let mut display = MockDisplay::new();
        // TODO: sectors shouldn't overdraw
        display.set_allow_overdraw(true);

        Sector::new(Point::new_equal(1), 11, 0.0.deg(), 360.0.deg())
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.assert_eq(&expected);
    }

    /// No radial lines should be drawn for sweep angles larger than 360°.
    #[test]
    #[ignore]
    fn issue_484_no_radial_lines_for_sweep_angles_larger_than_360_degree() {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::GREEN)
            .stroke_color(Rgb888::RED)
            .stroke_width(1)
            .build();

        let circle = Circle::new(Point::new_equal(1), 11);

        let mut expected = MockDisplay::new();
        circle.into_styled(style).draw(&mut expected).unwrap();

        let mut display = MockDisplay::new();
        // TODO: sectors shouldn't overdraw
        display.set_allow_overdraw(true);

        Sector::from_circle(circle, 90.0.deg(), -472.0.deg())
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.assert_eq(&expected);
    }

    /// The sector was mirrored along the Y axis if the start angle was exactly 360°.
    #[test]
    fn issue_484_sector_flips_at_360_degrees() {
        let mut display = MockDisplay::new();

        // This would trigger the out of bounds drawing check if the sector
        // would be mirrored along the Y axis.
        Sector::new(Point::new(-15, 0), 31, 360.0.deg(), 90.0.deg())
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();
    }
}
