use std::path::PathBuf;

use bdf_parser::BdfFont;
use bdf_to_mono::{Encoding, MonoFontData};
use clap::Clap;

#[derive(Clap)]
struct Args {
    #[clap(about = "BDF input")]
    bdf_file: PathBuf,
    #[clap(about = "Name of MonoFont constant")]
    mono_font_const: String,
    #[clap(long, about = "write PNG file")]
    png: Option<PathBuf>,
    #[clap(long, about = "write RAW file")]
    raw: Option<PathBuf>,
    #[clap(long, about = "encoding", possible_values = &["ascii", "latin1"], default_value = "ascii")]
    encoding: String,
}

fn main() {
    let args: Args = Args::parse();
    let encoding = match args.encoding.as_str() {
        "ascii" => Encoding::Ascii,
        "latin1" => Encoding::Latin1,
        _ => unreachable!(),
    };

    let bdf = std::fs::read(args.bdf_file).expect("couldn't open BDF file");
    let font = BdfFont::parse(&bdf).expect("couldn't parse BDF file");

    let bitmap = MonoFontData::new(&font, encoding).unwrap();

    if let Some(png_file) = &args.png {
        bitmap.save_png(png_file).unwrap();
    }

    if let Some(raw_file) = &args.raw {
        bitmap.save_png(raw_file).unwrap();
        println!(
            "{}",
            bitmap.rust(
                &args.mono_font_const,
                &raw_file.as_os_str().to_string_lossy()
            )
        );
    } else {
        println!("{}", bitmap.rust(&args.mono_font_const, "RAW_FILE"));
    }
}
