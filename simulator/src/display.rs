use embedded_graphics::{
    drawable::Pixel,
    geometry::{Point, Size},
    pixelcolor::{BinaryColor, PixelColor},
    DrawTarget,
};

/// Display
pub struct SimulatorDisplay<C> {
    size: Size,
    pixels: Box<[C]>,
}

impl<C> SimulatorDisplay<C>
where
    C: PixelColor,
{
    /// Creates a new display filled with a color.
    ///
    /// This constructor can be used if `C` doesn't implement `From<BinaryColor>` or another
    /// default color is wanted.
    pub fn with_default_color(size: Size, default_color: C) -> Self {
        let pixel_count = size.width as usize * size.height as usize;
        let pixels = vec![default_color; pixel_count].into_boxed_slice();

        SimulatorDisplay { size, pixels }
    }

    /// Returns the color of the pixel at a point.
    ///
    /// # Panics
    ///
    /// Panics if `point` is outside the display.
    pub fn get_pixel(&self, point: Point) -> C {
        let index = self
            .point_to_index(point)
            .expect("can't get point outside of display");

        self.pixels[index]
    }

    fn point_to_index(&self, point: Point) -> Option<usize> {
        let Point { x, y } = point;

        if x >= 0 && y >= 0 && (x as u32) < self.size.width && (y as u32) < self.size.height {
            Some(x as usize + y as usize * self.size.width as usize)
        } else {
            None
        }
    }
}

impl<C> SimulatorDisplay<C>
where
    C: PixelColor + From<BinaryColor>,
{
    /// Creates a new display.
    ///
    /// The display is filled with `C::from(BinaryColor::Off)`.
    pub fn new(size: Size) -> Self {
        Self::with_default_color(size, C::from(BinaryColor::Off))
    }
}

impl<C> DrawTarget<C> for SimulatorDisplay<C>
where
    C: PixelColor,
{
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<C>) -> Result<(), Self::Error> {
        let Pixel(point, color) = pixel;

        if let Some(index) = self.point_to_index(point) {
            self.pixels[index] = color;
        }

        Ok(())
    }

    fn size(&self) -> Size {
        self.size
    }
}
