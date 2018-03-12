mod font6x8;

const FONT_BUFFER_SIZE: usize = 1024;

pub use self::font6x8::Font6x8;

// Pixel buffer for 1BPP font data
pub type FontBuffer1BPP = [u8; FONT_BUFFER_SIZE];

#[derive(Copy, Clone)]
pub struct RenderedText {
    width: u32,
    height: u32,
    bitmap: FontBuffer1BPP,
}

#[derive(Copy, Clone)]
pub struct RenderedTextIterator {
    width: u32,
    height: u32,
    bitmap: FontBuffer1BPP,
    x: u32,
    y: u32,
}

impl IntoIterator for RenderedText {
    type IntoIter = RenderedTextIterator;
    type Item = (u32, u32, u8);

    fn into_iter(self) -> Self::IntoIter {
        RenderedTextIterator {
            width: self.width,
            height: self.height,
            bitmap: self.bitmap,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for RenderedTextIterator {
    type Item = (u32, u32, u8);

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Move pixel iterator crap from SSD1306 crate into here
    }
}

pub trait Font {
    fn render_str(chars: &str) -> Result<RenderedText, &'static str>;
}
