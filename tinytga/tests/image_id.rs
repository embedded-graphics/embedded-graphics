use tinytga::RawTga;

#[test]
fn has_image_id() {
    // image_id.tga contains the image ID: "e-g"
    let data = include_bytes!("./image_id.tga");

    let img = RawTga::from_slice(data).unwrap();

    assert_eq!(img.image_id(), Some("e-g".as_bytes()));
}

#[test]
fn no_image_id() {
    // type1_24bpp_bl.tga does not contain an image ID
    let data = include_bytes!("./type1_24bpp_bl.tga");

    let img = RawTga::from_slice(data).unwrap();

    assert_eq!(img.image_id(), None);
}
