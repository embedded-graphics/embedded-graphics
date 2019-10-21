use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::primitives::{Circle, Line, Rectangle, Triangle};

use sdl2::event::Event;
//todo any reason to feature gate gfx?
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::{MouseButton, MouseWheelDirection};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render;

/// A derivation of sdl2::event::Event mapped to embedded-graphics coordinates
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SimulatorEvent {
    /// A keypress event, fired on keyUp
    KeyUp {
        /// The key being released
        keycode: Keycode,
        /// Any modifier being held at the time of keyup
        keymod: Mod,
        /// Whether the key is repeating
        repeat: bool,
    },
    /// A keypress event, fired on keyDown
    KeyDown {
        /// The key being pressed
        keycode: Keycode,
        /// Any modifier being held at the time of keydown
        keymod: Mod,
        /// Whether the key is repeating
        repeat: bool,
    },
    /// A mouse click event, fired on mouseUp
    MouseButtonUp {
        /// The mouse button being released
        mouse_btn: MouseButton,
        /// The location of the mouse in Simulator coordinates
        point: Point,
    },
    /// A mouse click event, fired on mouseDown
    MouseButtonDown {
        /// The mouse button being pressed
        mouse_btn: MouseButton,
        /// The location of the mouse in Simulator coordinates
        point: Point,
    },
    /// A mouse wheel event
    MouseWheel {
        /// The scroll wheel delta in the x and y direction
        scroll_delta: Point,
        /// The directionality of the scroll (normal or flipped)
        direction: MouseWheelDirection,
    },
    /// An exit events
    Quit,
}

/// Simulator window
pub struct Window {
    scale: usize,
    pixel_spacing: usize,

    canvas: render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl Window {
    /// Create a new simulator window
    pub fn new(
        width: usize,
        height: usize,
        scale: usize,
        pixel_spacing: usize,
        title: &str,
    ) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window_width = width * scale + (width - 1) * pixel_spacing;
        let window_height = height * scale + (height - 1) * pixel_spacing;

        let window = video_subsystem
            .window(title, window_width as u32, window_height as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            scale,
            pixel_spacing,
            canvas,
            event_pump,
        }
    }

    fn set_color(&mut self, color: Rgb888) {
        self.canvas
            .set_draw_color(Color::RGB(color.r(), color.g(), color.b()));
    }

    /// Clear window
    pub fn clear(&mut self, color: Rgb888) {
        self.set_color(color);
        self.canvas.clear();
    }

    /// Present window
    pub fn present(&mut self) {
        self.canvas.present();
    }

    /// Draw pixel
    /// very costly, better to use a gfx primitive
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Rgb888) {
        self.set_color(color);

        let pitch = self.scale + self.pixel_spacing;

        let x = (x * pitch) as i32;
        let y = (y * pitch) as i32;
        let size = self.scale as u32;

        let r = Rect::new(x, y, size, size);
        self.canvas.fill_rect(r).unwrap();
    }

    // pub fn draw_rectangles<C>(&mut self, rects: &[Rectangle<C>])
    // where
    //     C: PixelColor + Into<Rgb888>,
    // {
    //     unimplemented!();
    // }

    pub fn draw_circle<C>(&mut self, item: &Circle<C>)
    where
        C: RgbColor + Into<Rgb888>,
    {
        let Circle {
            center,
            radius,
            style,
        } = item;

        let Point { x, y } = center;

        //todo, i16 from i32...
        if let Some(fill) = style.fill_color {
            //neccesary?
            let color = Color::RGB(fill.r(), fill.g(), fill.b());

            let _ = self
                .canvas
                .filled_circle(*x as i16, *y as i16, *radius as i16, color);
        } else if let Some(stroke) = style.stroke_color {
            //how to stroke? style.stroke_width
            let color = Color::RGB(stroke.r(), stroke.g(), stroke.b());

            let _ = self
                .canvas
                .circle(*x as i16, *y as i16, *radius as i16, color);
        }
    }

    pub fn draw_line<C>(&mut self, _item: &Line<C>)
    where
        C: RgbColor + Into<Rgb888>,
    {
        unimplemented!();
    }

    pub fn draw_triangle<C>(&mut self, _item: &Triangle<C>)
    where
        C: RgbColor + Into<Rgb888>,
    {
        unimplemented!();
    }

    pub fn draw_rectangle<C>(&mut self, item: &Rectangle<C>)
    where
        C: RgbColor + Into<Rgb888>,
    {
        let Rectangle {
            top_left,
            bottom_right,
            style,
        } = item;

        let Point { x: x1, y: y1 } = top_left;
        let Point { x: x2, y: y2 } = bottom_right;

        //todo, fill or stroke?
        //todo, i16 from i32...
        if let Some(fill) = style.fill_color {
            //neccesary?
            let color = Color::RGB(fill.r(), fill.g(), fill.b());

            let _ = self
                .canvas
                .rectangle(*x1 as i16, *y1 as i16, *x2 as i16, *y2 as i16, color);
        }
    }

    /// Return an iterator of all captured SimulatorEvent
    pub fn get_input_events(&mut self) -> impl Iterator<Item = SimulatorEvent> + '_ {
        let scale = self.scale.clone();
        let pixel_spacing = self.pixel_spacing.clone();
        self.event_pump
            .poll_iter()
            .filter_map(move |event| match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => Some(SimulatorEvent::Quit),
                Event::KeyDown {
                    keycode,
                    keymod,
                    repeat,
                    ..
                } => {
                    if let Some(valid_keycode) = keycode {
                        Some(SimulatorEvent::KeyDown {
                            keycode: valid_keycode,
                            keymod,
                            repeat,
                        })
                    } else {
                        None
                    }
                }
                Event::KeyUp {
                    keycode,
                    keymod,
                    repeat,
                    ..
                } => {
                    if let Some(valid_keycode) = keycode {
                        Some(SimulatorEvent::KeyUp {
                            keycode: valid_keycode,
                            keymod,
                            repeat,
                        })
                    } else {
                        None
                    }
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    let point = map_input_to_point((x, y), scale, pixel_spacing);
                    Some(SimulatorEvent::MouseButtonUp { point, mouse_btn })
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let point = map_input_to_point((x, y), scale, pixel_spacing);
                    Some(SimulatorEvent::MouseButtonDown { point, mouse_btn })
                }
                Event::MouseWheel {
                    x, y, direction, ..
                } => Some(SimulatorEvent::MouseWheel {
                    scroll_delta: Point::new(x, y),
                    direction,
                }),
                _ => None,
            })
    }
}

/// Convert SDL2 input event coordinates into a point on the simulator coordinate system
fn map_input_to_point(coords: (i32, i32), scale: usize, pixel_spacing: usize) -> Point {
    let pitch = (scale + pixel_spacing) as i32;
    Point::new(coords.0 / pitch, coords.1 / pitch)
}
