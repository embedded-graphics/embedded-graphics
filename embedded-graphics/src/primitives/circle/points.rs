use crate::{
    geometry::{Dimensions, Point},
    primitives::circle::{diameter_to_threshold, distance_iterator::DistanceIterator, Circle},
    primitives::rectangle,
    primitives::Primitive,
};

/// Iterator over all points inside the circle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    iter: DistanceIterator<rectangle::Points>,
    threshold: u32,
}

impl Points {
    pub(in crate::primitives) fn new(circle: &Circle) -> Self {
        let threshold = diameter_to_threshold(circle.diameter);

        Self {
            iter: DistanceIterator::new(circle.center_2x(), circle.bounding_box().points()),
            threshold,
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let threshold = self.threshold;
        self.iter
            .find(|(_, distance)| *distance < threshold)
            .map(|(point, _)| point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{geometry::Point, mock_display::MockDisplay, primitives::Primitive};

    fn test_circle(diameter: u32, pattern: &[&str]) {
        let display = MockDisplay::from_points(Circle::new(Point::new(0, 0), diameter).points());

        assert_eq!(display, MockDisplay::from_pattern(pattern));
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
