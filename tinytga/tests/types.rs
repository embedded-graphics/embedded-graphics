use tinytga::{Bpp, ImageOrigin, ImageType, RawTga, TgaHeader};

const HEADER_DEFAULT: TgaHeader = TgaHeader {
    id_len: 0,
    has_color_map: false,
    image_type: ImageType::Empty,
    color_map_start: 0,
    color_map_len: 0,
    color_map_depth: None,
    x_origin: 0,
    y_origin: 0,
    width: 9,
    height: 5,
    pixel_depth: Bpp::Bits8,
    image_origin: ImageOrigin::BottomLeft,
    alpha_channel_depth: 0,
};

#[test]
fn type1_16bpp_bl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type1_16bpp_bl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::ColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: Some(Bpp::Bits16),
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits16);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}

#[test]
fn type1_24bpp_bl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type1_24bpp_bl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::ColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: Some(Bpp::Bits24),
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits24);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}

#[test]
fn type1_16bpp_tl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type1_16bpp_tl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::ColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: Some(Bpp::Bits16),
            image_origin: ImageOrigin::TopLeft,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits16);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}

#[test]
fn type1_24bpp_tl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type1_24bpp_tl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::ColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: Some(Bpp::Bits24),
            image_origin: ImageOrigin::TopLeft,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits24);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}

#[test]
fn type2_16bpp_bl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type2_16bpp_bl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::Truecolor,
            pixel_depth: Bpp::Bits16,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits16);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits16);
}

#[test]
fn type2_24bpp_bl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type2_24bpp_bl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::Truecolor,
            pixel_depth: Bpp::Bits24,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits24);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits24);
}

#[test]
fn type2_16bpp_tl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type2_16bpp_tl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::Truecolor,
            pixel_depth: Bpp::Bits16,
            image_origin: ImageOrigin::TopLeft,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits16);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits16);
}

#[test]
fn type2_24bpp_tl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type2_24bpp_tl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::Truecolor,
            pixel_depth: Bpp::Bits24,
            image_origin: ImageOrigin::TopLeft,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits24);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits24);
}

#[test]
fn type3_bl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type3_bl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::Monochrome,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits8);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}

#[test]
fn type3_tl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type3_tl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::Monochrome,
            image_origin: ImageOrigin::TopLeft,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits8);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}

#[test]
fn type9_16bpp() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type9_16bpp_bl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::RleColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: Some(Bpp::Bits16),
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits16);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}

#[test]
fn type9_24bpp_bl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type9_24bpp_bl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::RleColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: Some(Bpp::Bits24),
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits24);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}

#[test]
fn type9_16bpp_tl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type9_16bpp_tl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::RleColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: Some(Bpp::Bits16),
            image_origin: ImageOrigin::TopLeft,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits16);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}

#[test]
fn type9_24bpp_tl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type9_24bpp_tl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::RleColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: Some(Bpp::Bits24),
            image_origin: ImageOrigin::TopLeft,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits24);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}

#[test]
fn type10_16bpp_bl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type10_16bpp_bl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::RleTruecolor,
            pixel_depth: Bpp::Bits16,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits16);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits16);
}

#[test]
fn type10_24bpp_bl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type10_24bpp_bl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::RleTruecolor,
            pixel_depth: Bpp::Bits24,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits24);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits24);
}

#[test]
fn type10_16bpp_tl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type10_16bpp_tl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::RleTruecolor,
            pixel_depth: Bpp::Bits16,
            image_origin: ImageOrigin::TopLeft,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits16);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits16);
}

#[test]
fn type10_24bpp_tl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type10_24bpp_tl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::RleTruecolor,
            pixel_depth: Bpp::Bits24,
            image_origin: ImageOrigin::TopLeft,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits24);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits24);
}

#[test]
fn type11_bl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type11_bl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::RleMonochrome,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits8);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}

#[test]
fn type11_tl() {
    let tga = RawTga::from_slice(include_bytes!("../tests/type11_tl.tga")).unwrap();

    assert_eq!(
        tga.header(),
        TgaHeader {
            image_type: ImageType::RleMonochrome,
            image_origin: ImageOrigin::TopLeft,
            ..HEADER_DEFAULT
        }
    );
    assert_eq!(tga.developer_directory(), None);
    assert_eq!(tga.extension_area(), None);

    assert_eq!(tga.color_bpp(), Bpp::Bits8);
    assert_eq!(tga.image_data_bpp(), Bpp::Bits8);
}
