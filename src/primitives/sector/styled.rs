use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::angle_consts::ANGLE_90DEG,
    geometry::{Angle, Dimensions, Size},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        common::{
            DistanceIterator, OriginLinearEquation, PlaneSector, PointType, NORMAL_VECTOR_SCALE,
        },
        OffsetOutline, Rectangle, Sector, Styled,
    },
    style::{PrimitiveStyle, StyledPrimitiveAreas},
    SaturatingCast,
};

/// Pixel iterator for each pixel in the sector border
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    iter: DistanceIterator,

    plane_sector: PlaneSector,

    outer_threshold: u32,
    inner_threshold: u32,

    stroke_threshold_inside: i32,
    stroke_threshold_outside: i32,

    bevel: Option<OriginLinearEquation>,
    bevel_threshold: i32,

    outer_color: Option<C>,
    inner_color: Option<C>,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    fn new(styled: &Styled<Sector, PrimitiveStyle<C>>) -> Self {
        let Styled { primitive, style } = styled;

        let stroke_area = styled.stroke_area();
        let fill_area = styled.fill_area();

        let stroke_area_circle = stroke_area.to_circle();

        let iter = if !style.is_transparent() {
            // PERF: The distance iterator should use the smaller sector bounding box
            stroke_area_circle.distances()
        } else {
            DistanceIterator::empty()
        };

        let outer_threshold = stroke_area_circle.threshold();
        let inner_threshold = fill_area.to_circle().threshold();

        let plane_sector = PlaneSector::new(stroke_area.angle_start, stroke_area.angle_sweep);

        let stroke_threshold_inside =
            style.inside_stroke_width().saturating_cast() * NORMAL_VECTOR_SCALE * 2
                - NORMAL_VECTOR_SCALE;
        let stroke_threshold_outside =
            style.outside_stroke_width().saturating_cast() * NORMAL_VECTOR_SCALE * 2
                + NORMAL_VECTOR_SCALE;

        // TODO: Polylines and sectors should use the same miter limit.
        let (bevel, bevel_threshold) = if primitive.angle_sweep.abs() < Angle::from_degrees(55.0) {
            let half_sweep = Angle::from_radians(primitive.angle_sweep.to_radians() / 2.0);

            let threshold =
                style.outside_stroke_width().saturating_cast() * NORMAL_VECTOR_SCALE * 4;

            (
                Some(OriginLinearEquation::with_angle(
                    primitive.angle_start + half_sweep + ANGLE_90DEG,
                )),
                threshold,
            )
        } else {
            (None, 0)
        };

        Self {
            iter,
            plane_sector,
            outer_threshold,
            inner_threshold,
            stroke_threshold_inside,
            stroke_threshold_outside,
            bevel,
            bevel_threshold,
            outer_color: styled.style.stroke_color,
            inner_color: styled.style.fill_color,
        }
    }
}

impl<C> Iterator for StyledPixels<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let outer_threshold = self.outer_threshold;

        loop {
            let (point, delta, distance) = self
                .iter
                .find(|(_, _, distance)| *distance < outer_threshold)?;

            let color = match self.plane_sector.point_type(
                delta,
                self.stroke_threshold_inside,
                self.stroke_threshold_outside,
            ) {
                Some(PointType::Stroke) => {
                    if let Some(bevel) = &self.bevel {
                        if bevel.distance(delta) >= self.bevel_threshold {
                            continue;
                        }
                    }

                    self.outer_color
                }
                Some(PointType::Fill) => {
                    if distance < self.inner_threshold {
                        self.inner_color
                    } else {
                        self.outer_color
                    }
                }
                None => continue,
            };

            if let Some(color) = color {
                return Some(Pixel(point, color));
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
    fn stroke_width_doesnt_affect_fill() {
        let mut expected = MockDisplay::new();
        let mut style = PrimitiveStyle::with_fill(BinaryColor::On);
        Sector::new(Point::new(5, 5), 4, 30.0.deg(), 120.0.deg())
            .into_styled(style)
            .draw(&mut expected)
            .unwrap();

        let mut with_stroke_width = MockDisplay::new();
        style.stroke_width = 1;
        Sector::new(Point::new(5, 5), 4, 30.0.deg(), 120.0.deg())
            .into_styled(style)
            .draw(&mut with_stroke_width)
            .unwrap();

        with_stroke_width.assert_eq(&expected);
    }

    // Check the rendering of a simple sector
    #[test]
    fn tiny_sector() {
        let mut display = MockDisplay::new();

        Sector::new(Point::zero(), 9, 30.0.deg(), 120.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "  #####  ", //
            " ##   ## ", //
            "##     ##", //
            "  ## ##  ", //
            "    #    ", //
        ]);
    }

    // Check the rendering of a filled sector with negative sweep
    #[test]
    fn tiny_sector_filled() {
        let mut display = MockDisplay::new();

        Sector::new(Point::zero(), 7, -30.0.deg(), -300.0.deg())
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "  ###  ", //
            " ##### ", //
            "###### ", //
            "#####  ", //
            "###### ", //
            " ##### ", //
            "  ###  ", //
        ]);
    }

    #[test]
    fn transparent_border() {
        let sector: Styled<Sector, PrimitiveStyle<BinaryColor>> =
            Sector::new(Point::new(-5, -5), 21, 0.0.deg(), 90.0.deg())
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert!(sector.into_pixels().count() > 0);
    }

    fn test_stroke_alignment(
        stroke_alignment: StrokeAlignment,
        diameter: u32,
        expected_pattern: &[&str],
    ) {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(3)
            .stroke_alignment(stroke_alignment)
            .build();

        let mut display = MockDisplay::new();

        Sector::with_center(Point::new(3, 10), diameter, 0.0.deg(), 90.0.deg())
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(expected_pattern);
    }

    #[test]
    fn stroke_alignment_inside() {
        test_stroke_alignment(
            StrokeAlignment::Inside,
            19 + 2,
            &[
                "   ####       ",
                "   ######     ",
                "   #######    ",
                "   ########   ",
                "   ###  ####  ",
                "   ###   #### ",
                "   ###    ### ",
                "   ###    ####",
                "   ###########",
                "   ###########",
                "   ###########",
            ],
        );
    }

    #[test]
    fn stroke_alignment_center() {
        test_stroke_alignment(
            StrokeAlignment::Center,
            19,
            &[
                "  #####       ",
                "  #######     ",
                "  ########    ",
                "  ### #####   ",
                "  ###   ####  ",
                "  ###    #### ",
                "  ###     ### ",
                "  ###     ####",
                "  ###      ###",
                "  ############",
                "  ############",
                "  ############",
            ],
        );
    }

    #[test]
    fn stroke_alignment_outside() {
        test_stroke_alignment(
            StrokeAlignment::Outside,
            19 - 4,
            &[
                "#######       ",
                "#########     ",
                "##########    ",
                "###   #####   ",
                "###     ####  ",
                "###      #### ",
                "###       ### ",
                "###       ####",
                "###        ###",
                "###        ###",
                "###        ###",
                "##############",
                "##############",
                "##############",
            ],
        );
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
    fn issue_484_line_join_90_deg() {
        let mut display = MockDisplay::<Rgb888>::new();

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

    /// The radial lines should be connected using a line join.
    #[test]
    fn issue_484_line_join_20_deg() {
        let mut display = MockDisplay::<Rgb888>::new();

        Sector::new(Point::new(-4, -3), 15, 0.0.deg(), 20.0.deg())
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
            "          R ",
            "       RRRR ",
            "     RRRRRRR",
            "  RRRRRRRRRR",
            " RRRRRRRRRRR",
            "  RRRRRRRRRR",
        ]);
    }

    /// The radial lines should be connected using a line join.
    // TODO: This test currently fails because a miter join is drawn instead of a bevel join
    #[test]
    #[ignore]
    fn issue_484_line_join_340_deg() {
        let mut display = MockDisplay::<Rgb888>::new();

        Sector::new(Point::new_equal(2), 15, 0.0.deg(), 340.0.deg())
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
            // TODO: update pattern
        ]);
    }

    /// The stroke for the radial lines shouldn't overlap the outer edge of the stroke on the
    /// circular part of the sector.
    #[test]
    #[ignore]
    fn issue_484_stroke_should_not_overlap_outer_edge() {
        let mut display = MockDisplay::<Rgb888>::new();

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
    fn issue_484_stroke_center_semicircle() {
        let mut display = MockDisplay::new();

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
            "..#############..",
            ".................",
            ".................",
        ]);
    }

    /// Both radial lines should be perfectly aligned for 180° sweep angle.
    #[test]
    fn issue_484_stroke_center_semicircle_vertical() {
        let mut display = MockDisplay::new();

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
            "      ....",
            "    ......",
            "  ....##..",
            "  ..####..",
            " ..#####..",
            " ..#####..",
            "..######..",
            "..######..",
            "..######..",
            "..######..",
            "..######..",
            " ..#####..",
            " ..#####..",
            "  ..####..",
            "  ....##..",
            "    ......",
            "      ....",
        ]);
    }

    /// The fill shouldn't overlap the stroke and there should be no gaps between stroke and fill.
    #[test]
    fn issue_484_gaps_and_overlap() {
        let mut display = MockDisplay::new();

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
            "       R                ",
            "      RRRRR             ",
            "      RRRRRRR           ",
            "      RRGGRRRRR         ",
            "      RRGGGGRRRR        ",
            "     RRGGGGGGGRRR       ",
            "     RRGGGGGGGGRRR      ",
            "     RRGGGGGGGGGRRR     ",
            "     RRGGGGGGGGGGRRR    ",
            "    RRGGGGGGGGGGGGRRR   ",
            "    RRGGGGGGGGGGGGGRR   ",
            "    RRGGGGGGGGGGGGGRRR  ",
            "    RRGGGGGGGGGGGGGGRR  ",
            "   RRGGGGGGGGGGGGGGGRRR ",
            "   RRGGGGGGGGGGGGGGGGRR ",
            "   RRGGGGGGGGGGGGGGGGRR ",
            "   RRGGGGGGGGGGGGGGGGRRR",
            "  RRGGGGGGGGGGGGGGGGGGRR",
            "  RRGGGGGGGGGGGGGGGGGGRR",
            "  RRGGGGGGGGGGGGGGGGGGRR",
            "  RRGGGGGGGGGGGGGGGGGGRR",
            " RRRRRRGGGGGGGGGGGGGGGRR",
            "   RRRRRRRRGGGGGGGGGGGRR",
            "       RRRRRRRRGGGGGGGRR",
            "           RRRRRRRRGGGRR",
            "               RRRRRRRRR",
            "                   RRRR ",
        ]);
    }

    /// No radial lines should be drawn if the sweep angle is 360°.
    #[test]
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

        Sector::new(Point::new_equal(1), 11, 0.0.deg(), 360.0.deg())
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.assert_eq(&expected);
    }

    /// No radial lines should be drawn for sweep angles larger than 360°.
    #[test]
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
