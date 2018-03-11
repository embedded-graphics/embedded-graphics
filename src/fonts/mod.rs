mod font6x8;

const FONT_BUFFER_SIZE: usize = 1024;

pub use self::font6x8::Font6x8;

// Pixel buffer for 1BPP font data
pub type FontBuffer1BPP = [u8; FONT_BUFFER_SIZE];

pub trait Font {
    fn render_str(chars: &str) -> Result<(FontBuffer1BPP, u32, u32), &'static str>;
}
