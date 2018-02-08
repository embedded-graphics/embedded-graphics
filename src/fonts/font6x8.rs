use super::*;
use super::super::image::{ Image1BPP };

const font: &[u8] = include_bytes!("../../font6x8_1bpp.raw");
const font_widths: &[u8] = include_bytes!("../../font6x8_widths.raw");
const char_height: u8 = 8;
const char_width: u8 = 6;
const first_charcode: u8 = 32;		// A space
const image_width: u8 = 192;
const max_stride: u8 = 1;	// 2 bytes is max glyph width
const byte_width: u8 = image_width / 8;

pub struct Font6x8 {}

impl Font6x8 {}

impl Font6x8 {
	fn render_str(text: &str) {
		for c in text.chars() {
			let byte_offset = (c as u8 - first_charcode);

			println!("charcode = {}, byte = {} {}", (c as u8 - first_charcode), byte_offset, byte_width);

			let bytes = [
				font[(byte_offset) as usize],
				font[(byte_offset + byte_width) as usize],
				font[(byte_offset + byte_width * 2) as usize],
				font[(byte_offset + byte_width * 3) as usize],
				font[(byte_offset + byte_width * 4) as usize],
				font[(byte_offset + byte_width * 5) as usize],
				font[(byte_offset + byte_width * 6) as usize],
				font[(byte_offset + byte_width * 7) as usize],
			];

			println!("{:?}", bytes);
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_str() {
        Font6x8::render_str("!");
    }
}

// Offset x, offset y, char block width
// fn char_pixel_info(c: char) -> (u8, u8, u8) {
// 	let char_index = (c as u8 - first_charcode) as usize;

// 	// As if all the chars were on one line, compute the char X offset for a variable width font
// 	let cum_offset: u32 = font_widths
// 		.iter()
// 		.take(char_index)
// 		.fold(0u32, |accum, &width| accum + width as u32);

// 	let char_width = font_widths[char_index];

// 	let row_offset = (cum_offset / image_width as u32) as u8;

// 	let pixel_offset = cum_offset - (row_offset * image_width) as u32;

// 	(
// 		pixel_offset as u8,
// 		row_offset * char_height,
// 		char_width
// 	)
// }
