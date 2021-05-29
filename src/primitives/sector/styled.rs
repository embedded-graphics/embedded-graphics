use crate::{
    draw_target::DrawTarget,
    geometry::angle_consts::ANGLE_90DEG,
    geometry::{Angle, Dimensions},
    pixelcolor::PixelColor,
    primitives::{
        common::{
            DistanceIterator, LineSide, LinearEquation, PlaneSector, PointType, NORMAL_VECTOR_SCALE,
        },
        styled::{StyledDimensions, StyledDrawable, StyledPixels},
        PrimitiveStyle, Rectangle, Sector,
    },
    Pixel,
};
use az::SaturatingAs;

/// Pixel iterator for each pixel in the sector border
#[derive(Clone, PartialEq, Debug)]
pub struct StyledPixelsIterator<C> {
    iter: DistanceIterator,

    plane_sector: PlaneSector,

    outer_threshold: u32,
    inner_threshold: u32,

    stroke_threshold_inside: i32,
    stroke_threshold_outside: i32,

    bevel: Option<(BevelKind, LinearEquation)>,

    stroke_color: Option<C>,
    fill_color: Option<C>,
}

impl<C: PixelColor> StyledPixelsIterator<C> {
    fn new(primitive: &Sector, style: &PrimitiveStyle<C>) -> Self {
        let stroke_area = style.stroke_area(primitive);
        let fill_area = style.fill_area(primitive);

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

        let inside_stroke_width: i32 = style.inside_stroke_width().saturating_as();
        let outside_stroke_width: i32 = style.outside_stroke_width().saturating_as();

        let stroke_threshold_inside =
            inside_stroke_width * NORMAL_VECTOR_SCALE * 2 - NORMAL_VECTOR_SCALE;
        let stroke_threshold_outside =
            outside_stroke_width * NORMAL_VECTOR_SCALE * 2 + NORMAL_VECTOR_SCALE;

        // TODO: Polylines and sectors should use the same miter limit.
        let angle_sweep_abs = primitive.angle_sweep.abs();
        let exterior_bevel = angle_sweep_abs < Angle::from_degrees(55.0);
        let interior_bevel = angle_sweep_abs > Angle::from_degrees(360.0 - 55.0)
            && angle_sweep_abs < Angle::from_degrees(360.0);

        let bevel = if exterior_bevel || interior_bevel {
            let half_sweep = primitive.angle_start
                + Angle::from_radians(primitive.angle_sweep.to_radians() / 2.0);
            let threshold = outside_stroke_width * NORMAL_VECTOR_SCALE * 4;

            if interior_bevel {
                Some((
                    BevelKind::Interior,
                    LinearEquation::with_angle_and_distance(half_sweep - ANGLE_90DEG, threshold),
                ))
            } else {
                Some((
                    BevelKind::Exterior,
                    LinearEquation::with_angle_and_distance(half_sweep + ANGLE_90DEG, threshold),
                ))
            }
        } else {
            None
        };

        Self {
            iter,
            plane_sector,
            outer_threshold,
            inner_threshold,
            stroke_threshold_inside,
            stroke_threshold_outside,
            bevel,
            stroke_color: style.stroke_color,
            fill_color: style.fill_color,
        }
    }
}

impl<C: PixelColor> Iterator for StyledPixelsIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let outer_threshold = self.outer_threshold;

        loop {
            let (point, delta, distance) = self
                .iter
                .find(|(_, _, distance)| *distance < outer_threshold)?;

            // Check if point is inside the radial stroke lines or the fill.
            let mut point_type = match self.plane_sector.point_type(
                delta,
                self.stroke_threshold_inside,
                self.stroke_threshold_outside,
            ) {
                Some(point_type) => point_type,
                None => continue,
            };

            // Bevel the line join.
            if point_type == PointType::Stroke {
                if let Some((kind, equation)) = self.bevel {
                    if equation.check_side(delta, LineSide::Left) {
                        match kind {
                            BevelKind::Interior => point_type = PointType::Fill,
                            BevelKind::Exterior => continue,
                        }
                    }
                }
            }

            // Add the outer circular stroke.
            if point_type == PointType::Fill && distance >= self.inner_threshold {
                point_type = PointType::Stroke;
            }

            let color = match point_type {
                PointType::Stroke => self.stroke_color,
                PointType::Fill => self.fill_color,
            };

            if let Some(color) = color {
                return Some(Pixel(point, color));
            }
        }
    }
}

impl<C: PixelColor> StyledPixels<PrimitiveStyle<C>> for Sector {
    type Iter = StyledPixelsIterator<C>;

    fn pixels(&self, style: &PrimitiveStyle<C>) -> Self::Iter {
        StyledPixelsIterator::new(self, style)
    }
}

impl<C: PixelColor> StyledDrawable<PrimitiveStyle<C>> for Sector {
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
        target.draw_iter(StyledPixelsIterator::new(self, style))
    }
}

impl<C: PixelColor> StyledDimensions<PrimitiveStyle<C>> for Sector {
    // FIXME: This doesn't take into account start/end angles. This should be fixed to close #405.
    fn styled_bounding_box(&self, style: &PrimitiveStyle<C>) -> Rectangle {
        let offset = style.outside_stroke_width().saturating_as();

        self.bounding_box().offset(offset)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum BevelKind {
    Interior,
    Exterior,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{AngleUnit, Point},
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        primitives::{
            Circle, Primitive, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment, Styled,
        },
        Drawable,
    };

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

        assert!(sector.pixels().count() > 0);
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
        let transparent = Sector::with_center(CENTER, SIZE, 0.0.deg(), 90.0.deg()).into_styled(
            PrimitiveStyleBuilder::<BinaryColor>::new()
                .stroke_width(3)
                .build(),
        );

        // TODO: Uncomment when arc bounding box is fixed in #405
        // let mut display = MockDisplay::new();
        // center.draw(&mut display).unwrap();
        // assert_eq!(display.affected_area(), center.bounding_box());

        assert_eq!(center.bounding_box(), inside.bounding_box());
        assert_eq!(outside.bounding_box(), inside.bounding_box());
        assert_eq!(transparent.bounding_box(), inside.bounding_box());
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
    #[test]
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
            "                  ",
            "       RRRRR      ",
            "     RRRRRRRRR    ",
            "   RRRRRRRRRRRRR  ",
            "   RRRRGGGGGRRRR  ",
            "  RRRRGGGGGGGRRRR ",
            "  RRRGGGGGGGGGRRR ",
            " RRRGGGGGGGGGGGRRR",
            " RRRGGGGRRRRRRRRRR",
            " RRRGGGRRRRRRRRRRR",
            " RRRGGGGRRRRRRRRRR",
            " RRRGGGGGGGRRRRRRR",
            "  RRRGGGGGGGGRRRR ",
            "  RRRRGGGGGGGRRRR ",
            "   RRRRGGGGGRRRR  ",
            "   RRRRRRRRRRRRR  ",
            "     RRRRRRRRR    ",
            "       RRRRR      ",
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
