use anyhow::{anyhow, Result};
use bdf_parser::BdfFont;
use bdf_to_mono::{Encoding, MonoFontData};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    let fonts_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fonts");

    let mut rust = HashMap::new();
    rust.insert(Encoding::Ascii, RUST_HEADER.to_string());
    rust.insert(Encoding::Latin1, RUST_HEADER.to_string());

    let mut paths = Vec::new();

    for entry in fonts_dir.join("src").read_dir()? {
        let path = entry?.path();

        // Ignore directories and non BDF files
        if path.is_file() && path.extension() == Some(OsStr::new("bdf")) {
            paths.push(path);
        }
    }

    // Sort paths to make sure the order of fonts in the generated files doesn't change.
    paths.sort();

    for file in paths {
        println!("Converting {}", file.file_name().unwrap().to_string_lossy());

        let font_name = file
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .replace("B", "Bold")
            .replace("O", "Italic");
        let font_const = format!("FONT_{}", font_name.to_ascii_uppercase());

        let bdf = fs::read(file)?;
        let font = BdfFont::parse(&bdf).map_err(|_| anyhow!("couldn't parse BDF file"))?;

        for encoding in [Encoding::Ascii, Encoding::Latin1].iter().copied() {
            let data = MonoFontData::new(&font, encoding)?;

            let raw_file = raw_directory(&fonts_dir, encoding)?
                .join(&font_name)
                .with_extension("raw");
            data.save_raw(raw_file)?;

            let png_file = png_directory(&fonts_dir, encoding)?
                .join(&font_name)
                .with_extension("png");
            data.save_png(png_file)?;

            let raw_file_path = format!("../../../fonts/{}/raw/{}.raw", encoding, font_name);
            rust.get_mut(&encoding)
                .unwrap()
                .push_str(&data.rust(&font_const, &raw_file_path));
        }
    }

    let mono_font = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/mono_font");
    fs::write(
        mono_font.join("ascii/generated.rs"),
        &rust.get(&Encoding::Ascii).unwrap(),
    )?;
    fs::write(
        mono_font.join("latin1/generated.rs"),
        &rust.get(&Encoding::Latin1).unwrap(),
    )?;

    Ok(())
}

fn raw_directory(base: &Path, encoding: Encoding) -> Result<PathBuf> {
    let raw_directory = base.join(&encoding.to_string()).join("raw");
    fs::create_dir_all(&raw_directory)?;

    Ok(raw_directory)
}

fn png_directory(base: &Path, encoding: Encoding) -> Result<PathBuf> {
    let png_directory = base.join(&encoding.to_string()).join("png");
    fs::create_dir_all(&png_directory)?;

    Ok(png_directory)
}

const RUST_HEADER: &str = r#"
    // GENERATED CODE DO NOT MODIFY!
    // Any manual changes to this file will be overwritten!

    use crate::{mono_font::{MonoFont, MonoFontBuilder}, geometry::Size, image::ImageRaw};
"#;
