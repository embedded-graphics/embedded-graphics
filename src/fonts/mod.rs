pub mod font6x8;

use super::image::{ Image1BPP };

pub trait Font {
	fn render_str(chars: &str) -> Image1BPP;
}