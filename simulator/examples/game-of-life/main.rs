//! # Example: Game of Life
//!
//! A zero player cellular automaton 'game' https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
//! Code slightly adapted from the wasm example for no_std use cases where Vec is not available
//! https://github.com/rustwasm/wasm_game_of_life

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;

use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, WindowBuilder};

use heapless::consts::*;
use heapless::Vec;

use std::thread;
use std::time::Duration;

mod life;
use life::{Cell, Universe};

/// The width and height of the display
const DISP_SIZE_X: u32 = 160;
const DISP_SIZE_Y: u32 = 128;

fn main() {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(DISP_SIZE_X, DISP_SIZE_Y));
    let mut window = WindowBuilder::new(&display)
        .title("Game Of Life")
        .scale(2)
        .build();

    //wasted space, but there is no U20480
    let cells = Vec::<Cell, U32768>::new();
    //more wasted space, a copy to edit while we keep last values
    let next = Vec::<Cell, U32768>::new();

    let mut universe = Universe::new(DISP_SIZE_X as u32, DISP_SIZE_Y as u32, cells, next);

    'running: loop {
        window.update(&display);

        for event in window.events() {
            if let SimulatorEvent::Quit = event {
                break 'running;
            }
        }
        universe.tick();

        universe
            .iter()
            .map(|(row, col, cell)| {
                let color = if cell == Cell::Alive {
                    BinaryColor::On
                } else {
                    BinaryColor::Off
                };
                Pixel(Point::new(col as i32, row as i32), color)
            })
            .draw(&mut display);

        thread::sleep(Duration::from_millis(50));
    }
}
