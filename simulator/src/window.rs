use crate::display::SimulatorDisplay;
use crate::theme::BinaryColorTheme;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::pixelcolor::{PixelColor, Rgb888, RgbColor};
use embedded_graphics::DrawTarget;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::{MouseButton, MouseWheelDirection};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render;
use std::convert::TryFrom;

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
}

/// Simulator window
pub struct Window {
    scale: usize,
    pixel_spacing: usize,
    theme: BinaryColorTheme,

    canvas: render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    input_events: Vec<SimulatorEvent>,
}

impl Window {
    /// Creates a new simulator window.
    pub(crate) fn new(
        display_size: Size,
        scale: usize,
        pixel_spacing: usize,
        theme: BinaryColorTheme,
        title: &str,
    ) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let width = display_size.width as usize;
        let height = display_size.height as usize;
        let window_width = width * scale + (width - 1) * pixel_spacing;
        let window_height = height * scale + (height - 1) * pixel_spacing;

        let window = video_subsystem
            .window(title, window_width as u32, window_height as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            scale,
            pixel_spacing,
            theme,
            canvas,
            event_pump,
            input_events: vec![],
        }
    }

    /// Updates the window.
    pub fn update<C>(&mut self, display: &SimulatorDisplay<C>)
    where
        C: PixelColor + Into<Rgb888>,
    {
        //self.window.clear(self.theme.convert(BinaryColor::Off));
        self.canvas
            .set_draw_color(self.convert_color(Rgb888::BLACK));
        self.canvas.clear();

        let Size { width, height } = display.size();
        let width = i32::try_from(width).expect("display width too large");
        let height = i32::try_from(height).expect("display height too large");

        let pixel_pitch = i32::try_from(self.scale + self.pixel_spacing)
            .expect("pixel scale and/or spacing too large");
        let pixel_size = u32::try_from(self.scale).expect("pixel scale too large");

        for y in 0..height {
            for x in 0..width {
                let point = Point { x, y };
                let color = display.get_pixel(point);

                //let color = self.theme.convert(color);

                let x = x * pixel_pitch;
                let y = y * pixel_pitch;

                let r = Rect::new(x, y, pixel_size, pixel_size);
                self.canvas.set_draw_color(self.convert_color(color));
                self.canvas.fill_rect(r).unwrap();
            }
        }

        self.canvas.present();
    }

    fn convert_color<C>(&self, color: C) -> Color
    where
        C: PixelColor + Into<Rgb888>,
    {
        let color = self.theme.convert(color.into());
        Color::RGB(color.r(), color.g(), color.b())
    }

    /// Handle events
    pub fn handle_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return true;
                }
                Event::KeyDown {
                    keycode,
                    keymod,
                    repeat,
                    ..
                } => {
                    if let Some(valid_keycode) = keycode {
                        self.input_events.push(SimulatorEvent::KeyDown {
                            keycode: valid_keycode,
                            keymod,
                            repeat,
                        });
                    }
                }
                Event::KeyUp {
                    keycode,
                    keymod,
                    repeat,
                    ..
                } => {
                    if let Some(valid_keycode) = keycode {
                        self.input_events.push(SimulatorEvent::KeyUp {
                            keycode: valid_keycode,
                            keymod,
                            repeat,
                        });
                    }
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    let point = map_input_to_point((x, y), self.scale, self.pixel_spacing);
                    self.input_events
                        .push(SimulatorEvent::MouseButtonUp { point, mouse_btn });
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let point = map_input_to_point((x, y), self.scale, self.pixel_spacing);
                    self.input_events
                        .push(SimulatorEvent::MouseButtonDown { point, mouse_btn });
                }
                Event::MouseWheel {
                    x, y, direction, ..
                } => {
                    self.input_events.push(SimulatorEvent::MouseWheel {
                        scroll_delta: Point::new(x, y),
                        direction,
                    });
                }
                _ => {}
            }
        }

        false
    }

    /// Return all captured input events
    pub fn get_input_events(&mut self) -> Vec<SimulatorEvent> {
        let input_events = self.input_events.clone();
        self.input_events.clear();
        input_events
    }
}

/// Convert SDL2 input event coordinates into a point on the simulator coordinate system
fn map_input_to_point(coords: (i32, i32), scale: usize, pixel_spacing: usize) -> Point {
    let pitch = (scale + pixel_spacing) as i32;
    Point::new(coords.0 / pitch, coords.1 / pitch)
}
