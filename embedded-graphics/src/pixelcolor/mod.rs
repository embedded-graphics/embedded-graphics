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
//! use embedded_graphics::{egrectangle, geometry::Size, prelude::*, primitive_style};
//!
//! /// Color with 3 states.
//! #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
//! impl DrawTarget<EpdColor> for EpdDisplay {
//!     type Error = core::convert::Infallible;
//!
//!     fn draw_pixel(&mut self, item: Pixel<EpdColor>) -> Result<(), Self::Error> {
//!         let Pixel(point, color) = item;
//!         match color {
//!             EpdColor::White => {} // draw white pixel at `point`
//!             EpdColor::Black => {} // draw black pixel at `point`
//!             EpdColor::Red => {}   // draw red pixel at `point`
//!         }
//!
//!         Ok(())
//!     }
//!
//!     fn size(&self) -> Size {
//!         Size::zero()
//!     }
//! }
//!
//! let mut display = EpdDisplay {};
//!
//! egrectangle!(
//!     top_left = (0, 0),
//!     bottom_right = (100, 100),
//!     style = primitive_style!(fill_color = EpdColor::White)
//! )
//! .draw(&mut display)?;
//!
//! egrectangle!(
//!     top_left = (100, 0),
//!     bottom_right = (200, 100),
//!     style = primitive_style!(fill_color = EpdColor::Black)
//! )
//! .draw(&mut display)?;
//!
//! egrectangle!(
//!     top_left = (200, 0),
//!     bottom_right = (300, 100),
//!     style = primitive_style!(fill_color = EpdColor::Red)
//! )
//! .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! The implementation of the [`DrawTarget`] trait uses a `match` statement to
//! draw the specified color and doesn't depend on the raw data conversions,
//! see the [`raw` module] documentation for an example that uses this feature.
//!
//! [`DrawTarget`]: ../trait.DrawTarget.html
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
use raw::RawData;
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
    type Raw: RawData;
}

/// Convert a [`PixelColor`] into its underlying storage type
///
/// This trait provides the `into_storage` method for implementors of [`PixelColor`]. This method
/// exposes the underlying storage value of a pixel color type.
///
/// # Examples
///
/// ## Get the `u16` representing an `Rgb565` color
///
/// This example converts an [`Rgb565`] color into its underlying `u16` represenation.
///
/// ```rust
/// use embedded_graphics::{prelude::*, pixelcolor::Rgb565};
///
/// let color = Rgb565::new(0x1f, 0x00, 0x0a);
///
/// let raw = color.into_storage();
///
/// assert_eq!(raw, 0b11111_000000_01010u16);
/// ```
///
/// [`PixelColor`]: ./trait.PixelColor.html
/// [`Rgb565`]: ./struct.Rgb565.html
pub trait IntoStorage {
    /// The underlying storage type for the pixel color
    type Storage;

    /// Convert the `PixelColor` into its raw storage form
    fn into_storage(self) -> Self::Storage;
}

impl<C> IntoStorage for C
where
    C: PixelColor,
    C::Raw: From<C>,
{
    type Storage = <<C as PixelColor>::Raw as RawData>::Storage;

    fn into_storage(self) -> Self::Storage {
        C::Raw::from(self).into_inner()
    }
}
