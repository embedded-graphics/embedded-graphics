use anyhow::{anyhow, Result};
use bdf_parser::BdfFont;
use bdf_to_mono::{bdf_to_bitmap, Bitmap, Encoding};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use png_target::PngTarget;
use std::{ffi::OsStr, fs, path::Path};

fn main() -> Result<()> {
    let fonts_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fonts");

    let mut ascii_rs = include_str!("../assets/header.tmpl").to_string();
    let mut latin1_rs = ascii_rs.clone();

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

        let bdf = fs::read(file)?;
        let font = BdfFont::parse(&bdf).map_err(|_| anyhow!("couldn't parse BDF file"))?;

        let ascii_out = Output::new(&font_name, &font, Encoding::Ascii)?;
        let latin1_out = Output::new(&font_name, &font, Encoding::Latin1)?;

        ascii_out.write_raw(&fonts_dir)?;
        latin1_out.write_raw(&fonts_dir)?;

        ascii_out.write_png(&fonts_dir)?;
        latin1_out.write_png(&fonts_dir)?;

        ascii_rs.push_str(&ascii_out.rust_struct());
        latin1_rs.push_str(&latin1_out.rust_struct());
    }

    let mono_font = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/mono_font");
    fs::write(mono_font.join("ascii/generated.rs"), &ascii_rs)?;
    fs::write(mono_font.join("latin1/generated.rs"), &latin1_rs)?;

    Ok(())
}

struct Output {
    name: String,
    encoding: Encoding,
    bitmap: Bitmap,
    png: PngTarget<BinaryColor>,
}

impl Output {
    fn new(name: &str, font: &BdfFont, encoding: Encoding) -> Result<Self> {
        let bitmap = bdf_to_bitmap(font, encoding)?;
        let png = bitmap_to_png(&bitmap)?;

        Ok(Self {
            name: name.to_string(),
            encoding,
            bitmap,
            png,
        })
    }

    fn write_raw(&self, base_path: &Path) -> Result<()> {
        let raw_directory = base_path.join(&self.encoding.to_string()).join("raw");
        fs::create_dir_all(&raw_directory)?;

        let raw_file = raw_directory.join(&self.name).with_extension("raw");
        fs::write(&raw_file, &self.bitmap.data)?;

        Ok(())
    }

    fn write_png(&self, base_path: &Path) -> Result<()> {
        let png_directory = base_path.join(&self.encoding.to_string()).join("png");
        fs::create_dir_all(&png_directory)?;

        let png_file = png_directory.join(&self.name).with_extension("png");
        self.png.save(&png_file)?;

        Ok(())
    }

    fn rust_struct(&self) -> String {
        let output = include_str!("../assets/font.tmpl");
        let output = output.replace(
            "$NAME$",
            &format!("FONT_{}", self.name.to_ascii_uppercase()),
        );
        let output = output.replace(
            "$RAW_FILE$",
            &format!("../../../fonts/{}/raw/{}.raw", self.encoding, self.name),
        );
        let output = output.replace("$IMAGE_WIDTH$", &format!("{}", self.bitmap.width));
        // TODO: read from file
        let output = output.replace("$CHAR_WIDTH$", &(self.bitmap.glyph_width).to_string());
        let output = output.replace("$CHAR_HEIGHT$", &(self.bitmap.glyph_height).to_string());
        let output = output.replace("$BASELINE$", &(self.bitmap.baseline).to_string());
        let output = output.replace(
            "$CHARACTER_SPACING$",
            &self.bitmap.character_spacing.to_string(),
        );
        let output = output.replace("$PNG_DATA$", &self.png.to_base64());
        let output = output.replace("$GLYPH_INDICES$", encoding_to_glyph_indices(self.encoding));

        output
    }
}

fn bitmap_to_png(bitmap: &Bitmap) -> Result<PngTarget<BinaryColor>> {
    let mut image = PngTarget::new(Size::new(bitmap.width as u32, bitmap.height as u32), 1);

    image
        .bounding_box()
        .points()
        .filter(|p| bitmap.pixel(p.x as usize, p.y as usize))
        .map(|p| Pixel(p, BinaryColor::On))
        .draw(&mut image)
        .unwrap();

    Ok(image)
}

fn encoding_to_glyph_indices(encoding: Encoding) -> &'static str {
    match encoding {
        Encoding::Ascii => "ASCII_GLYPH_INDICES",
        Encoding::Latin1 => "LATIN1_GLYPH_INDICES",
    }
}
