mod font6x8;

// TODO: Move into a crate prelude
pub use self::font6x8::Font6x8;

pub trait Font<'a> {
    fn render_str(chars: &'a str) -> Self;
}
