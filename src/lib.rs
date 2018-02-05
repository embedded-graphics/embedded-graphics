#![no_std]

pub trait Drawing {
	fn set_pixel(&mut self, x: u8, y: u8, value: u8);
}
