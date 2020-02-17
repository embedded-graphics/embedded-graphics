use tinytga::{ImageType, Tga, TgaFooter, TgaHeader};

#[test]
#[ignore]
fn cbw8() {
    let data = include_bytes!("./cbw8.tga");

    let img = Tga::from_slice(data).unwrap();

    println!("{:#?}", img.header);
    println!("{:#?}", img.footer);
    println!("Pixel data len {:#?}", img.pixel_data.len());
    println!("Pixel data {:#?}", img.pixel_data);

    assert_eq!(
        img.header,
        TgaHeader {
            id_len: 26,
            has_color_map: false,
            image_type: ImageType::RleMonochrome,
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
            extension_area_offset: 8238,
            developer_directory_offset: 0
        })
    );

    let pixels = img.into_iter().collect::<Vec<_>>();

    assert_eq!(pixels.len(), 128 * 128);
}
