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

macro_rules! web_color {
    ($ident:ident, $r:expr, $g:expr, $b:expr, $name:expr, $value:expr) => {
        #[doc = "<span style=\"background:rgb("]
        #[doc = $value]
        #[doc = ");border-radius:3px;display:inline-block;width:0.9em;height:0.9em\"></span>"]
        #[doc = $name]
        const $ident: Self = Self::with_rgb888($r, $g, $b);
    };
}

#[rustfmt::skip]
macro_rules! impl_web_colors {
    ($mod:ident, $container_str:expr, $container:path) => {
        #[doc = "Named web colors for the"]
        #[doc = $container_str]
        #[doc = "color type."]
        #[allow(unused)]
        impl WebColors for $container {
            web_color!(CSS_ALICE_BLUE, 240, 248, 255, "Alice Blue", "240, 248, 255");
            web_color!(CSS_ANTIQUE_WHITE, 250, 235, 215, "Antique White", "250, 235, 215");
            web_color!(CSS_AQUA, 0, 255, 255, "Aqua", "0, 255, 255");
            web_color!(CSS_AQUAMARINE, 127, 255, 212, "Aquamarine", "127, 255, 212");
            web_color!(CSS_AZURE, 240, 255, 255, "Azure", "240, 255, 255");
            web_color!(CSS_BEIGE, 245, 245, 220, "Beige", "245, 245, 220");
            web_color!(CSS_BISQUE, 255, 228, 196, "Bisque", "255, 228, 196");
            web_color!(CSS_BLACK, 0, 0, 0, "Black", "0, 0, 0");
            web_color!(CSS_BLANCHED_ALMOND, 255, 235, 205, "Blanched Almond", "255, 235, 205");
            web_color!(CSS_BLUE, 0, 0, 255, "Blue", "0, 0, 255");
            web_color!(CSS_BLUE_VIOLET, 138, 43, 226, "Blue Violet", "138, 43, 226");
            web_color!(CSS_BROWN, 165, 42, 42, "Brown", "165, 42, 42");
            web_color!(CSS_BURLY_WOOD, 222, 184, 135, "Burly Wood", "222, 184, 135");
            web_color!(CSS_CADET_BLUE, 95, 158, 160, "Cadet Blue", "95, 158, 160");
            web_color!(CSS_CHARTREUSE, 127, 255, 0, "Chartreuse", "127, 255, 0");
            web_color!(CSS_CHOCOLATE, 210, 105, 30, "Chocolate", "210, 105, 30");
            web_color!(CSS_CORAL, 255, 127, 80, "Coral", "255, 127, 80");
            web_color!(CSS_CORNFLOWER_BLUE, 100, 149, 237, "Cornflower Blue", "100, 149, 237");
            web_color!(CSS_CORNSILK, 255, 248, 220, "Cornsilk", "255, 248, 220");
            web_color!(CSS_CRIMSON, 220, 20, 60, "Crimson", "220, 20, 60");
            web_color!(CSS_CYAN, 0, 255, 255, "Cyan", "0, 255, 255");
            web_color!(CSS_DARK_BLUE, 0, 0, 139, "Dark Blue", "0, 0, 139");
            web_color!(CSS_DARK_CYAN, 0, 139, 139, "Dark Cyan", "0, 139, 139");
            web_color!(CSS_DARK_GOLDENROD, 184, 134, 11, "Dark Goldenrod", "184, 134, 11");
            web_color!(CSS_DARK_GRAY, 169, 169, 169, "Dark Gray", "169, 169, 169");
            web_color!(CSS_DARK_GREEN, 0, 100, 0, "Dark Green", "0, 100, 0");
            web_color!(CSS_DARK_KHAKI, 189, 183, 107, "Dark Khaki", "189, 183, 107");
            web_color!(CSS_DARK_MAGENTA, 139, 0, 139, "Dark Magenta", "139, 0, 139");
            web_color!(CSS_DARK_OLIVE_GREEN, 85, 107, 47, "Dark Olive Green", "85, 107, 47");
            web_color!(CSS_DARK_ORANGE, 255, 140, 0, "Dark Orange", "255, 140, 0");
            web_color!(CSS_DARK_ORCHID, 153, 50, 204, "Dark Orchid", "153, 50, 204");
            web_color!(CSS_DARK_RED, 139, 0, 0, "Dark Red", "139, 0, 0");
            web_color!(CSS_DARK_SALMON, 233, 150, 122, "Dark Salmon", "233, 150, 122");
            web_color!(CSS_DARK_SEA_GREEN, 143, 188, 143, "Dark Sea Green", "143, 188, 143");
            web_color!(CSS_DARK_SLATE_BLUE, 72, 61, 139, "Dark Slate Blue", "72, 61, 139");
            web_color!(CSS_DARK_SLATE_GRAY, 47, 79, 79, "Dark Slate Gray", "47, 79, 79");
            web_color!(CSS_DARK_TURQUOISE, 0, 206, 209, "Dark Turquoise", "0, 206, 209");
            web_color!(CSS_DARK_VIOLET, 148, 0, 211, "Dark Violet", "148, 0, 211");
            web_color!(CSS_DEEP_PINK, 255, 20, 147, "Deep Pink", "255, 20, 147");
            web_color!(CSS_DEEP_SKY_BLUE, 0, 191, 255, "Deep Sky Blue", "0, 191, 255");
            web_color!(CSS_DIM_GRAY, 105, 105, 105, "Dim Gray", "105, 105, 105");
            web_color!(CSS_DODGER_BLUE, 30, 144, 255, "Dodger Blue", "30, 144, 255");
            web_color!(CSS_FIRE_BRICK, 178, 34, 34, "Fire Brick", "178, 34, 34");
            web_color!(CSS_FLORAL_WHITE, 255, 250, 240, "Floral White", "255, 250, 240");
            web_color!(CSS_FOREST_GREEN, 34, 139, 34, "Forest Green", "34, 139, 34");
            web_color!(CSS_FUCHSIA, 255, 0, 255, "Fuchsia", "255, 0, 255");
            web_color!(CSS_GAINSBORO, 220, 220, 220, "Gainsboro", "220, 220, 220");
            web_color!(CSS_GHOST_WHITE, 248, 248, 255, "Ghost White", "248, 248, 255");
            web_color!(CSS_GOLD, 255, 215, 0, "Gold", "255, 215, 0");
            web_color!(CSS_GOLDENROD, 218, 165, 32, "Goldenrod", "218, 165, 32");
            web_color!(CSS_GRAY, 128, 128, 128, "Gray", "128, 128, 128");
            web_color!(CSS_GREEN, 0, 128, 0, "Green", "0, 128, 0");
            web_color!(CSS_GREEN_YELLOW, 173, 255, 47, "Green Yellow", "173, 255, 47");
            web_color!(CSS_HONEYDEW, 240, 255, 240, "Honeydew", "240, 255, 240");
            web_color!(CSS_HOT_PINK, 255, 105, 180, "Hot Pink", "255, 105, 180");
            web_color!(CSS_INDIAN_RED, 205, 92, 92, "Indian Red", "205, 92, 92");
            web_color!(CSS_INDIGO, 75, 0, 130, "Indigo", "75, 0, 130");
            web_color!(CSS_IVORY, 255, 255, 240, "Ivory", "255, 255, 240");
            web_color!(CSS_KHAKI, 240, 230, 140, "Khaki", "240, 230, 140");
            web_color!(CSS_LAVENDER, 230, 230, 250, "Lavender", "230, 230, 250");
            web_color!(CSS_LAVENDER_BLUSH, 255, 240, 245, "Lavender Blush", "255, 240, 245");
            web_color!(CSS_LAWN_GREEN, 124, 252, 0, "Lawn Green", "124, 252, 0");
            web_color!(CSS_LEMON_CHIFFON, 255, 250, 205, "Lemon Chiffon", "255, 250, 205");
            web_color!(CSS_LIGHT_BLUE, 173, 216, 230, "Light Blue", "173, 216, 230");
            web_color!(CSS_LIGHT_CORAL, 240, 128, 128, "Light Coral", "240, 128, 128");
            web_color!(CSS_LIGHT_CYAN, 224, 255, 255, "Light Cyan", "224, 255, 255");
            web_color!(CSS_LIGHT_GOLDENROD_YELLOW, 250, 250, 210, "Light Goldenrod Yellow", "250, 250, 210");
            web_color!(CSS_LIGHT_GRAY, 211, 211, 211, "Light Gray", "211, 211, 211");
            web_color!(CSS_LIGHT_GREEN, 144, 238, 144, "Light Green", "144, 238, 144");
            web_color!(CSS_LIGHT_PINK, 255, 182, 193, "Light Pink", "255, 182, 193");
            web_color!(CSS_LIGHT_SALMON, 255, 160, 122, "Light Salmon", "255, 160, 122");
            web_color!(CSS_LIGHT_SEA_GREEN, 32, 178, 170, "Light Sea Green", "32, 178, 170");
            web_color!(CSS_LIGHT_SKY_BLUE, 135, 206, 250, "Light Sky Blue", "135, 206, 250");
            web_color!(CSS_LIGHT_SLATE_GRAY, 119, 136, 153, "Light Slate Gray", "119, 136, 153");
            web_color!(CSS_LIGHT_STEEL_BLUE, 176, 196, 222, "Light Steel Blue", "176, 196, 222");
            web_color!(CSS_LIGHT_YELLOW, 255, 255, 224, "Light Yellow", "255, 255, 224");
            web_color!(CSS_LIME, 0, 255, 0, "Lime", "0, 255, 0");
            web_color!(CSS_LIME_GREEN, 50, 205, 50, "Lime Green", "50, 205, 50");
            web_color!(CSS_LINEN, 250, 240, 230, "Linen", "250, 240, 230");
            web_color!(CSS_MAGENTA, 255, 0, 255, "Magenta", "255, 0, 255");
            web_color!(CSS_MAROON, 128, 0, 0, "Maroon", "128, 0, 0");
            web_color!(CSS_MEDIUM_AQUAMARINE, 102, 205, 170, "Medium Aquamarine", "102, 205, 170");
            web_color!(CSS_MEDIUM_BLUE, 0, 0, 205, "Medium Blue", "0, 0, 205");
            web_color!(CSS_MEDIUM_ORCHID, 186, 85, 211, "Medium Orchid", "186, 85, 211");
            web_color!(CSS_MEDIUM_PURPLE, 147, 112, 219, "Medium Purple", "147, 112, 219");
            web_color!(CSS_MEDIUM_SEA_GREEN, 60, 179, 113, "Medium Sea Green", "60, 179, 113");
            web_color!(CSS_MEDIUM_SLATE_BLUE, 123, 104, 238, "Medium Slate Blue", "123, 104, 238");
            web_color!(CSS_MEDIUM_SPRING_GREEN, 0, 250, 154, "Medium Spring Green", "0, 250, 154");
            web_color!(CSS_MEDIUM_TURQUOISE, 72, 209, 204, "Medium Turquoise", "72, 209, 204");
            web_color!(CSS_MEDIUM_VIOLET_RED, 199, 21, 133, "Medium Violet Red", "199, 21, 133");
            web_color!(CSS_MIDNIGHT_BLUE, 25, 25, 112, "Midnight Blue", "25, 25, 112");
            web_color!(CSS_MINT_CREAM, 245, 255, 250, "Mint Cream", "245, 255, 250");
            web_color!(CSS_MISTY_ROSE, 255, 228, 225, "Misty Rose", "255, 228, 225");
            web_color!(CSS_MOCCASIN, 255, 228, 181, "Moccasin", "255, 228, 181");
            web_color!(CSS_NAVAJO_WHITE, 255, 222, 173, "Navajo White", "255, 222, 173");
            web_color!(CSS_NAVY, 0, 0, 128, "Navy", "0, 0, 128");
            web_color!(CSS_OLD_LACE, 253, 245, 230, "Old Lace", "253, 245, 230");
            web_color!(CSS_OLIVE, 128, 128, 0, "Olive", "128, 128, 0");
            web_color!(CSS_OLIVE_DRAB, 107, 142, 35, "Olive Drab", "107, 142, 35");
            web_color!(CSS_ORANGE, 255, 165, 0, "Orange", "255, 165, 0");
            web_color!(CSS_ORANGE_RED, 255, 69, 0, "Orange Red", "255, 69, 0");
            web_color!(CSS_ORCHID, 218, 112, 214, "Orchid", "218, 112, 214");
            web_color!(CSS_PALE_GOLDENROD, 238, 232, 170, "Pale Goldenrod", "238, 232, 170");
            web_color!(CSS_PALE_GREEN, 152, 251, 152, "Pale Green", "152, 251, 152");
            web_color!(CSS_PALE_TURQUOISE, 175, 238, 238, "Pale Turquoise", "175, 238, 238");
            web_color!(CSS_PALE_VIOLET_RED, 219, 112, 147, "Pale Violet Red", "219, 112, 147");
            web_color!(CSS_PAPAYA_WHIP, 255, 239, 213, "Papaya Whip", "255, 239, 213");
            web_color!(CSS_PEACH_PUFF, 255, 218, 185, "Peach Puff", "255, 218, 185");
            web_color!(CSS_PERU, 205, 133, 63, "Peru", "205, 133, 63");
            web_color!(CSS_PINK, 255, 192, 203, "Pink", "255, 192, 203");
            web_color!(CSS_PLUM, 221, 160, 221, "Plum", "221, 160, 221");
            web_color!(CSS_POWDER_BLUE, 176, 224, 230, "Powder Blue", "176, 224, 230");
            web_color!(CSS_PURPLE, 128, 0, 128, "Purple", "128, 0, 128");
            web_color!(CSS_REBECCAPURPLE, 102, 51, 153, "Rebeccapurple", "102, 51, 153");
            web_color!(CSS_RED, 255, 0, 0, "Red", "255, 0, 0");
            web_color!(CSS_ROSY_BROWN, 188, 143, 143, "Rosy Brown", "188, 143, 143");
            web_color!(CSS_ROYAL_BLUE, 65, 105, 225, "Royal Blue", "65, 105, 225");
            web_color!(CSS_SADDLE_BROWN, 139, 69, 19, "Saddle Brown", "139, 69, 19");
            web_color!(CSS_SALMON, 250, 128, 114, "Salmon", "250, 128, 114");
            web_color!(CSS_SANDY_BROWN, 244, 164, 96, "Sandy Brown", "244, 164, 96");
            web_color!(CSS_SEA_GREEN, 46, 139, 87, "Sea Green", "46, 139, 87");
            web_color!(CSS_SEASHELL, 255, 245, 238, "Seashell", "255, 245, 238");
            web_color!(CSS_SIENNA, 160, 82, 45, "Sienna", "160, 82, 45");
            web_color!(CSS_SILVER, 192, 192, 192, "Silver", "192, 192, 192");
            web_color!(CSS_SKY_BLUE, 135, 206, 235, "Sky Blue", "135, 206, 235");
            web_color!(CSS_SLATE_BLUE, 106, 90, 205, "Slate Blue", "106, 90, 205");
            web_color!(CSS_SLATE_GRAY, 112, 128, 144, "Slate Gray", "112, 128, 144");
            web_color!(CSS_SNOW, 255, 250, 250, "Snow", "255, 250, 250");
            web_color!(CSS_SPRING_GREEN, 0, 255, 127, "Spring Green", "0, 255, 127");
            web_color!(CSS_STEEL_BLUE, 70, 130, 180, "Steel Blue", "70, 130, 180");
            web_color!(CSS_TAN, 210, 180, 140, "Tan", "210, 180, 140");
            web_color!(CSS_TEAL, 0, 128, 128, "Teal", "0, 128, 128");
            web_color!(CSS_THISTLE, 216, 191, 216, "Thistle", "216, 191, 216");
            web_color!(CSS_TOMATO, 255, 99, 71, "Tomato", "255, 99, 71");
            web_color!(CSS_TURQUOISE, 64, 224, 208, "Turquoise", "64, 224, 208");
            web_color!(CSS_VIOLET, 238, 130, 238, "Violet", "238, 130, 238");
            web_color!(CSS_WHEAT, 245, 222, 179, "Wheat", "245, 222, 179");
            web_color!(CSS_WHITE, 255, 255, 255, "White", "255, 255, 255");
            web_color!(CSS_WHITE_SMOKE, 245, 245, 245, "White Smoke", "245, 245, 245");
            web_color!(CSS_YELLOW, 255, 255, 0, "Yellow", "255, 255, 0");
            web_color!(CSS_YELLOW_GREEN, 154, 205, 50, "Yellow Green", "154, 205, 50");
        }
    };
}

impl_web_colors!(rgb555, "Rgb555", crate::pixelcolor::Rgb555);
impl_web_colors!(bgr555, "Bgr555", crate::pixelcolor::Bgr555);
impl_web_colors!(rgb565, "Rgb565", crate::pixelcolor::Rgb565);
impl_web_colors!(bgr565, "Bgr565", crate::pixelcolor::Bgr565);
impl_web_colors!(rgb888, "Rgb888", crate::pixelcolor::Rgb888);
impl_web_colors!(bgr888, "Bgr888", crate::pixelcolor::Bgr888);

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
