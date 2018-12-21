//! Pixel color trait
//!
//! Driver implementers should implement `PixelColor` for the struct used to store pixels for the
//! target display. An example can be found in
//! [the simulator](../../simulator/struct.SimPixelColor.html). A simpler example looks like this:
//!
//! ```rust
//! use embedded_graphics::drawable::Pixel;
//! use embedded_graphics::unsignedcoord::UnsignedCoord;
//! use embedded_graphics::pixelcolor::PixelColor;
//!
//! // `Copy` and `Clone` are bounds on `PixelColor` and are required.
//! // `PartialEq` is for the `assert_eq!()` later in this example.
//! // `Debug` is for convenience :)
//! #[derive(Copy, Clone, PartialEq, Debug)]
//! struct CustomPixelColor {
//!     pub value: u16,
//! }
//!
//! impl CustomPixelColor {
//!     fn new(color: u16) -> Self {
//!         CustomPixelColor { value: color }
//!     }
//! }
//!
//! impl PixelColor for CustomPixelColor {}
//!
//! // `From<u8>` is a bound on `PixelColor` so must be implemented for your pixel colour type
//! impl From<u8> for CustomPixelColor {
//!     fn from(other: u8) -> Self {
//!         CustomPixelColor {
//!             value: other as u16,
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let colour = CustomPixelColor::new(127u16);
//!
//!     assert_eq!(colour.value, 127u16);
//! }
//! ```

/// Pixel color
pub trait PixelColor: Clone + Copy + From<u8> {}

impl PixelColor for u8 {}
impl PixelColor for u16 {}
impl PixelColor for u32 {}

/// Pixel wrapper around `u8` type
///
/// See [`PixelColor`] for usage
///
/// [`PixelColor`]: ../pixelcolor/index.html
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PixelColorU8(pub u8);

impl From<u8> for PixelColorU8 {
    fn from(other: u8) -> Self {
        PixelColorU8(other)
    }
}

impl PixelColorU8 {
    /// Get the inner value of the pixel
    ///
    /// ```
    /// #
    /// # use embedded_graphics::pixelcolor::PixelColorU8;
    /// #
    /// let color = PixelColorU8(100u8);
    /// assert_eq!(color.into_inner(), 100u8);
    /// ```
    pub fn into_inner(self) -> u8 {
        self.0
    }
}

impl PixelColor for PixelColorU8 {}

/// Pixel wrapper around `u16` type
///
/// See [`PixelColor`] for usage
///
/// [`PixelColor`]: ../pixelcolor/index.html
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PixelColorU16(pub u16);

impl From<u8> for PixelColorU16 {
    fn from(other: u8) -> Self {
        PixelColorU16(other as u16)
    }
}

impl From<u16> for PixelColorU16 {
    fn from(other: u16) -> Self {
        PixelColorU16(other)
    }
}

impl PixelColorU16 {
    /// Get the inner value of the pixel
    ///
    /// ```
    /// #
    /// # use embedded_graphics::pixelcolor::PixelColorU16;
    /// #
    /// let color = PixelColorU16(100u16);
    /// assert_eq!(color.into_inner(), 100u16);
    /// ```
    pub fn into_inner(self) -> u16 {
        self.0
    }
}

impl PixelColor for PixelColorU16 {}

/// Pixel wrapper around `u32` type
///
/// See [`PixelColor`] for usage
///
/// [`PixelColor`]: ../pixelcolor/index.html
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PixelColorU32(pub u32);

impl From<u8> for PixelColorU32 {
    fn from(other: u8) -> Self {
        PixelColorU32(other as u32)
    }
}

impl From<u16> for PixelColorU32 {
    fn from(other: u16) -> Self {
        PixelColorU32(other as u32)
    }
}

impl From<u32> for PixelColorU32 {
    fn from(other: u32) -> Self {
        PixelColorU32(other)
    }
}

impl PixelColorU32 {
    /// Get the inner value of the pixel
    ///
    /// ```
    /// #
    /// # use embedded_graphics::pixelcolor::PixelColorU32;
    /// #
    /// let color = PixelColorU32(100u32);
    /// assert_eq!(color.into_inner(), 100u32);
    /// ```
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl PixelColor for PixelColorU32 {}
