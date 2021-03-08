use bdf_parser::BdfFont;
use bdf_to_mono::{bdf_to_bitmap, Encoding};
use std::io::Write;

fn main() {
    let bdf_file = std::env::args().nth(1).expect("missing BDF file argument");
    let bdf = std::fs::read(&bdf_file).expect("couldn't open BDF file");
    let font = BdfFont::parse(&bdf).expect("couldn't parse BDF file");

    // TODO: make encoding configurable
    let bitmap = bdf_to_bitmap(&font, Encoding::Ascii).unwrap();

    std::io::stdout().write_all(&bitmap.data).unwrap()
}
