//! Pixel color types.
//!
//! This module contains structs for commonly used pixel color formats and
//! conversions between them. The [`raw` module] provides additional functions
//! to convert colors to and from raw data for use with images and displays.
//!
//! # Implementing custom color types
//!
//! Custom color types can be added by implementing the [`PixelColor`] trait.
//! The following example shows how to implement a new color type for a
//! 3 color EPD display.
//!
//! ```
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::egrectangle;
//!
//! /// Color with 3 states.
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! pub enum EpdColor {
//!     White,
//!     Black,
//!     Red,
//! }
//!
//! /// The `Raw` can be is set to `()` because `EpdColor` doesn't need to be
//! /// converted to raw data for the display and isn't stored in images.
//! impl PixelColor for EpdColor {
//!     type Raw = ();
//! }
//!
//! /// Mock EPD display.
//! pub struct EpdDisplay {}
//!
//! impl Drawing<EpdColor> for EpdDisplay {
//!     fn draw<T>(&mut self, item: T)
//!     where
//!         T: IntoIterator<Item = Pixel<EpdColor>>
//!     {
//!         for Pixel(point, color) in item {
//!             match color {
//!                 EpdColor::White => {} // draw white pixel at `point`
//!                 EpdColor::Black => {} // draw black pixel at `point`
//!                 EpdColor::Red => {} // draw red pixel at `point`
//!             }
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let mut display = EpdDisplay {};
//!
//!     display.draw(
//!         egrectangle!((0, 0), (100, 100), fill = Some(EpdColor::White))
//!     );
//!
//!     display.draw(
//!         egrectangle!((100, 0), (200, 100), fill = Some(EpdColor::Black))
//!     );
//!
//!     display.draw(
//!         egrectangle!((200, 0), (300, 100), fill = Some(EpdColor::Red))
//!     );
//! }
//! ```
//!
//! The implementation of the [`Drawing`] trait uses a `match` statement to
//! draw the specified color and doesn't depend on the raw data conversions,
//! see the [`raw` module] documentation for an example that uses this feature.
//!
//! [`Drawing`]: ../trait.Drawing.html
//! [`PixelColor`]: trait.PixelColor.html
//! [`Raw`]: trait.PixelColor.html#associatedtype.Raw
//! [`raw` module]: raw/index.html

mod binary_color;
mod conversion;
mod gray_color;
pub mod raw;
mod rgb_color;

pub use binary_color::*;
pub use gray_color::*;
pub use rgb_color::*;

/// Pixel color trait.
///
/// This trait is used to represent types that can be used in drawing methods.
///
/// See the [module-level documentation] for more details.
///
/// [module-level documentation]: index.html
pub trait PixelColor: Copy + PartialEq {
    /// Raw data type.
    ///
    /// Specifies the raw storage type that can be used to represent this color.
    ///
    /// See the [`raw` module documentation] for more details.
    ///
    /// [`raw` module documentation]: raw/index.html
    type Raw: raw::RawData;
}
