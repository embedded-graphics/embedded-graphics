use crate::{
    draw_target::{DrawTarget, DrawTargetExt},
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        common::{Scanline, StrokeOffset, ThickSegmentIter},
        polyline::{self, scanline_iterator::ScanlineIterator, Polyline},
        Primitive, Rectangle,
    },
    style::{PrimitiveStyle, Styled},
    transform::Transform,
};

impl<'a, C> Styled<Polyline<'a>, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    /// Compute the bounding box of the non-translated polyline.
    pub(in crate::primitives::polyline) fn untranslated_bounding_box(&self) -> Rectangle {
        if self.style.effective_stroke_color().is_some() && self.primitive.vertices.len() > 1 {
            let (min, max) = ThickSegmentIter::new(
                self.primitive.vertices,
                self.style.stroke_width,
                StrokeOffset::None,
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

    fn draw_thick<D>(&self, stroke_color: C, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        for line in ScanlineIterator::new(self) {
            let rect = line.to_rectangle();

            if !rect.is_zero_sized() {
                target.fill_solid(&rect, stroke_color)?;
            }
        }

        Ok(())
    }
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
pub struct StyledPixels<'a, C>
where
    C: PixelColor,
{
    stroke_color: Option<C>,
    line_iter: StyledIter<'a>,
}

impl<'a, C> StyledPixels<'a, C>
where
    C: PixelColor,
{
    pub(in crate::primitives) fn new(styled: &Styled<Polyline<'a>, PrimitiveStyle<C>>) -> Self {
        let line_iter = if styled.style.stroke_width <= 1 {
            StyledIter::Thin(styled.primitive.points())
        } else {
            let mut scanline_iter = ScanlineIterator::new(styled);
            let line_iter = scanline_iter.next().unwrap_or_else(|| Scanline::new(0));

            StyledIter::Thick {
                scanline_iter,
                line_iter,
                translate: styled.primitive.translate,
            }
        };

        StyledPixels {
            stroke_color: styled.style.effective_stroke_color(),
            line_iter,
        }
    }
}

impl<'a, C> Iterator for StyledPixels<'a, C>
where
    C: PixelColor,
{
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

impl<'a, C> IntoPixels for &Styled<Polyline<'a>, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<'a, C>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<'a, C> Drawable for Styled<Polyline<'a>, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        if let Some(stroke_color) = self.style.stroke_color {
            match self.style.stroke_width {
                0 => Ok(()),
                1 => display.draw_iter(
                    self.primitive
                        .points()
                        .map(|point| Pixel(point, stroke_color)),
                ),
                _ => {
                    if self.primitive.translate != Point::zero() {
                        self.draw_thick(
                            stroke_color,
                            &mut display.translated(self.primitive.translate),
                        )
                    } else {
                        self.draw_thick(stroke_color, display)
                    }
                }
            }
        } else {
            Ok(())
        }
    }
}

impl<C> Dimensions for Styled<Polyline<'_>, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        self.untranslated_bounding_box()
            .translate(self.primitive.translate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Drawable,
        geometry::Point,
        iterator::{IntoPixels, PixelIteratorExt},
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb565, RgbColor},
        primitives::Primitive,
        style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
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

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
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
            ])
        );
    }

    #[test]
    fn thick_stroke() {
        let mut display = MockDisplay::new();

        Polyline::new(&PATTERN)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 4))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
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
            ])
        );
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

        pl.into_pixels().draw(&mut d2).unwrap();

        assert_eq!(d1, d2);
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

            assert_eq!(
                display,
                MockDisplay::from_pattern(expected),
                "Joint {}",
                case
            );
        }
    }

    #[test]
    fn degenerate_joint() {
        let mut display = MockDisplay::new();

        Polyline::new(&[Point::new(2, 5), Point::new(25, 5), Point::new(5, 2)])
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 5))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "     ####                 ",
                "     ##########           ",
                "     #################    ",
                "  ########################",
                "  ########################",
                "  ########################",
                "  ########################",
                "  ########################",
            ])
        );
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

            assert_eq!(display, expected_display, "{:?}", alignment);
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
            .into_pixels()
            .draw(&mut display)
            .unwrap();

        assert_eq!(display, expected_display);
    }

    #[test]
    fn empty_styled_iterators() {
        let points: [Point; 3] = [Point::new(2, 5), Point::new(3, 4), Point::new(4, 3)];

        // No stroke width = no pixels
        assert!(Polyline::new(&points)
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 0))
            .into_pixels()
            .eq(core::iter::empty()));

        // No stroke color = no pixels
        assert!(Polyline::new(&points)
            .into_styled(
                PrimitiveStyleBuilder::<Rgb565>::new()
                    .stroke_width(1)
                    .build()
            )
            .into_pixels()
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
            pl.into_styled(PrimitiveStyle::<Rgb565>::new())
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
            .into_pixels()
            .draw(&mut display)
            .unwrap();

        assert_eq!(display, MockDisplay::new());
    }
}
