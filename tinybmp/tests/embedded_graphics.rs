use embedded_graphics::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    image::Image,
    mock_display::MockDisplay,
    pixelcolor::{BinaryColor, Gray8, GrayColor, Rgb555, Rgb565, Rgb888, RgbColor},
};
use tinybmp::Bmp;

#[test]
fn negative_top_left() {
    let image = Bmp::from_slice(include_bytes!("./chessboard-4px-color-16bit.bmp")).unwrap();
    let image: Image<_, Rgb565> = Image::new(&image).translate(Point::new(-1, -1));

    assert_eq!(image.top_left(), Point::new(-1, -1));
    assert_eq!(image.bottom_right(), Point::new(3, 3));
    assert_eq!(image.size(), Size::new(4, 4));
}

#[test]
fn dimensions() {
    let image = Bmp::from_slice(include_bytes!("./chessboard-4px-color-16bit.bmp")).unwrap();
    let image: Image<_, Rgb565> = Image::new(&image).translate(Point::new(100, 200));

    assert_eq!(image.top_left(), Point::new(100, 200));
    assert_eq!(image.bottom_right(), Point::new(104, 204));
    assert_eq!(image.size(), Size::new(4, 4));
}

#[test]
#[ignore]
fn it_can_have_negative_offsets() {
    let image = Bmp::from_slice(include_bytes!("./chessboard-4px-color-16bit.bmp")).unwrap();
    let image: Image<_, Rgb565> = Image::new(&image).translate(Point::new(-1, -1));

    assert_eq!(image.into_iter().count(), 9);

    let it = image.into_iter();

    let expected: [Pixel<Rgb565>; 9] = [
        Pixel(Point::new(0, 0), Rgb565::RED),
        Pixel(Point::new(1, 0), Rgb565::BLACK),
        Pixel(Point::new(2, 0), Rgb565::GREEN),
        //
        Pixel(Point::new(0, 1), Rgb565::BLACK),
        Pixel(Point::new(1, 1), Rgb565::BLUE),
        Pixel(Point::new(2, 1), Rgb565::BLACK),
        //
        Pixel(Point::new(0, 2), Rgb565::WHITE),
        Pixel(Point::new(1, 2), Rgb565::BLACK),
        Pixel(Point::new(2, 2), Rgb565::WHITE),
    ];

    for (idx, pixel) in it.enumerate() {
        assert_eq!(pixel, expected[idx]);
    }
}

fn create_color_pattern<C>() -> [[C; 4]; 2]
where
    C: RgbColor,
{
    [
        [C::BLACK, C::RED, C::GREEN, C::YELLOW],
        [C::BLUE, C::MAGENTA, C::CYAN, C::WHITE],
    ]
}

macro_rules! test_pattern {
    ($color_type:ident, $image_data:expr) => {
        let image = Bmp::from_slice($image_data).unwrap();
        let image: Image<_, $color_type> = Image::new(&image);

        let pattern = create_color_pattern();

        assert_eq!(image.size(), Size::new(4, 2));

        let mut iter = image.into_iter();
        for (y, row) in pattern.iter().enumerate() {
            for (x, &expected_color) in row.iter().enumerate() {
                let pos = Point::new(x as i32, y as i32);
                let pixel = iter.next().unwrap();

                assert_eq!(pixel, Pixel(pos, expected_color));
            }
        }

        assert!(iter.next().is_none());
    };
}

#[test]
fn colors_rgb555() {
    test_pattern!(Rgb555, include_bytes!("./colors_rgb555.bmp"));
}

#[test]
fn colors_rgb565() {
    test_pattern!(Rgb565, include_bytes!("./colors_rgb565.bmp"));
}

#[test]
fn colors_rgb888_24bit() {
    test_pattern!(Rgb888, include_bytes!("./colors_rgb888_24bit.bmp"));
}

#[test]
#[ignore]
fn colors_rgb888_32bit() {
    test_pattern!(Rgb888, include_bytes!("./colors_rgb888_32bit.bmp"));
}

#[test]
fn colors_grey8() {
    let image = Bmp::from_slice(include_bytes!("./colors_grey8.bmp")).unwrap();
    let image: Image<_, Gray8> = Image::new(&image);

    assert_eq!(image.size(), Size::new(3, 1));

    let mut iter = image.into_iter();

    let p = iter.next().unwrap();
    assert_eq!(p.0, Point::new(0, 0));
    assert_eq!(p.1, Gray8::BLACK);

    let p = iter.next().unwrap();
    assert_eq!(p.0, Point::new(1, 0));
    assert_eq!(p.1, Gray8::new(128));

    let p = iter.next().unwrap();
    assert_eq!(p.0, Point::new(2, 0));
    assert_eq!(p.1, Gray8::WHITE);

    assert!(iter.next().is_none());
}

/// Test for issue #136
#[test]
fn issue_136_row_size_is_multiple_of_4_bytes() {
    let image = Bmp::from_slice(include_bytes!("./issue_136.bmp")).unwrap();
    let image: Image<_, Rgb565> = Image::new(&image);

    let mut display = MockDisplay::new();
    image
        .into_iter()
        .map(|Pixel(p, c)| {
            Pixel(
                p,
                match c {
                    Rgb565::BLACK => BinaryColor::Off,
                    Rgb565::WHITE => BinaryColor::On,
                    _ => panic!("Unexpected color in image"),
                },
            )
        })
        .draw(&mut display)
        .unwrap();

    assert_eq!(
        display,
        MockDisplay::from_pattern(&[
            "####.####",
            "#....#...",
            "####.#.##",
            "#....#..#",
            "####.####",
        ])
    );
}
