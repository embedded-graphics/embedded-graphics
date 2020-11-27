//! Named colors as defined in the CSS specification.

use crate::pixelcolor::{Bgr555, Bgr565, Bgr888, PixelColor, Rgb555, Rgb565, Rgb888, RgbColor};

macro_rules! color_doc {
    (@internal $text:expr, $($rest:tt)*) => {
        #[doc = $text]
        $($rest)*
    };

    ($name:expr, ($r: expr, $g: expr, $b:expr), $($rest:tt)*) => {
        color_doc!(@internal concat!(
            "<span style=\"",
                "background:rgb(", $r, ",", $g, ",", $b, ");",
                "border-radius:3px;",
                "display:inline-block;",
                "width:0.9em;",
                "height:0.9em",
            "\"></span> ",
            $name
        ), $($rest)*);
    };
}

macro_rules! web_colors_trait {
    ([$(($ident:ident, $name:expr, ($r:expr, $g:expr, $b:expr)),)*]) => {
        /// Named colors as defined in the CSS specification.
        ///
        /// This list includes the [basic color keywords] as well as all colors in the
        /// [extended color keywords] list.
        ///
        /// Note that the `grEy` variants of some colors as defined in the spec are absent from this list.
        /// The `grAy` variants of these colors should be used instead.
        ///
        /// [basic color keywords]: https://drafts.csswg.org/css-color-3/#html4
        /// [extended color keywords]: https://drafts.csswg.org/css-color-3/#svg-color
        #[rustfmt::skip]
        pub trait WebColors: PixelColor + RgbColor {
            $( color_doc! {
                $name, ($r, $g, $b),
                const $ident: Self;
            } )*
        }
    };
}

macro_rules! impl_web_colors {
    ($type:ty, [$(($ident:ident, $name:expr, ($r:expr, $g:expr, $b:expr)),)*]) => {
        #[doc = "Named web colors."]
        #[allow(unused)]
        impl WebColors for $type {
            $( color_doc! {
                $name, ($r, $g, $b),
                const $ident: Self = Self::with_rgb888($r, $g, $b);
            } )*
        }
    };
}

macro_rules! web_colors {
    (($($type:ty),*), $colors:tt) => {
        web_colors_trait!($colors);
        $( impl_web_colors!($type, $colors); )*
    };
}

#[rustfmt::skip]
web_colors!(
    (Rgb555, Rgb565, Rgb888, Bgr555, Bgr565, Bgr888),
    [
        (CSS_ALICE_BLUE, "Alice Blue", (240, 248, 255)),
        (CSS_ANTIQUE_WHITE, "Antique White", (250, 235, 215)),
        (CSS_AQUA, "Aqua", (0, 255, 255)),
        (CSS_AQUAMARINE, "Aquamarine", (127, 255, 212)),
        (CSS_AZURE, "Azure", (240, 255, 255)),
        (CSS_BEIGE, "Beige", (245, 245, 220)),
        (CSS_BISQUE, "Bisque", (255, 228, 196)),
        (CSS_BLACK, "Black", (0, 0, 0)),
        (CSS_BLANCHED_ALMOND, "Blanched Almond", (255, 235, 205)),
        (CSS_BLUE, "Blue", (0, 0, 255)),
        (CSS_BLUE_VIOLET, "Blue Violet", (138, 43, 226)),
        (CSS_BROWN, "Brown", (165, 42, 42)),
        (CSS_BURLY_WOOD, "Burly Wood", (222, 184, 135)),
        (CSS_CADET_BLUE, "Cadet Blue", (95, 158, 160)),
        (CSS_CHARTREUSE, "Chartreuse", (127, 255, 0)),
        (CSS_CHOCOLATE, "Chocolate", (210, 105, 30)),
        (CSS_CORAL, "Coral", (255, 127, 80)),
        (CSS_CORNFLOWER_BLUE, "Cornflower Blue", (100, 149, 237)),
        (CSS_CORNSILK, "Cornsilk", (255, 248, 220)),
        (CSS_CRIMSON, "Crimson", (220, 20, 60)),
        (CSS_CYAN, "Cyan", (0, 255, 255)),
        (CSS_DARK_BLUE, "Dark Blue", (0, 0, 139)),
        (CSS_DARK_CYAN, "Dark Cyan", (0, 139, 139)),
        (CSS_DARK_GOLDENROD, "Dark Goldenrod", (184, 134, 11)),
        (CSS_DARK_GRAY, "Dark Gray", (169, 169, 169)),
        (CSS_DARK_GREEN, "Dark Green", (0, 100, 0)),
        (CSS_DARK_KHAKI, "Dark Khaki", (189, 183, 107)),
        (CSS_DARK_MAGENTA, "Dark Magenta", (139, 0, 139)),
        (CSS_DARK_OLIVE_GREEN, "Dark Olive Green", (85, 107, 47)),
        (CSS_DARK_ORANGE, "Dark Orange", (255, 140, 0)),
        (CSS_DARK_ORCHID, "Dark Orchid", (153, 50, 204)),
        (CSS_DARK_RED, "Dark Red", (139, 0, 0)),
        (CSS_DARK_SALMON, "Dark Salmon", (233, 150, 122)),
        (CSS_DARK_SEA_GREEN, "Dark Sea Green", (143, 188, 143)),
        (CSS_DARK_SLATE_BLUE, "Dark Slate Blue", (72, 61, 139)),
        (CSS_DARK_SLATE_GRAY, "Dark Slate Gray", (47, 79, 79)),
        (CSS_DARK_TURQUOISE, "Dark Turquoise", (0, 206, 209)),
        (CSS_DARK_VIOLET, "Dark Violet", (148, 0, 211)),
        (CSS_DEEP_PINK, "Deep Pink", (255, 20, 147)),
        (CSS_DEEP_SKY_BLUE, "Deep Sky Blue", (0, 191, 255)),
        (CSS_DIM_GRAY, "Dim Gray", (105, 105, 105)),
        (CSS_DODGER_BLUE, "Dodger Blue", (30, 144, 255)),
        (CSS_FIRE_BRICK, "Fire Brick", (178, 34, 34)),
        (CSS_FLORAL_WHITE, "Floral White", (255, 250, 240)),
        (CSS_FOREST_GREEN, "Forest Green", (34, 139, 34)),
        (CSS_FUCHSIA, "Fuchsia", (255, 0, 255)),
        (CSS_GAINSBORO, "Gainsboro", (220, 220, 220)),
        (CSS_GHOST_WHITE, "Ghost White", (248, 248, 255)),
        (CSS_GOLD, "Gold", (255, 215, 0)),
        (CSS_GOLDENROD, "Goldenrod", (218, 165, 32)),
        (CSS_GRAY, "Gray", (128, 128, 128)),
        (CSS_GREEN, "Green", (0, 128, 0)),
        (CSS_GREEN_YELLOW, "Green Yellow", (173, 255, 47)),
        (CSS_HONEYDEW, "Honeydew", (240, 255, 240)),
        (CSS_HOT_PINK, "Hot Pink", (255, 105, 180)),
        (CSS_INDIAN_RED, "Indian Red", (205, 92, 92)),
        (CSS_INDIGO, "Indigo", (75, 0, 130)),
        (CSS_IVORY, "Ivory", (255, 255, 240)),
        (CSS_KHAKI, "Khaki", (240, 230, 140)),
        (CSS_LAVENDER, "Lavender", (230, 230, 250)),
        (CSS_LAVENDER_BLUSH, "Lavender Blush", (255, 240, 245)),
        (CSS_LAWN_GREEN, "Lawn Green", (124, 252, 0)),
        (CSS_LEMON_CHIFFON, "Lemon Chiffon", (255, 250, 205)),
        (CSS_LIGHT_BLUE, "Light Blue", (173, 216, 230)),
        (CSS_LIGHT_CORAL, "Light Coral", (240, 128, 128)),
        (CSS_LIGHT_CYAN, "Light Cyan", (224, 255, 255)),
        (CSS_LIGHT_GOLDENROD_YELLOW, "Light Goldenrod Yellow", (250, 250, 210)),
        (CSS_LIGHT_GRAY, "Light Gray", (211, 211, 211)),
        (CSS_LIGHT_GREEN, "Light Green", (144, 238, 144)),
        (CSS_LIGHT_PINK, "Light Pink", (255, 182, 193)),
        (CSS_LIGHT_SALMON, "Light Salmon", (255, 160, 122)),
        (CSS_LIGHT_SEA_GREEN, "Light Sea Green", (32, 178, 170)),
        (CSS_LIGHT_SKY_BLUE, "Light Sky Blue", (135, 206, 250)),
        (CSS_LIGHT_SLATE_GRAY, "Light Slate Gray", (119, 136, 153)),
        (CSS_LIGHT_STEEL_BLUE, "Light Steel Blue", (176, 196, 222)),
        (CSS_LIGHT_YELLOW, "Light Yellow", (255, 255, 224)),
        (CSS_LIME, "Lime", (0, 255, 0)),
        (CSS_LIME_GREEN, "Lime Green", (50, 205, 50)),
        (CSS_LINEN, "Linen", (250, 240, 230)),
        (CSS_MAGENTA, "Magenta", (255, 0, 255)),
        (CSS_MAROON, "Maroon", (128, 0, 0)),
        (CSS_MEDIUM_AQUAMARINE, "Medium Aquamarine", (102, 205, 170)),
        (CSS_MEDIUM_BLUE, "Medium Blue", (0, 0, 205)),
        (CSS_MEDIUM_ORCHID, "Medium Orchid", (186, 85, 211)),
        (CSS_MEDIUM_PURPLE, "Medium Purple", (147, 112, 219)),
        (CSS_MEDIUM_SEA_GREEN, "Medium Sea Green", (60, 179, 113)),
        (CSS_MEDIUM_SLATE_BLUE, "Medium Slate Blue", (123, 104, 238)),
        (CSS_MEDIUM_SPRING_GREEN, "Medium Spring Green", (0, 250, 154)),
        (CSS_MEDIUM_TURQUOISE, "Medium Turquoise", (72, 209, 204)),
        (CSS_MEDIUM_VIOLET_RED, "Medium Violet Red", (199, 21, 133)),
        (CSS_MIDNIGHT_BLUE, "Midnight Blue", (25, 25, 112)),
        (CSS_MINT_CREAM, "Mint Cream", (245, 255, 250)),
        (CSS_MISTY_ROSE, "Misty Rose", (255, 228, 225)),
        (CSS_MOCCASIN, "Moccasin", (255, 228, 181)),
        (CSS_NAVAJO_WHITE, "Navajo White", (255, 222, 173)),
        (CSS_NAVY, "Navy", (0, 0, 128)),
        (CSS_OLD_LACE, "Old Lace", (253, 245, 230)),
        (CSS_OLIVE, "Olive", (128, 128, 0)),
        (CSS_OLIVE_DRAB, "Olive Drab", (107, 142, 35)),
        (CSS_ORANGE, "Orange", (255, 165, 0)),
        (CSS_ORANGE_RED, "Orange Red", (255, 69, 0)),
        (CSS_ORCHID, "Orchid", (218, 112, 214)),
        (CSS_PALE_GOLDENROD, "Pale Goldenrod", (238, 232, 170)),
        (CSS_PALE_GREEN, "Pale Green", (152, 251, 152)),
        (CSS_PALE_TURQUOISE, "Pale Turquoise", (175, 238, 238)),
        (CSS_PALE_VIOLET_RED, "Pale Violet Red", (219, 112, 147)),
        (CSS_PAPAYA_WHIP, "Papaya Whip", (255, 239, 213)),
        (CSS_PEACH_PUFF, "Peach Puff", (255, 218, 185)),
        (CSS_PERU, "Peru", (205, 133, 63)),
        (CSS_PINK, "Pink", (255, 192, 203)),
        (CSS_PLUM, "Plum", (221, 160, 221)),
        (CSS_POWDER_BLUE, "Powder Blue", (176, 224, 230)),
        (CSS_PURPLE, "Purple", (128, 0, 128)),
        (CSS_REBECCAPURPLE, "Rebeccapurple", (102, 51, 153)),
        (CSS_RED, "Red", (255, 0, 0)),
        (CSS_ROSY_BROWN, "Rosy Brown", (188, 143, 143)),
        (CSS_ROYAL_BLUE, "Royal Blue", (65, 105, 225)),
        (CSS_SADDLE_BROWN, "Saddle Brown", (139, 69, 19)),
        (CSS_SALMON, "Salmon", (250, 128, 114)),
        (CSS_SANDY_BROWN, "Sandy Brown", (244, 164, 96)),
        (CSS_SEA_GREEN, "Sea Green", (46, 139, 87)),
        (CSS_SEASHELL, "Seashell", (255, 245, 238)),
        (CSS_SIENNA, "Sienna", (160, 82, 45)),
        (CSS_SILVER, "Silver", (192, 192, 192)),
        (CSS_SKY_BLUE, "Sky Blue", (135, 206, 235)),
        (CSS_SLATE_BLUE, "Slate Blue", (106, 90, 205)),
        (CSS_SLATE_GRAY, "Slate Gray", (112, 128, 144)),
        (CSS_SNOW, "Snow", (255, 250, 250)),
        (CSS_SPRING_GREEN, "Spring Green", (0, 255, 127)),
        (CSS_STEEL_BLUE, "Steel Blue", (70, 130, 180)),
        (CSS_TAN, "Tan", (210, 180, 140)),
        (CSS_TEAL, "Teal", (0, 128, 128)),
        (CSS_THISTLE, "Thistle", (216, 191, 216)),
        (CSS_TOMATO, "Tomato", (255, 99, 71)),
        (CSS_TURQUOISE, "Turquoise", (64, 224, 208)),
        (CSS_VIOLET, "Violet", (238, 130, 238)),
        (CSS_WHEAT, "Wheat", (245, 222, 179)),
        (CSS_WHITE, "White", (255, 255, 255)),
        (CSS_WHITE_SMOKE, "White Smoke", (245, 245, 245)),
        (CSS_YELLOW, "Yellow", (255, 255, 0)),
        (CSS_YELLOW_GREEN, "Yellow Green", (154, 205, 50)),
    ]
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::IntoStorage;

    #[test]
    fn max_channels() {
        assert_eq!(Rgb555::WHITE, Rgb555::CSS_WHITE);
        assert_eq!(Bgr555::WHITE, Bgr555::CSS_WHITE);
        assert_eq!(Rgb565::WHITE, Rgb565::CSS_WHITE);
        assert_eq!(Bgr565::WHITE, Bgr565::CSS_WHITE);
        assert_eq!(Rgb888::WHITE, Rgb888::CSS_WHITE);
        assert_eq!(Bgr888::WHITE, Bgr888::CSS_WHITE);

        assert_eq!(Rgb555::RED, Rgb555::CSS_RED);
        assert_eq!(Bgr555::RED, Bgr555::CSS_RED);
        assert_eq!(Rgb565::RED, Rgb565::CSS_RED);
        assert_eq!(Bgr565::RED, Bgr565::CSS_RED);
        assert_eq!(Rgb888::RED, Rgb888::CSS_RED);
        assert_eq!(Bgr888::RED, Bgr888::CSS_RED);

        assert_eq!(Rgb555::GREEN, Rgb555::CSS_LIME);
        assert_eq!(Bgr555::GREEN, Bgr555::CSS_LIME);
        assert_eq!(Rgb565::GREEN, Rgb565::CSS_LIME);
        assert_eq!(Bgr565::GREEN, Bgr565::CSS_LIME);
        assert_eq!(Rgb888::GREEN, Rgb888::CSS_LIME);
        assert_eq!(Bgr888::GREEN, Bgr888::CSS_LIME);

        assert_eq!(Rgb555::BLUE, Rgb555::CSS_BLUE);
        assert_eq!(Bgr555::BLUE, Bgr555::CSS_BLUE);
        assert_eq!(Rgb565::BLUE, Rgb565::CSS_BLUE);
        assert_eq!(Bgr565::BLUE, Bgr565::CSS_BLUE);
        assert_eq!(Rgb888::BLUE, Rgb888::CSS_BLUE);
        assert_eq!(Bgr888::BLUE, Bgr888::CSS_BLUE);
    }

    #[test]
    fn conversion() {
        // 24bit: 7FFF00
        assert_eq!(Rgb565::CSS_CHARTREUSE.into_storage(), 0x7fe0);
    }
}
