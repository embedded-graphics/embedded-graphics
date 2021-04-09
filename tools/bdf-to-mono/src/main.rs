use bdf_parser::BdfFont;
use bdf_to_mono::{Encoding, MonoFontData};

fn main() {
    let bdf_file = std::env::args().nth(1).expect("missing BDF file argument");
    let bdf = std::fs::read(&bdf_file).expect("couldn't open BDF file");
    let font = BdfFont::parse(&bdf).expect("couldn't parse BDF file");

    // TODO: make encoding configurable
    let _bitmap = MonoFontData::new(&font, Encoding::Ascii).unwrap();

    // TODO: add command line arguments
    // std::io::stdout().write_all(&bitmap.data).unwrap()
}
