use bdf_to_mono::{bdf_to_bitmap, Bitmap};
use image::{GrayImage, Luma};
use std::{fs, path::PathBuf, str::FromStr};

fn main() -> std::io::Result<()> {
    fs::create_dir_all("../../data/fonts")?;

    for entry in fs::read_dir("../../assets/fonts/")? {
        let file = entry?;
        assert!(file.file_type()?.is_file());

        println!("Converting {}", file.file_name().to_string_lossy());

        let bdf = fs::read_to_string(&file.path())?;
        let bitmap = bdf_to_bitmap(&bdf).unwrap();

        let mut output_file = PathBuf::from_str("../../data/fonts/").unwrap();
        output_file.push(file.path().file_stem().unwrap().to_str().unwrap());
        output_file.set_extension("raw");

        fs::write(&output_file, &bitmap.data)?;

        output_file.set_extension("png");
        bitmap_to_image(&bitmap).save(output_file).unwrap();
    }

    Ok(())
}

fn bitmap_to_image(bitmap: &Bitmap) -> GrayImage {
    let mut image = GrayImage::new(bitmap.width as u32, bitmap.height as u32);

    for y in 0..bitmap.height {
        for x in 0..bitmap.width {
            if bitmap.pixel(x, y) {
                image.put_pixel(x as u32, y as u32, Luma([255]));
            }
        }
    }

    image
}
