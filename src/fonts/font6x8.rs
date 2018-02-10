use super::*;
use super::super::image::{ Image1BPP };

const font: &[u8] = include_bytes!("../../font6x8_1bpp.raw");
// const font_widths: &[u8] = include_bytes!("../../font6x8_widths.raw");
const char_height: u32 = 8;
const char_width: u32 = 6;
const first_charcode: u32 = 32;		// A space
const image_width: u32 = 192;
const max_stride: u32 = 1;	// 2 bytes is max glyph width
const byte_width: u32 = image_width / 8;
const max_string_length: u32 = 8;
const chars_per_row: u32 = image_width / char_width;
const bits_per_char: u32 = char_width * char_height;

use super::{ FontBuffer1BPP, FONT_BUFFER_SIZE };

pub struct Font6x8 {}

impl Font for Font6x8 {
	fn render_str(text: &str) -> Result<(FontBuffer1BPP, u32, u32), &'static str> {
		// let bytes_per_char = char_height;

		let bytes_in_row = FONT_BUFFER_SIZE as u32 / char_height as u32;
		let bits_in_row = bytes_in_row * 8;

		// Would we fill the buffer up by rendering this string?
		if text.len() * (char_width * char_height) as usize > FONT_BUFFER_SIZE * 8 {
			Err("String exceeds max length")
		} else {
			let text_width = text.len() as u32 * char_width;

			let mut bitmap: FontBuffer1BPP = [0; FONT_BUFFER_SIZE];

			let mut col_counter = 0;

			for (idx, c) in text.chars().enumerate() {
				// Char _code_ offset from first char, most often a space
				// E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
				let char_offset = c as u32 - first_charcode;
				let row = char_offset / chars_per_row;

				// Top left corner of character, in pixels
				// let char_x = (char_offset % chars_per_row) * char_width;
				let char_x = (char_offset - (row * chars_per_row)) * (char_width - 1);
				let char_y = row * char_height;

				// println!("'{}' -> ({},{})", c, char_x, char_y);
				// println!("'{}' -> offset {}, row {}", c, char_offset, row);

				// Walk down column of character box
				for char_walk_y in 0..char_height {
					// Walk along each wrong of character box
					for char_walk_x in 0..char_width {
						// (x, y) coord turned into a bit index from top left (0, 0) of font bitmap
						let bitmap_bit_index = (
							(char_offset)
							+ (char_x + char_walk_x)
							+ (char_walk_y * image_width)
						);

						// Where to put the value of this bit into the resulting output array
						let out_bit_index = (
							(idx as u32 * char_width)
							+ char_walk_x
							+ (char_walk_y * bits_in_row)
						);

						let bitmap_byte = bitmap_bit_index / 8;
						let bitmap_bit = 7 - (bitmap_bit_index % 8);

						let out_byte = out_bit_index / 8;
						let out_bit = out_bit_index % 8;

						let bit_value = (font[bitmap_byte as usize] >> bitmap_bit) & 1;

						// println!("out pos {}:{} selected from {}:{} (idx {}) = {}", out_byte, out_bit, bitmap_byte, bitmap_bit, bitmap_bit_index, bit_value);

						if bit_value == 0 {
						    bitmap[out_byte as usize] &= !(1 << out_bit);
						} else {
						    bitmap[out_byte as usize] |= 1 << out_bit;
						}
					}
				}
			}

			Ok((bitmap, text_width, char_height))
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_a_bang() {
    	let (buf, w, h) = Font6x8::render_str("!)(").unwrap();

    	println!("{:?}", buf);

    	assert_eq!(true, true);

        // assert_eq!(
        // 	[
        // 		buf[0],
        // 	],
        	
        // );
    }

    // #[test]
    // fn it_gets_a_capital_a() {
    //     assert_eq!(coords_to_index(0, 0), (0, 0));
    // }
}
