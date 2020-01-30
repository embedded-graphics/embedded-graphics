use tinytga::{ImageType, Pixel, Tga, TgaFooter, TgaHeader};

#[test]
#[ignore]
fn ubw8() {
    let data = include_bytes!("./ubw8.tga");

    let img = Tga::from_slice(data).unwrap();

    println!("{:#?}", img.header);
    println!("{:#?}", img.footer);
    println!("Pixel data len {:#?}", img.pixel_data.len());

    assert_eq!(
        img.header,
        TgaHeader {
            id_len: 26,
            has_color_map: false,
            image_type: ImageType::Monochrome,
            color_map_start: 0,
            color_map_len: 0,
            color_map_depth: 0,
            x_origin: 0,
            y_origin: 0,
            width: 128,
            height: 128,
            pixel_depth: 8,
            image_descriptor: 0
        }
    );

    assert_eq!(
        img.footer,
        Some(TgaFooter {
            extension_area_offset: 20526,
            developer_directory_offset: 0
        })
    );

    let pixels = img.into_iter().collect::<Vec<Pixel>>();

    assert_eq!(pixels.len(), 128 * 128);
}
