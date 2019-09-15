use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render;

/// A derivation of sdl2::event::Event mapped to embedded-graphics coordinates
pub enum SimulatorEvent {
    /// A keypress event, fired on keyDown
    KeyDown {
        /// The key being pressed
        keycode: Option<Keycode>,
        /// Any modifier being held at the time of keydown
        keymod: Mod,
    },
    /// A mouse click event, fired on mouseUp
    MouseButtonUp {
        /// The mouse button being clicked
        mouse_btn: MouseButton,
        /// The location of the click in Simulator coordinates
        point: Point,
    },
}

/// Simulator window
pub struct Window {
    scale: usize,
    pixel_spacing: usize,

    canvas: render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    input_events: Vec<SimulatorEvent>,
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
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            scale,
            pixel_spacing,
            canvas,
            event_pump,
            input_events: vec![],
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
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Rgb888) {
        self.set_color(color);

        let pitch = self.scale + self.pixel_spacing;

        let x = (x * pitch) as i32;
        let y = (y * pitch) as i32;
        let size = self.scale as u32;

        let r = Rect::new(x, y, size, size);
        self.canvas.fill_rect(r).unwrap();
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
                    keycode, keymod, ..
                } => {
                    self.input_events
                        .push(SimulatorEvent::KeyDown { keycode, keymod });
                    return false;
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    let point = self.map_input_to_point((x, y));
                    self.input_events
                        .push(SimulatorEvent::MouseButtonUp { point, mouse_btn });
                    return false;
                }
                _ => {}
            }
        }

        false
    }

    pub fn get_input_event(&mut self) -> Option<SimulatorEvent> {
        self.input_events.pop()
    }

    /// Convert SDL2 input event coordinates into a point on the simulator coordinate system
    fn map_input_to_point(&self, coords: (i32, i32)) -> Point {
        let pitch = (self.scale + self.pixel_spacing) as i32;
        Point::new(coords.0 / pitch, coords.1 / pitch)
    }
}
