use super::*;

const FONT_IMAGE: &[u8] = include_bytes!("../../font6x8_1bpp.raw");
// TODO: Non-fixed-width font
// const FONT_CHAR_WIDTHS: &[u8] = include_bytes!("../../font6x8_widths.raw");
const CHAR_HEIGHT: u32 = 8;
const CHAR_WIDTH: u32 = 6;
const FIRST_CHARCODE: u32 = 32;		// A space
const FONT_IMAGE_WIDTH: u32 = 192;
const CHARS_PER_ROW: u32 = FONT_IMAGE_WIDTH / CHAR_WIDTH;

use super::{ FontBuffer1BPP, FONT_BUFFER_SIZE };

#[derive(Debug, Clone, Copy)]
pub struct Font6x8 {}

impl Font for Font6x8 {
	fn render_str(text: &str) -> Result<(FontBuffer1BPP, u32, u32), &'static str> {
		let bytes_in_row = FONT_BUFFER_SIZE as u32 / CHAR_HEIGHT;
		let bits_in_row = bytes_in_row * 8;

		// Would we fill the buffer up by rendering this string?
		if text.len() * (CHAR_WIDTH * CHAR_HEIGHT) as usize > FONT_BUFFER_SIZE * 8 {
			Err("String exceeds max length")
		} else {
			let mut bitmap: FontBuffer1BPP = [0; FONT_BUFFER_SIZE];

			for (idx, c) in text.chars().enumerate() {
				// Char _code_ offset from first char, most often a space
				// E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
				let char_offset = c as u32 - FIRST_CHARCODE;
				let row = char_offset / CHARS_PER_ROW;

				// Top left corner of character, in pixels
				let char_x = (char_offset - (row * CHARS_PER_ROW)) * CHAR_WIDTH;
				let char_y = row * CHAR_HEIGHT;

				// Walk down column of character box
				for char_walk_y in 0..CHAR_HEIGHT {
					// Walk along each row of character box
					for char_walk_x in 0..CHAR_WIDTH {
						// (x, y) coord turned into a bit index from top left (0, 0) of font bitmap
						let bitmap_bit_index = 
							char_x 		// X pixel offset for char
							+ (FONT_IMAGE_WIDTH * char_y)		// Character row offset (row 0 = 0, row 1 = (192 * 8) = 1536)
							+ char_walk_x		// X offset for the pixel block that comprises this char
							+ (char_walk_y * FONT_IMAGE_WIDTH)		// Y offset for pixel block
						;

						// Where to put the value of this bit into the resulting output array
						let out_bit_index =
							(idx as u32 * CHAR_WIDTH)
							+ char_walk_x
							+ (char_walk_y * bits_in_row)
						;

						let bitmap_byte = bitmap_bit_index / 8;
						let bitmap_bit = 7 - (bitmap_bit_index % 8);

						let out_byte = out_bit_index / 8;
						let out_bit = 7 - (out_bit_index % 8);

						let bit_value = (FONT_IMAGE[bitmap_byte as usize] >> bitmap_bit) & 1;

						if bit_value == 0 {
						    bitmap[out_byte as usize] &= !(1 << out_bit);
						} else {
						    bitmap[out_byte as usize] |= 1 << out_bit;
						}
					}
				}
			}

 			Ok((bitmap, bits_in_row, CHAR_HEIGHT))
		}
	}
}
