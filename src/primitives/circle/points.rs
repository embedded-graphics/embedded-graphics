use crate::{
    geometry::Point,
    primitives::{circle::Circle, common::DistanceIterator},
};

/// Iterator over all points inside the circle.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Points {
    iter: DistanceIterator,
    threshold: u32,
}

impl Points {
    pub(in crate::primitives) fn new(circle: &Circle) -> Self {
        Self {
            iter: circle.distances(),
            threshold: circle.threshold(),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let threshold = self.threshold;
        self.iter
            .find(|(_, _, distance)| *distance < threshold)
            .map(|(point, ..)| point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Point, mock_display::MockDisplay, pixelcolor::BinaryColor, primitives::PointsIter,
    };

    fn test_circle(diameter: u32, pattern: &[&str]) {
        let display = MockDisplay::from_points(
            Circle::new(Point::new(0, 0), diameter).points(),
            BinaryColor::On,
        );

        display.assert_pattern(pattern);
    }

    #[test]
    fn circle_1() {
        #[rustfmt::skip]
        test_circle(1, &[
            "#",
        ],);
    }

    #[test]
    fn circle_2() {
        #[rustfmt::skip]
        test_circle(2, &[
            "##",
            "##",
        ],);
    }

    #[test]
    fn circle_3() {
        #[rustfmt::skip]
        test_circle(3, &[
            " # ",
            "###",
            " # ",
        ],);
    }

    #[test]
    fn circle_4() {
        #[rustfmt::skip]
        test_circle(4, &[
            " ## ",
            "####",
            "####",
            " ## ",
        ],);
    }

    #[test]
    fn circle_5() {
        #[rustfmt::skip]
        test_circle(5, &[
            " ### ",
            "#####",
            "#####",
            "#####",
            " ### ",
        ],);
    }

    #[test]
    fn circle_6() {
        #[rustfmt::skip]
        test_circle(6, &[
            " #### ",
            "######",
            "######",
            "######",
            "######",
            " #### ",
        ],);
    }

    #[test]
    fn circle_7() {
        #[rustfmt::skip]
        test_circle(7, &[
            "  ###  ",
            " ##### ",
            "#######",
            "#######",
            "#######",
            " ##### ",
            "  ###  ",
        ],);
    }

    #[test]
    fn circle_8() {
        #[rustfmt::skip]
        test_circle(8, &[
            "  ####  ",
            " ###### ",
            "########",
            "########",
            "########",
            "########",
            " ###### ",
            "  ####  ",
        ],);
    }

    #[test]
    fn circle_9() {
        #[rustfmt::skip]
        test_circle(9, &[
            "  #####  ",
            " ####### ",
            "#########",
            "#########",
            "#########",
            "#########",
            "#########",
            " ####### ",
            "  #####  ",
        ],);
    }
}
