use crate::drawable::Pixel;
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::primitives::line::{self, Line};
use crate::primitives::triangle::sort_two_yx;
use crate::primitives::triangle::sort_yx;
use crate::primitives::triangle::IterState;
use crate::primitives::triangle::Triangle;
use crate::primitives::Primitive;
use crate::style::PrimitiveStyle;
use crate::style::Styled;

/// Pixel iterator for each pixel in the triangle border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledTriangleIterator<C: PixelColor>
where
    C: PixelColor,
{
    line_a: line::Points,
    line_b: line::Points,
    line_c: line::Points,
    cur_ac: Option<Point>,
    cur_b: Option<Point>,
    next_ac: Option<Point>,
    next_b: Option<Point>,
    x: i32,
    max_y: i32,
    min_y: i32,
    style: PrimitiveStyle<C>,
}

impl<C> StyledTriangleIterator<C>
where
    C: PixelColor,
{
    pub(crate) fn new(styled: &Styled<Triangle, PrimitiveStyle<C>>) -> Self {
        let (v1, v2, v3) = sort_yx(
            styled.primitive.p1,
            styled.primitive.p2,
            styled.primitive.p3,
        );

        let mut line_a = Line::new(v1, v2).points();
        let mut line_b = Line::new(v1, v3).points();
        let mut line_c = Line::new(v2, v3).points();

        let next_ac = line_a.next().or_else(|| line_c.next());
        let next_b = line_b.next();

        StyledTriangleIterator {
            line_a,
            line_b,
            line_c,
            cur_ac: None,
            cur_b: None,
            next_ac,
            next_b,
            x: 0,
            min_y: v1.y,
            max_y: v3.y,
            style: styled.style,
        }
    }
    fn update_ac(&mut self) -> IterState {
        if let Some(ac) = self.next_ac {
            self.cur_ac = Some(ac);
            self.next_ac = self.line_a.next().or_else(|| self.line_c.next());
            self.x = 0;
            IterState::Border(ac)
        } else {
            IterState::None
        }
    }

    fn update_b(&mut self) -> IterState {
        if let Some(b) = self.next_b {
            self.cur_b = Some(b);
            self.next_b = self.line_b.next();
            self.x = 0;
            IterState::Border(b)
        } else {
            IterState::None
        }
    }

    fn points(&mut self) -> IterState {
        match (self.cur_ac, self.cur_b) {
            // Point of ac line or b line is missing
            (None, _) => self.update_ac(),
            (_, None) => self.update_b(),
            // Both points are present
            (Some(ac), Some(b)) => {
                match (self.next_ac, self.next_b) {
                    (Some(n_ac), Some(n_b)) => {
                        // If y component differs, take new points from edge until both side have
                        // the same y
                        if n_ac.y < n_b.y {
                            self.update_ac()
                        } else if n_ac.y > n_b.y {
                            self.update_b()
                        } else {
                            let (l, r) = sort_two_yx(n_ac, n_b);
                            IterState::LeftRight(l, r)
                        }
                    }
                    (None, Some(_)) => self.update_b(),
                    (Some(_), None) => self.update_ac(),
                    (None, None) => {
                        let (l, r) = sort_two_yx(ac, b);
                        IterState::LeftRight(l, r)
                    }
                }
            }
        }
    }
}

impl<C> Iterator for StyledTriangleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.style.stroke_color.is_none() && self.style.fill_color.is_none() {
            return None;
        }

        loop {
            match self.points() {
                IterState::Border(point) => {
                    // Draw edges of the triangle
                    if self.style.stroke_width > 0 {
                        if let Some(stroke_color) = self.style.stroke_color {
                            self.x += 1;
                            return Some(Pixel(point, stroke_color));
                        }
                    } else if let Some(fill_color) = self.style.fill_color {
                        self.x += 1;
                        return Some(Pixel(point, fill_color));
                    }
                }
                IterState::LeftRight(l, r) => {
                    // Fill the space between the left and right points
                    if let Some(color) = self.style.fill_color {
                        if l.x + self.x < r.x {
                            let point = Point::new(l.x + self.x, l.y);
                            self.x += 1;
                            return Some(Pixel(point, color));
                        } else if l.x + self.x >= r.x {
                            // We reached the right edge, move on to next row
                            self.cur_ac = None;
                            self.cur_b = None;
                        }
                    } else {
                        // We don't want to fill the triangle
                        self.cur_ac = None;
                        self.cur_b = None;
                    }
                }
                IterState::None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Drawable,
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        style::PrimitiveStyleBuilder,
        transform::Transform,
    };

    #[test]
    fn unfilled_no_stroke_width_no_triangle() {
        let mut tri = Triangle::new(Point::new(2, 2), Point::new(4, 2), Point::new(2, 4))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 0))
            .into_iter();

        assert_eq!(tri.next(), None);
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
    fn styled_off_screen_still_draws_points() {
        let off_screen = Triangle::new(Point::new(10, 10), Point::new(20, 20), Point::new(30, -30))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));
        let on_screen = off_screen.translate(Point::new(0, 35));

        assert!(off_screen.into_iter().eq(on_screen
            .into_iter()
            .map(|Pixel(p, col)| Pixel(p - Point::new(0, 35), col))));
    }
}
