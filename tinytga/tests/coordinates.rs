use embedded_graphics::prelude::*;
use tinytga::{Bpp, ImageOrigin, ImageType, RawTga, TgaHeader};

#[test]
fn coordinates() {
    let data = include_bytes!("./chessboard_4px_raw.tga");

    let img = RawTga::from_slice(data).unwrap();

    println!("{:#?}", img.header());
    println!("Raw image data len {:#?}", img.image_data().len());
    println!("Raw image data {:#?}", img.image_data());

    assert_eq!(
        img.header(),
        TgaHeader {
            id_len: 0,
            has_color_map: false,
            image_type: ImageType::Truecolor,
            color_map_start: 0,
            color_map_len: 0,
            color_map_depth: None,
            x_origin: 0,
            y_origin: 4,
            width: 4,
            height: 4,
            pixel_depth: Bpp::Bits24,
            image_origin: ImageOrigin::TopLeft,
            alpha_channel_depth: 0,
        }
    );

    assert_eq!(img.extension_area(), None);
    assert_eq!(img.developer_directory(), None);

    let coords: Vec<_> = img.pixels().map(|p| p.position).collect();

    assert_eq!(coords.len(), 4 * 4);
    assert_eq!(
        coords,
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(3, 1),
            Point::new(0, 2),
            Point::new(1, 2),
            Point::new(2, 2),
            Point::new(3, 2),
            Point::new(0, 3),
            Point::new(1, 3),
            Point::new(2, 3),
            Point::new(3, 3),
        ]
    );
}
