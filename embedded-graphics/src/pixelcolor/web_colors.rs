//! Named colors as defined in the CSS specification
//!
//! This list includes the [basic color keywords] as well as all colors in the
//! [extended color keywords] list.
//!
//! Note that the `grEy` variants of some colors as defined in the spec are absent from this list.
//! The `grAy` variants of these colors should be used instead.
//!
//! [basic color keywords]: https://drafts.csswg.org/css-color-3/#html4
//! [extended color keywords]: https://drafts.csswg.org/css-color-3/#svg-color

use crate::pixelcolor::{PixelColor, RgbColor};

macro_rules! conv {
    ($value:expr, $from_max:expr, $to_max:expr) => {
        (($value * $to_max as u16 + $from_max / 2) / $from_max) as u8
    };
}

macro_rules! trait_const {
    ($ident:ident, $name:expr,  $value:expr) => {
        #[doc = "<span style=\"background:rgb("]
        #[doc = $value]
        #[doc = ");border-radius:3px;display:inline-block;width:0.9em;height:0.9em\"></span>"]
        #[doc = $name]
        const $ident: Self;
    };
}

/// Web colors.
#[rustfmt::skip]
pub trait WebColors: PixelColor + RgbColor {
    trait_const!(CSS_ALICE_BLUE, "Alice Blue", "240, 248, 255");
    trait_const!(CSS_ANTIQUE_WHITE, "Antique White", "250, 235, 215");
    trait_const!(CSS_AQUA, "Aqua", "0, 255, 255");
    trait_const!(CSS_AQUAMARINE, "Aquamarine", "127, 255, 212");
    trait_const!(CSS_AZURE, "Azure", "240, 255, 255");
    trait_const!(CSS_BEIGE, "Beige", "245, 245, 220");
    trait_const!(CSS_BISQUE, "Bisque", "255, 228, 196");
    trait_const!(CSS_BLACK, "Black", "0, 0, 0");
    trait_const!(CSS_BLANCHED_ALMOND, "Blanched Almond", "255, 235, 205");
    trait_const!(CSS_BLUE, "Blue", "0, 0, 255");
    trait_const!(CSS_BLUE_VIOLET, "Blue Violet", "138, 43, 226");
    trait_const!(CSS_BROWN, "Brown", "165, 42, 42");
    trait_const!(CSS_BURLY_WOOD, "Burly Wood", "222, 184, 135");
    trait_const!(CSS_CADET_BLUE, "Cadet Blue", "95, 158, 160");
    trait_const!(CSS_CHARTREUSE, "Chartreuse", "127, 255, 0");
    trait_const!(CSS_CHOCOLATE, "Chocolate", "210, 105, 30");
    trait_const!(CSS_CORAL, "Coral", "255, 127, 80");
    trait_const!(CSS_CORNFLOWER_BLUE, "Cornflower Blue", "100, 149, 237");
    trait_const!(CSS_CORNSILK, "Cornsilk", "255, 248, 220");
    trait_const!(CSS_CRIMSON, "Crimson", "220, 20, 60");
    trait_const!(CSS_CYAN, "Cyan", "0, 255, 255");
    trait_const!(CSS_DARK_BLUE, "Dark Blue", "0, 0, 139");
    trait_const!(CSS_DARK_CYAN, "Dark Cyan", "0, 139, 139");
    trait_const!(CSS_DARK_GOLDENROD, "Dark Goldenrod", "184, 134, 11");
    trait_const!(CSS_DARK_GRAY, "Dark Gray", "169, 169, 169");
    trait_const!(CSS_DARK_GREEN, "Dark Green", "0, 100, 0");
    trait_const!(CSS_DARK_KHAKI, "Dark Khaki", "189, 183, 107");
    trait_const!(CSS_DARK_MAGENTA, "Dark Magenta", "139, 0, 139");
    trait_const!(CSS_DARK_OLIVE_GREEN, "Dark Olive Green", "85, 107, 47");
    trait_const!(CSS_DARK_ORANGE, "Dark Orange", "255, 140, 0");
    trait_const!(CSS_DARK_ORCHID, "Dark Orchid", "153, 50, 204");
    trait_const!(CSS_DARK_RED, "Dark Red", "139, 0, 0");
    trait_const!(CSS_DARK_SALMON, "Dark Salmon", "233, 150, 122");
    trait_const!(CSS_DARK_SEA_GREEN, "Dark Sea Green", "143, 188, 143");
    trait_const!(CSS_DARK_SLATE_BLUE, "Dark Slate Blue", "72, 61, 139");
    trait_const!(CSS_DARK_SLATE_GRAY, "Dark Slate Gray", "47, 79, 79");
    trait_const!(CSS_DARK_TURQUOISE, "Dark Turquoise", "0, 206, 209");
    trait_const!(CSS_DARK_VIOLET, "Dark Violet", "148, 0, 211");
    trait_const!(CSS_DEEP_PINK, "Deep Pink", "255, 20, 147");
    trait_const!(CSS_DEEP_SKY_BLUE, "Deep Sky Blue", "0, 191, 255");
    trait_const!(CSS_DIM_GRAY, "Dim Gray", "105, 105, 105");
    trait_const!(CSS_DODGER_BLUE, "Dodger Blue", "30, 144, 255");
    trait_const!(CSS_FIRE_BRICK, "Fire Brick", "178, 34, 34");
    trait_const!(CSS_FLORAL_WHITE, "Floral White", "255, 250, 240");
    trait_const!(CSS_FOREST_GREEN, "Forest Green", "34, 139, 34");
    trait_const!(CSS_FUCHSIA, "Fuchsia", "255, 0, 255");
    trait_const!(CSS_GAINSBORO, "Gainsboro", "220, 220, 220");
    trait_const!(CSS_GHOST_WHITE, "Ghost White", "248, 248, 255");
    trait_const!(CSS_GOLD, "Gold", "255, 215, 0");
    trait_const!(CSS_GOLDENROD, "Goldenrod", "218, 165, 32");
    trait_const!(CSS_GRAY, "Gray", "128, 128, 128");
    trait_const!(CSS_GREEN, "Green", "0, 128, 0");
    trait_const!(CSS_GREEN_YELLOW, "Green Yellow", "173, 255, 47");
    trait_const!(CSS_HONEYDEW, "Honeydew", "240, 255, 240");
    trait_const!(CSS_HOT_PINK, "Hot Pink", "255, 105, 180");
    trait_const!(CSS_INDIAN_RED, "Indian Red", "205, 92, 92");
    trait_const!(CSS_INDIGO, "Indigo", "75, 0, 130");
    trait_const!(CSS_IVORY, "Ivory", "255, 255, 240");
    trait_const!(CSS_KHAKI, "Khaki", "240, 230, 140");
    trait_const!(CSS_LAVENDER, "Lavender", "230, 230, 250");
    trait_const!(CSS_LAVENDER_BLUSH, "Lavender Blush", "255, 240, 245");
    trait_const!(CSS_LAWN_GREEN, "Lawn Green", "124, 252, 0");
    trait_const!(CSS_LEMON_CHIFFON, "Lemon Chiffon", "255, 250, 205");
    trait_const!(CSS_LIGHT_BLUE, "Light Blue", "173, 216, 230");
    trait_const!(CSS_LIGHT_CORAL, "Light Coral", "240, 128, 128");
    trait_const!(CSS_LIGHT_CYAN, "Light Cyan", "224, 255, 255");
    trait_const!( CSS_LIGHT_GOLDENROD_YELLOW, "Light Goldenrod Yellow", "250, 250, 210" );
    trait_const!(CSS_LIGHT_GRAY, "Light Gray", "211, 211, 211");
    trait_const!(CSS_LIGHT_GREEN, "Light Green", "144, 238, 144");
    trait_const!(CSS_LIGHT_PINK, "Light Pink", "255, 182, 193");
    trait_const!(CSS_LIGHT_SALMON, "Light Salmon", "255, 160, 122");
    trait_const!(CSS_LIGHT_SEA_GREEN, "Light Sea Green", "32, 178, 170");
    trait_const!(CSS_LIGHT_SKY_BLUE, "Light Sky Blue", "135, 206, 250");
    trait_const!(CSS_LIGHT_SLATE_GRAY, "Light Slate Gray", "119, 136, 153");
    trait_const!(CSS_LIGHT_STEEL_BLUE, "Light Steel Blue", "176, 196, 222");
    trait_const!(CSS_LIGHT_YELLOW, "Light Yellow", "255, 255, 224");
    trait_const!(CSS_LIME, "Lime", "0, 255, 0");
    trait_const!(CSS_LIME_GREEN, "Lime Green", "50, 205, 50");
    trait_const!(CSS_LINEN, "Linen", "250, 240, 230");
    trait_const!(CSS_MAGENTA, "Magenta", "255, 0, 255");
    trait_const!(CSS_MAROON, "Maroon", "128, 0, 0");
    trait_const!(CSS_MEDIUM_AQUAMARINE, "Medium Aquamarine", "102, 205, 170");
    trait_const!(CSS_MEDIUM_BLUE, "Medium Blue", "0, 0, 205");
    trait_const!(CSS_MEDIUM_ORCHID, "Medium Orchid", "186, 85, 211");
    trait_const!(CSS_MEDIUM_PURPLE, "Medium Purple", "147, 112, 219");
    trait_const!(CSS_MEDIUM_SEA_GREEN, "Medium Sea Green", "60, 179, 113");
    trait_const!(CSS_MEDIUM_SLATE_BLUE, "Medium Slate Blue", "123, 104, 238");
    trait_const!( CSS_MEDIUM_SPRING_GREEN, "Medium Spring Green", "0, 250, 154" );
    trait_const!(CSS_MEDIUM_TURQUOISE, "Medium Turquoise", "72, 209, 204");
    trait_const!(CSS_MEDIUM_VIOLET_RED, "Medium Violet Red", "199, 21, 133");
    trait_const!(CSS_MIDNIGHT_BLUE, "Midnight Blue", "25, 25, 112");
    trait_const!(CSS_MINT_CREAM, "Mint Cream", "245, 255, 250");
    trait_const!(CSS_MISTY_ROSE, "Misty Rose", "255, 228, 225");
    trait_const!(CSS_MOCCASIN, "Moccasin", "255, 228, 181");
    trait_const!(CSS_NAVAJO_WHITE, "Navajo White", "255, 222, 173");
    trait_const!(CSS_NAVY, "Navy", "0, 0, 128");
    trait_const!(CSS_OLD_LACE, "Old Lace", "253, 245, 230");
    trait_const!(CSS_OLIVE, "Olive", "128, 128, 0");
    trait_const!(CSS_OLIVE_DRAB, "Olive Drab", "107, 142, 35");
    trait_const!(CSS_ORANGE, "Orange", "255, 165, 0");
    trait_const!(CSS_ORANGE_RED, "Orange Red", "255, 69, 0");
    trait_const!(CSS_ORCHID, "Orchid", "218, 112, 214");
    trait_const!(CSS_PALE_GOLDENROD, "Pale Goldenrod", "238, 232, 170");
    trait_const!(CSS_PALE_GREEN, "Pale Green", "152, 251, 152");
    trait_const!(CSS_PALE_TURQUOISE, "Pale Turquoise", "175, 238, 238");
    trait_const!(CSS_PALE_VIOLET_RED, "Pale Violet Red", "219, 112, 147");
    trait_const!(CSS_PAPAYA_WHIP, "Papaya Whip", "255, 239, 213");
    trait_const!(CSS_PEACH_PUFF, "Peach Puff", "255, 218, 185");
    trait_const!(CSS_PERU, "Peru", "205, 133, 63");
    trait_const!(CSS_PINK, "Pink", "255, 192, 203");
    trait_const!(CSS_PLUM, "Plum", "221, 160, 221");
    trait_const!(CSS_POWDER_BLUE, "Powder Blue", "176, 224, 230");
    trait_const!(CSS_PURPLE, "Purple", "128, 0, 128");
    trait_const!(CSS_REBECCAPURPLE, "Rebeccapurple", "102, 51, 153");
    trait_const!(CSS_RED, "Red", "255, 0, 0");
    trait_const!(CSS_ROSY_BROWN, "Rosy Brown", "188, 143, 143");
    trait_const!(CSS_ROYAL_BLUE, "Royal Blue", "65, 105, 225");
    trait_const!(CSS_SADDLE_BROWN, "Saddle Brown", "139, 69, 19");
    trait_const!(CSS_SALMON, "Salmon", "250, 128, 114");
    trait_const!(CSS_SANDY_BROWN, "Sandy Brown", "244, 164, 96");
    trait_const!(CSS_SEA_GREEN, "Sea Green", "46, 139, 87");
    trait_const!(CSS_SEASHELL, "Seashell", "255, 245, 238");
    trait_const!(CSS_SIENNA, "Sienna", "160, 82, 45");
    trait_const!(CSS_SILVER, "Silver", "192, 192, 192");
    trait_const!(CSS_SKY_BLUE, "Sky Blue", "135, 206, 235");
    trait_const!(CSS_SLATE_BLUE, "Slate Blue", "106, 90, 205");
    trait_const!(CSS_SLATE_GRAY, "Slate Gray", "112, 128, 144");
    trait_const!(CSS_SNOW, "Snow", "255, 250, 250");
    trait_const!(CSS_SPRING_GREEN, "Spring Green", "0, 255, 127");
    trait_const!(CSS_STEEL_BLUE, "Steel Blue", "70, 130, 180");
    trait_const!(CSS_TAN, "Tan", "210, 180, 140");
    trait_const!(CSS_TEAL, "Teal", "0, 128, 128");
    trait_const!(CSS_THISTLE, "Thistle", "216, 191, 216");
    trait_const!(CSS_TOMATO, "Tomato", "255, 99, 71");
    trait_const!(CSS_TURQUOISE, "Turquoise", "64, 224, 208");
    trait_const!(CSS_VIOLET, "Violet", "238, 130, 238");
    trait_const!(CSS_WHEAT, "Wheat", "245, 222, 179");
    trait_const!(CSS_WHITE, "White", "255, 255, 255");
    trait_const!(CSS_WHITE_SMOKE, "White Smoke", "245, 245, 245");
    trait_const!(CSS_YELLOW, "Yellow", "255, 255, 0");
    trait_const!(CSS_YELLOW_GREEN, "Yellow Green", "154, 205, 50");
}

macro_rules! u24_color {
    ($ident:ident, $r:expr, $g:expr, $b:expr, $container:path, $name:expr,  $value:expr) => {
        #[doc = "<span style=\"background:rgb("]
        #[doc = $value]
        #[doc = ");border-radius:3px;display:inline-block;width:0.9em;height:0.9em\"></span>"]
        #[doc = $name]
        const $ident: Self = Self::new($r, $g, $b);
    };
}

#[rustfmt::skip]
macro_rules! impl_u24_web_colors {
    ($mod:ident, $container_str:expr, $container:path) => {
        #[doc = "Named web colors for the"]
        #[doc = $container_str]
        #[doc = "color type."]
        #[allow(unused)]
        impl WebColors for $container {
            u24_color!(CSS_ALICE_BLUE, 240, 248, 255, $container, "Alice Blue", "240, 248, 255");
            u24_color!(CSS_ANTIQUE_WHITE, 250, 235, 215, $container, "Antique White", "250, 235, 215");
            u24_color!(CSS_AQUA, 0, 255, 255, $container, "Aqua", "0, 255, 255");
            u24_color!(CSS_AQUAMARINE, 127, 255, 212, $container, "Aquamarine", "127, 255, 212");
            u24_color!(CSS_AZURE, 240, 255, 255, $container, "Azure", "240, 255, 255");
            u24_color!(CSS_BEIGE, 245, 245, 220, $container, "Beige", "245, 245, 220");
            u24_color!(CSS_BISQUE, 255, 228, 196, $container, "Bisque", "255, 228, 196");
            u24_color!(CSS_BLACK, 0, 0, 0, $container, "Black", "0, 0, 0");
            u24_color!(CSS_BLANCHED_ALMOND, 255, 235, 205, $container, "Blanched Almond", "255, 235, 205");
            u24_color!(CSS_BLUE, 0, 0, 255, $container, "Blue", "0, 0, 255");
            u24_color!(CSS_BLUE_VIOLET, 138, 43, 226, $container, "Blue Violet", "138, 43, 226");
            u24_color!(CSS_BROWN, 165, 42, 42, $container, "Brown", "165, 42, 42");
            u24_color!(CSS_BURLY_WOOD, 222, 184, 135, $container, "Burly Wood", "222, 184, 135");
            u24_color!(CSS_CADET_BLUE, 95, 158, 160, $container, "Cadet Blue", "95, 158, 160");
            u24_color!(CSS_CHARTREUSE, 127, 255, 0, $container, "Chartreuse", "127, 255, 0");
            u24_color!(CSS_CHOCOLATE, 210, 105, 30, $container, "Chocolate", "210, 105, 30");
            u24_color!(CSS_CORAL, 255, 127, 80, $container, "Coral", "255, 127, 80");
            u24_color!(CSS_CORNFLOWER_BLUE, 100, 149, 237, $container, "Cornflower Blue", "100, 149, 237");
            u24_color!(CSS_CORNSILK, 255, 248, 220, $container, "Cornsilk", "255, 248, 220");
            u24_color!(CSS_CRIMSON, 220, 20, 60, $container, "Crimson", "220, 20, 60");
            u24_color!(CSS_CYAN, 0, 255, 255, $container, "Cyan", "0, 255, 255");
            u24_color!(CSS_DARK_BLUE, 0, 0, 139, $container, "Dark Blue", "0, 0, 139");
            u24_color!(CSS_DARK_CYAN, 0, 139, 139, $container, "Dark Cyan", "0, 139, 139");
            u24_color!(CSS_DARK_GOLDENROD, 184, 134, 11, $container, "Dark Goldenrod", "184, 134, 11");
            u24_color!(CSS_DARK_GRAY, 169, 169, 169, $container, "Dark Gray", "169, 169, 169");
            u24_color!(CSS_DARK_GREEN, 0, 100, 0, $container, "Dark Green", "0, 100, 0");
            u24_color!(CSS_DARK_KHAKI, 189, 183, 107, $container, "Dark Khaki", "189, 183, 107");
            u24_color!(CSS_DARK_MAGENTA, 139, 0, 139, $container, "Dark Magenta", "139, 0, 139");
            u24_color!(CSS_DARK_OLIVE_GREEN, 85, 107, 47, $container, "Dark Olive Green", "85, 107, 47");
            u24_color!(CSS_DARK_ORANGE, 255, 140, 0, $container, "Dark Orange", "255, 140, 0");
            u24_color!(CSS_DARK_ORCHID, 153, 50, 204, $container, "Dark Orchid", "153, 50, 204");
            u24_color!(CSS_DARK_RED, 139, 0, 0, $container, "Dark Red", "139, 0, 0");
            u24_color!(CSS_DARK_SALMON, 233, 150, 122, $container, "Dark Salmon", "233, 150, 122");
            u24_color!(CSS_DARK_SEA_GREEN, 143, 188, 143, $container, "Dark Sea Green", "143, 188, 143");
            u24_color!(CSS_DARK_SLATE_BLUE, 72, 61, 139, $container, "Dark Slate Blue", "72, 61, 139");
            u24_color!(CSS_DARK_SLATE_GRAY, 47, 79, 79, $container, "Dark Slate Gray", "47, 79, 79");
            u24_color!(CSS_DARK_TURQUOISE, 0, 206, 209, $container, "Dark Turquoise", "0, 206, 209");
            u24_color!(CSS_DARK_VIOLET, 148, 0, 211, $container, "Dark Violet", "148, 0, 211");
            u24_color!(CSS_DEEP_PINK, 255, 20, 147, $container, "Deep Pink", "255, 20, 147");
            u24_color!(CSS_DEEP_SKY_BLUE, 0, 191, 255, $container, "Deep Sky Blue", "0, 191, 255");
            u24_color!(CSS_DIM_GRAY, 105, 105, 105, $container, "Dim Gray", "105, 105, 105");
            u24_color!(CSS_DODGER_BLUE, 30, 144, 255, $container, "Dodger Blue", "30, 144, 255");
            u24_color!(CSS_FIRE_BRICK, 178, 34, 34, $container, "Fire Brick", "178, 34, 34");
            u24_color!(CSS_FLORAL_WHITE, 255, 250, 240, $container, "Floral White", "255, 250, 240");
            u24_color!(CSS_FOREST_GREEN, 34, 139, 34, $container, "Forest Green", "34, 139, 34");
            u24_color!(CSS_FUCHSIA, 255, 0, 255, $container, "Fuchsia", "255, 0, 255");
            u24_color!(CSS_GAINSBORO, 220, 220, 220, $container, "Gainsboro", "220, 220, 220");
            u24_color!(CSS_GHOST_WHITE, 248, 248, 255, $container, "Ghost White", "248, 248, 255");
            u24_color!(CSS_GOLD, 255, 215, 0, $container, "Gold", "255, 215, 0");
            u24_color!(CSS_GOLDENROD, 218, 165, 32, $container, "Goldenrod", "218, 165, 32");
            u24_color!(CSS_GRAY, 128, 128, 128, $container, "Gray", "128, 128, 128");
            u24_color!(CSS_GREEN, 0, 128, 0, $container, "Green", "0, 128, 0");
            u24_color!(CSS_GREEN_YELLOW, 173, 255, 47, $container, "Green Yellow", "173, 255, 47");
            u24_color!(CSS_HONEYDEW, 240, 255, 240, $container, "Honeydew", "240, 255, 240");
            u24_color!(CSS_HOT_PINK, 255, 105, 180, $container, "Hot Pink", "255, 105, 180");
            u24_color!(CSS_INDIAN_RED, 205, 92, 92, $container, "Indian Red", "205, 92, 92");
            u24_color!(CSS_INDIGO, 75, 0, 130, $container, "Indigo", "75, 0, 130");
            u24_color!(CSS_IVORY, 255, 255, 240, $container, "Ivory", "255, 255, 240");
            u24_color!(CSS_KHAKI, 240, 230, 140, $container, "Khaki", "240, 230, 140");
            u24_color!(CSS_LAVENDER, 230, 230, 250, $container, "Lavender", "230, 230, 250");
            u24_color!(CSS_LAVENDER_BLUSH, 255, 240, 245, $container, "Lavender Blush", "255, 240, 245");
            u24_color!(CSS_LAWN_GREEN, 124, 252, 0, $container, "Lawn Green", "124, 252, 0");
            u24_color!(CSS_LEMON_CHIFFON, 255, 250, 205, $container, "Lemon Chiffon", "255, 250, 205");
            u24_color!(CSS_LIGHT_BLUE, 173, 216, 230, $container, "Light Blue", "173, 216, 230");
            u24_color!(CSS_LIGHT_CORAL, 240, 128, 128, $container, "Light Coral", "240, 128, 128");
            u24_color!(CSS_LIGHT_CYAN, 224, 255, 255, $container, "Light Cyan", "224, 255, 255");
            u24_color!(CSS_LIGHT_GOLDENROD_YELLOW, 250, 250, 210, $container, "Light Goldenrod Yellow", "250, 250, 210");
            u24_color!(CSS_LIGHT_GRAY, 211, 211, 211, $container, "Light Gray", "211, 211, 211");
            u24_color!(CSS_LIGHT_GREEN, 144, 238, 144, $container, "Light Green", "144, 238, 144");
            u24_color!(CSS_LIGHT_PINK, 255, 182, 193, $container, "Light Pink", "255, 182, 193");
            u24_color!(CSS_LIGHT_SALMON, 255, 160, 122, $container, "Light Salmon", "255, 160, 122");
            u24_color!(CSS_LIGHT_SEA_GREEN, 32, 178, 170, $container, "Light Sea Green", "32, 178, 170");
            u24_color!(CSS_LIGHT_SKY_BLUE, 135, 206, 250, $container, "Light Sky Blue", "135, 206, 250");
            u24_color!(CSS_LIGHT_SLATE_GRAY, 119, 136, 153, $container, "Light Slate Gray", "119, 136, 153");
            u24_color!(CSS_LIGHT_STEEL_BLUE, 176, 196, 222, $container, "Light Steel Blue", "176, 196, 222");
            u24_color!(CSS_LIGHT_YELLOW, 255, 255, 224, $container, "Light Yellow", "255, 255, 224");
            u24_color!(CSS_LIME, 0, 255, 0, $container, "Lime", "0, 255, 0");
            u24_color!(CSS_LIME_GREEN, 50, 205, 50, $container, "Lime Green", "50, 205, 50");
            u24_color!(CSS_LINEN, 250, 240, 230, $container, "Linen", "250, 240, 230");
            u24_color!(CSS_MAGENTA, 255, 0, 255, $container, "Magenta", "255, 0, 255");
            u24_color!(CSS_MAROON, 128, 0, 0, $container, "Maroon", "128, 0, 0");
            u24_color!(CSS_MEDIUM_AQUAMARINE, 102, 205, 170, $container, "Medium Aquamarine", "102, 205, 170");
            u24_color!(CSS_MEDIUM_BLUE, 0, 0, 205, $container, "Medium Blue", "0, 0, 205");
            u24_color!(CSS_MEDIUM_ORCHID, 186, 85, 211, $container, "Medium Orchid", "186, 85, 211");
            u24_color!(CSS_MEDIUM_PURPLE, 147, 112, 219, $container, "Medium Purple", "147, 112, 219");
            u24_color!(CSS_MEDIUM_SEA_GREEN, 60, 179, 113, $container, "Medium Sea Green", "60, 179, 113");
            u24_color!(CSS_MEDIUM_SLATE_BLUE, 123, 104, 238, $container, "Medium Slate Blue", "123, 104, 238");
            u24_color!(CSS_MEDIUM_SPRING_GREEN, 0, 250, 154, $container, "Medium Spring Green", "0, 250, 154");
            u24_color!(CSS_MEDIUM_TURQUOISE, 72, 209, 204, $container, "Medium Turquoise", "72, 209, 204");
            u24_color!(CSS_MEDIUM_VIOLET_RED, 199, 21, 133, $container, "Medium Violet Red", "199, 21, 133");
            u24_color!(CSS_MIDNIGHT_BLUE, 25, 25, 112, $container, "Midnight Blue", "25, 25, 112");
            u24_color!(CSS_MINT_CREAM, 245, 255, 250, $container, "Mint Cream", "245, 255, 250");
            u24_color!(CSS_MISTY_ROSE, 255, 228, 225, $container, "Misty Rose", "255, 228, 225");
            u24_color!(CSS_MOCCASIN, 255, 228, 181, $container, "Moccasin", "255, 228, 181");
            u24_color!(CSS_NAVAJO_WHITE, 255, 222, 173, $container, "Navajo White", "255, 222, 173");
            u24_color!(CSS_NAVY, 0, 0, 128, $container, "Navy", "0, 0, 128");
            u24_color!(CSS_OLD_LACE, 253, 245, 230, $container, "Old Lace", "253, 245, 230");
            u24_color!(CSS_OLIVE, 128, 128, 0, $container, "Olive", "128, 128, 0");
            u24_color!(CSS_OLIVE_DRAB, 107, 142, 35, $container, "Olive Drab", "107, 142, 35");
            u24_color!(CSS_ORANGE, 255, 165, 0, $container, "Orange", "255, 165, 0");
            u24_color!(CSS_ORANGE_RED, 255, 69, 0, $container, "Orange Red", "255, 69, 0");
            u24_color!(CSS_ORCHID, 218, 112, 214, $container, "Orchid", "218, 112, 214");
            u24_color!(CSS_PALE_GOLDENROD, 238, 232, 170, $container, "Pale Goldenrod", "238, 232, 170");
            u24_color!(CSS_PALE_GREEN, 152, 251, 152, $container, "Pale Green", "152, 251, 152");
            u24_color!(CSS_PALE_TURQUOISE, 175, 238, 238, $container, "Pale Turquoise", "175, 238, 238");
            u24_color!(CSS_PALE_VIOLET_RED, 219, 112, 147, $container, "Pale Violet Red", "219, 112, 147");
            u24_color!(CSS_PAPAYA_WHIP, 255, 239, 213, $container, "Papaya Whip", "255, 239, 213");
            u24_color!(CSS_PEACH_PUFF, 255, 218, 185, $container, "Peach Puff", "255, 218, 185");
            u24_color!(CSS_PERU, 205, 133, 63, $container, "Peru", "205, 133, 63");
            u24_color!(CSS_PINK, 255, 192, 203, $container, "Pink", "255, 192, 203");
            u24_color!(CSS_PLUM, 221, 160, 221, $container, "Plum", "221, 160, 221");
            u24_color!(CSS_POWDER_BLUE, 176, 224, 230, $container, "Powder Blue", "176, 224, 230");
            u24_color!(CSS_PURPLE, 128, 0, 128, $container, "Purple", "128, 0, 128");
            u24_color!(CSS_REBECCAPURPLE, 102, 51, 153, $container, "Rebeccapurple", "102, 51, 153");
            u24_color!(CSS_RED, 255, 0, 0, $container, "Red", "255, 0, 0");
            u24_color!(CSS_ROSY_BROWN, 188, 143, 143, $container, "Rosy Brown", "188, 143, 143");
            u24_color!(CSS_ROYAL_BLUE, 65, 105, 225, $container, "Royal Blue", "65, 105, 225");
            u24_color!(CSS_SADDLE_BROWN, 139, 69, 19, $container, "Saddle Brown", "139, 69, 19");
            u24_color!(CSS_SALMON, 250, 128, 114, $container, "Salmon", "250, 128, 114");
            u24_color!(CSS_SANDY_BROWN, 244, 164, 96, $container, "Sandy Brown", "244, 164, 96");
            u24_color!(CSS_SEA_GREEN, 46, 139, 87, $container, "Sea Green", "46, 139, 87");
            u24_color!(CSS_SEASHELL, 255, 245, 238, $container, "Seashell", "255, 245, 238");
            u24_color!(CSS_SIENNA, 160, 82, 45, $container, "Sienna", "160, 82, 45");
            u24_color!(CSS_SILVER, 192, 192, 192, $container, "Silver", "192, 192, 192");
            u24_color!(CSS_SKY_BLUE, 135, 206, 235, $container, "Sky Blue", "135, 206, 235");
            u24_color!(CSS_SLATE_BLUE, 106, 90, 205, $container, "Slate Blue", "106, 90, 205");
            u24_color!(CSS_SLATE_GRAY, 112, 128, 144, $container, "Slate Gray", "112, 128, 144");
            u24_color!(CSS_SNOW, 255, 250, 250, $container, "Snow", "255, 250, 250");
            u24_color!(CSS_SPRING_GREEN, 0, 255, 127, $container, "Spring Green", "0, 255, 127");
            u24_color!(CSS_STEEL_BLUE, 70, 130, 180, $container, "Steel Blue", "70, 130, 180");
            u24_color!(CSS_TAN, 210, 180, 140, $container, "Tan", "210, 180, 140");
            u24_color!(CSS_TEAL, 0, 128, 128, $container, "Teal", "0, 128, 128");
            u24_color!(CSS_THISTLE, 216, 191, 216, $container, "Thistle", "216, 191, 216");
            u24_color!(CSS_TOMATO, 255, 99, 71, $container, "Tomato", "255, 99, 71");
            u24_color!(CSS_TURQUOISE, 64, 224, 208, $container, "Turquoise", "64, 224, 208");
            u24_color!(CSS_VIOLET, 238, 130, 238, $container, "Violet", "238, 130, 238");
            u24_color!(CSS_WHEAT, 245, 222, 179, $container, "Wheat", "245, 222, 179");
            u24_color!(CSS_WHITE, 255, 255, 255, $container, "White", "255, 255, 255");
            u24_color!(CSS_WHITE_SMOKE, 245, 245, 245, $container, "White Smoke", "245, 245, 245");
            u24_color!(CSS_YELLOW, 255, 255, 0, $container, "Yellow", "255, 255, 0");
            u24_color!(CSS_YELLOW_GREEN, 154, 205, 50, $container, "Yellow Green", "154, 205, 50");
        }
    };
}

impl_u24_web_colors!(rgb888, "Rgb888", crate::pixelcolor::Rgb888);
impl_u24_web_colors!(bgr888, "Bgr888", crate::pixelcolor::Bgr888);

#[rustfmt::skip]
macro_rules! impl_u16_web_colors {
    ($mod:ident, $container_str:expr, $container:path) => {
        #[doc = "Named web colors for the"]
        #[doc = $container_str]
        #[doc = "color type."]
        #[allow(unused)]
        impl WebColors for $container {
            u16_color!(CSS_ALICE_BLUE, 240, 248, 255, $container, "Alice Blue", "240, 248, 255");
            u16_color!(CSS_ANTIQUE_WHITE, 250, 235, 215, $container, "Antique White", "250, 235, 215");
            u16_color!(CSS_AQUA, 0, 255, 255, $container, "Aqua", "0, 255, 255");
            u16_color!(CSS_AQUAMARINE, 127, 255, 212, $container, "Aquamarine", "127, 255, 212");
            u16_color!(CSS_AZURE, 240, 255, 255, $container, "Azure", "240, 255, 255");
            u16_color!(CSS_BEIGE, 245, 245, 220, $container, "Beige", "245, 245, 220");
            u16_color!(CSS_BISQUE, 255, 228, 196, $container, "Bisque", "255, 228, 196");
            u16_color!(CSS_BLACK, 0, 0, 0, $container, "Black", "0, 0, 0");
            u16_color!(CSS_BLANCHED_ALMOND, 255, 235, 205, $container, "Blanched Almond", "255, 235, 205");
            u16_color!(CSS_BLUE, 0, 0, 255, $container, "Blue", "0, 0, 255");
            u16_color!(CSS_BLUE_VIOLET, 138, 43, 226, $container, "Blue Violet", "138, 43, 226");
            u16_color!(CSS_BROWN, 165, 42, 42, $container, "Brown", "165, 42, 42");
            u16_color!(CSS_BURLY_WOOD, 222, 184, 135, $container, "Burly Wood", "222, 184, 135");
            u16_color!(CSS_CADET_BLUE, 95, 158, 160, $container, "Cadet Blue", "95, 158, 160");
            u16_color!(CSS_CHARTREUSE, 127, 255, 0, $container, "Chartreuse", "127, 255, 0");
            u16_color!(CSS_CHOCOLATE, 210, 105, 30, $container, "Chocolate", "210, 105, 30");
            u16_color!(CSS_CORAL, 255, 127, 80, $container, "Coral", "255, 127, 80");
            u16_color!(CSS_CORNFLOWER_BLUE, 100, 149, 237, $container, "Cornflower Blue", "100, 149, 237");
            u16_color!(CSS_CORNSILK, 255, 248, 220, $container, "Cornsilk", "255, 248, 220");
            u16_color!(CSS_CRIMSON, 220, 20, 60, $container, "Crimson", "220, 20, 60");
            u16_color!(CSS_CYAN, 0, 255, 255, $container, "Cyan", "0, 255, 255");
            u16_color!(CSS_DARK_BLUE, 0, 0, 139, $container, "Dark Blue", "0, 0, 139");
            u16_color!(CSS_DARK_CYAN, 0, 139, 139, $container, "Dark Cyan", "0, 139, 139");
            u16_color!(CSS_DARK_GOLDENROD, 184, 134, 11, $container, "Dark Goldenrod", "184, 134, 11");
            u16_color!(CSS_DARK_GRAY, 169, 169, 169, $container, "Dark Gray", "169, 169, 169");
            u16_color!(CSS_DARK_GREEN, 0, 100, 0, $container, "Dark Green", "0, 100, 0");
            u16_color!(CSS_DARK_KHAKI, 189, 183, 107, $container, "Dark Khaki", "189, 183, 107");
            u16_color!(CSS_DARK_MAGENTA, 139, 0, 139, $container, "Dark Magenta", "139, 0, 139");
            u16_color!(CSS_DARK_OLIVE_GREEN, 85, 107, 47, $container, "Dark Olive Green", "85, 107, 47");
            u16_color!(CSS_DARK_ORANGE, 255, 140, 0, $container, "Dark Orange", "255, 140, 0");
            u16_color!(CSS_DARK_ORCHID, 153, 50, 204, $container, "Dark Orchid", "153, 50, 204");
            u16_color!(CSS_DARK_RED, 139, 0, 0, $container, "Dark Red", "139, 0, 0");
            u16_color!(CSS_DARK_SALMON, 233, 150, 122, $container, "Dark Salmon", "233, 150, 122");
            u16_color!(CSS_DARK_SEA_GREEN, 143, 188, 143, $container, "Dark Sea Green", "143, 188, 143");
            u16_color!(CSS_DARK_SLATE_BLUE, 72, 61, 139, $container, "Dark Slate Blue", "72, 61, 139");
            u16_color!(CSS_DARK_SLATE_GRAY, 47, 79, 79, $container, "Dark Slate Gray", "47, 79, 79");
            u16_color!(CSS_DARK_TURQUOISE, 0, 206, 209, $container, "Dark Turquoise", "0, 206, 209");
            u16_color!(CSS_DARK_VIOLET, 148, 0, 211, $container, "Dark Violet", "148, 0, 211");
            u16_color!(CSS_DEEP_PINK, 255, 20, 147, $container, "Deep Pink", "255, 20, 147");
            u16_color!(CSS_DEEP_SKY_BLUE, 0, 191, 255, $container, "Deep Sky Blue", "0, 191, 255");
            u16_color!(CSS_DIM_GRAY, 105, 105, 105, $container, "Dim Gray", "105, 105, 105");
            u16_color!(CSS_DODGER_BLUE, 30, 144, 255, $container, "Dodger Blue", "30, 144, 255");
            u16_color!(CSS_FIRE_BRICK, 178, 34, 34, $container, "Fire Brick", "178, 34, 34");
            u16_color!(CSS_FLORAL_WHITE, 255, 250, 240, $container, "Floral White", "255, 250, 240");
            u16_color!(CSS_FOREST_GREEN, 34, 139, 34, $container, "Forest Green", "34, 139, 34");
            u16_color!(CSS_FUCHSIA, 255, 0, 255, $container, "Fuchsia", "255, 0, 255");
            u16_color!(CSS_GAINSBORO, 220, 220, 220, $container, "Gainsboro", "220, 220, 220");
            u16_color!(CSS_GHOST_WHITE, 248, 248, 255, $container, "Ghost White", "248, 248, 255");
            u16_color!(CSS_GOLD, 255, 215, 0, $container, "Gold", "255, 215, 0");
            u16_color!(CSS_GOLDENROD, 218, 165, 32, $container, "Goldenrod", "218, 165, 32");
            u16_color!(CSS_GRAY, 128, 128, 128, $container, "Gray", "128, 128, 128");
            u16_color!(CSS_GREEN, 0, 128, 0, $container, "Green", "0, 128, 0");
            u16_color!(CSS_GREEN_YELLOW, 173, 255, 47, $container, "Green Yellow", "173, 255, 47");
            u16_color!(CSS_HONEYDEW, 240, 255, 240, $container, "Honeydew", "240, 255, 240");
            u16_color!(CSS_HOT_PINK, 255, 105, 180, $container, "Hot Pink", "255, 105, 180");
            u16_color!(CSS_INDIAN_RED, 205, 92, 92, $container, "Indian Red", "205, 92, 92");
            u16_color!(CSS_INDIGO, 75, 0, 130, $container, "Indigo", "75, 0, 130");
            u16_color!(CSS_IVORY, 255, 255, 240, $container, "Ivory", "255, 255, 240");
            u16_color!(CSS_KHAKI, 240, 230, 140, $container, "Khaki", "240, 230, 140");
            u16_color!(CSS_LAVENDER, 230, 230, 250, $container, "Lavender", "230, 230, 250");
            u16_color!(CSS_LAVENDER_BLUSH, 255, 240, 245, $container, "Lavender Blush", "255, 240, 245");
            u16_color!(CSS_LAWN_GREEN, 124, 252, 0, $container, "Lawn Green", "124, 252, 0");
            u16_color!(CSS_LEMON_CHIFFON, 255, 250, 205, $container, "Lemon Chiffon", "255, 250, 205");
            u16_color!(CSS_LIGHT_BLUE, 173, 216, 230, $container, "Light Blue", "173, 216, 230");
            u16_color!(CSS_LIGHT_CORAL, 240, 128, 128, $container, "Light Coral", "240, 128, 128");
            u16_color!(CSS_LIGHT_CYAN, 224, 255, 255, $container, "Light Cyan", "224, 255, 255");
            u16_color!(CSS_LIGHT_GOLDENROD_YELLOW, 250, 250, 210, $container, "Light Goldenrod Yellow", "250, 250, 210");
            u16_color!(CSS_LIGHT_GRAY, 211, 211, 211, $container, "Light Gray", "211, 211, 211");
            u16_color!(CSS_LIGHT_GREEN, 144, 238, 144, $container, "Light Green", "144, 238, 144");
            u16_color!(CSS_LIGHT_PINK, 255, 182, 193, $container, "Light Pink", "255, 182, 193");
            u16_color!(CSS_LIGHT_SALMON, 255, 160, 122, $container, "Light Salmon", "255, 160, 122");
            u16_color!(CSS_LIGHT_SEA_GREEN, 32, 178, 170, $container, "Light Sea Green", "32, 178, 170");
            u16_color!(CSS_LIGHT_SKY_BLUE, 135, 206, 250, $container, "Light Sky Blue", "135, 206, 250");
            u16_color!(CSS_LIGHT_SLATE_GRAY, 119, 136, 153, $container, "Light Slate Gray", "119, 136, 153");
            u16_color!(CSS_LIGHT_STEEL_BLUE, 176, 196, 222, $container, "Light Steel Blue", "176, 196, 222");
            u16_color!(CSS_LIGHT_YELLOW, 255, 255, 224, $container, "Light Yellow", "255, 255, 224");
            u16_color!(CSS_LIME, 0, 255, 0, $container, "Lime", "0, 255, 0");
            u16_color!(CSS_LIME_GREEN, 50, 205, 50, $container, "Lime Green", "50, 205, 50");
            u16_color!(CSS_LINEN, 250, 240, 230, $container, "Linen", "250, 240, 230");
            u16_color!(CSS_MAGENTA, 255, 0, 255, $container, "Magenta", "255, 0, 255");
            u16_color!(CSS_MAROON, 128, 0, 0, $container, "Maroon", "128, 0, 0");
            u16_color!(CSS_MEDIUM_AQUAMARINE, 102, 205, 170, $container, "Medium Aquamarine", "102, 205, 170");
            u16_color!(CSS_MEDIUM_BLUE, 0, 0, 205, $container, "Medium Blue", "0, 0, 205");
            u16_color!(CSS_MEDIUM_ORCHID, 186, 85, 211, $container, "Medium Orchid", "186, 85, 211");
            u16_color!(CSS_MEDIUM_PURPLE, 147, 112, 219, $container, "Medium Purple", "147, 112, 219");
            u16_color!(CSS_MEDIUM_SEA_GREEN, 60, 179, 113, $container, "Medium Sea Green", "60, 179, 113");
            u16_color!(CSS_MEDIUM_SLATE_BLUE, 123, 104, 238, $container, "Medium Slate Blue", "123, 104, 238");
            u16_color!(CSS_MEDIUM_SPRING_GREEN, 0, 250, 154, $container, "Medium Spring Green", "0, 250, 154");
            u16_color!(CSS_MEDIUM_TURQUOISE, 72, 209, 204, $container, "Medium Turquoise", "72, 209, 204");
            u16_color!(CSS_MEDIUM_VIOLET_RED, 199, 21, 133, $container, "Medium Violet Red", "199, 21, 133");
            u16_color!(CSS_MIDNIGHT_BLUE, 25, 25, 112, $container, "Midnight Blue", "25, 25, 112");
            u16_color!(CSS_MINT_CREAM, 245, 255, 250, $container, "Mint Cream", "245, 255, 250");
            u16_color!(CSS_MISTY_ROSE, 255, 228, 225, $container, "Misty Rose", "255, 228, 225");
            u16_color!(CSS_MOCCASIN, 255, 228, 181, $container, "Moccasin", "255, 228, 181");
            u16_color!(CSS_NAVAJO_WHITE, 255, 222, 173, $container, "Navajo White", "255, 222, 173");
            u16_color!(CSS_NAVY, 0, 0, 128, $container, "Navy", "0, 0, 128");
            u16_color!(CSS_OLD_LACE, 253, 245, 230, $container, "Old Lace", "253, 245, 230");
            u16_color!(CSS_OLIVE, 128, 128, 0, $container, "Olive", "128, 128, 0");
            u16_color!(CSS_OLIVE_DRAB, 107, 142, 35, $container, "Olive Drab", "107, 142, 35");
            u16_color!(CSS_ORANGE, 255, 165, 0, $container, "Orange", "255, 165, 0");
            u16_color!(CSS_ORANGE_RED, 255, 69, 0, $container, "Orange Red", "255, 69, 0");
            u16_color!(CSS_ORCHID, 218, 112, 214, $container, "Orchid", "218, 112, 214");
            u16_color!(CSS_PALE_GOLDENROD, 238, 232, 170, $container, "Pale Goldenrod", "238, 232, 170");
            u16_color!(CSS_PALE_GREEN, 152, 251, 152, $container, "Pale Green", "152, 251, 152");
            u16_color!(CSS_PALE_TURQUOISE, 175, 238, 238, $container, "Pale Turquoise", "175, 238, 238");
            u16_color!(CSS_PALE_VIOLET_RED, 219, 112, 147, $container, "Pale Violet Red", "219, 112, 147");
            u16_color!(CSS_PAPAYA_WHIP, 255, 239, 213, $container, "Papaya Whip", "255, 239, 213");
            u16_color!(CSS_PEACH_PUFF, 255, 218, 185, $container, "Peach Puff", "255, 218, 185");
            u16_color!(CSS_PERU, 205, 133, 63, $container, "Peru", "205, 133, 63");
            u16_color!(CSS_PINK, 255, 192, 203, $container, "Pink", "255, 192, 203");
            u16_color!(CSS_PLUM, 221, 160, 221, $container, "Plum", "221, 160, 221");
            u16_color!(CSS_POWDER_BLUE, 176, 224, 230, $container, "Powder Blue", "176, 224, 230");
            u16_color!(CSS_PURPLE, 128, 0, 128, $container, "Purple", "128, 0, 128");
            u16_color!(CSS_REBECCAPURPLE, 102, 51, 153, $container, "Rebeccapurple", "102, 51, 153");
            u16_color!(CSS_RED, 255, 0, 0, $container, "Red", "255, 0, 0");
            u16_color!(CSS_ROSY_BROWN, 188, 143, 143, $container, "Rosy Brown", "188, 143, 143");
            u16_color!(CSS_ROYAL_BLUE, 65, 105, 225, $container, "Royal Blue", "65, 105, 225");
            u16_color!(CSS_SADDLE_BROWN, 139, 69, 19, $container, "Saddle Brown", "139, 69, 19");
            u16_color!(CSS_SALMON, 250, 128, 114, $container, "Salmon", "250, 128, 114");
            u16_color!(CSS_SANDY_BROWN, 244, 164, 96, $container, "Sandy Brown", "244, 164, 96");
            u16_color!(CSS_SEA_GREEN, 46, 139, 87, $container, "Sea Green", "46, 139, 87");
            u16_color!(CSS_SEASHELL, 255, 245, 238, $container, "Seashell", "255, 245, 238");
            u16_color!(CSS_SIENNA, 160, 82, 45, $container, "Sienna", "160, 82, 45");
            u16_color!(CSS_SILVER, 192, 192, 192, $container, "Silver", "192, 192, 192");
            u16_color!(CSS_SKY_BLUE, 135, 206, 235, $container, "Sky Blue", "135, 206, 235");
            u16_color!(CSS_SLATE_BLUE, 106, 90, 205, $container, "Slate Blue", "106, 90, 205");
            u16_color!(CSS_SLATE_GRAY, 112, 128, 144, $container, "Slate Gray", "112, 128, 144");
            u16_color!(CSS_SNOW, 255, 250, 250, $container, "Snow", "255, 250, 250");
            u16_color!(CSS_SPRING_GREEN, 0, 255, 127, $container, "Spring Green", "0, 255, 127");
            u16_color!(CSS_STEEL_BLUE, 70, 130, 180, $container, "Steel Blue", "70, 130, 180");
            u16_color!(CSS_TAN, 210, 180, 140, $container, "Tan", "210, 180, 140");
            u16_color!(CSS_TEAL, 0, 128, 128, $container, "Teal", "0, 128, 128");
            u16_color!(CSS_THISTLE, 216, 191, 216, $container, "Thistle", "216, 191, 216");
            u16_color!(CSS_TOMATO, 255, 99, 71, $container, "Tomato", "255, 99, 71");
            u16_color!(CSS_TURQUOISE, 64, 224, 208, $container, "Turquoise", "64, 224, 208");
            u16_color!(CSS_VIOLET, 238, 130, 238, $container, "Violet", "238, 130, 238");
            u16_color!(CSS_WHEAT, 245, 222, 179, $container, "Wheat", "245, 222, 179");
            u16_color!(CSS_WHITE, 255, 255, 255, $container, "White", "255, 255, 255");
            u16_color!(CSS_WHITE_SMOKE, 245, 245, 245, $container, "White Smoke", "245, 245, 245");
            u16_color!(CSS_YELLOW, 255, 255, 0, $container, "Yellow", "255, 255, 0");
            u16_color!(CSS_YELLOW_GREEN, 154, 205, 50, $container, "Yellow Green", "154, 205, 50");
        }
    };
}

macro_rules! u16_color {
    ($ident:ident, $r:expr, $g:expr, $b:expr, $container:path, $name:expr,  $value:expr) => {
        #[doc = "<span style=\"background:rgb("]
        #[doc = $value]
        #[doc = ");border-radius:3px;display:inline-block;width:0.9em;height:0.9em\"></span>"]
        #[doc = $name]
        const $ident: Self = Self::new(
            conv!($r, 255u16, Self::MAX_R),
            conv!($g, 255u16, Self::MAX_G),
            conv!($b, 255u16, Self::MAX_B),
        );
    };
}

impl_u16_web_colors!(rgb555, "Rgb555", crate::pixelcolor::Rgb555);
impl_u16_web_colors!(bgr555, "Bgr555", crate::pixelcolor::Bgr555);
impl_u16_web_colors!(rgb565, "Rgb565", crate::pixelcolor::Rgb565);
impl_u16_web_colors!(bgr565, "Bgr565", crate::pixelcolor::Bgr565);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelcolor::{
        Bgr555, Bgr565, Bgr888, IntoStorage, Rgb555, Rgb565, Rgb888, RgbColor,
    };

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
