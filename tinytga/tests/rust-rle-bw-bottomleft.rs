use tinytga::Tga;

#[test]
fn rust_rle_bw_bottomleft() {
    let data = include_bytes!("./rust-rle-bw-bottomleft.tga");

    let img = Tga::from_bytes(data);

    println!("{:#?}", img);

    assert!(img.is_ok());
}
