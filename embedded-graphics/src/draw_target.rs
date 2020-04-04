use crate::{
    drawable::{self, Drawable},
    geometry::{Point, Size},
    image::{Image, ImageDimensions, IntoPixelIter},
    pixelcolor::PixelColor,
    primitives::{self, Primitive},
    style::{PrimitiveStyle, Styled},
};

/// Defines a display that can be used to render [`Drawable`] objects.
///
/// To to add embedded-graphics support to a display driver, `DrawTarget` must be implemented. Once
/// a `DrawTarget` is defined, it can be used to render [`Drawable`]s. Note that any iterator over
/// [`Pixel`]s can be drawn as [`Drawable`] is implemented for `Iterator<Item = Pixel<C:
/// PixelColor>>`. See the [`Drawable`] trait documentation for more details.
///
/// `DrawTarget` provides default implementations of methods to draw [`primitive`]s and clear the
/// display which delegate to [`DrawTarget::draw_iter`]. If the target display supports accelerated
/// drawing commands, these methods can be overridden with specialised implementations that take
/// advantage of the hardware to speed up drawing operations.
///
/// Note that some displays require a "flush" operation to write changes from a framebuffer to the
/// display. See docs associated with the chosen display driver for details on how to update the
/// display.
///
/// # Examples
///
/// ## Implement `DrawTarget` for an 8 bit grayscale display
///
/// This example uses an imaginary display that has a 64x64px framebuffer of 8 bit values that
/// is sent to the display over a (simplified) SPI interface.
///
/// ```rust
/// use core::convert::TryInto;
/// use embedded_graphics::{
///     drawable::Pixel,
///     egcircle,
///     geometry::Size,
///     pixelcolor::{Gray8, GrayColor},
///     prelude::*,
///     primitive_style, DrawTarget,
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
/// impl DrawTarget<Gray8> for ExampleDisplay {
///     type Error = core::convert::Infallible;
///
///     /// Draw a `Pixel` that has a color defined as `Gray8`.
///     fn draw_pixel(&mut self, pixel: Pixel<Gray8>) -> Result<(), Self::Error> {
///         let Pixel(coord, color) = pixel;
///
///         // Place an (x, y) pixel at the right index in the framebuffer. If the pixel coordinates
///         // are out of bounds (negative or greater than (63, 63)), this operation will be a
///         // noop.
///         if let Ok((x @ 0..=63, y @ 0..=63)) = coord.try_into() {
///             let index: u32 = x + y * 64;
///             self.framebuffer[index as usize] = color.luma();
///         }
///
///         Ok(())
///     }
///
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
/// let circle = egcircle!(
///     top_left = (22, 22),
///     diameter = 20,
///     style = primitive_style!(stroke_color = Gray8::WHITE, stroke_width = 1)
/// );
/// circle.draw(&mut display)?;
///
/// // Update the display
/// display.flush().expect("Failed to send data to display");
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// ## Hardware Acceleration
///
/// In addition to defining [`draw_pixel`], an implementation of [`DrawTarget`] can also provide
/// alternative implementations for hardware accelerated drawing operations. This example implements
/// `DrawTarget` for a display without a framebuffer that supports hardware accelerated drawing of
/// styled [`Rectangle`]s.
///
/// The default implementations of [`draw_rectangle`] as well as other shape drawing methods
/// ([`draw_circle`], etc) defer to [`draw_iter`] internally. In this example, the default
/// implementation of [`draw_rectangle`] is overridden to allow usage of accelerated draw commands
/// specific to the targeted hardware.
///
/// As this example doesn't use a framebuffer, a "flush" operation is not required. All draw
/// operations are performed in "immediate mode" directly on the display. As each drawing operation
/// requires communication with the display that may fail, a custom error type `CommError` is
/// introduced.
///
/// ```rust
/// # use embedded_graphics::prelude::*;
/// # use embedded_graphics::DrawTarget;
/// # use embedded_graphics::{egrectangle, primitive_style};
/// # use embedded_graphics::primitives::rectangle::Rectangle;
/// # use embedded_graphics::pixelcolor::{Gray8, GrayColor};
/// # use embedded_graphics::drawable::Pixel;
/// # use embedded_graphics::style::{PrimitiveStyle, Styled};
/// # use core::convert::TryFrom;
/// #
/// # struct SPI1;
/// #
/// # impl SPI1 {
/// #     pub fn send_bytes(&self, buf: &[u8]) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// # }
/// #
/// /// SPI communication error
/// #[derive(Debug)]
/// struct CommError;
///
/// /// A fake display which uses hardware drawing commands instead of a framebuffer
/// struct FastExampleDisplay {
///     iface: SPI1,
/// }
///
/// impl FastExampleDisplay {
///     /// Draw a rectangle using hardware accelerated commands
///     pub fn fast_rectangle(
///         &self,
///         rect: &Styled<Rectangle, PrimitiveStyle<Gray8>>,
///     ) -> Result<(), CommError> {
///         // Send rectangle drawing commands to the display
///
///         Ok(())
///     }
/// }
///
/// impl DrawTarget<Gray8> for FastExampleDisplay {
///     type Error = CommError;
///
///     /// Draw a `pixel` that has a color defined as `Gray8`
///     fn draw_pixel(&mut self, pixel: Pixel<Gray8>) -> Result<(), Self::Error> {
///         let Pixel(coord, color) = pixel;
///
///         // Send commands directly to the display to set an individual pixel to the given color
///
///         Ok(())
///     }
///
///     fn size(&self) -> Size {
///         Size::new(64, 64)
///     }
///
///     /// Use the accelerated method when drawing rectangles
///     ///
///     /// This method overrides the default implementation. If `fast_rectangle()` fails, the error
///     /// will be propagated through this method.
///     fn draw_rectangle(
///         &mut self,
///         item: &Styled<Rectangle, PrimitiveStyle<Gray8>>,
///     ) -> Result<(), Self::Error> {
///         self.fast_rectangle(item)
///     }
/// }
///
/// let mut display = FastExampleDisplay { iface: SPI1 };
///
/// // Draw a rectangle from (10, 20) to (30, 40) with a white stroke
/// let rect = egrectangle!(
///     top_left = (10, 20),
///     bottom_right = (30, 40),
///     style = primitive_style!(stroke_color = Gray8::WHITE, stroke_width = 1)
/// )
/// .draw(&mut display)?;
///
/// // Draw a rectangle on the display using accelerated `draw_rectangle()` function
/// # Ok::<(), CommError>(())
/// ```
///
/// [`Drawable`]: ../drawable/trait.Drawable.html
/// [`Pixel`]: ../drawable/struct.Pixel.html
/// [`draw_pixel`]: ./trait.DrawTarget.html#method.draw_pixel
/// [`DrawTarget::draw_iter`]: ./trait.DrawTarget.html#method.draw_iter
/// [`DrawTarget`]: ./trait.DrawTarget.html
/// [`Rectangle`]: ../primitives/rectangle/struct.Rectangle.html
/// [`primitive`]: ../primitives/index.html
/// [`draw_rectangle`]: ./trait.DrawTarget.html#method.draw_rectangle
/// [`draw_circle`]: ./trait.DrawTarget.html#method.draw_circle
/// [`draw_iter`]: ./trait.DrawTarget.html#method.draw_iter
pub trait DrawTarget<C>
where
    C: PixelColor,
{
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

    /// Draws a pixel on the display.
    fn draw_pixel(&mut self, item: drawable::Pixel<C>) -> Result<(), Self::Error>;

    /// Draws an object from an iterator over its pixels.
    fn draw_iter<T>(&mut self, item: T) -> Result<(), Self::Error>
    where
        T: IntoIterator<Item = drawable::Pixel<C>>,
    {
        for pixel in item {
            self.draw_pixel(pixel)?;
        }

        Ok(())
    }

    /// Returns the dimensions of the `DrawTarget` in pixels.
    fn size(&self) -> Size;

    /// Clears the display with the supplied color.
    ///
    /// This default implementation can be replaced if the implementing driver provides an
    /// accelerated clearing method.
    fn clear(&mut self, color: C) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        primitives::Rectangle::new(Point::zero(), Point::zero() + self.size())
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(self)
    }

    /// Draws a styled line primitive.
    ///
    /// This default trait method can be overridden if a display provides hardware-accelerated
    /// methods for drawing lines.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Styled`] [`Line`] primitive. To draw a line,
    /// call [`draw`] on a `Styled<Line>` object.
    ///
    /// [`Line`]: ../primitives/line/struct.Line.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    /// [`Styled`]: ../style/struct.Styled.html
    fn draw_line(
        &mut self,
        item: &Styled<primitives::Line, PrimitiveStyle<C>>,
    ) -> Result<(), Self::Error> {
        self.draw_iter(item)
    }

    /// Draws a styled triangle primitive.
    ///
    /// This default trait method can be overridden if a display provides hardware-accelerated
    /// methods for drawing triangles.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Styled`] [`Triangle`] primitive. To draw a
    /// triangle, call [`draw`] on a `Styled<Triangle>` object.
    ///
    /// [`Triangle`]: ../primitives/triangle/struct.Triangle.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    /// [`Styled`]: ../style/struct.Styled.html
    fn draw_triangle(
        &mut self,
        item: &Styled<primitives::Triangle, PrimitiveStyle<C>>,
    ) -> Result<(), Self::Error> {
        self.draw_iter(item)
    }

    /// Draws a styled rectangle primitive.
    ///
    /// This default trait method can be overridden if a display provides hardware-accelerated
    /// methods for drawing rectangle.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Styled`] [`Rectangle`] primitive. To draw a
    /// rectangle, call [`draw`] on a `Styled<Rectangle>` object.
    ///
    /// [`Rectangle`]: ../primitives/rectangle/struct.Rectangle.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    /// [`Styled`]: ../style/struct.Styled.html
    fn draw_rectangle(
        &mut self,
        item: &Styled<primitives::Rectangle, PrimitiveStyle<C>>,
    ) -> Result<(), Self::Error> {
        self.draw_iter(item)
    }

    /// Draws a styled circle primitive.
    ///
    /// This default trait method can be overridden if a display provides hardware-accelerated
    /// methods for drawing circles.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Styled`] [`Circle`] primitive. To draw a
    /// circle, call [`draw`] on a `Styled<Circle>` object.
    ///
    /// [`Circle`]: ../primitives/circle/struct.Circle.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    /// [`Styled`]: ../style/struct.Styled.html
    fn draw_circle(
        &mut self,
        item: &Styled<primitives::Circle, PrimitiveStyle<C>>,
    ) -> Result<(), Self::Error> {
        self.draw_iter(item)
    }

    /// Draws an image with known size
    ///
    /// This default trait method can be overridden if a display provides hardware-accelerated
    /// methods for drawing an image with known size.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Image`] primitive. To draw an
    /// image, call [`draw`] on a `Image` object.
    ///
    /// [`Image`]: ../image/struct.Image.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    fn draw_image<'a, 'b, I>(&mut self, item: &'a Image<'b, I, C>) -> Result<(), Self::Error>
    where
        &'b I: IntoPixelIter<C>,
        I: ImageDimensions,
        C: PixelColor + From<<C as PixelColor>::Raw>,
    {
        self.draw_iter(item.into_iter())
    }
}
