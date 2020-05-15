//! Rounded rectangle corner radii configuration

use crate::geometry::Size;

/// The definition of each corner radius for a rounded rectangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CornerRadii {
    /// Top left corner radius
    pub top_left: Size,

    /// Top right corner radius
    pub top_right: Size,

    /// Bottom right corner radius
    pub bottom_right: Size,

    /// Bottom left corner radius
    pub bottom_left: Size,
}

impl CornerRadii {
    /// Create a new set of corner radii with all corners having equal values.
    pub fn new_equal(radius: Size) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    /// Confine corner radii that are too large to a given bounding rectangle
    pub(crate) fn confine(self, bounding_box: Size) -> Self {
        // Compute overlap for each edge
        let overlap_top =
            (self.top_left.width + self.top_right.width).saturating_sub(bounding_box.width);
        let overlap_right =
            (self.top_right.height + self.bottom_right.height).saturating_sub(bounding_box.height);
        let overlap_bottom =
            (self.bottom_left.width + self.bottom_right.width).saturating_sub(bounding_box.width);
        let overlap_left =
            (self.top_left.height + self.bottom_left.height).saturating_sub(bounding_box.height);

        let largest_overlap = overlap_top
            .max(overlap_right)
            .max(overlap_bottom)
            .max(overlap_left);

        if largest_overlap > 0 {
            // Reduce each corner radius by (largest overlap / 2), rounding up by adding 1
            let reduce_by = Size::new_equal((largest_overlap + 1) / 2);

            Self {
                top_left: self.top_left.saturating_sub(reduce_by),
                top_right: self.top_right.saturating_sub(reduce_by),
                bottom_right: self.bottom_right.saturating_sub(reduce_by),
                bottom_left: self.bottom_left.saturating_sub(reduce_by),
            }
        } else {
            self
        }
    }
}
