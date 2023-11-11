//! Geometry module.

mod point;
mod size;

pub use point::Point;
pub use size::Size;

use crate::primitives::Rectangle;

/// Adds the ability to get the bounding box of an item.
///
/// The exact definition of the bounding box depends on the item:
///
/// * Primitives ([`Rectangle`], [`Circle`], ...)
///
///    For unstyled [primitives] the bounding box is defined as the smallest rectangle that surrounds the entire primitive.
/// * Styled primitives and other [`Drawable`]s ([`Image`], [`Text`], ...)
///
///    The bounding box of a drawable is defined as the smallest rectangle that contains all drawn pixels.
///    While all builtin [`Drawable`]s in embedded-graphics provide an implementation of this trait, this might
///    not be true for third party drawables.
///
///    Note that a styled primitive can have a different bounding box than the underlying unstyled primitive;
///    depending on the stroke width and alignment the bounding box of the styled primitive may be larger.
/// * [`DrawTarget`]s (displays, simulator, ...)
///
///    The bounding box of a draw target is defined as the area that should be used for drawing operations.
///    For most display drivers the top left corner of the bounding box will be at the origin but other draw targets
///    can have different positions of the top left corner.
///
/// The bounding box will be returned as a [`Rectangle`]. The methods provided by [`Rectangle`] make
/// it easy to implement additional functions like hit testing (by using [`contains`]) or drawing a focus
/// rectangle around a drawable (by converting the rectangle into a [`Styled`]).
///
/// # Implementation notes
///
/// `Dimensions` should be implemented for `Drawable`s if the bounding box is known before [`Drawable::draw`] is
/// executed. The implementation must return a rectangle that contains all drawn pixels.
/// [`MockDisplay::affected_area`] can be a used in unit tests to make sure a drawable returns a bounding box with
/// the correct dimensions.
///
/// [`DrawTarget`]s  (display drivers, etc) are required to implement `Dimensions`. The
/// implementation must return a rectangle representing the drawing area.
///
/// [`Drawable`]: super::Drawable
/// [`Drawable::draw`]: super::Drawable::draw
/// [`DrawTarget`]: super::draw_target::DrawTarget
/// [`ImageDrawable`]: super::image::ImageDrawable
/// [`Rectangle`]: super::primitives::rectangle::Rectangle
/// [`points`]: super::primitives::PointsIter
/// [`MockDisplay::affected_area`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/mock_display/struct.MockDisplay.html#method.affected_area
/// [`contains`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/trait.ContainsPoint.html#tymethod.contains
/// [primitives]: https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/index.html
/// [`Circle`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/circle/struct.Circle.html
/// [`Image`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.Image.html
/// [`Text`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/fonts/struct.Text.html
/// [`Styled`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/style/styled/struct.Styled.html
pub trait Dimensions {
    /// Returns the bounding box.
    fn bounding_box(&self) -> Rectangle;
}

/// Anchor point.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum AnchorPoint {
    /// Top left.
    TopLeft,
    /// Top center.
    TopCenter,
    /// Top right.
    TopRight,
    /// Center left.
    CenterLeft,
    /// Center.
    Center,
    /// Center right.
    CenterRight,
    /// Bottom left.
    BottomLeft,
    /// Bottom center.
    BottomCenter,
    /// Bottom right.
    BottomRight,
}

impl AnchorPoint {
    /// Creates an anchor point from an X and Y component.
    pub fn from_xy(x: AnchorX, y: AnchorY) -> Self {
        match (y, x) {
            (AnchorY::Top, AnchorX::Left) => AnchorPoint::TopLeft,
            (AnchorY::Top, AnchorX::Center) => AnchorPoint::TopCenter,
            (AnchorY::Top, AnchorX::Right) => AnchorPoint::TopRight,
            (AnchorY::Center, AnchorX::Left) => AnchorPoint::CenterLeft,
            (AnchorY::Center, AnchorX::Center) => AnchorPoint::Center,
            (AnchorY::Center, AnchorX::Right) => AnchorPoint::CenterRight,
            (AnchorY::Bottom, AnchorX::Left) => AnchorPoint::BottomLeft,
            (AnchorY::Bottom, AnchorX::Center) => AnchorPoint::BottomCenter,
            (AnchorY::Bottom, AnchorX::Right) => AnchorPoint::BottomRight,
        }
    }

    /// Returns the X axis component.
    pub fn x(self) -> AnchorX {
        match self {
            AnchorPoint::TopLeft | AnchorPoint::CenterLeft | AnchorPoint::BottomLeft => {
                AnchorX::Left
            }
            AnchorPoint::TopCenter | AnchorPoint::Center | AnchorPoint::BottomCenter => {
                AnchorX::Center
            }
            AnchorPoint::TopRight | AnchorPoint::CenterRight | AnchorPoint::BottomRight => {
                AnchorX::Right
            }
        }
    }

    /// Returns the Y axis component.
    pub fn y(self) -> AnchorY {
        match self {
            AnchorPoint::TopLeft | AnchorPoint::TopCenter | AnchorPoint::TopRight => AnchorY::Top,
            AnchorPoint::CenterLeft | AnchorPoint::Center | AnchorPoint::CenterRight => {
                AnchorY::Center
            }
            AnchorPoint::BottomLeft | AnchorPoint::BottomCenter | AnchorPoint::BottomRight => {
                AnchorY::Bottom
            }
        }
    }
}

/// X axis anchor point.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum AnchorX {
    /// Left.
    Left,
    /// Center.
    Center,
    /// Right.
    Right,
}

/// Y axis anchor point.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum AnchorY {
    /// Top.
    Top,
    /// Center.
    Center,
    /// Bottom.
    Bottom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const ANCHOR_TESTS: &[((AnchorY, AnchorX), AnchorPoint)] = &[
        ((AnchorY::Top, AnchorX::Left), AnchorPoint::TopLeft),
        ((AnchorY::Top, AnchorX::Center), AnchorPoint::TopCenter),
        ((AnchorY::Top, AnchorX::Right), AnchorPoint::TopRight),
        ((AnchorY::Center, AnchorX::Left), AnchorPoint::CenterLeft),
        ((AnchorY::Center, AnchorX::Center), AnchorPoint::Center),
        ((AnchorY::Center, AnchorX::Right), AnchorPoint::CenterRight),
        ((AnchorY::Bottom, AnchorX::Left), AnchorPoint::BottomLeft),
        ((AnchorY::Bottom, AnchorX::Center), AnchorPoint::BottomCenter),
        ((AnchorY::Bottom, AnchorX::Right), AnchorPoint::BottomRight),
    ];

    #[test]
    fn anchor_conversion() {
        for ((y, x), p) in ANCHOR_TESTS.iter().copied() {
            assert_eq!(p.x(), x);
            assert_eq!(p.y(), y);

            assert_eq!(AnchorPoint::from_xy(x, y), p);
        }
    }
}
