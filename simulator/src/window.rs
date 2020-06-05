use crate::{display::SimulatorDisplay, framebuffer::Framebuffer, output_settings::OutputSettings};
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
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
    /// Mouse move event
    MouseMove {
        /// The current mouse position
        point: Point,
    },
    /// An exit event
    Quit,
}

/// Simulator window
#[allow(dead_code)]
pub struct Window {
    framebuffer: Option<Framebuffer>,
    sdl_window: Option<SdlWindow>,
    title: String,
    output_settings: OutputSettings,
}

impl Window {
    /// Creates a new simulator window.
    pub fn new(title: &str, output_settings: &OutputSettings) -> Self {
        Self {
            framebuffer: None,
            sdl_window: None,
            title: String::from(title),
            output_settings: output_settings.clone(),
        }
    }

    /// Updates the window.
    pub fn update<C>(&mut self, display: &SimulatorDisplay<C>)
    where
        C: PixelColor + Into<Rgb888>,
    {
        if let Ok(path) = std::env::var("EG_SIMULATOR_DUMP") {
            display
                .to_image_buffer(&self.output_settings)
                .save(path)
                .unwrap();
            std::process::exit(0);
        }

        if self.framebuffer.is_none() {
            self.framebuffer = Some(Framebuffer::new(display, &self.output_settings));
        }

        if self.sdl_window.is_none() {
            self.sdl_window = Some(SdlWindow::new(display, &self.title, &self.output_settings));
        }

        let framebuffer = self.framebuffer.as_mut().unwrap();
        let sdl_window = self.sdl_window.as_mut().unwrap();

        framebuffer.update(display);
        sdl_window.update(&framebuffer);
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

    /// Returns an iterator of all captured SimulatorEvents.
    ///
    /// # Panics
    ///
    /// Panics if called before `update` is called at least once.
    pub fn events(&mut self) -> impl Iterator<Item = SimulatorEvent> + '_ {
        self.sdl_window
            .as_mut()
            .unwrap()
            .events(&self.output_settings)
    }
}

#[allow(dead_code)]
struct SdlWindow {
    canvas: render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl SdlWindow {
    #[allow(dead_code)]
    pub fn new<C>(
        display: &SimulatorDisplay<C>,
        title: &str,
        output_settings: &OutputSettings,
    ) -> Self
    where
        C: PixelColor + Into<Rgb888>,
    {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let size = output_settings.framebuffer_size(display);

        let window = video_subsystem
            .window(title, size.width, size.height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self { canvas, event_pump }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, framebuffer: &Framebuffer) {
        let Size { width, height } = framebuffer.size();

        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, width, height)
            .unwrap();

        texture
            .update(None, framebuffer.data.as_ref(), width as usize * 3)
            .unwrap();

        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }

    /// Handle events
    /// Return an iterator of all captured SimulatorEvent
    pub fn events(
        &mut self,
        output_settings: &OutputSettings,
    ) -> impl Iterator<Item = SimulatorEvent> + '_ {
        let output_settings = output_settings.clone();
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
                    let point = output_settings.output_to_display(Point::new(x, y));
                    Some(SimulatorEvent::MouseButtonUp { point, mouse_btn })
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let point = output_settings.output_to_display(Point::new(x, y));
                    Some(SimulatorEvent::MouseButtonDown { point, mouse_btn })
                }
                Event::MouseWheel {
                    x, y, direction, ..
                } => Some(SimulatorEvent::MouseWheel {
                    scroll_delta: Point::new(x, y),
                    direction,
                }),
                Event::MouseMotion { x, y, .. } => {
                    let point = output_settings.output_to_display(Point::new(x, y));
                    Some(SimulatorEvent::MouseMove { point })
                }
                _ => None,
            })
    }
}
