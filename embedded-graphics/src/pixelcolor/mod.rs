//! Pixel color

mod rgb565;

pub use self::rgb565::Rgb565;

/// Pixel color trait
///
/// This trait should be implemented for any type an embedded graphics item can be rendered into.
/// It is implemented for `u8`, `u16` and `u32` so any display drivers that use these types for
/// their pixel values should work out of the box.
///
/// If a custom pixel type is used in a driver, `PixelColor` must be implemented for it to be
/// compatible. Below is a contrived example that wraps a `u16` in a struct.
///
/// ```rust
/// use embedded_graphics::drawable::Pixel;
/// use embedded_graphics::unsignedcoord::UnsignedCoord;
/// use embedded_graphics::pixelcolor::PixelColor;
///
/// // `Copy` and `Clone` are bounds on `PixelColor` and are required.
/// // `PartialEq` is for the `assert_eq!()` later in this example.
/// // `Debug` is for convenience :)
/// #[derive(Copy, Clone, PartialEq, Debug)]
/// struct CustomPixelColor {
///     pub value: u16,
/// }
///
/// impl CustomPixelColor {
///     fn new(color: u16) -> Self {
///         CustomPixelColor { value: color }
///     }
/// }
///
/// impl PixelColor for CustomPixelColor {}
///
/// // `From<u8>` is a bound on `PixelColor` so must be implemented for your pixel colour type. You
/// // can also implement `From` for other types like `u16`, etc for convenience.
/// impl From<u8> for CustomPixelColor {
///     fn from(other: u8) -> Self {
///         CustomPixelColor {
///             value: other as u16,
///         }
///     }
/// }
///
/// fn main() {
///     let colour = CustomPixelColor::new(127u16);
///
///     assert_eq!(colour.value, 127u16);
/// }
/// ```
pub trait PixelColor: Clone + Copy + From<u8> {}

impl PixelColor for u8 {}
impl PixelColor for u16 {}
impl PixelColor for u32 {}
