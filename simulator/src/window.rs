use crate::{display::SimulatorDisplay, theme::BinaryColorTheme};
use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::{PixelColor, Rgb888, RgbColor},
};
use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
    mouse::{MouseButton, MouseWheelDirection},
    render,
};
use std::{thread, time::Duration};

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
    /// An exit event
    Quit,
}

/// Simulator window
pub struct Window {
    scale: usize,
    pixel_spacing: usize,
    theme: BinaryColorTheme,

    canvas: render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
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
        }
    }

    /// Updates the window.
    pub fn update<C>(&mut self, display: &SimulatorDisplay<C>)
    where
        C: PixelColor + Into<Rgb888>,
    {
        let (width, height) = self.canvas.window().size();

        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, width, height)
            .unwrap();

        texture
            .with_lock(None, |data: &mut [u8], pitch: usize| {
                let pixel_pitch = (self.scale + self.pixel_spacing) as i32;

                for y in 0..height {
                    for x in 0..width {
                        let source_point = Point {
                            x: x as i32 / pixel_pitch,
                            y: y as i32 / pixel_pitch,
                        };
                        let color = if x as i32 % pixel_pitch < self.scale as i32
                            && y as i32 % pixel_pitch < self.scale as i32
                        {
                            display.get_pixel(source_point).into()
                        } else {
                            Rgb888::BLACK
                        };
                        let color = self.theme.convert(color);

                        let index = x as usize * 3 + y as usize * pitch;
                        data[index] = color.r();
                        data[index + 1] = color.g();
                        data[index + 2] = color.b();
                    }
                }
            })
            .unwrap();

        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }

    /// Shows a static display.
    ///
    /// This methods updates the window once and loops until the simulator window
    /// is closed.
    pub fn show_static<C>(&mut self, display: &SimulatorDisplay<C>)
    where
        C: PixelColor + Into<Rgb888>,
    {
        self.update(&display);

        'running: loop {
            if self.events().any(|e| e == SimulatorEvent::Quit) {
                break 'running;
            }
            thread::sleep(Duration::from_millis(20));
        }
    }

    /// Handle events
    /// Return an iterator of all captured SimulatorEvent
    pub fn events(&mut self) -> impl Iterator<Item = SimulatorEvent> + '_ {
        let scale = self.scale;
        let pixel_spacing = self.pixel_spacing;
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
