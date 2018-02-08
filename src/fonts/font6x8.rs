use super::*;
use super::super::image::{ Image1BPP };

const font: &[u8] = include_bytes!("../../font6x8_1bpp.raw");
const font_widths: &[u8] = include_bytes!("../../font6x8_widths.raw");
const char_height: u32 = 8;
const char_width: u32 = 6;
const first_charcode: u32 = 32;		// A space
const image_width: u32 = 192;
const max_stride: u32 = 1;	// 2 bytes is max glyph width
const byte_width: u32 = image_width / 8;
const max_string_length: u32 = 8;

use super::{ FontBuffer1BPP, FONT_BUFFER_SIZE };

pub struct Font6x8 {}

impl Font for Font6x8 {
	fn render_str(text: &str) -> Result<(FontBuffer1BPP, u32, u32), &'static str> {
		let bytes_per_char = char_height;

		let bytes_in_row = FONT_BUFFER_SIZE as u32 / char_height as u32;

		if text.len() * bytes_per_char as usize > FONT_BUFFER_SIZE {
			Err("String exceeds max length")
		} else {
			let mut bitmap: FontBuffer1BPP = [0; FONT_BUFFER_SIZE];

			for row in 0..char_height {
				for (idx, c) in text.chars().enumerate() {
					let char_offset = c as u32 - first_charcode;
					let font_byte_offset = char_offset + (byte_width * row);

					bitmap[idx + (bytes_in_row * row) as usize] = font[font_byte_offset as usize];
				}
			}

			Ok((bitmap, (text.len() * 8) as u32, char_height))
		}
	}
}
