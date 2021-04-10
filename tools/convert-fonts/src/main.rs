use anyhow::{anyhow, Result};
use bdf_parser::BdfFont;
use bdf_to_mono::{Encoding, MonoFontData};
use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    let fonts_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fonts");

    let mut rust_ascii = RUST_HEADER.to_string();
    let mut rust_latin1 = rust_ascii.clone();

    let mut paths = Vec::new();

    for entry in fonts_dir.join("src").read_dir()? {
        let path = entry?.path();

        // Ignore directories and non BDF files
        if path.is_file() && path.extension() == Some(OsStr::new("bdf")) {
            paths.push(path);
        }
    }

    let mut fonts = paths
        .iter()
        .map(|file| {
            println!("Parsing {}", file.file_name().unwrap().to_string_lossy());

            Font::new(file, &fonts_dir)
        })
        .collect::<Result<Vec<_>>>()?;

    fonts.sort_by(|a, b| {
        let (aw, ah) = a.ascii.glyph_size();
        let (bw, bh) = b.ascii.glyph_size();

        aw.cmp(&bw)
            .then(ah.cmp(&bh))
            .then(a.constant.cmp(&b.constant))
    });

    for font in fonts.iter() {
        println!("Converting {}", font.file_stem.to_string_lossy());

        font.save_files()?;

        rust_ascii.push_str(&font.rust(Encoding::Ascii));
        rust_latin1.push_str(&font.rust(Encoding::Latin1));
    }

    let mono_font = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/mono_font");
    fs::write(mono_font.join("ascii/generated.rs"), &rust_ascii)?;
    fs::write(mono_font.join("latin1/generated.rs"), &rust_latin1)?;

    update_font_table("../../src/mono_font/mod.rs", Encoding::Ascii, &fonts)?;
    update_font_table("../../src/mono_font/ascii/mod.rs", Encoding::Ascii, &fonts)?;
    update_font_table(
        "../../src/mono_font/latin1/mod.rs",
        Encoding::Latin1,
        &fonts,
    )?;

    Ok(())
}

fn update_font_table(file: &str, encoding: Encoding, fonts: &[Font]) -> Result<()> {
    let start_tag = match encoding {
        Encoding::Ascii => "//START-FONT-TABLE-ASCII",
        Encoding::Latin1 => "//START-FONT-TABLE-LATIN1",
    };

    let input = fs::read_to_string(file)?;
    let mut output = Vec::new();

    // Copy all lines to the start tag.
    let mut lines = input.lines();
    for line in &mut lines {
        output.push(line.to_string());
        if line.trim() == start_tag {
            break;
        }
    }

    output.push("//! | Type | Screenshot | | Type | Screenshot |".to_string());
    output.push("//! |------|------------|-|------|------------|".to_string());

    // Split table into two columns. The split position is rounded upward to make sure the left
    // column has more entries for an odd number of entries.
    let (left, right) = fonts.split_at((fonts.len() + 1) / 2);
    let mut right = right.iter();

    for left_font in left {
        let line = if let Some(right_font) = right.next() {
            format!(
            "//! | `{left_name}` | ![{left_name}]({left_png_data}) | | `{right_name}` | ![{right_name}]({right_png_data}) |",
            left_name = left_font.constant,
            left_png_data = left_font.data(encoding).png_data(),
            right_name = right_font.constant,
            right_png_data = right_font.data(encoding).png_data(),
        )
        } else {
            // Add empty fields to right column, if the left column contains more entries.
            format!(
                "//! | `{left_name}` | ![{left_name}]({left_png_data}) | | | |",
                left_name = left_font.constant,
                left_png_data = left_font.data(encoding).png_data(),
            )
        };

        output.push(line);
    }

    // Skip old table content and add lines after the end tag to the output.
    let mut take = false;
    for line in lines {
        if line.trim() == "//END-FONT-TABLE" {
            take = true;
        }

        if take {
            output.push(line.to_string());
        }
    }
    output.push(String::new());

    fs::write(file, output.join("\n"))?;

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

struct Font {
    file_stem: OsString,
    constant: String,
    ascii: MonoFontData,
    latin1: MonoFontData,
    fonts_dir: PathBuf,
}

impl Font {
    fn new(file: &Path, fonts_dir: &Path) -> Result<Self> {
        let file_stem = file.file_stem().unwrap().to_owned();

        let constant = format!(
            "FONT_{}",
            file_stem
                .to_string_lossy()
                .to_ascii_uppercase()
                .replace("O", "_ITALIC")
                .replace("B", "_BOLD")
        );

        let bdf_data = fs::read(file)?;
        let bdf = BdfFont::parse(&bdf_data).map_err(|_| anyhow!("couldn't parse BDF file"))?;

        let ascii = MonoFontData::new(&bdf, Encoding::Ascii)?;
        let latin1 = MonoFontData::new(&bdf, Encoding::Latin1)?;

        Ok(Self {
            file_stem,
            constant,
            ascii,
            latin1,
            fonts_dir: fonts_dir.into(),
        })
    }

    fn data(&self, encoding: Encoding) -> &MonoFontData {
        match encoding {
            Encoding::Ascii => &self.ascii,
            Encoding::Latin1 => &self.latin1,
        }
    }

    fn save_files(&self) -> Result<()> {
        self.save_files_for_encoding(Encoding::Ascii)?;
        self.save_files_for_encoding(Encoding::Latin1)
    }

    fn save_files_for_encoding(&self, encoding: Encoding) -> Result<()> {
        let data = self.data(encoding);

        let raw_file = raw_directory(&self.fonts_dir, encoding)?
            .join(&self.file_stem)
            .with_extension("raw");
        data.save_raw(raw_file)?;

        let png_file = png_directory(&self.fonts_dir, encoding)?
            .join(&self.file_stem)
            .with_extension("png");
        data.save_png(png_file)?;

        Ok(())
    }

    fn rust(&self, encoding: Encoding) -> String {
        let data = self.data(encoding);

        let raw_file_path = format!(
            "../../../fonts/{}/raw/{}.raw",
            encoding,
            self.file_stem.to_string_lossy()
        );
        data.rust(&self.constant, &raw_file_path)
    }
}
