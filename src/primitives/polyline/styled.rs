use crate::{
    draw_target::{DrawTarget, DrawTargetExt},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::{
        common::{Scanline, StrokeOffset, ThickSegmentIter},
        polyline::{self, scanline_iterator::ScanlineIterator, Polyline},
        styled::{StyledDimensions, StyledDrawable, StyledPixels},
        PointsIter, PrimitiveStyle, Rectangle,
    },
    transform::Transform,
    Pixel,
};

/// Compute the bounding box of the non-translated polyline.
pub(in crate::primitives::polyline) fn untranslated_bounding_box<C: PixelColor>(
    primitive: &Polyline,
    style: &PrimitiveStyle<C>,
) -> Rectangle {
    if style.effective_stroke_color().is_some() && primitive.vertices.len() > 1 {
        let (min, max) =
            ThickSegmentIter::new(primitive.vertices, style.stroke_width, StrokeOffset::None).fold(
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
        Rectangle::new(primitive.bounding_box().center(), Size::zero())
    }
}

fn draw_thick<D>(
    polyline: &Polyline,
    style: &PrimitiveStyle<D::Color>,
    stroke_color: D::Color,
    target: &mut D,
) -> Result<(), D::Error>
where
    D: DrawTarget,
{
    for line in ScanlineIterator::new(polyline, style) {
        let rect = line.to_rectangle();

        if !rect.is_zero_sized() {
            target.fill_solid(&rect, stroke_color)?;
        }
    }

    Ok(())
}

#[derive(Clone, Debug)]
enum StyledIter<'a> {
    Thin(polyline::Points<'a>),
    Thick {
        scanline_iter: ScanlineIterator<'a>,
        line_iter: Scanline,
        translate: Point,
    },
}

/// Pixel iterator for each pixel in the line
#[derive(Clone, Debug)]
pub struct StyledPixelsIterator<'a, C> {
    stroke_color: Option<C>,
    line_iter: StyledIter<'a>,
}

impl<'a, C: PixelColor> StyledPixelsIterator<'a, C> {
    pub(in crate::primitives) fn new(primitive: &Polyline<'a>, style: &PrimitiveStyle<C>) -> Self {
        let line_iter = if style.stroke_width <= 1 {
            StyledIter::Thin(primitive.points())
        } else {
            let mut scanline_iter = ScanlineIterator::new(primitive, style);
            let line_iter = scanline_iter
                .next()
                .unwrap_or_else(|| Scanline::new_empty(0));

            StyledIter::Thick {
                scanline_iter,
                line_iter,
                translate: primitive.translate,
            }
        };

        StyledPixelsIterator {
            stroke_color: style.effective_stroke_color(),
            line_iter,
        }
    }
}

impl<C: PixelColor> Iterator for StyledPixelsIterator<'_, C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Return none if stroke color is none
        let stroke_color = self.stroke_color?;

        match self.line_iter {
            StyledIter::Thin(ref mut it) => it.next(),
            StyledIter::Thick {
                ref mut scanline_iter,
                ref mut line_iter,
                translate,
            } => {
                // We've got a line to iterate over, so get it's next pixel.
                if let Some(p) = line_iter.next() {
                    Some(p)
                }
                // Finished this line. Get the next one from the scanline iterator.
                else {
                    *line_iter = scanline_iter.next()?;

                    line_iter.next()
                }
                .map(|p| p + translate)
            }
        }
        .map(|point| Pixel(point, stroke_color))
    }
}

impl<'a, C: PixelColor> StyledPixels<PrimitiveStyle<C>> for Polyline<'a> {
    type Iter = StyledPixelsIterator<'a, C>;

    fn pixels(&self, style: &PrimitiveStyle<C>) -> Self::Iter {
        StyledPixelsIterator::new(self, style)
    }
}

impl<C: PixelColor> StyledDrawable<PrimitiveStyle<C>> for Polyline<'_> {
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
        if let Some(stroke_color) = style.stroke_color {
            match style.stroke_width {
                0 => Ok(()),
                1 => target.draw_iter(self.points().map(|point| Pixel(point, stroke_color))),
                _ => {
                    if self.translate != Point::zero() {
                        draw_thick(
                            self,
                            style,
                            stroke_color,
                            &mut target.translated(self.translate),
                        )
                    } else {
                        draw_thick(self, style, stroke_color, target)
                    }
                }
            }
        } else {
            Ok(())
        }
    }
}

impl<C: PixelColor> StyledDimensions<PrimitiveStyle<C>> for Polyline<'_> {
    fn styled_bounding_box(&self, style: &PrimitiveStyle<C>) -> Rectangle {
        untranslated_bounding_box(self, style).translate(self.translate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Point,
        iterator::PixelIteratorExt,
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb565, RgbColor},
        primitives::{Primitive, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
        Drawable,
    };

    // Smaller test pattern for mock display
    pub(in crate::primitives::polyline) const PATTERN: [Point; 4] = [
        Point::new(5, 10),
        Point::new(13, 5),
        Point::new(20, 10),
        Point::new(30, 5),
    ];

    #[test]
    fn one_px_stroke() {
        let mut display = MockDisplay::new();

        Polyline::new(&PATTERN)
            .translate(Point::new(-5, -5))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "        #                #",
            "      ## ##            ## ",
            "     #     #         ##   ",
            "   ##       #      ##     ",
            " ##          ##  ##       ",
            "#              ##         ",
        ]);
    }

    #[test]
    fn one_px_stroke_translated() {
        let mut display = MockDisplay::new();

        Polyline::new(&PATTERN)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                               ",
            "                               ",
            "                               ",
            "                               ",
            "                               ",
            "             #                #",
            "           ## ##            ## ",
            "          #     #         ##   ",
            "        ##       #      ##     ",
            "      ##          ##  ##       ",
            "     #              ##         ",
        ]);
    }

    #[test]
    fn thick_stroke() {
        let mut display = MockDisplay::new();

        Polyline::new(&PATTERN)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 4))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "                                ",
            "                                ",
            "                                ",
            "             #                  ",
            "           #####            ##  ",
            "          #######         ##### ",
            "        ##########      ####### ",
            "       #############  ##########",
            "     ######## ################# ",
            "    #######     #############   ",
            "     ####         #########     ",
            "      #            ######       ",
            "                     ##         ",
        ]);
    }

    #[test]
    fn thick_stroke_translated() {
        let mut display = MockDisplay::new();

        let styled = Polyline::new(&PATTERN)
            .translate(Point::new(-4, -3))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 4));

        assert_eq!(
            styled.bounding_box(),
            Rectangle::new(Point::zero(), Size::new(28, 10))
        );

        styled.draw(&mut display).unwrap();

        display.assert_pattern(&[
            "         #                  ",
            "       #####            ##  ",
            "      #######         ##### ",
            "    ##########      ####### ",
            "   #############  ##########",
            " ######## ################# ",
            "#######     #############   ",
            " ####         #########     ",
            "  #            ######       ",
            "                 ##         ",
        ]);
    }

    #[test]
    fn thick_stroke_points() {
        let mut d1 = MockDisplay::new();
        let mut d2 = MockDisplay::new();

        let pl = Polyline::new(&PATTERN)
            .translate(Point::new(2, 3))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 4));

        pl.draw(&mut d1).unwrap();

        pl.pixels().draw(&mut d2).unwrap();

        d1.assert_eq(&d2);
    }

    #[test]
    fn joints() {
        let cases: [(&str, &[Point], &[&str]); 4] = [
            (
                "Bevel with outside on right",
                &[Point::new(0, 6), Point::new(25, 6), Point::new(3, 1)],
                &[
                    "   ###                    ",
                    "   #######                ",
                    "   ###########            ",
                    "   ################       ",
                    "#######################   ",
                    "##########################",
                    "##########################",
                    "##########################",
                ],
            ),
            (
                "Bevel with outside on left",
                &[Point::new(0, 2), Point::new(20, 2), Point::new(3, 8)],
                &[
                    "##################### ",
                    "##################### ",
                    "##################### ",
                    "######################",
                    "          ############",
                    "        ############  ",
                    "     ############     ",
                    "   ###########        ",
                    "   #########          ",
                    "    #####             ",
                    "    ##                ",
                ],
            ),
            (
                "Miter with outside on right",
                &[Point::new(0, 6), Point::new(10, 6), Point::new(3, 1)],
                &[
                    "    #          ",
                    "   ####        ",
                    "  ######       ",
                    "   ######      ",
                    "###########    ",
                    "############   ",
                    "############## ",
                    "###############",
                ],
            ),
            (
                "Miter with outside on left",
                &[Point::new(0, 2), Point::new(10, 2), Point::new(3, 8)],
                &[
                    "################",
                    "############### ",
                    "##############  ",
                    "############    ",
                    "      #####     ",
                    "    ######      ",
                    "   ######       ",
                    "  ######        ",
                    "   ###          ",
                    "    #           ",
                ],
            ),
        ];

        for (case, points, expected) in cases.iter() {
            let mut display = MockDisplay::new();

            Polyline::new(points)
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 4))
                .draw(&mut display)
                .unwrap();

            display.assert_pattern_with_message(expected, |f| write!(f, "Join {}", case));
        }
    }

    #[test]
    fn degenerate_joint() {
        let mut display = MockDisplay::new();

        Polyline::new(&[Point::new(2, 5), Point::new(25, 5), Point::new(5, 2)])
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 5))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "     ####                 ",
            "     ##########           ",
            "     #################    ",
            "  ########################",
            "  ########################",
            "  ########################",
            "  ########################",
            "  ########################",
        ]);
    }

    #[test]
    fn alignment_has_no_effect() {
        let base_style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(3);

        let mut expected_display = MockDisplay::new();

        Polyline::new(&PATTERN)
            .into_styled(base_style.build())
            .draw(&mut expected_display)
            .unwrap();

        for alignment in [StrokeAlignment::Inside, StrokeAlignment::Outside].iter() {
            let mut display = MockDisplay::new();

            Polyline::new(&PATTERN)
                .into_styled(base_style.stroke_alignment(*alignment).build())
                .draw(&mut display)
                .unwrap();

            display.assert_eq_with_message(&expected_display, |f| write!(f, "{:?}", alignment));
        }
    }

    #[test]
    fn thick_points() {
        let base_style = PrimitiveStyle::with_stroke(BinaryColor::On, 5);

        let mut expected_display = MockDisplay::new();
        let mut display = MockDisplay::new();

        Polyline::new(&PATTERN)
            .into_styled(base_style)
            .draw(&mut expected_display)
            .unwrap();

        Polyline::new(&PATTERN)
            .into_styled(base_style)
            .pixels()
            .draw(&mut display)
            .unwrap();

        display.assert_eq(&expected_display);
    }

    #[test]
    fn empty_styled_iterators() {
        let points: [Point; 3] = [Point::new(2, 5), Point::new(3, 4), Point::new(4, 3)];

        // No stroke width = no pixels
        assert!(Polyline::new(&points)
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 0))
            .pixels()
            .eq(core::iter::empty()));

        // No stroke color = no pixels
        assert!(Polyline::new(&points)
            .into_styled(
                PrimitiveStyleBuilder::<Rgb565>::new()
                    .stroke_width(1)
                    .build()
            )
            .pixels()
            .eq(core::iter::empty()));
    }

    #[test]
    fn bounding_box() {
        let pl = Polyline::new(&PATTERN);

        let styled = pl.into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 5));

        let mut display = MockDisplay::new();
        styled.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), styled.bounding_box());

        assert_eq!(
            pl.into_styled(
                PrimitiveStyleBuilder::<Rgb565>::new()
                    .stroke_width(5)
                    .build()
            )
            .bounding_box(),
            Rectangle::new(pl.bounding_box().center(), Size::zero()),
            "transparent"
        );

        assert_eq!(
            pl.into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
                .bounding_box(),
            Rectangle::new(pl.bounding_box().center(), Size::zero()),
            "filled"
        );

        assert_eq!(
            Polyline::new(&PATTERN[0..2])
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 5))
                .bounding_box(),
            Rectangle::new(Point::new(4, 3), Size::new(11, 9)),
            "two points"
        );

        assert_eq!(
            Polyline::new(&PATTERN[0..1])
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 5))
                .bounding_box(),
            Rectangle::new(Point::new(5, 10), Size::zero()),
            "one point"
        );
    }

    #[test]
    fn translated_bounding_box() {
        let by = Point::new(10, 12);
        let pl = Polyline::new(&PATTERN).translate(by);

        assert_eq!(
            pl.into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
                .bounding_box(),
            Rectangle::new(Point::new(15, 17), Size::new(26, 6)),
            "thin translated"
        );

        assert_eq!(
            pl.into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 5))
                .bounding_box(),
            Rectangle::new(Point::new(14, 14), Size::new(28, 11)),
            "thick translated"
        );

        assert_eq!(
            Polyline::new(&PATTERN[0..2])
                .translate(by)
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 5))
                .bounding_box(),
            Rectangle::new(Point::new(14, 15), Size::new(11, 9)),
            "two points translated"
        );

        assert_eq!(
            Polyline::new(&PATTERN[0..1])
                .translate(by)
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 5))
                .bounding_box(),
            Rectangle::new(Point::new(15, 22), Size::zero()),
            "one point translated"
        );
    }

    #[test]
    fn empty_line_no_draw() {
        let mut display = MockDisplay::new();

        Polyline::new(&[])
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 2))
            .pixels()
            .draw(&mut display)
            .unwrap();

        display.assert_eq(&MockDisplay::new());
    }

    #[test]
    fn issue_489_overdraw() {
        let mut display = MockDisplay::new();

        // Panics if pixel is drawn twice.
        Polyline::new(&[Point::new(10, 5), Point::new(5, 10), Point::new(10, 10)])
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 5))
            .draw(&mut display)
            .unwrap();
    }

    #[test]
    fn issue_471_spurs() {
        let points = [Point::new(10, 70), Point::new(20, 50), Point::new(31, 30)];

        let line = Polyline::new(&points)
            .translate(Point::new(0, -15))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 18));

        let bb = line.bounding_box();

        // Check bounding box is correct
        assert_eq!(bb, Rectangle::new(Point::new(1, 11), Size::new(39, 49)));

        let mut display = MockDisplay::new();
        line.draw(&mut display).unwrap();

        // Check no pixels are drawn outside bounding box
        assert_eq!(display.affected_area(), bb);
    }

    #[test]
    // FIXME: Un-ignore when more polyline spur fixes are made. This test checks for a smaller
    // spur created from a different set of points than `issue_471_spurs`.
    #[ignore]
    fn issue_471_spurs_2() {
        let points = [Point::new(13, 65), Point::new(20, 50), Point::new(31, 30)];

        let line = Polyline::new(&points)
            .translate(Point::new(0, -15))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 18));

        let bb = line.bounding_box();

        // Check bounding box is correct
        assert_eq!(bb, Rectangle::new(Point::new(4, 26), Size::new(36, 44)));

        let mut display = MockDisplay::new();
        line.draw(&mut display).unwrap();

        // Check no pixels are drawn outside bounding box
        assert_eq!(display.affected_area(), bb);
    }
}
