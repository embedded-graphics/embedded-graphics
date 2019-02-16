use nom::*;

/// Image type
#[derive(Debug, Copy, Clone)]
pub enum ImageType {
    /// Image contains no pixel data
    Empty = 0,

    /// Color mapped image
    ColorMapped = 1,

    /// Truecolor image
    Truecolor = 2,

    /// Monochrome (greyscale) image
    Monochrome = 3,

    /// Run length encoded color mapped image
    RleColorMapped = 9,

    /// Run length encoded truecolor image
    RleTruecolor = 10,

    /// Run length encoded monochrome (greyscale) image
    RleMonochrome = 11,
}

// /// Color map type
// #[derive(Debug, Copy, Clone)]
// pub enum ColorMapType {
//     /// Each pixel is represented by an index into a color pallette
//     Pseudocolor,

//     /// Similar to pseudocolor, but each RGB component has its own index into a color pallette
//     Directcolor,

//     /// Store pixel information in the image, no pallette is used
//     Truecolor,
// }

/// TGA header structure, referenced from <https://www.fileformat.info/format/tga/egff.htm>
#[derive(Debug, Copy, Clone)]
pub struct TgaHeader {
    /// Image ID field length
    pub id_len: u8,

    /// Whether a color map is included in the image data
    pub has_color_map: bool,

    /// Image type
    pub image_type: ImageType,

    /// Color map origin
    pub color_map_start: u16,

    /// Length of color map
    pub color_map_len: u16,

    /// Number of bits in each color pallette entry, typically 15, 16, 24, or 32 bits
    pub color_map_depth: u8,

    /// Image origin (X)
    pub x_origin: u16,

    /// Image origin (Y)
    pub y_origin: u16,

    /// Image width in pixels
    pub width: u16,

    /// Image heigh in pixels
    pub height: u16,

    /// Pixel bit depth (8, 16, 24, 32 bits)
    pub pixel_depth: u8,

    /// Image descriptor (unused)
    ///
    /// Bits 0:3: Number of bits per pixel designated to alpha channel
    /// Bits 4:5: Image origin:
    ///
    /// * `00` = bottom left
    /// * `01` = bottom right
    /// * `10` = top left
    /// * `11` = top right
    pub image_descriptor: u8,
}

named!(has_color_map<&[u8], bool>,
    map_res!(
        le_u8,
        |b| match b {
            0 => Ok(false),
            1 => Ok(true),
            _other => Err(())
        }
    )
);

named!(image_type<&[u8], ImageType>,
    map_res!(
        le_u8,
        |b| match b {
            0 => Ok(ImageType::Empty),
            1 => Ok(ImageType::ColorMapped),
            2 => Ok(ImageType::Truecolor),
            3 => Ok(ImageType::Monochrome),
            9 => Ok(ImageType::RleColorMapped),
            10 => Ok(ImageType::RleTruecolor),
            11 => Ok(ImageType::RleMonochrome),
            _other => Err(())
        }
    )
);

named!(pub header<&[u8], TgaHeader>,
    do_parse!(
        id_len: le_u8 >>
        has_color_map: has_color_map >>
        image_type: image_type >>
        color_map_start: le_u16 >>
        color_map_len: le_u16 >>
        color_map_depth: le_u8 >>
        x_origin: le_u16 >>
        y_origin: le_u16 >>
        width: le_u16 >>
        height: le_u16 >>
        pixel_depth: le_u8 >>
        image_descriptor: le_u8 >>
        ({
            TgaHeader {
                id_len,
                has_color_map,
                image_type,
                color_map_start,
                color_map_len,
                color_map_depth,
                x_origin,
                y_origin,
                width,
                height,
                pixel_depth,
                image_descriptor,
            }
        })
    )
);
