use tinytga::{ImageType, Tga, TgaFooter, TgaHeader};

#[test]
fn coordinates() {
    let data = include_bytes!("./chessboard_4px_raw.tga");

    let img = Tga::from_slice(data).unwrap();

    println!("{:#?}", img.header);
    println!("{:#?}", img.footer);
    println!("Pixel data len {:#?}", img.pixel_data.len());
    println!("Pixel data {:#?}", img.pixel_data);

    assert_eq!(
        img.header,
        TgaHeader {
            id_len: 0,
            has_color_map: false,
            image_type: ImageType::Truecolor,
            color_map_start: 0,
            color_map_len: 0,
            color_map_depth: 0,
            x_origin: 0,
            y_origin: 4,
            width: 4,
            height: 4,
            pixel_depth: 24,
            image_descriptor: 32
        }
    );

    assert_eq!(
        img.footer,
        Some(TgaFooter {
            extension_area_offset: 0,
            developer_directory_offset: 0
        })
    );

    let coords = img
        .into_iter()
        .map(|p| (p.x, p.y))
        .collect::<Vec<(u32, u32)>>();

    assert_eq!(coords.len(), 4 * 4);
    assert_eq!(
        coords,
        vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 3),
        ]
    );
}
