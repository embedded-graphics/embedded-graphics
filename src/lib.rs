// #![no_std]

pub mod image;
pub mod fonts;

pub trait Drawing {
	fn set_pixel(&mut self, x: u32, y: u32, value: u8);
	fn draw_image_8bpp(&mut self, image: &image::Image8BPP, x: u32, y: u32);
	fn draw_image_1bpp(&mut self, image: &image::Image1BPP, x: u32, y: u32);
}
