use crate::{
    common::{tests::U32Color, PixelArrangement, PixelArrangementEnum},
    draw_target::DrawTarget,
    framebuffer::Framebuffer,
    geometry::{Point, Size},
    pixelcolor::{
        raw::{
            order::{DataOrder, DataOrderEnum},
            RawData, RawU16, RawU24, RawU32,
        },
        BinaryColor, Gray2, Gray4, Gray8, Rgb565, Rgb888, StorablePixelColor,
    },
    Pixel,
};

fn draw_points<F>(framebuffer: &mut F, points: &[((i32, i32), F::Color)])
where
    F: Framebuffer,
    F::Color: StorablePixelColor,
    <F as DrawTarget>::Error: core::fmt::Debug,
{
    framebuffer
        .draw_iter(points.iter().copied().map(|(p, c)| {
            let p = Point::from(p);
            let p = match F::PixelArrangement::ARRANGEMENT {
                PixelArrangementEnum::Horizontal => p,
                PixelArrangementEnum::Vertical => p.swap_xy(),
            };

            Pixel(p, c)
        }))
        .unwrap();
}

fn check_bits<F>(framebuffer: &F, expected_data: &mut [u8])
where
    F: Framebuffer,
    F::Color: StorablePixelColor,
{
    // Reverse bits in expected data for LSB0 bit order.
    if F::DataOrder::ORDER == DataOrderEnum::Alternate {
        for byte in expected_data.iter_mut() {
            match <F::Color as StorablePixelColor>::Raw::BITS_PER_PIXEL {
                1 => *byte = byte.reverse_bits(),
                2 => {
                    let pixels = [
                        (*byte >> 6) & 0b11,
                        (*byte >> 4) & 0b11,
                        (*byte >> 2) & 0b11,
                        (*byte >> 0) & 0b11,
                    ];

                    *byte = pixels[3] << 6 | pixels[2] << 4 | pixels[1] << 2 | pixels[0];
                }
                4 => {
                    let pixels = [(*byte >> 4) & 0b1111, (*byte >> 0) & 0b1111];

                    *byte = pixels[1] << 4 | pixels[0];
                }
                _ => unreachable!(),
            }
        }
    }

    assert_eq!(framebuffer.data(), expected_data);
}

pub trait FbTest {
    const HORIZONTAL_SIZE: Size;
    type Color: StorablePixelColor;

    fn test<F>(framebuffer: &mut F)
    where
        F: Framebuffer<Color = Self::Color>,
        <F as DrawTarget>::Error: core::fmt::Debug;
}

pub struct RawU1Test;

impl FbTest for RawU1Test {
    const HORIZONTAL_SIZE: Size = Size::new(9, 5);
    type Color = BinaryColor;

    fn test<F>(framebuffer: &mut F)
    where
        F: Framebuffer<Color = Self::Color>,
        <F as DrawTarget>::Error: core::fmt::Debug,
    {
        use BinaryColor::{Off, On};
        draw_points(
            framebuffer,
            &[
                ((0, 0), On),  //
                ((2, 2), On),  //
                ((1, 4), On),  //
                ((1, 1), On),  //
                ((1, 1), Off), //
                ((-1, 0), On), //
                ((0, -1), On), //
                ((0, 5), On),  //
                ((2, 0), On),  //
                ((8, 4), On),  //
                ((8, 0), On),  //
                ((9, 0), On),  //
            ],
        );

        #[rustfmt::skip]
            check_bits(
                framebuffer,
                &mut [
                    0b10100000, 0b1_0000000,
                    0b00000000, 0b0_0000000,
                    0b00100000, 0b0_0000000,
                    0b00000000, 0b0_0000000,
                    0b01000000, 0b1_0000000,
                ],
            );
    }
}

pub struct RawU2Test;

impl FbTest for RawU2Test {
    const HORIZONTAL_SIZE: Size = Size::new(6, 4);
    type Color = Gray2;

    fn test<F>(framebuffer: &mut F)
    where
        F: Framebuffer<Color = Self::Color>,
        <F as DrawTarget>::Error: core::fmt::Debug,
    {
        let g = Gray2::new;
        draw_points(
            framebuffer,
            &[
                ((0, 0), g(1)),  //
                ((5, 3), g(2)),  //
                ((1, 1), g(3)),  //
                ((1, 2), g(0)),  //
                ((-1, 0), g(3)), //
                ((0, -1), g(3)), //
                ((6, 0), g(3)),  //
                ((0, 4), g(3)),  //
            ],
        );

        check_bits(
            framebuffer,
            &mut [
                0b01000000, 0b00000000, //
                0b00110000, 0b00000000, //
                0b00000000, 0b00000000, //
                0b00000000, 0b00100000, //
            ],
        );
    }
}

pub struct RawU4Test;

impl FbTest for RawU4Test {
    const HORIZONTAL_SIZE: Size = Size::new(3, 2);
    type Color = Gray4;

    fn test<F>(framebuffer: &mut F)
    where
        F: Framebuffer<Color = Self::Color>,
        <F as DrawTarget>::Error: core::fmt::Debug,
    {
        let g = Gray4::new;
        draw_points(
            framebuffer,
            &[
                ((0, 0), g(0x1)),  //
                ((2, 1), g(0xF)),  //
                ((1, 0), g(0xA)),  //
                ((1, 1), g(0xB)),  //
                ((-1, 0), g(0xF)), //
                ((0, -1), g(0xF)), //
                ((3, 0), g(0xF)),  //
                ((0, 2), g(0xF)),  //
            ],
        );

        check_bits(
            framebuffer,
            &mut [
                0x1A, 0x00, //
                0x0B, 0xF0, //
            ],
        );
    }
}

pub struct RawU8Test;

impl FbTest for RawU8Test {
    const HORIZONTAL_SIZE: Size = Size::new(3, 2);
    type Color = Gray8;

    fn test<F>(framebuffer: &mut F)
    where
        F: Framebuffer<Color = Self::Color>,
        <F as DrawTarget>::Error: core::fmt::Debug,
    {
        let g = Gray8::new;
        draw_points(
            framebuffer,
            &[
                ((0, 0), g(0x10)),  //
                ((2, 1), g(0x22)),  //
                ((1, 0), g(0x3F)),  //
                ((1, 1), g(0xF0)),  //
                ((-1, 0), g(0xFF)), //
                ((0, -1), g(0xFF)), //
                ((3, 0), g(0xFF)),  //
                ((0, 2), g(0xFF)),  //
            ],
        );

        assert_eq!(
            framebuffer.data(),
            &[
                0x10, 0x3F, 0x00, //
                0x00, 0xF0, 0x22, //
            ]
        );
    }
}

pub struct RawU16Test;

impl FbTest for RawU16Test {
    const HORIZONTAL_SIZE: Size = Size::new(3, 2);
    type Color = Rgb565;

    fn test<F>(framebuffer: &mut F)
    where
        F: Framebuffer<Color = Self::Color>,
        <F as DrawTarget>::Error: core::fmt::Debug,
    {
        let c = |c| Rgb565::from(RawU16::new(c));
        draw_points(
            framebuffer,
            &[
                ((0, 0), c(0x1000)),  //
                ((2, 1), c(0x0001)),  //
                ((1, 0), c(0x1234)),  //
                ((1, 1), c(0x8765)),  //
                ((-1, 0), c(0xFFFF)), //
                ((0, -1), c(0xFFFF)), //
                ((3, 0), c(0xFFFF)),  //
                ((0, 2), c(0xFFFF)),  //
            ],
        );

        let mut expected_data = [
            0x00, 0x10, 0x34, 0x12, 0x00, 0x00, //
            0x00, 0x00, 0x65, 0x87, 0x01, 0x00, //
        ];

        // Swap bytes for big endian byte order.
        if F::DataOrder::ORDER == DataOrderEnum::Alternate {
            for chunk in expected_data.chunks_mut(2) {
                let value = u16::from_le_bytes(chunk.try_into().unwrap());
                chunk.copy_from_slice(&value.to_be_bytes());
            }
        }

        assert_eq!(framebuffer.data(), &expected_data);
    }
}

pub struct RawU24Test;

impl FbTest for RawU24Test {
    const HORIZONTAL_SIZE: Size = Size::new(3, 2);
    type Color = Rgb888;

    fn test<F>(framebuffer: &mut F)
    where
        F: Framebuffer<Color = Self::Color>,
        <F as DrawTarget>::Error: core::fmt::Debug,
    {
        let c = |c| Rgb888::from(RawU24::new(c));
        draw_points(
            framebuffer,
            &[
                ((0, 0), c(0x100000)),  //
                ((2, 1), c(0x000001)),  //
                ((1, 0), c(0x123456)),  //
                ((1, 1), c(0x876543)),  //
                ((-1, 0), c(0xFFFFFF)), //
                ((0, -1), c(0xFFFFFF)), //
                ((3, 0), c(0xFFFFFF)),  //
                ((0, 2), c(0xFFFFFF)),  //
            ],
        );

        let mut expected_data = [
            0x00, 0x00, 0x10, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x43, 0x65, 0x87, 0x01, 0x00, 0x00, //
        ];

        // Swap bytes for big endian byte order.
        if F::DataOrder::ORDER == DataOrderEnum::Alternate {
            for chunk in expected_data.chunks_mut(3) {
                (chunk[0], chunk[2]) = (chunk[2], chunk[0]);
            }
        }

        assert_eq!(framebuffer.data(), &expected_data);
    }
}

pub struct RawU32Test;

impl FbTest for RawU32Test {
    const HORIZONTAL_SIZE: Size = Size::new(3, 2);
    type Color = U32Color;

    fn test<F>(framebuffer: &mut F)
    where
        F: Framebuffer<Color = Self::Color>,
        <F as DrawTarget>::Error: core::fmt::Debug,
    {
        let c = |c| U32Color::from(RawU32::new(c));
        draw_points(
            framebuffer,
            &[
                ((0, 0), c(0x10000000)),  //
                ((2, 1), c(0x00000001)),  //
                ((1, 0), c(0x12345678)),  //
                ((1, 1), c(0x87654321)),  //
                ((-1, 0), c(0xFFFFFFFF)), //
                ((0, -1), c(0xFFFFFFFF)), //
                ((3, 0), c(0xFFFFFFFF)),  //
                ((0, 2), c(0xFFFFFFFF)),  //
            ],
        );

        let mut expected_data = [
            0x00, 0x00, 0x00, 0x10, 0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x21, 0x43, 0x65, 0x87, 0x01, 0x00, 0x00, 0x00, //
        ];

        // Swap bytes for big endian byte order.
        if F::DataOrder::ORDER == DataOrderEnum::Alternate {
            for chunk in expected_data.chunks_mut(4) {
                let value = u32::from_be_bytes(chunk.try_into().unwrap());
                chunk.copy_from_slice(&value.to_le_bytes());
            }
        }

        assert_eq!(framebuffer.data(), &expected_data);
    }
}
macro_rules! impl_framebuffer_tests {
    () => {
        impl_test!(raw_u1_msb0_horizontal, RawU1Test, <Msb0, Horizontal>);
        impl_test!(raw_u1_lsb0_horizontal, RawU1Test, <Lsb0, Horizontal>);
        impl_test!(raw_u1_msb0_vertical, RawU1Test, <Msb0, Vertical>);
        impl_test!(raw_u1_lsb0_vertical, RawU1Test, <Lsb0, Vertical>);

        impl_test!(raw_u2_msb0_horizontal, RawU2Test, <Msb0, Horizontal>);
        impl_test!(raw_u2_lsb0_horizontal, RawU2Test, <Lsb0, Horizontal>);
        impl_test!(raw_u2_msb0_vertical, RawU2Test, <Msb0, Vertical>);
        impl_test!(raw_u2_lsb0_vertical, RawU2Test, <Lsb0, Vertical>);

        impl_test!(raw_u4_msb0_horizontal, RawU4Test, <Msb0, Horizontal>);
        impl_test!(raw_u4_lsb0_horizontal, RawU4Test, <Lsb0, Horizontal>);
        impl_test!(raw_u4_msb0_vertical, RawU4Test, <Msb0, Vertical>);
        impl_test!(raw_u4_lsb0_vertical, RawU4Test, <Lsb0, Vertical>);

        impl_test!(raw_u8_le_horizontal, RawU8Test, <LittleEndian, Horizontal>);
        impl_test!(raw_u8_be_horizontal, RawU8Test, <BigEndian, Horizontal>);
        impl_test!(raw_u8_le_vertical, RawU8Test, <LittleEndian, Vertical>);
        impl_test!(raw_u8_be_vertical, RawU8Test, <BigEndian, Vertical>);

        impl_test!(raw_u16_le_horizontal, RawU16Test, <LittleEndian, Horizontal>);
        impl_test!(raw_u16_be_horizontal, RawU16Test, <BigEndian, Horizontal>);
        impl_test!(raw_u16_le_vertical, RawU16Test, <LittleEndian, Vertical>);
        impl_test!(raw_u16_be_vertical, RawU16Test, <BigEndian, Vertical>);

        impl_test!(raw_u24_le_horizontal, RawU24Test, <LittleEndian, Horizontal>);
        impl_test!(raw_u24_be_horizontal, RawU24Test, <BigEndian, Horizontal>);
        impl_test!(raw_u24_le_vertical, RawU24Test, <LittleEndian, Vertical>);
        impl_test!(raw_u24_be_vertical, RawU24Test, <BigEndian, Vertical>);

        impl_test!(raw_u32_le_horizontal, RawU32Test, <LittleEndian, Horizontal>);
        impl_test!(raw_u32_be_horizontal, RawU32Test, <BigEndian, Horizontal>);
        impl_test!(raw_u32_le_vertical, RawU32Test, <LittleEndian, Vertical>);
        impl_test!(raw_u32_be_vertical, RawU32Test, <BigEndian, Vertical>);
    };
}
