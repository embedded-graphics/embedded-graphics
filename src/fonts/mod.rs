mod font6x8;

pub use self::font6x8::Font6x8;

// TODO: Add to crate prelude
pub trait Font<'a> {
    fn render_str(chars: &'a str) -> Self;
}
