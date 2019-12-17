use tinytga::{ImageType, Tga, TgaHeader};

const HEADER_DEFAULT: TgaHeader = TgaHeader {
    id_len: 0,
    has_color_map: false,
    image_type: ImageType::Empty,
    color_map_start: 0,
    color_map_len: 0,
    color_map_depth: 0,
    x_origin: 0,
    y_origin: 0,
    width: 9,
    height: 5,
    pixel_depth: 8,
    image_descriptor: 0,
};

#[test]
fn type1_bl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type1_bl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::ColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: 24,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}

#[test]
fn type1_tl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type1_tl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::ColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: 24,
            image_descriptor: 32,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}

#[test]
fn type2_bl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type2_bl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            image_type: ImageType::Truecolor,
            pixel_depth: 24,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}

#[test]
fn type2_tl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type2_tl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            image_type: ImageType::Truecolor,
            pixel_depth: 24,
            image_descriptor: 32,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}

#[test]
fn type3_bl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type3_bl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            image_type: ImageType::Monochrome,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}

#[test]
fn type3_tl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type3_tl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            image_type: ImageType::Monochrome,
            image_descriptor: 32,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}

#[test]
fn type9_bl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type9_bl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::RleColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: 24,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}

#[test]
fn type9_tl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type9_tl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            has_color_map: true,
            image_type: ImageType::RleColorMapped,
            color_map_start: 0,
            color_map_len: 8,
            color_map_depth: 24,
            image_descriptor: 32,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}

#[test]
fn type10_bl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type10_bl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            image_type: ImageType::RleTruecolor,
            pixel_depth: 24,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}

#[test]
fn type10_tl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type10_tl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            image_type: ImageType::RleTruecolor,
            pixel_depth: 24,
            image_descriptor: 32,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}

#[test]
fn type11_bl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type11_bl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            image_type: ImageType::RleMonochrome,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}

#[test]
fn type11_tl() {
    let tga = Tga::from_slice(include_bytes!("../tests/type11_tl.tga")).unwrap();

    assert_eq!(
        tga.header,
        TgaHeader {
            image_type: ImageType::RleMonochrome,
            image_descriptor: 32,
            ..HEADER_DEFAULT
        }
    );
    assert!(tga.footer.is_none());
}
