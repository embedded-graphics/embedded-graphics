//! Rounded rectangle corner radii configuration

use crate::geometry::Size;

/// The definition of each corner radius for a rounded rectangle.
///
/// # Examples
///
/// ## Create a radii configuration with equal corners
///
/// This example create a `CornerRadii` instance where each corner has an equal, elliptical radius
/// of 10px x 8px.
///
/// ```rust
/// use embedded_graphics::{geometry::Size, primitives::CornerRadii};
///
/// let radii = CornerRadii::new(Size::new(10, 8));
///
/// assert_eq!(
///     radii,
///     CornerRadii {
///         top_left: Size::new(10, 8),
///         top_right: Size::new(10, 8),
///         bottom_right: Size::new(10, 8),
///         bottom_left: Size::new(10, 8),
///     }
/// );
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
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
    ///
    /// To create a `CornerRadii` instance with different radii for each corner, use the
    /// [`CornerRadiiBuilder`] builder.
    pub const fn new(radius: Size) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    /// Confine corner radii that are too large to a given bounding rectangle
    pub(in crate::primitives) fn confine(self, bounding_box: Size) -> Self {
        let mut overlap = 0;
        let mut size = 0;
        let mut corner_size = 0;

        let top_radii = self.top_left.width + self.top_right.width;
        let right_radii = self.top_right.height + self.bottom_right.height;
        let bottom_radii = self.bottom_left.width + self.bottom_right.width;
        let left_radii = self.top_left.height + self.bottom_left.height;

        let o = top_radii.saturating_sub(bounding_box.width);
        if o > overlap {
            size = bounding_box.width;
            corner_size = top_radii;
            overlap = o;
        }

        let o = right_radii.saturating_sub(bounding_box.height);
        if o > overlap {
            size = bounding_box.height;
            corner_size = right_radii;
            overlap = o;
        }

        let o = bottom_radii.saturating_sub(bounding_box.width);
        if o > overlap {
            size = bounding_box.width;
            corner_size = bottom_radii;
            overlap = o;
        }

        let o = left_radii.saturating_sub(bounding_box.height);
        if o > overlap {
            size = bounding_box.height;
            corner_size = left_radii;
            overlap = o;
        }

        if overlap > 0 && corner_size > 0 {
            Self {
                top_left: (self.top_left * size) / corner_size,
                top_right: (self.top_right * size) / corner_size,
                bottom_right: (self.bottom_right * size) / corner_size,
                bottom_left: (self.bottom_left * size) / corner_size,
            }
        } else {
            self
        }
    }
}

/// [`CornerRadii`] builder.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct CornerRadiiBuilder {
    corners: CornerRadii,
}

impl CornerRadiiBuilder {
    /// Create a new corner radii builder.
    ///
    /// All radii are defaulted to 0px x 0px.
    pub const fn new() -> Self {
        Self {
            corners: CornerRadii::new(Size::zero()),
        }
    }

    /// Set all corner radii to the same value.
    ///
    /// # Examples
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
    pub const fn all(mut self, radius: Size) -> Self {
        self.corners = CornerRadii::new(radius);

        self
    }

    /// Set the top left and top right corner radii to the same value.
    ///
    /// # Examples
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
    pub const fn top(mut self, radius: Size) -> Self {
        self.corners.top_left = radius;
        self.corners.top_right = radius;

        self
    }

    /// Set the top right and bottom right corner radii to the same value.
    ///
    /// # Examples
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
    pub const fn right(mut self, radius: Size) -> Self {
        self.corners.top_right = radius;
        self.corners.bottom_right = radius;

        self
    }

    /// Set the bottom left and bottom right corner radii to the same value.
    ///
    /// # Examples
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
    pub const fn bottom(mut self, radius: Size) -> Self {
        self.corners.bottom_left = radius;
        self.corners.bottom_right = radius;

        self
    }

    /// Set the top left and bottom left corner radii to the same value.
    ///
    /// # Examples
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
    pub const fn left(mut self, radius: Size) -> Self {
        self.corners.top_left = radius;
        self.corners.bottom_left = radius;

        self
    }

    /// Set the top left corner radius.
    ///
    /// # Examples
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
    pub const fn top_left(mut self, radius: Size) -> Self {
        self.corners.top_left = radius;

        self
    }

    /// Set the top right corner radius.
    ///
    /// # Examples
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
    pub const fn top_right(mut self, radius: Size) -> Self {
        self.corners.top_right = radius;

        self
    }

    /// Set the bottom right corner radius.
    ///
    /// # Examples
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
    pub const fn bottom_right(mut self, radius: Size) -> Self {
        self.corners.bottom_right = radius;

        self
    }

    /// Set the bottom left corner radius.
    ///
    /// # Examples
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
    pub const fn bottom_left(mut self, radius: Size) -> Self {
        self.corners.bottom_left = radius;

        self
    }

    /// Consume the builder and produce a [`CornerRadii`] configuration.
    pub const fn build(self) -> CornerRadii {
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

    #[test]
    fn corner_radii_exact_size() {
        let corners = CornerRadii {
            top_left: Size::new(10, 15),
            top_right: Size::new(10, 15),
            bottom_right: Size::new(10, 15),
            bottom_left: Size::new(10, 15),
        };

        assert_eq!(corners.confine(Size::new(20, 30)), corners);
    }

    #[test]
    fn corner_radii_single_overlap() {
        let corners = CornerRadii {
            // Create an overlap of 5px in the Y direction
            top_left: Size::new(10, 20),
            top_right: Size::new(10, 15),
            bottom_right: Size::new(10, 15),
            bottom_left: Size::new(10, 15),
        };

        assert_eq!(
            corners.confine(Size::new(20, 30)),
            CornerRadii {
                top_left: Size::new(8, 17),
                top_right: Size::new(8, 12),
                bottom_right: Size::new(8, 12),
                bottom_left: Size::new(8, 12)
            }
        );
    }

    #[test]
    fn corner_radii_1px_overlap() {
        let corners = CornerRadii {
            // 1px overlap in Y
            top_left: Size::new(10, 16),
            // 1px overlap in X
            top_right: Size::new(11, 15),
            bottom_right: Size::new(10, 15),
            bottom_left: Size::new(10, 15),
        };

        assert_eq!(
            corners.confine(Size::new(20, 30)),
            CornerRadii {
                top_left: Size::new(9, 15),
                top_right: Size::new(10, 14),
                bottom_right: Size::new(9, 14),
                bottom_left: Size::new(9, 14),
            }
        );
    }

    #[test]
    fn corner_radii_multiple_overlap() {
        let corners = CornerRadii {
            // Create an overlap of 5px in the Y direction
            top_left: Size::new(10, 20),
            top_right: Size::new(10, 15),
            // Create an overlap of 8px in the X direction
            bottom_right: Size::new(18, 15),
            bottom_left: Size::new(10, 15),
        };

        assert_eq!(
            corners.confine(Size::new(20, 30)),
            CornerRadii {
                top_left: Size::new(7, 14),
                top_right: Size::new(7, 10),
                bottom_right: Size::new(12, 10),
                bottom_left: Size::new(7, 10),
            }
        );
    }
}
