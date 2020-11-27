use crate::{
    geometry::{Dimensions, Point, Size},
    primitives::{
        ellipse::{compute_threshold, is_point_inside_ellipse, Ellipse},
        rectangle::{self, Rectangle},
    },
};

/// Iterator over all points inside the ellipse
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    iter: rectangle::Points,
    center_2x: Point,
    size_sq: Size,
    threshold: u32,
}

impl Points {
    pub(in crate::primitives) fn new(ellipse: &Ellipse) -> Self {
        let (size_sq, threshold) = compute_threshold(ellipse.size);

        Self {
            iter: ellipse.bounding_box().points(),
            center_2x: ellipse.center_2x(),
            size_sq,
            threshold,
        }
    }

    pub(in crate::primitives) fn empty() -> Self {
        Self {
            iter: Rectangle::zero().points(),
            center_2x: Point::zero(),
            size_sq: Size::zero(),
            threshold: 0,
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        for point in &mut self.iter {
            if is_point_inside_ellipse(self.size_sq, point * 2 - self.center_2x, self.threshold) {
                return Some(point);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{Circle, PointsIter};

    #[test]
    fn matches_circles_points() {
        for diameter in 0..50 {
            let circle_points = Circle::new(Point::new(0, 0), diameter).points();

            let ellipse_points =
                Ellipse::new(Point::new(0, 0), Size::new(diameter, diameter)).points();

            assert!(circle_points.eq(ellipse_points), "diameter = {}", diameter);
        }
    }
}
