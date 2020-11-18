use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        common::{ClosedThickSegmentIter, Scanline, StrokeOffset},
        triangle::{
            scanline_intersections::PointType, scanline_iterator::ScanlineIterator, Triangle,
        },
        Rectangle,
    },
    style::{PrimitiveStyle, StrokeAlignment, Styled},
};

/// Pixel iterator for each pixel in the triangle border
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    lines_iter: ScanlineIterator,
    current_line: Scanline,
    current_color: Option<C>,
    fill_color: Option<C>,
    stroke_color: Option<C>,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    pub(in crate::primitives) fn new(styled: &Styled<Triangle, PrimitiveStyle<C>>) -> Self {
        let style = styled.style;

        let mut lines_iter = ScanlineIterator::new(
            &styled.primitive,
            style.stroke_width,
            StrokeOffset::from(style.stroke_alignment),
            style.fill_color.is_some(),
            &styled.bounding_box(),
        );

        let (current_line, point_type) = lines_iter
            .next()
            .unwrap_or_else(|| (Scanline::new(0), PointType::Stroke));

        let current_color = match point_type {
            PointType::Stroke => styled.style.effective_stroke_color(),
            PointType::Fill => styled.style.fill_color,
        };

        Self {
            lines_iter,
            current_line,
            current_color,
            fill_color: styled.style.fill_color,
            stroke_color: styled.style.effective_stroke_color(),
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
            if let Some(p) = self.current_line.next() {
                return Some(Pixel(p, self.current_color?));
            } else {
                let (next_line, next_type) = self.lines_iter.next()?;

                self.current_line = next_line;

                self.current_color = match next_type {
                    PointType::Stroke => self.stroke_color,
                    PointType::Fill => self.fill_color,
                };
            }
        }
    }
}

impl<C> IntoPixels for &Styled<Triangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<Self::Color>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<C> Drawable for Styled<Triangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        if !self.style.is_transparent() {
            for (line, kind) in ScanlineIterator::new(
                &self.primitive,
                self.style.stroke_width,
                StrokeOffset::from(self.style.stroke_alignment),
                self.style.fill_color.is_some(),
                &self.bounding_box(),
            ) {
                let color = match kind {
                    PointType::Stroke => self.style.effective_stroke_color(),
                    PointType::Fill => self.style.fill_color,
                };

                if let Some(color) = color {
                    let rect = line.to_rectangle();

                    if !rect.is_zero_sized() {
                        display.fill_solid(&rect, color)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<C> Dimensions for Styled<Triangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        if !self.style.is_transparent() {
            // Short circuit special cases
            if self.style.stroke_width < 2 || self.style.stroke_alignment == StrokeAlignment::Inside
            {
                return self.primitive.bounding_box();
            }

            let t = self.primitive.sorted_clockwise();

            let (min, max) = ClosedThickSegmentIter::new(
                &[t.p1, t.p2, t.p3],
                self.style.stroke_width,
                StrokeOffset::from(self.style.stroke_alignment),
            )
            .fold(
                (
                    Point::new_equal(core::i32::MAX),
                    Point::new_equal(core::i32::MIN),
                ),
                |(min, max), segment| {
                    let bb = segment.edges_bounding_box();

                    (
                        min.component_min(bb.top_left),
                        max.component_max(bb.bottom_right().unwrap_or(bb.top_left)),
                    )
                },
            );

            Rectangle::with_corners(min, max)
        } else {
            Rectangle::new(self.primitive.bounding_box().center(), Size::zero())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Drawable,
        geometry::Point,
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb565, Rgb888, RgbColor},
        primitives::{Line, Primitive},
        style::PrimitiveStyleBuilder,
        style::StrokeAlignment,
        transform::Transform,
    };

    #[test]
    fn unfilled_no_stroke_width_no_triangle() {
        let mut tri = Triangle::new(Point::new(2, 2), Point::new(4, 2), Point::new(2, 4))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 0))
            .into_pixels();

        assert_eq!(tri.next(), None);
    }

    #[test]
    fn issue_308_infinite() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);

        Triangle::new(Point::new(10, 10), Point::new(20, 30), Point::new(30, -10))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();
    }

    #[test]
    fn it_draws_filled_strokeless_tri() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);

        Triangle::new(Point::new(2, 2), Point::new(2, 4), Point::new(4, 2))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "     ",
                "     ",
                "  ###",
                "  ## ",
                "  #  ",
            ])
        );
    }

    #[test]
    fn stroke_fill_colors() {
        let mut display: MockDisplay<Rgb888> = MockDisplay::new();
        display.set_allow_overdraw(true);

        Triangle::new(Point::new(2, 2), Point::new(8, 2), Point::new(2, 8))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(Rgb888::RED)
                    .fill_color(Rgb888::GREEN)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "          ",
                "          ",
                "  RRRRRRR ",
                "  RGGGGR  ",
                "  RGGGR   ",
                "  RGGR    ",
                "  RGR     ",
                "  RR      ",
                "  R       ",
            ])
        );
    }

    #[test]
    fn off_screen() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);

        Triangle::new(Point::new(5, 5), Point::new(10, 15), Point::new(15, -5))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "          #####",
                "         ######",
                "        ###### ",
                "       ####### ",
                "      ######## ",
                "     ######### ",
                "     ########  ",
                "      #######  ",
                "      #######  ",
                "       ######  ",
                "       #####   ",
                "        ####   ",
                "        ####   ",
                "         ###   ",
                "         ##    ",
                "          #    ",
            ])
        );
    }

    #[test]
    fn styled_off_screen_still_draws_points() {
        let off_screen = Triangle::new(Point::new(10, 10), Point::new(20, 20), Point::new(30, -30))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));
        let on_screen = off_screen.translate(Point::new(0, 35));

        assert!(off_screen.into_pixels().eq(on_screen
            .into_pixels()
            .map(|Pixel(p, col)| Pixel(p - Point::new(0, 35), col))));
    }

    #[test]
    fn styled_stroke_equals_lines() {
        let triangle = Triangle::new(Point::new(10, 10), Point::new(30, 20), Point::new(20, 25));

        let styled = triangle.into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));

        let mut tri_display: MockDisplay<BinaryColor> = MockDisplay::new();
        styled.draw(&mut tri_display).unwrap();

        let mut lines_display: MockDisplay<BinaryColor> = MockDisplay::new();
        lines_display.set_allow_overdraw(true);
        Line::new(triangle.p1, triangle.p2)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut lines_display)
            .unwrap();
        Line::new(triangle.p2, triangle.p3)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut lines_display)
            .unwrap();
        Line::new(triangle.p3, triangle.p1)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut lines_display)
            .unwrap();

        assert_eq!(tri_display, lines_display);
    }

    #[test]
    fn no_stroke_overdraw() {
        let triangle = Triangle::new(Point::new(10, 10), Point::new(30, 20), Point::new(20, 25));

        let styled = triangle.into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));

        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        styled.draw(&mut display).unwrap();
    }

    #[test]
    fn bounding_box() {
        let triangle = Triangle::new(Point::new(10, 10), Point::new(30, 20), Point::new(20, 25));

        let styled = triangle.into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 20));

        let mut display = MockDisplay::new();

        display.set_allow_overdraw(true);

        styled.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), styled.bounding_box());
    }

    #[test]
    fn transparent_bounding_box() {
        let triangle = Triangle::new(Point::new(10, 10), Point::new(30, 20), Point::new(20, 25));

        let styled = triangle.into_styled::<BinaryColor>(PrimitiveStyle::new());

        assert_eq!(
            styled.bounding_box(),
            Rectangle::new(triangle.bounding_box().center(), Size::zero())
        );
    }

    #[test]
    fn outside_rendering_missing_lines() {
        let p1 = Point::new(10, 11);
        let p2 = Point::new(20, 11);
        let p3 = Point::new(8, 4);

        let styled = Triangle::new(p1, p2, p3).into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_alignment(StrokeAlignment::Outside)
                .stroke_width(5)
                .stroke_color(Rgb565::RED)
                .fill_color(Rgb565::GREEN)
                .build(),
        );

        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        styled.draw(&mut display).unwrap();

        assert_eq!(
            display,
            // Believe it or not, this is actually a triangle.
            MockDisplay::from_pattern(&[
                "          R            ",
                "         RRRR          ",
                "        RRRRRRR        ",
                "       RRRRRRRRR       ",
                "     RRRRRRRRRRRRR     ",
                "    RRRRRRRRRRRRRRRR   ",
                "    RRRRRRGRRRRRRRRRRR ",
                "     RRRRRGGGRRRRRRRRRR",
                "     RRRRRGGGGGRRRRRRRR",
                "     RRRRRGGGGGGRRRRRRR",
                "     RRRRRRGGGGGGGRRRR ",
                "      RRRRRRRRRRRRRRRR ",
                "      RRRRRRRRRRRRRRRR ",
                "      RRRRRRRRRRRRRRR  ",
                "       RRRRRRRRRRRRRR  ",
                "       RRRRRRRRRRRRRR  ",
            ])
        );
    }

    #[test]
    fn thick_stroke_only_no_overdraw() {
        let p1 = Point::new(10, 11);
        let p2 = Point::new(20, 11);
        let p3 = Point::new(8, 4);

        let styled = Triangle::new(p1, p2, p3).into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_alignment(StrokeAlignment::Outside)
                .stroke_width(5)
                .stroke_color(Rgb565::RED)
                .build(),
        );

        let mut display = MockDisplay::new();

        styled.draw(&mut display).unwrap();
    }

    #[test]
    fn inner_fill_leak() {
        let p1 = Point::new(0, 20);
        let p2 = Point::new(20, 0);
        let p3 = Point::new(14, 24);

        let styled = Triangle::new(p1, p2, p3).into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_alignment(StrokeAlignment::Inside)
                .stroke_width(5)
                .stroke_color(Rgb565::RED)
                .fill_color(Rgb565::GREEN)
                .build(),
        );

        let mut display = MockDisplay::new();

        styled.draw(&mut display).unwrap();

        assert_eq!(
            display,
            // In the failing case, there are some `G`s sitting on the end of each line that
            // shouldn't be there.
            MockDisplay::from_pattern(&[
                "                    R",
                "                   RR",
                "                  RR ",
                "                 RRR ",
                "                RRRR ",
                "               RRRRR ",
                "              RRRRR  ",
                "             RRRRRR  ",
                "            RRRRRRR  ",
                "           RRRRRRRR  ",
                "          RRRRRRRR   ",
                "         RRRRRRRRR   ",
                "        RRRRRRRRRR   ",
                "       RRRRRRRRRRR   ",
                "      RRRRRRRRRRR    ",
                "     RRRRRRRRRRRR    ",
                "    RRRRRRRGRRRRR    ",
                "   RRRRRRRGRRRRRR    ",
                "  RRRRRRRRGRRRRR     ",
                " RRRRRRRRRRRRRRR     ",
                "RRRRRRRRRRRRRRRR     ",
                "  RRRRRRRRRRRRRR     ",
                "      RRRRRRRRR      ",
                "         RRRRRR      ",
                "             RR      ",
            ])
        );
    }

    #[test]
    fn colinear() {
        let p1 = Point::new(90, 80);
        let p2 = Point::new(100, 70);
        let p3 = Point::new(95, 75);

        let t = Triangle::new(p1, p2, p3).translate(Point::new(-85, -70));

        let styled = t.into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_alignment(StrokeAlignment::Inside)
                .stroke_width(5)
                .stroke_color(Rgb565::RED)
                .fill_color(Rgb565::GREEN)
                .build(),
        );

        let mut display = MockDisplay::new();

        styled.draw(&mut display).unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "               R",
                "              R ",
                "             R  ",
                "            R   ",
                "           R    ",
                "          R     ",
                "         R      ",
                "        R       ",
                "       R        ",
                "      R         ",
                "     R          ",
            ])
        );
    }

    // Original bug has a weird "lump" drawn at one end of a colinear triangle.
    #[test]
    fn colinear_lump() {
        let p1 = Point::new(90, 80);
        let p2 = Point::new(100, 70);
        let p3 = Point::new(102, 73);

        let t = Triangle::new(p1, p2, p3).translate(Point::new(-90, -70));

        let styled = t.into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_alignment(StrokeAlignment::Inside)
                .stroke_width(5)
                .stroke_color(Rgb565::RED)
                .fill_color(Rgb565::GREEN)
                .build(),
        );

        let mut display = MockDisplay::new();

        styled.draw(&mut display).unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "          R  ",
                "         RRR ",
                "        RRRR ",
                "       RRRRRR",
                "      RRRRRR ",
                "     RRRRR   ",
                "    RRRR     ",
                "   RRR       ",
                "  RRR        ",
                " RR          ",
                "R            ",
            ])
        );
    }

    #[test]
    fn colinear_lump_2() {
        let p1 = Point::new(90, 80);
        let p2 = Point::new(100, 70);
        let p3 = Point::new(102, 73);

        let t = Triangle::new(p1, p2, p3).translate(Point::new(-90, -70));

        let styled = t.into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_alignment(StrokeAlignment::Inside)
                .stroke_width(5)
                .stroke_color(Rgb565::RED)
                .fill_color(Rgb565::GREEN)
                .build(),
        );

        let mut display = MockDisplay::new();

        styled.draw(&mut display).unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "          R  ",
                "         RRR ",
                "        RRRR ",
                "       RRRRRR",
                "      RRRRRR ",
                "     RRRRR   ",
                "    RRRR     ",
                "   RRR       ",
                "  RRR        ",
                " RR          ",
                "R            ",
            ])
        );
    }
}
