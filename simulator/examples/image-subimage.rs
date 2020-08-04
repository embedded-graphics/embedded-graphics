use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::Rectangle,
    style::{PrimitiveStyleBuilder, StrokeAlignment},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::mouse::MouseButton;
use std::{thread, time::Duration};
use tinytga::EgTga;

/// Pipes image based on [plums](https://opengameart.org/content/plums) by surt on OpenGameArt.org.
const PIPES_IMAGE: &[u8] = include_bytes!("./assets/pipes.tga");

const TILE_SIZE: Size = Size::new(32, 32);
const BACKGROUND_COLOR: Rgb888 = Rgb888::new(20, 12, 28);

fn position_to_tile(position: Point) -> Point {
    position.component_div(Point::zero() + TILE_SIZE)
}

fn tile_to_rectangle(tile: Point) -> Rectangle {
    let top_left = tile.component_mul(Point::zero() + TILE_SIZE);

    Rectangle::new(top_left, TILE_SIZE)
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum MouseState {
    Idle,
    Draw,
    Erase,
}

const SIZE: usize = 8;
const VALID_TILE_COORDINATES: Rectangle =
    Rectangle::new(Point::zero(), Size::new_equal(SIZE as u32));

struct Pipes {
    state: [[bool; SIZE]; SIZE],
}

impl Pipes {
    fn new() -> Self {
        Self {
            state: INITIAL_STATE,
        }
    }

    fn get_state(&self, tile: Point) -> bool {
        if VALID_TILE_COORDINATES.contains(tile) {
            self.state[tile.y as usize][tile.x as usize]
        } else {
            false
        }
    }

    fn set_state(&mut self, tile: Point, value: bool) {
        if VALID_TILE_COORDINATES.contains(tile) {
            self.state[tile.y as usize][tile.x as usize] = value;
        }
    }

    fn iter(&self) -> impl Iterator<Item = (Point, usize)> + '_ {
        self.state.iter().enumerate().flat_map(move |(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_x, state)| **state)
                .map(move |(x, _state)| {
                    let tile = Point::new(x as i32, y as i32);
                    (tile, self.get_tile_index(tile))
                })
        })
    }

    fn get_tile_index(&self, tile: Point) -> usize {
        let tile_above = tile - Point::new(0, 1);
        let tile_below = tile + Point::new(0, 1);
        let tile_left = tile - Point::new(1, 0);
        let tile_right = tile + Point::new(1, 0);

        let mut index = 0;
        if self.get_state(tile_above) {
            index += 1;
        }
        if self.get_state(tile_below) {
            index += 2;
        }
        if self.get_state(tile_left) {
            index += 4;
        }
        if self.get_state(tile_right) {
            index += 8;
        }

        index
    }
}

const TILE_LAYOUT: &[Point] = &[
    Point::new(3, 1), //
    Point::new(3, 2), //A
    Point::new(3, 2), // B
    Point::new(3, 2), //AB
    Point::new(3, 1), //  L
    Point::new(2, 2), //A L
    Point::new(2, 1), // BL
    Point::new(2, 3), //ABL
    Point::new(3, 1), //   R
    Point::new(1, 2), //A  R
    Point::new(1, 1), // B R
    Point::new(1, 4), //AB R
    Point::new(3, 1), //  LR
    Point::new(2, 4), //A LR
    Point::new(1, 3), // BLR
    Point::new(0, 0), //ABLR
];

const T: bool = true;
const F: bool = false;
const INITIAL_STATE: [[bool; SIZE]; SIZE] = [
    [F, F, F, F, F, F, F, F],
    [T, T, T, F, T, T, T, F],
    [T, F, F, F, T, F, F, F],
    [T, T, F, F, T, F, T, T],
    [T, F, F, F, T, F, F, T],
    [T, T, T, F, T, T, T, T],
    [F, F, F, F, F, F, F, F],
    [F, F, F, F, F, F, F, F],
];

fn main() -> Result<(), core::convert::Infallible> {
    let mut pipes = Pipes::new();

    let outline_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::WHITE)
        .stroke_width(2)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();

    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(256, 256));

    let all_tiles: EgTga<Rgb888> = EgTga::from_slice(PIPES_IMAGE).unwrap();

    let pipe_tiles: Vec<_> = TILE_LAYOUT
        .iter()
        .map(|tile| {
            all_tiles.sub_image(&Rectangle::new(
                tile.component_mul(Point::new(32, 32)),
                TILE_SIZE,
            ))
        })
        .collect();

    let mut mouse_state = MouseState::Idle;
    let mut highlighted_tile = Point::new(0, 0);

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Pipes", &output_settings);

    'running: loop {
        display.clear(BACKGROUND_COLOR)?;

        for (tile, tile_index) in pipes.iter() {
            let tile_position = tile.component_mul(Point::new_equal(TILE_SIZE.width as i32));

            let image: Image<_, Rgb888> = Image::new(&pipe_tiles[tile_index], tile_position);
            image.draw(&mut display)?;
        }

        let area = tile_to_rectangle(highlighted_tile);
        area.into_styled(outline_style).draw(&mut display)?;

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running Ok(()),
                SimulatorEvent::MouseMove { point } => {
                    highlighted_tile = position_to_tile(point);
                }
                SimulatorEvent::MouseButtonDown { mouse_btn, .. } => {
                    mouse_state = match mouse_btn {
                        MouseButton::Left => MouseState::Draw,
                        MouseButton::Right => MouseState::Erase,
                        _ => MouseState::Idle,
                    };
                }
                SimulatorEvent::MouseButtonUp { .. } => {
                    mouse_state = MouseState::Idle;
                }
                _ => (),
            }
        }

        if mouse_state != MouseState::Idle {
            pipes.set_state(highlighted_tile, mouse_state == MouseState::Draw);
        }

        thread::sleep(Duration::from_millis(50));
    }
}
