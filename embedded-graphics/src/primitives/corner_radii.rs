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

/// [`CornerRadii`] builder.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct CornerRadiiBuilder {
    corners: CornerRadii,
}

impl CornerRadiiBuilder {
    /// Create a new corner radii builder.
    ///
    /// All radii are defaulted to 0px x 0px.
    pub fn new() -> Self {
        Self {
            corners: CornerRadii::default(),
        }
    }

    /// Set all corner radii to the same value.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Size,
    ///     primitives::{CornerRadii, CornerRadiiBuilder},
    /// };
    ///
    /// let corners = CornerRadiiBuilder::new().all(Size::new(10, 20)).build();
    ///
    /// assert_eq!(
    ///     corners,
    ///     CornerRadii {
    ///         top_left: Size::new(10, 20),
    ///         top_right: Size::new(10, 20),
    ///         bottom_right: Size::new(10, 20),
    ///         bottom_left: Size::new(10, 20),
    ///     }
    /// );
    /// ```
    pub fn all(mut self, radius: Size) -> Self {
        self.corners = CornerRadii::new_equal(radius);

        self
    }

    /// Set the top left and top right corner radii to the same value.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Size,
    ///     primitives::{CornerRadii, CornerRadiiBuilder},
    /// };
    ///
    /// let corners = CornerRadiiBuilder::new().top(Size::new(10, 20)).build();
    ///
    /// assert_eq!(
    ///     corners,
    ///     CornerRadii {
    ///         top_left: Size::new(10, 20),
    ///         top_right: Size::new(10, 20),
    ///         bottom_right: Size::zero(),
    ///         bottom_left: Size::zero(),
    ///     }
    /// );
    /// ```
    pub fn top(mut self, radius: Size) -> Self {
        self.corners.top_left = radius;
        self.corners.top_right = radius;

        self
    }

    /// Set the top right and bottom right corner radii to the same value.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Size,
    ///     primitives::{CornerRadii, CornerRadiiBuilder},
    /// };
    ///
    /// let corners = CornerRadiiBuilder::new().right(Size::new(10, 20)).build();
    ///
    /// assert_eq!(
    ///     corners,
    ///     CornerRadii {
    ///         top_left: Size::zero(),
    ///         top_right: Size::new(10, 20),
    ///         bottom_right: Size::new(10, 20),
    ///         bottom_left: Size::zero(),
    ///     }
    /// );
    /// ```
    pub fn right(mut self, radius: Size) -> Self {
        self.corners.top_right = radius;
        self.corners.bottom_right = radius;

        self
    }

    /// Set the bottom left and bottom right corner radii to the same value.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Size,
    ///     primitives::{CornerRadii, CornerRadiiBuilder},
    /// };
    ///
    /// let corners = CornerRadiiBuilder::new().bottom(Size::new(10, 20)).build();
    ///
    /// assert_eq!(
    ///     corners,
    ///     CornerRadii {
    ///         top_left: Size::zero(),
    ///         top_right: Size::zero(),
    ///         bottom_right: Size::new(10, 20),
    ///         bottom_left: Size::new(10, 20),
    ///     }
    /// );
    /// ```
    pub fn bottom(mut self, radius: Size) -> Self {
        self.corners.bottom_left = radius;
        self.corners.bottom_right = radius;

        self
    }

    /// Set the top left and bottom left corner radii to the same value.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Size,
    ///     primitives::{CornerRadii, CornerRadiiBuilder},
    /// };
    ///
    /// let corners = CornerRadiiBuilder::new().left(Size::new(10, 20)).build();
    ///
    /// assert_eq!(
    ///     corners,
    ///     CornerRadii {
    ///         top_left: Size::new(10, 20),
    ///         top_right: Size::zero(),
    ///         bottom_right: Size::zero(),
    ///         bottom_left: Size::new(10, 20),
    ///     }
    /// );
    /// ```
    pub fn left(mut self, radius: Size) -> Self {
        self.corners.top_left = radius;
        self.corners.bottom_left = radius;

        self
    }

    /// Set the top left corner radius.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Size,
    ///     primitives::{CornerRadii, CornerRadiiBuilder},
    /// };
    ///
    /// let corners = CornerRadiiBuilder::new()
    ///     .top_left(Size::new(10, 20))
    ///     .build();
    ///
    /// assert_eq!(
    ///     corners,
    ///     CornerRadii {
    ///         top_left: Size::new(10, 20),
    ///         top_right: Size::zero(),
    ///         bottom_right: Size::zero(),
    ///         bottom_left: Size::zero(),
    ///     }
    /// );
    /// ```
    pub fn top_left(mut self, radius: Size) -> Self {
        self.corners.top_left = radius;

        self
    }

    /// Set the top right corner radius.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Size,
    ///     primitives::{CornerRadii, CornerRadiiBuilder},
    /// };
    ///
    /// let corners = CornerRadiiBuilder::new()
    ///     .top_right(Size::new(10, 20))
    ///     .build();
    ///
    /// assert_eq!(
    ///     corners,
    ///     CornerRadii {
    ///         top_left: Size::zero(),
    ///         top_right: Size::new(10, 20),
    ///         bottom_right: Size::zero(),
    ///         bottom_left: Size::zero(),
    ///     }
    /// );
    /// ```
    pub fn top_right(mut self, radius: Size) -> Self {
        self.corners.top_right = radius;

        self
    }

    /// Set the bottom right corner radius.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Size,
    ///     primitives::{CornerRadii, CornerRadiiBuilder},
    /// };
    ///
    /// let corners = CornerRadiiBuilder::new()
    ///     .bottom_right(Size::new(10, 20))
    ///     .build();
    ///
    /// assert_eq!(
    ///     corners,
    ///     CornerRadii {
    ///         top_left: Size::zero(),
    ///         top_right: Size::zero(),
    ///         bottom_right: Size::new(10, 20),
    ///         bottom_left: Size::zero(),
    ///     }
    /// );
    /// ```
    pub fn bottom_right(mut self, radius: Size) -> Self {
        self.corners.bottom_right = radius;

        self
    }

    /// Set the bottom left corner radius.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Size,
    ///     primitives::{CornerRadii, CornerRadiiBuilder},
    /// };
    ///
    /// let corners = CornerRadiiBuilder::new()
    ///     .bottom_left(Size::new(10, 20))
    ///     .build();
    ///
    /// assert_eq!(
    ///     corners,
    ///     CornerRadii {
    ///         top_left: Size::zero(),
    ///         top_right: Size::zero(),
    ///         bottom_right: Size::zero(),
    ///         bottom_left: Size::new(10, 20),
    ///     }
    /// );
    /// ```
    pub fn bottom_left(mut self, radius: Size) -> Self {
        self.corners.bottom_left = radius;

        self
    }

    /// Consume the builder and produce a [`CornerRadii`] configuration.
    pub fn build(self) -> CornerRadii {
        self.corners
    }
}

impl From<&CornerRadii> for CornerRadiiBuilder {
    fn from(corners: &CornerRadii) -> Self {
        Self { corners: *corners }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_radii_to_builder() {
        let radii = CornerRadii {
            top_left: Size::new(1, 2),
            top_right: Size::new(3, 4),
            bottom_right: Size::new(5, 6),
            bottom_left: Size::new(7, 8),
        };

        let builder: CornerRadiiBuilder = (&radii).into();

        assert_eq!(builder.build(), radii);
    }
}
