use crate::{
    draw_target::DrawTarget,
    geometry::PointExt,
    pixelcolor::PixelColor,
    primitives::{
        line::{dotted_bresenham::DottedLinePoints, thick_points::ThickPoints, Line, StrokeOffset},
        styled::{StyledDimensions, StyledDrawable, StyledPixels},
        Circle, PrimitiveStyle, Rectangle, StrokeStyle,
    },
    Pixel,
};
use az::SaturatingAs;

/// Styled line iterator.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct StyledPixelsIterator<C> {
    stroke_color: Option<C>,
    line_iter: ThickPoints,
}

impl<C: PixelColor> StyledPixelsIterator<C> {
    pub(in crate::primitives::line) fn new(primitive: &Line, style: &PrimitiveStyle<C>) -> Self {
        // Note: stroke color will be None if stroke width is 0
        let stroke_color = style.effective_stroke_color();
        let stroke_width = style.stroke_width.saturating_as();

        let mut line_iter = ThickPoints::new(primitive, stroke_width);

        if style.stroke_style == StrokeStyle::Dotted {
            match style.stroke_width {
                1 => line_iter.skip_one_point_out_of_two(),
                2 => line_iter.skip_two_points_out_of_four(),
                /* 3 => {
                    let delta = primitive.delta();
                    let (max, min) = if delta.x >= delta.y {
                        (delta.x, delta.y)
                    } else {
                        (delta.y, delta.x)
                    };

                    if 2 * min > max {
                        // When 4 bresenham lines are used, draw 4 x 2 pixels
                        line_iter.skip_two_points_out_of_four();
                    } else {
                        // When 3 bresenham lines are used, draw 3 x 3 pixels
                        line_iter.skip_three_points_out_of_six();
                    }
                } */
                // ON TRYING TO DRAW PARTIAL LINE for `thickness = 3`
                //
                // The issue with this is that it looks weird for 45° lines
                // (the dots are elongated and look like dashes).
                //
                // Since diagonal lines use 4 (and not 3) bresenham lines,
                // setting `alternate = 2` seems reasonable. That way, the dots
                // are rounder and their surface would be 8 pixels instead of 12.
                // But then, the dots seem very close with some angles.
                _ => {}
            }
        }

        Self {
            stroke_color,
            line_iter,
        }
    }
}

impl<C: PixelColor> Iterator for StyledPixelsIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Return none if stroke color is none
        let stroke_color = self.stroke_color?;

        self.line_iter
            .next()
            .map(|point| Pixel(point, stroke_color))
    }
}

impl<C: PixelColor> StyledPixels<PrimitiveStyle<C>> for Line {
    type Iter = StyledPixelsIterator<C>;

    fn pixels(&self, style: &PrimitiveStyle<C>) -> Self::Iter {
        StyledPixelsIterator::new(self, style)
    }
}

impl<C: PixelColor> StyledDrawable<PrimitiveStyle<C>> for Line {
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
        let dot_size = style.stroke_width;
        if dot_size == 0 {
            return Ok(());
        }

        let Some(stroke_color) = style.effective_stroke_color() else {
            return Ok(());
        };

        if style.stroke_style == StrokeStyle::Dotted && dot_size > 2 {
            // Draw circles along the line.
            let length = self.delta().length_squared().isqrt() as u32;
            // The gaps between dots ideally have the same size as the dots
            // (both positive and negative error is allowed).
            // The 2 endpoint dots take half the space of a regular dot.
            let nb_dots_desired = (length + dot_size) / (2 * dot_size) + 1;
            let line_points = DottedLinePoints::new(self, nb_dots_desired);
            let dot_style = PrimitiveStyle::with_fill(stroke_color);

            for position in line_points {
                Circle::with_center(position, dot_size).draw_styled(&dot_style, target)?;
            }
            Ok(())
        } else {
            // Draw line, optionally skipping some pixels.
            target.draw_iter(StyledPixelsIterator::new(self, style))
        }
    }
}

impl<C: PixelColor> StyledDimensions<PrimitiveStyle<C>> for Line {
    fn styled_bounding_box(&self, style: &PrimitiveStyle<C>) -> Rectangle {
        let (l, r) = self.extents(style.stroke_width, StrokeOffset::None);

        let min = l
            .start
            .component_min(l.end)
            .component_min(r.start)
            .component_min(r.end);
        let max = l
            .start
            .component_max(l.end)
            .component_max(r.start)
            .component_max(r.end);

        Rectangle::with_corners(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Dimensions, Point},
        mock_display::MockDisplay,
        pixelcolor::{Rgb888, RgbColor},
        primitives::{Primitive, PrimitiveStyleBuilder},
        Drawable,
    };

    #[test]
    fn bounding_box() {
        let lines = [
            (
                Line::new(Point::new(10, 20), Point::new(10, 50)),
                "vertical",
            ),
            (
                Line::new(Point::new(20, 20), Point::new(50, 20)),
                "horizontal",
            ),
            (
                Line::new(Point::new(20, 20), Point::new(55, 55)),
                "diagonal",
            ),
            (Line::new(Point::new(20, 20), Point::new(55, 55)), "thin"),
            (
                Line::new(Point::new(40, 40), Point::new(13, 14)),
                "random angle 1",
            ),
            (
                Line::new(Point::new(30, 30), Point::new(12, 53)),
                "random angle 2",
            ),
        ];

        for (line, name) in lines.iter() {
            for thickness in 1..15 {
                let style = PrimitiveStyle::with_stroke(Rgb888::RED, thickness);
                let styled = line.into_styled(style);

                let mut display = MockDisplay::new();
                styled.draw(&mut display).unwrap();
                assert_eq!(
                    display.affected_area(),
                    styled.bounding_box(),
                    "{}, {} px",
                    name,
                    thickness
                );
            }
        }
    }

    #[test]
    fn bounding_box_is_independent_of_colors() {
        let line = Line::new(Point::new(5, 5), Point::new(11, 14));

        let transparent_line = line.into_styled(
            PrimitiveStyleBuilder::<Rgb888>::new()
                .stroke_width(10)
                .build(),
        );
        let stroked_line = line.into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 10));

        assert_eq!(transparent_line.bounding_box(), stroked_line.bounding_box(),);
    }
}
