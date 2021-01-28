use bdf_to_mono::bdf_to_bitmap;
use std::io::Write;

fn main() {
    let bdf_file = std::env::args().nth(1).expect("missing BDF file argument");
    let bdf = std::fs::read_to_string(&bdf_file).expect("couldn't open BDF file");

    let bitmap = bdf_to_bitmap(&bdf).unwrap();

    std::io::stdout().write_all(&bitmap.data).unwrap()
}
