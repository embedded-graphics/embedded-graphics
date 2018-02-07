use super::*;
use super::super::image::{ Image1BPP };

const font: &[u8] = include_bytes!("../../font6x8_1bpp.raw");
const font_widths: &[u8] = include_bytes!("../../font6x8_widths.raw");
const char_height: u8 = 8;
const first_charcode: u8 = 32;		// A space
const image_width: u8 = 192;

pub struct Font6x8 {}

impl Font6x8 {}

impl Font for Font6x8 {
	fn render_str(&self, chars: &str) -> Image1BPP {
		let (x, y, width) = char_pixel_info();

		let ptr = font.as_ptr();



		Image1BPP {
			width,
			height: char_height,
			imagedata: ;
		}
	}
}

// Offset x, offset y, char block width
fn char_pixel_info(c: char) -> (u8, u8, u8) {
	let char_index = (first_charcode - c as u8) as usize;

	// As if all the chars were on one line, compute the char X offset for a variable width font
	let cum_offset: u32 = font_widths
		.iter()
		.take(char_index)
		.fold(0u32, |accum, &width| accum + width as u32);

	let char_width = font_widths[char_index];

	let row_offset = (cum_offset / image_width as u32) as u8;

	let pixel_offset = cum_offset - (row_offset * image_width) as u32;

	(
		pixel_offset as u8,
		row_offset * char_height,
		char_width
	)
}