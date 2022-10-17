//! Pixel data arrangement.

/// Horizontal pixel arrangement.
///
/// The pixel data is arranged in rows starting from the top left corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum Horizontal {}

impl PixelArrangement for Horizontal {
    const IS_HORIZONTAL: bool = true;
}
impl private::Sealed for Horizontal {}

/// Vertical pixel arrangement.
///
/// The pixel data is arranged in rows starting from the top left corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum Vertical {}

impl PixelArrangement for Vertical {
    const IS_HORIZONTAL: bool = false;
}
impl private::Sealed for Vertical {}

/// Marker trait for pixel arrangements.
pub trait PixelArrangement: private::Sealed {
    /// TODO: docs
    const IS_HORIZONTAL: bool;
}

mod private {
    pub trait Sealed {}
}
