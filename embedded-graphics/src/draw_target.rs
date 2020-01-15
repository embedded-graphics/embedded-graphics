use crate::{
    drawable::{self, Drawable},
    geometry::{Point, Size},
    pixelcolor::PixelColor,
    primitives::{self, Primitive},
    style::{PrimitiveStyle, Styled},
};

/// Defines a display that can be used to render [`Drawable`] objects.
///
/// To use this crate in a driver, `DrawTarget` must be implemented. This trait defines how a
/// display draws pixels, and optionally provides a way to define accelerated drawing methods for
/// graphical primitives such as lines, rectangles, triangles, and circles.
///
/// Once a `DrawTarget` is defined, it can be used to render [`Drawable`]s. Note that any iterator
/// over [`Pixel`]s has a default implementation for the [`Drawable`] trait. See the [`Drawable`]
/// trait documentation for more details.
///
/// Here's an example for an imaginary display that has a 64x64px framebuffer of 8 bit values that
/// communicates over a (simplified) SPI interface:
///
/// ```rust
/// use embedded_graphics::{
///     drawable::Pixel,
///     egcircle,
///     geometry::Size,
///     pixelcolor::{Gray8, GrayColor},
///     prelude::*,
///     primitive_style, DrawTarget,
/// };
///
/// # struct SPI1;
/// #
/// # impl SPI1 {
/// #     pub fn send_bytes(&self, buf: &[u8]) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// # }
/// #
/// /// A fake display 64px x 64px where each pixel is stored as a single `u8`
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
///         // Place an (x, y) pixel at the right index in the framebuffer
///         let index = coord.x + coord.y * 64;
///         self.framebuffer[index as usize] = color.luma();
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
/// // Draw a circle centered around `(32, 32)` with a radius of `10` and a white stroke
/// let circle = egcircle!(
///     center = (32, 32),
///     radius = 10,
///     style = primitive_style!(stroke_color = Gray8::WHITE)
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
/// alternative implementations of drawing methods for graphical primitives. Here is an example of
/// how a display with accelerated methods can implement [`DrawTarget`]:
///
/// ```rust
/// # use embedded_graphics::prelude::*;
/// # use embedded_graphics::DrawTarget;
/// # use embedded_graphics::{egrectangle, primitive_style};
/// # use embedded_graphics::primitives::rectangle::Rectangle;
/// # use embedded_graphics::pixelcolor::{Gray8, GrayColor};
/// # use embedded_graphics::drawable::Pixel;
/// # use embedded_graphics::style::{PrimitiveStyle, Styled};
/// #
/// # struct SPI1;
/// #
/// # impl SPI1 {
/// #     pub fn send_bytes(&self, buf: &[u8]) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// # }
/// #
/// /// A fake display 64px x 64px where each pixel is stored as a single `u8`
/// struct FastExampleDisplay {
///     framebuffer: [u8; 64 * 64],
///     iface: SPI1,
/// }
///
/// impl FastExampleDisplay {
///     /// Send buffer to the display
///     pub fn flush(&self) -> Result<(), ()> {
///         self.iface.send_bytes(&self.framebuffer)
///     }
///
///     /// A HW-accelerated method for drawing rectangles
///     pub fn fast_rectangle(&self, rect: &Styled<Rectangle, PrimitiveStyle<Gray8>>) {
///         // Does some speedy drawing
///     }
/// }
///
/// impl DrawTarget<Gray8> for FastExampleDisplay {
///     type Error = core::convert::Infallible;
///
///     /// Draw a `pixel` that has a color defined as `Gray8`
///     fn draw_pixel(&mut self, pixel: Pixel<Gray8>) -> Result<(), Self::Error> {
///         let Pixel(coord, color) = pixel;
///         // Place an (x, y) pixel at the right index in the framebuffer
///         let index = coord.x + coord.y * 64;
///         self.framebuffer[index as usize] = color.luma();
///
///         Ok(())
///     }
///
///     fn size(&self) -> Size {
///         Size::new(64, 64)
///     }
///
///     /// Use the accelerated method when drawing rectangles
///     fn draw_rectangle(
///         &mut self,
///         item: &Styled<Rectangle, PrimitiveStyle<Gray8>>,
///     ) -> Result<(), Self::Error> {
///         self.fast_rectangle(item);
///
///         Ok(())
///     }
/// }
///
/// let mut display = FastExampleDisplay {
///     framebuffer: [0; 4096],
///     iface: SPI1,
/// };
///
/// // Draw a rectangle from (10, 20) to (30, 40) with a white stroke
/// let rect = egrectangle!(
///     top_left = (10, 20),
///     bottom_right = (30, 40),
///     style = primitive_style!(stroke_color = Gray8::WHITE)
/// );
/// rect.draw(&mut display)?; // Uses the accelerated draw_rectangle function
///
/// // Update the display
/// display.flush().expect("Failed to send data to display");
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// [`Drawable`]: ../drawable/trait.Drawable.html
/// [`Pixel`]: ../drawable/struct.Pixel.html
/// [`draw_pixel`]: ./trait.DrawTarget.html#method.draw_pixel
/// [`DrawTarget`]: ./trait.DrawTarget.html
pub trait DrawTarget<C>
where
    C: PixelColor,
{
    /// Error type to return when a draw or display change operation fails.
    ///
    /// This error is returned when a drawing operation fails, or when an error updating/clearing
    /// the display occurs. The examples in this crate use `core::convert::Infallible`, however a
    /// more descriptive error type should be used with real hardware.
    type Error;

    /// Draws a pixel on the display.
    ///
    /// Note that some displays require a "flush" operation to actually write changes to the
    /// framebuffer.
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
    /// This default implementation should be replaced if the implementing driver provides an
    /// accelerated clearing method.
    fn clear(&mut self, color: C) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        primitives::Rectangle::new(Point::zero(), Point::zero() + self.size())
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(self)
    }

    /// Draws a line primitive.
    ///
    /// This default trait method should be overridden if a display provides hardware-accelerated
    /// methods for drawing lines.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Line`] primitive. To draw a line, call
    /// [`draw`] on a [`Line`] primitive object.
    ///
    /// [`Line`]: ../primitives/line/struct.Line.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    fn draw_line(
        &mut self,
        item: &Styled<primitives::Line, PrimitiveStyle<C>>,
    ) -> Result<(), Self::Error> {
        self.draw_iter(item)
    }

    /// Draws a triangle primitive.
    ///
    /// This default trait method should be overridden if a display provides hardware-accelerated
    /// methods for drawing triangles.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Triangle`] primitive. To draw a triangle,
    /// call [`draw`] on a [`Triangle`] primitive object.
    ///
    /// [`Triangle`]: ../primitives/triangle/struct.Triangle.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    fn draw_triangle(
        &mut self,
        item: &Styled<primitives::Triangle, PrimitiveStyle<C>>,
    ) -> Result<(), Self::Error> {
        self.draw_iter(item)
    }

    /// Draws a rectangle primitive.
    ///
    /// This default trait method should be overridden if a display provides hardware-accelerated
    /// methods for drawing rectangle.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Rectangle`] primitive. To draw a rectangle,
    /// call [`draw`] on a [`Rectangle`] primitive object.
    ///
    /// [`Rectangle`]: ../primitives/rectangle/struct.Rectangle.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    fn draw_rectangle(
        &mut self,
        item: &Styled<primitives::Rectangle, PrimitiveStyle<C>>,
    ) -> Result<(), Self::Error> {
        self.draw_iter(item)
    }

    /// Draws a circle primitive.
    ///
    /// This default trait method should be overridden if a display provides hardware-accelerated
    /// methods for drawing circles.
    ///
    /// # Caution
    ///
    /// This method should not be called directly from application code. It is used to define the
    /// internals of the [`draw`] method used for the [`Circle`] primitive. To draw a circle, call
    /// [`draw`] on a [`Circle`] primitive object.
    ///
    /// [`Circle`]: ../primitives/circle/struct.Circle.html
    /// [`draw`]: ./trait.DrawTarget.html#method.draw
    fn draw_circle(
        &mut self,
        item: &Styled<primitives::Circle, PrimitiveStyle<C>>,
    ) -> Result<(), Self::Error> {
        self.draw_iter(item)
    }
}
