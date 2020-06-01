use crate::{
    drawable::Pixel,
    geometry::{Point, Size},
    pixelcolor::PixelColor,
    primitives::{rectangle::Rectangle, Primitive},
};

/// A target for embedded-graphics drawing operations.
///
/// The `DrawTarget` trait is used to add embedded-graphics support to a display
/// driver or similar targets like framebuffers or image files.
/// Targets are required to at least implement the `size` and `draw_iter` methods. All other
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
/// acceleration only the two required methods `size` and `draw_iter` need to be implemented.
///
/// To reduce the overhead caused by communicating with the display for each drawing operation
/// the display driver uses and framebuffer to store the pixel data in memory. This way all drawing
/// operations can be executed in local memory and the actual display is only updated on demand
/// by calling the `flush` method.
///
/// Because all drawing operations are using a local framebuffer no communication error can occur
/// while they are executed and the `Error` type can be set to `core::convert::Infallible`.
///
/// ```rust
/// use core::convert::TryInto;
/// use embedded_graphics::{
///     drawable::Pixel,
///     geometry::Size,
///     pixelcolor::{Gray8, GrayColor},
///     prelude::*,
///     primitives::Circle,
///     style::PrimitiveStyle,
///     DrawTarget,
/// };
/// #
/// # struct SPI1;
/// #
/// # impl SPI1 {
/// #     pub fn send_bytes(&self, buf: &[u8]) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// # }
/// #
///
/// /// A fake 64px x 64px display where each pixel is stored as a single `u8`
/// struct ExampleDisplay {
///     framebuffer: [u8; 64 * 64],
///     iface: SPI1,
/// }
///
/// impl ExampleDisplay {
///     /// Send buffer to the display
///     pub fn flush(&self) -> Result<(), ()> {
///         self.iface.send_bytes(&self.framebuffer)
///     }
/// }
///
/// impl DrawTarget for ExampleDisplay {
///     type Color = Gray8;
///     type Error = core::convert::Infallible;
///
///     fn size(&self) -> Size {
///         Size::new(64, 64)
///     }
///
///     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
///     where
///         I: IntoIterator<Item = Pixel<Self::Color>> {
///         for Pixel(coord, color) in pixels.into_iter() {
///             // Place an (x, y) pixel at the right index in the framebuffer. If the pixel
///             // coordinates are out of bounds (negative or greater than (63, 63)), this operation
///             // will be a noop.
///             if let Ok((x @ 0..=63, y @ 0..=63)) = coord.try_into() {
///                 let index: u32 = x + y * 64;
///                 self.framebuffer[index as usize] = color.luma();
///             }
///         }
///
///         Ok(())
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
/// To leverage this feature in a `DrawTarget` the default implementation of `fill_solid` needs to be
/// overridden by a custom implementation. Instead of drawing individual pixels the target specific
/// version will only send a single command to the display controller. Because the command size
/// is independent of the filled area all `fill_soild` calls will only transmit 8 bytes to the display, which
/// is far less then what is required to transmit each pixel color inside the filled area.
/// ```rust
/// use core::convert::TryInto;
/// use embedded_graphics::{
///     drawable::Pixel,
///     geometry::Size,
///     pixelcolor::{raw::RawU16, Rgb565, RgbColor},
///     prelude::*,
///     primitives::{Rectangle, Circle},
///     style::{PrimitiveStyle, PrimitiveStyleBuilder},
///     DrawTarget,
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
/// /// A fake 64px x 64px display where each pixel is stored as a single `u16`
/// struct ExampleDisplay {
///     framebuffer: [u16; 64 * 64],
///     iface: SPI1,
/// }
///
/// impl ExampleDisplay {
///     /// Send buffer to the display
///     pub fn flush(&self) -> Result<(), ()> {
///         self.iface.send_bytes(&self.framebuffer)
///     }
///
///     /// Send commands to the display
///     pub fn send_commands(&self, commands: &[u8]) -> Result<(), core::convert::Infallible> {
///         // Send data marked as commands to the display.
///
///         Ok(())
///     }
/// }
///
/// impl DrawTarget for ExampleDisplay {
///     type Color = Rgb565;
///     type Error = core::convert::Infallible;
///
///     fn size(&self) -> Size {
///         Size::new(64, 64)
///     }
///
///     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
///     where
///         I: IntoIterator<Item = Pixel<Self::Color>> {
///         for Pixel(coord, color) in pixels.into_iter() {
///             if let Ok((x @ 0..=63, y @ 0..=63)) = coord.try_into() {
///                 let index: u32 = x + y * 64;
///                 self.framebuffer[index as usize] = RawU16::from(color).into_inner();
///             }
///         }
///
///         Ok(())
///     }
///
///     fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
///         // Clamp area coordinates to display size using Rectangle::intersection
///         if let Some(area) = area.intersection(&Rectangle::new(Point::zero(), self.size())) {
///             // Do not draw the rectangle if its size is zero
///             let  bottom_right = if let Some(bottom_right) = area.bottom_right() {
///                 bottom_right
///             } else {
///                 return Ok(())
///             };
///
///             self.send_commands(&[
///                 // Draw rectangle command
///                 0x22,
///                 // Top left X coordinate
///                 area.top_left.x as u8,
///                 // Top left Y coordinate
///                 area.top_left.y as u8,
///                 // Bottom right X coordinate
///                 bottom_right.x as u8,
///                 // Bottom right Y coordinate
///                 bottom_right.y as u8,
///                 // Fill color red channel
///                 color.r(),
///                 // Fill color green channel
///                 color.g(),
///                 // Fill color blue channel
///                 color.b(),
///             ])
///         } else {
///             // If given area is completely outside the drawable area, do nothing
///             Ok(())
///         }
///     }
/// }
///
/// let mut display = ExampleDisplay {
///     framebuffer: [0; 4096],
///     iface: SPI1,
/// };
///
/// // Draw a rectangle with 5px red stroke and green fill. The stroke and fill can be broken down
/// // into multiple individual rectangles, so uses `fill_solid` internally.
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
/// // Draw a circle with top-left at `(5, 5)` with a diameter of `10` and a magenta stroke with cyan
/// // fill. This shape cannot be optimised by calls to `fill_solid` as it contains transparent
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
/// // Update the display
/// display.flush().expect("Failed to send data to display");
/// # Ok::<(), core::convert::Infallible>(())
/// ```
pub trait DrawTarget {
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

    /// Returns the dimensions of the `DrawTarget` in pixels.
    ///
    /// This should return the size of the entire drawable area of the display. If a display
    /// supports drawing to pixels outside the visible area, that area should also be reported in
    /// the dimensions returned by `size`.
    fn size(&self) -> Size;

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
    /// Use this method to fill an opaque area with given pixel colors. Pixel coordinates are
    /// iterated over from the top left to the bottom right corner of the area. The provided
    /// iterator should provide pixel color values based on this ordering.
    ///
    /// This method should not attempt to draw any pixels that fall outside the drawable area of the
    /// target display. The `area` argument can be clipped to the drawable area using the
    /// [`Rectangle::intersection`] method.
    ///
    /// The default implementation of this method delegates to [`draw_iter`](#method.draw_iter).
    ///
    /// # Examples
    ///
    /// This is an example implementation of `fill_contiguous` that delegates to `draw_iter`. It
    /// demonstrates the usage of [`Rectangle::intersection`] on the passed `area` argument to only
    /// draw visible pixels. If there is no intersection between `area` and the display area, no
    /// pixels will be drawn.
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     drawable::Pixel,
    ///     geometry::Size,
    ///     pixelcolor::{Gray8, GrayColor},
    ///     prelude::*,
    ///     primitives::Rectangle,
    ///     DrawTarget,
    /// };
    ///
    /// struct ExampleDisplay;
    ///
    /// impl DrawTarget for ExampleDisplay {
    ///     type Color = Gray8;
    ///     type Error = core::convert::Infallible;
    ///
    ///     fn size(&self) -> Size {
    ///         Size::new(64, 64)
    ///     }
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
    ///         if let Some(area) = Rectangle::new(Point::zero(), self.size()).intersection(&area) {
    ///             self.draw_iter(
    ///                 area.points()
    ///                     .zip(colors)
    ///                     .map(|(pos, color)| Pixel(pos, color)),
    ///             )
    ///         } else {
    ///             Ok(())
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// [`Rectangle::intersection`]: ../primitives/rectangle/struct.Rectangle.html#method.intersection
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
    /// If the target display provides optimised hardware commands for filling a rectangular area of
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
    /// The default implementation of this method delegates to [`fill_solid`] where the fill area
    /// is specified as `(0, 0)` to `(width, height)`. If the target hardware supports a more
    /// optimised way of filling the entire display with a solid color, this method should be
    /// overridden to use those commands.
    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.fill_solid(&Rectangle::new(Point::zero(), self.size()), color)
    }
}
