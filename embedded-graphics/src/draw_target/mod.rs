//! A target for embedded-graphics drawing operations.

mod clipped;
mod cropped;
mod translated;

use crate::{
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    primitives::{rectangle::Rectangle, Primitive},
    Pixel,
};

pub use clipped::Clipped;
pub use cropped::Cropped;
pub use translated::Translated;

/// A target for embedded-graphics drawing operations.
///
/// The `DrawTarget` trait is used to add embedded-graphics support to a display
/// driver or similar targets like framebuffers or image files.
/// Targets are required to at least implement the [`size`] and [`draw_iter`] methods. All other
/// methods provide default implementations which use these methods internally.
///
/// Because the default implementations cannot use features specific to the target hardware they
/// can be overridden to improve performance. These target specific implementations might, for
/// example, use hardware accelerated drawing operations provided by a display controller or
/// specialized hardware modules in a microcontroller.
///
/// Note that some displays require a "flush" operation to write changes from a framebuffer to the
/// display. See docs associated with the chosen display driver for details on how to update the
/// display.
///
/// # Examples
///
/// ## Minimum implementation
///
/// In this example `DrawTarget` is implemented for an an imaginary 64px x 64px 8-bit grayscale display
/// that is connected using a simplified SPI interface. Because the hardware doesn't support any
/// acceleration only the two required methods [`size`] and [`draw_iter`] need to be implemented.
///
/// To reduce the overhead caused by communicating with the display for each drawing operation
/// the display driver uses and framebuffer to store the pixel data in memory. This way all drawing
/// operations can be executed in local memory and the actual display is only updated on demand
/// by calling the `flush` method.
///
/// Because all drawing operations are using a local framebuffer no communication error can occur
/// while they are executed and the [`Error` type] can be set to `core::convert::Infallible`.
///
/// ```rust
/// use core::convert::TryInto;
/// use embedded_graphics::{
///     pixelcolor::{Gray8, GrayColor},
///     prelude::*,
///     primitives::Circle,
///     style::PrimitiveStyle,
/// };
/// #
/// # struct SPI1;
/// #
/// # impl SPI1 {
/// #     pub fn send_bytes(&self, buf: &[u8]) -> Result<(), CommError> {
/// #         Ok(())
/// #     }
/// # }
/// #
///
/// /// SPI communication error
/// #[derive(Debug)]
/// struct CommError;
///
/// /// A fake 64px x 64px display.
/// struct ExampleDisplay {
///     /// The framebuffer with one `u8` value per pixel.
///     framebuffer: [u8; 64 * 64],
///
///     /// The interface to the display controller.
///     iface: SPI1,
/// }
///
/// impl ExampleDisplay {
///     /// Updates the display from the framebuffer.
///     pub fn flush(&self) -> Result<(), CommError> {
///         self.iface.send_bytes(&self.framebuffer)
///     }
/// }
///
/// impl DrawTarget for ExampleDisplay {
///     type Color = Gray8;
///     // `ExampleDisplay` uses a framebuffer and doesn't need to communicate with the display
///     // controller to draw pixel, which means that drawing operations can never fail. To reflect
///     // this the type `Infallible` was chosen as the `Error` type.
///     type Error = core::convert::Infallible;
///
///     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
///     where
///         I: IntoIterator<Item = Pixel<Self::Color>> {
///         for Pixel(coord, color) in pixels.into_iter() {
///             // Check if the pixel coordinates are out of bounds (negative or greater than
///             // (63,63)). `DrawTarget` implementation are required to discard any out of bounds
///             // pixels without returning an error or causing a panic.
///             if let Ok((x @ 0..=63, y @ 0..=63)) = coord.try_into() {
///                 // Calculate the index in the framebuffer.
///                 let index: u32 = x + y * 64;
///                 self.framebuffer[index as usize] = color.luma();
///             }
///         }
///
///         Ok(())
///     }
/// }
///
/// impl OriginDimensions for ExampleDisplay {
///     fn size(&self) -> Size {
///         Size::new(64, 64)
///     }
/// }
///
/// let mut display = ExampleDisplay {
///     framebuffer: [0; 4096],
///     iface: SPI1,
/// };
///
/// // Draw a circle with top-left at `(22, 22)` with a diameter of `20` and a white stroke
/// let circle = Circle::new(Point::new(22, 22), 20)
///     .into_styled(PrimitiveStyle::with_stroke(Gray8::WHITE, 1));
///
/// circle.draw(&mut display)?;
///
/// // Update the display
/// display.flush().unwrap();
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// # Hardware acceleration - solid rectangular fill
///
/// This example uses an imaginary display with 16bpp RGB565 colors and hardware support for
/// filling of rectangular areas with a solid color. A real display controller that supports this
/// operation is the SSD1331 with it's "Draw Rectangle" (`22h`) command which this example
/// is loosely based on.
///
/// To leverage this feature in a `DrawTarget`, the default implementation of [`fill_solid`] can be
/// overridden by a custom implementation. Instead of drawing individual pixels, this target
/// specific version will only send a single command to the display controller in one transaction.
/// Because the command size is independent of the filled area, all [`fill_solid`] calls will only
/// transmit 8 bytes to the display, which is far less then what is required to transmit each pixel
/// color inside the filled area.
/// ```rust
/// use core::convert::TryInto;
/// use embedded_graphics::{
///     pixelcolor::{raw::RawU16, Rgb565, RgbColor},
///     prelude::*,
///     primitives::{Rectangle, Circle},
///     style::{PrimitiveStyle, PrimitiveStyleBuilder},
/// };
/// #
/// # struct SPI1;
/// #
/// # impl SPI1 {
/// #     pub fn send_bytes(&self, buf: &[u16]) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// # }
/// #
///
/// /// SPI communication error
/// #[derive(Debug)]
/// struct CommError;
///
/// /// An example display connected over SPI.
/// struct ExampleDisplay {
///     iface: SPI1,
/// }
///
/// impl ExampleDisplay {
///     /// Send a single pixel to the display
///     pub fn set_pixel(&self, x: u32, y: u32, color: u16) -> Result<(), CommError> {
///         // ...
///
///         Ok(())
///     }
///
///     /// Send commands to the display
///     pub fn send_commands(&self, commands: &[u8]) -> Result<(), CommError> {
///         // Send data marked as commands to the display.
///
///         Ok(())
///     }
/// }
///
/// impl DrawTarget for ExampleDisplay {
///     type Color = Rgb565;
///     type Error = CommError;
///
///     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
///     where
///         I: IntoIterator<Item = Pixel<Self::Color>> {
///         for Pixel(coord, color) in pixels.into_iter() {
///             // Check if the pixel coordinates are out of bounds (negative or greater than
///             // (63,63)). `DrawTarget` implementation are required to discard any out of bounds
///             // pixels without returning an error or causing a panic.
///             if let Ok((x @ 0..=63, y @ 0..=63)) = coord.try_into() {
///                 self.set_pixel(x, y, RawU16::from(color).into_inner())?;
///             }
///         }
///
///         Ok(())
///     }
///
///     fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
///         // Clamp the rectangle coordinates to the valid range by determining
///         // the intersection of the fill area and the visible display area
///         // by using Rectangle::intersection.
///         let area = area.intersection(&Rectangle::new(Point::zero(), self.size()));
///
///         // Do not send a draw rectangle command if the intersection size if zero.
///         // The size is checked by using `Rectangle::bottom_right`, which returns `None`
///         // if the size is zero.
///         let bottom_right = if let Some(bottom_right) = area.bottom_right() {
///             bottom_right
///         } else {
///             return Ok(())
///         };
///
///         self.send_commands(&[
///             // Draw rectangle command
///             0x22,
///             // Top left X coordinate
///             area.top_left.x as u8,
///             // Top left Y coordinate
///             area.top_left.y as u8,
///             // Bottom right X coordinate
///             bottom_right.x as u8,
///             // Bottom right Y coordinate
///             bottom_right.y as u8,
///             // Fill color red channel
///             color.r(),
///             // Fill color green channel
///             color.g(),
///             // Fill color blue channel
///             color.b(),
///         ])
///     }
/// }
///
/// impl OriginDimensions for ExampleDisplay {
///     fn size(&self) -> Size {
///         Size::new(64, 64)
///     }
/// }
///
/// let mut display = ExampleDisplay {
///     iface: SPI1,
/// };
///
/// // Draw a rectangle with 5px red stroke and green fill.
/// // The stroke and fill can be broken down into multiple individual rectangles,
/// // so this uses `fill_solid` internally.
/// Rectangle::new(Point::new(20, 20), Size::new(50, 40))
///     .into_styled(
///         PrimitiveStyleBuilder::new()
///             .stroke_color(Rgb565::RED)
///             .stroke_width(5)
///             .fill_color(Rgb565::GREEN)
///             .build(),
///     )
///     .draw(&mut display)?;
///
/// // Draw a circle with top-left at `(5, 5)` with a diameter of `10` and a magenta stroke with
/// // cyan fill. This shape cannot be optimized by calls to `fill_solid` as it contains transparent
/// // pixels as well as pixels of different colors. It will instead delegate to `draw_iter`
/// // internally.
/// Circle::new(Point::new(5, 5), 10)
///     .into_styled(
///         PrimitiveStyleBuilder::new()
///             .stroke_color(Rgb565::MAGENTA)
///             .stroke_width(1)
///             .fill_color(Rgb565::CYAN)
///             .build(),
///     )
///     .draw(&mut display)?;
///
/// # Ok::<(), CommError>(())
/// ```
///
/// [`fill_solid`]: #method.fill_solid
/// [`draw_iter`]: #tymethod.draw_iter
/// [`size`]: #tymethod.size
/// [`Error` type]: #associatedtype.Error
pub trait DrawTarget: Dimensions {
    /// The pixel color type the targetted display supports.
    type Color: PixelColor;

    /// Error type to return when a drawing operation fails.
    ///
    /// This error is returned if an error occurred during a drawing operation. This mainly applies
    /// to drivers that need to communicate with the display for each drawing operation, where a
    /// communication error can occur. For drivers that use an internal framebuffer where drawing
    /// operations can never fail, [`core::convert::Infallible`] can instead be used as the `Error`
    /// type.
    ///
    /// [`core::convert::Infallible`]: https://doc.rust-lang.org/stable/core/convert/enum.Infallible.html
    type Error;

    // TODO: Reenable this in a new PR with modified primitives iterators
    // // TODO: Mention performance
    // // TODO: Mention default impl behaviour
    // /// Fill a given area with a transparent pixel iterator.
    // ///
    // /// Iteration order is guaranteed from top left to bottom right, however some pixels may be
    // /// transparent which are represented as `None`.
    // fn fill_sparse<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    // where
    //     I: IntoIterator<Item = Option<Self::Color>>,
    // {
    //     self.draw_iter(
    //         area.points()
    //             .zip(colors)
    //             .filter_map(|(pos, color)| color.map(|c| Pixel(pos, c))),
    //     )
    // }

    /// Draw individual pixels to the display without a defined order.
    ///
    /// Due to the unordered nature of the pixel iterator, this method is likely to be the slowest
    /// drawing method for a display that writes data to the hardware immediately. If possible, the
    /// other methods in this trait should be implemented to improve performance when rendering
    /// more contiguous pixel patterns.
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>;

    /// Fill a given area with an iterator providing a contiguous stream of pixel colors.
    ///
    /// Use this method to fill an area with contiguous, non-transparent pixel colors. Pixel
    /// coordinates are iterated over from the top left to the bottom right corner of the area in
    /// row-first order. The provided iterator must provide pixel color values based on this
    /// ordering to produce correct output.
    ///
    /// As seen in the example below, the [`Points::points`] method can be used to get an
    /// iterator over all points in the provided area.
    ///
    /// The provided iterator is not required to provide `width * height` pixels to completely fill
    /// the area. In this case, `fill_contiguous` should return without error.
    ///
    /// This method should not attempt to draw any pixels that fall outside the drawable area of the
    /// target display. The `area` argument can be clipped to the drawable area using the
    /// [`Rectangle::intersection`] method.
    ///
    /// The default implementation of this method delegates to [`draw_iter`](#tymethod.draw_iter).
    ///
    /// # Examples
    ///
    /// This is an example implementation of `fill_contiguous` that delegates to [`draw_iter`]. This
    /// delegation behaviour is undesirable in a real application as it will be as slow as the
    /// default trait implementation, however is shown here for demonstration purposes.
    ///
    /// The example demonstrates the usage of [`Rectangle::intersection`] on the passed `area`
    /// argument to only draw visible pixels. If there is no intersection between `area` and the
    /// display area, no pixels will be drawn.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     pixelcolor::{Gray8, GrayColor},
    ///     prelude::*,
    ///     primitives::{ContainsPoint, Rectangle},
    /// };
    ///
    /// struct ExampleDisplay;
    ///
    /// impl DrawTarget for ExampleDisplay {
    ///     type Color = Gray8;
    ///     type Error = core::convert::Infallible;
    ///
    ///     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    ///     where
    ///         I: IntoIterator<Item = Pixel<Self::Color>>,
    ///     {
    ///         // Draw pixels to the display
    ///
    ///         Ok(())
    ///     }
    ///
    ///     fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    ///     where
    ///         I: IntoIterator<Item = Self::Color>,
    ///     {
    ///         // Clamp area to drawable part of the display target
    ///         let drawable_area = area.intersection(&Rectangle::new(Point::zero(), self.size()));
    ///
    ///         // Check that there are visible pixels to be drawn
    ///         if drawable_area.size != Size::zero() {
    ///             self.draw_iter(
    ///                 area.points()
    ///                     .zip(colors)
    ///                     .filter(|(pos, _color)| drawable_area.contains(*pos))
    ///                     .map(|(pos, color)| Pixel(pos, color)),
    ///             )
    ///         } else {
    ///             Ok(())
    ///         }
    ///     }
    /// }
    ///
    /// impl OriginDimensions for ExampleDisplay {
    ///     fn size(&self) -> Size {
    ///         Size::new(64, 64)
    ///     }
    /// }
    ///
    /// ```
    ///
    /// [`draw_iter`]: #tymethod.draw_iter
    /// [`Rectangle::intersection`]: ../primitives/rectangle/struct.Rectangle.html#method.intersection
    /// [`Points::points`]: ../primitives/trait.Primitive.html#tymethod.points
    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.draw_iter(
            area.points()
                .zip(colors)
                .map(|(pos, color)| Pixel(pos, color)),
        )
    }

    /// Fill a given area with a solid color.
    ///
    /// If the target display provides optimized hardware commands for filling a rectangular area of
    /// the display with a solid color, this method should be overridden to use those commands to
    /// improve performance.
    ///
    /// The default implementation of this method calls [`fill_contiguous`](#method.fill_contiguous)
    /// with an iterator that repeats the given `color` for every point in `area`.
    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        self.fill_contiguous(area, core::iter::repeat(color))
    }

    /// Fill the entire display with a solid color.
    ///
    /// If the target hardware supports a more optimized way of filling the entire display with a
    /// solid color, this method should be overridden to use those commands.
    ///
    /// The default implementation of this method delegates to [`fill_solid`] where the fill area
    /// is specified as `(0, 0)` with size `(width, height)` as returned from the [`size`] method.
    ///
    /// [`size`]: #method.size
    /// [`fill_solid`]: #method.fill_solid
    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.fill_solid(&self.bounding_box(), color)
    }
}

/// Extension trait for `DrawTarget`s.
pub trait DrawTargetExt: DrawTarget + Sized {
    /// Creates a translated draw target based on this draw target.
    ///
    /// All drawing operations are translated by `offset` pixels, before being passed to the base
    /// draw target.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::{
    ///     prelude::*,
    ///     mock_display::MockDisplay,
    ///     pixelcolor::BinaryColor,
    ///     fonts::{Text, Font6x8},
    ///     style::TextStyle,
    /// };
    ///
    /// let mut display = MockDisplay::new();
    /// let mut translated_display = display.translated(Point::new(10, 5));
    ///
    /// // Draws text at position (10, 5) in the display coordinate system
    /// Text::new("Text", Point::zero())
    ///     .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
    ///     .draw(&mut translated_display)?;
    /// #
    /// # let mut expected = MockDisplay::new();
    /// #
    /// # Text::new("Text", Point::new(10, 5))
    /// #     .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
    /// #     .draw(&mut expected)?;
    /// #
    /// # assert_eq!(display, expected);
    /// #
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    fn translated(&mut self, offset: Point) -> Translated<'_, Self>;

    /// Creates a cropped draw target based on this draw target.
    ///
    /// A cropped draw target is a draw target for a rectangular subregion of the base draw target.
    /// Its coordinate system is shifted so that the origin coincides with `area.top_left` in the
    /// base draw target's coordinate system.
    ///
    /// The bounding box of the returned target will always be contained inside the bounding box
    /// of the base target. If any of the requested `area` lies outside the base target's bounding 
    /// box the intersection of the base target's bounding box and `area` will be used.
    ///
    /// Drawing operations outside the bounding box will not be clipped.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::{
    ///     prelude::*,
    ///     mock_display::MockDisplay,
    ///     pixelcolor::Rgb565,
    ///     fonts::{Text, Font6x8},
    ///     style::TextStyle,
    ///     primitives::Rectangle,
    /// };
    ///
    /// /// Fills a draw target with a blue background and prints centered yellow text.
    /// fn draw_text<T>(target: &mut T, text: &str) -> Result<(), T::Error>
    /// where
    ///     T: DrawTarget<Color = Rgb565>,
    /// {
    ///     target.clear(Rgb565::BLUE)?;
    ///
    ///     let target_size = target.bounding_box().size;
    ///     let text_size = Font6x8::CHARACTER_SIZE.component_mul(Size::new(text.len() as u32, 1));
    ///
    ///     let text_position = Point::zero() + (target_size - text_size) / 2;
    ///
    ///     Text::new(text, text_position)
    ///         .into_styled(TextStyle::new(Font6x8, Rgb565::YELLOW))
    ///         .draw(target)
    /// }
    ///
    ///
    /// let mut display = MockDisplay::new();
    /// display.set_allow_overdraw(true);
    ///
    /// let area = Rectangle::new(Point::new(5, 10), Size::new(40, 15));
    /// let mut cropped_display = display.cropped(&area);
    ///
    /// draw_text(&mut cropped_display, "Text")?;
    /// #
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    fn cropped(&mut self, area: &Rectangle) -> Cropped<'_, Self>;

    /// Creates a clipped draw target based on this draw target.
    ///
    /// A clipped draw target is a draw target for a rectangular subregion of the base draw target.
    /// The coordinate system of the created draw target is equal to the base target's coordinate
    /// system. All drawing operations outside the bounding box will be clipped.
    ///
    /// The bounding box of the returned target will always be contained inside the bounding box
    /// of the base target. If the requested `area` is overlapping the base target's bounding box
    /// the intersection of the base target's bounding box and `area` will be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::{
    ///     prelude::*,
    ///     mock_display::MockDisplay,
    ///     pixelcolor::BinaryColor,
    ///     fonts::{Text, Font12x16},
    ///     style::TextStyle,
    ///     primitives::Rectangle,
    /// };
    ///
    /// let mut display = MockDisplay::new();
    ///
    /// let area = Rectangle::new(Point::zero(), Size::new(4 * 12, 16));
    /// let mut clipped_display = display.clipped(&area);
    ///
    /// // Only the first 4 characters will be drawn, because the others are outside
    /// // the clipping area
    /// Text::new("Clipped", Point::zero())
    ///     .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
    ///     .draw(&mut clipped_display)?;
    /// #
    /// # let mut expected = MockDisplay::new();
    /// #
    /// # Text::new("Clip", Point::zero())
    /// #     .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
    /// #     .draw(&mut expected)?;
    /// #
    /// # assert_eq!(display, expected);
    /// #
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    fn clipped(&mut self, area: &Rectangle) -> Clipped<'_, Self>;
}

impl<T> DrawTargetExt for T
where
    T: DrawTarget,
{
    fn translated(&mut self, offset: Point) -> Translated<'_, Self> {
        Translated::new(self, offset)
    }

    fn cropped(&mut self, area: &Rectangle) -> Cropped<'_, Self> {
        Cropped::new(self, area)
    }

    fn clipped(&mut self, area: &Rectangle) -> Clipped<'_, Self> {
        Clipped::new(self, area)
    }
}
