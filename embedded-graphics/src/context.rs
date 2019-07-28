//! Drawing context

use crate::coord::Coord;
use crate::drawable::Pixel;
use crate::fonts::Font;
use crate::pixelcolor::{BinaryColor, PixelColor};
use crate::primitives::{Circle, Line, Rectangle, Triangle};
use crate::style::Style;
use crate::transform::Transform;
use crate::Drawing;
use core::marker::PhantomData;

/// Drawing context.
#[derive(Debug)]
pub struct Context<'a, C, D>
where
    D: Drawing<C>,
    C: PixelColor,
{
    drawing: &'a mut D,

    style: Style<C>,
    offset: Coord,

    color_type: PhantomData<C>,
}

impl<'a, C, D> Context<'a, C, D>
where
    D: Drawing<C>,
    C: PixelColor + From<BinaryColor>,
{
    /// Draws a circle.
    pub fn circle(&mut self, center: Coord, radius: u32) {
        let circle = Circle {
            center: center,
            radius,
            style: self.style,
        };

        self.drawing.draw(circle.translate(self.offset));
    }

    /// Draws a line.
    pub fn line(&mut self, start: Coord, end: Coord) {
        let line = Line {
            start,
            end,
            style: self.style,
        };

        self.drawing.draw(line.translate(self.offset));
    }

    /// Draws a rectangle.
    pub fn rectangle(&mut self, top_left: Coord, bottom_right: Coord) {
        let rectangle = Rectangle {
            top_left,
            bottom_right,
            style: self.style,
        };

        self.drawing.draw(rectangle.translate(self.offset));
    }

    /// Draws a triangle.
    pub fn triangle(&mut self, p1: Coord, p2: Coord, p3: Coord) {
        let triangle = Triangle {
            p1,
            p2,
            p3,
            style: self.style,
        };

        self.drawing.draw(triangle.translate(self.offset));
    }

    /// Draws text.
    pub fn text<'b, F>(&mut self, text: &'b str, p: Coord)
    where
        F: Font<'b, C> + Transform + IntoIterator<Item = Pixel<C>>,
    {
        let text = F::render_str(text)
            .style(self.style)
            .translate(p + self.offset);

        self.drawing.draw(text.into_iter());
    }

    /// Changes the fill color.
    pub fn set_fill_color<T>(&mut self, color: T)
    where
        T: Into<Option<C>>,
    {
        self.style.fill_color = color.into();
    }

    /// Changes the stroke color.
    pub fn set_stroke_color<T>(&mut self, color: T)
    where
        T: Into<Option<C>>,
    {
        self.style.stroke_color = color.into();
    }

    /// Changes the stroke width.
    pub fn set_stroke_width(&mut self, width: u8) {
        self.style.stroke_width = width;
    }

    /// Translates the context.
    pub fn translate(&mut self, delta: Coord) {
        self.offset += delta;
    }

    /// Resets the context translation.
    pub fn reset_translate(&mut self) {
        self.offset = Coord::new(0, 0);
    }
}

/// Extension trait to for context function.
pub trait ContextExt<'a, C, D>
where
    D: Drawing<C>,
    C: PixelColor,
{
    /// Creates a drawing context.
    fn context(&'a mut self) -> Context<'a, C, D>;
}

impl<'a, C, D> ContextExt<'a, C, D> for D
where
    D: Drawing<C>,
    C: PixelColor,
{
    fn context(&'a mut self) -> Context<'a, C, D> {
        Context {
            drawing: self,
            style: Style::default(),
            offset: Coord::new(0, 0),
            color_type: PhantomData,
        }
    }
}
