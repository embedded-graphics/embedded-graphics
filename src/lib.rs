#![no_std]
// TODO: Docs
// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

pub mod image;
pub mod fonts;
pub mod drawable;
pub mod primitives;

// TODO: Prelude: drawable::Drawable, primitives, text::Text?

pub trait Drawing {
    // fn draw_image_8bpp(&mut self, image: &image::Image8BPP, x: u32, y: u32);
    // fn draw_image_1bpp(&mut self, image: &image::Image1BPP, x: u32, y: u32);
    // fn draw_text_1bpp(&mut self, text: &str, x: u32, y: u32);
    // fn line(&mut self, start: (u32, u32), end: (u32, u32), value: u8);
    // fn rect(&mut self, tl: (u32, u32), br: (u32, u32), value: u8);
    // fn center_circle(&mut self, center: (u32, u32), radius: u32, value: u8);

    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = drawable::Pixel>;
}
