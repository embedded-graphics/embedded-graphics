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

macro_rules! color {
    ($ident:ident, $r:expr, $g:expr, $b:expr, $name:expr,  $value:expr) => {

        #[doc = "<span style=\"background:rgb("]
        #[doc = $value]
        #[doc = ");border-radius:3px;display:inline-block;width:0.9em;height:0.9em\"></span>"]
        #[doc = $name]
        const $ident: Self = Self::new($r, $g, $b);
    };
}

macro_rules! assoc {
    ($ident:ident, $r:expr, $g:expr, $b:expr, $name:expr, $value:expr) => {
        #[doc = "<span style=\"background:rgb("]
        #[doc = $value]
        #[doc = ");border-radius:3px;display:inline-block;width:0.9em;height:0.9em\"></span>"]
        #[doc = $name]
        const $ident: Self;
    };
}

/// Web-safe colors
#[rustfmt::skip]
pub trait WebColor {
    assoc!(ALICE_BLUE, 240, 248, 255, "Alice Blue", "240, 248, 255");
    assoc!(ANTIQUE_WHITE, 250, 235, 215, "Antique White", "250, 235, 215");
    assoc!(AQUA, 0, 255, 255, "Aqua", "0, 255, 255");
    assoc!(AQUAMARINE, 127, 255, 212, "Aquamarine", "127, 255, 212");
    assoc!(AZURE, 240, 255, 255, "Azure", "240, 255, 255");
    assoc!(BEIGE, 245, 245, 220, "Beige", "245, 245, 220");
    assoc!(BISQUE, 255, 228, 196, "Bisque", "255, 228, 196");
    assoc!(BLACK, 0, 0, 0, "Black", "0, 0, 0");
    assoc!(BLANCHED_ALMOND, 255, 235, 205, "Blanched Almond", "255, 235, 205");
    assoc!(BLUE, 0, 0, 255, "Blue", "0, 0, 255");
    assoc!(BLUE_VIOLET, 138, 43, 226, "Blue Violet", "138, 43, 226");
    assoc!(BROWN, 165, 42, 42, "Brown", "165, 42, 42");
    assoc!(BURLY_WOOD, 222, 184, 135, "Burly Wood", "222, 184, 135");
    assoc!(CADET_BLUE, 95, 158, 160, "Cadet Blue", "95, 158, 160");
    assoc!(CHARTREUSE, 127, 255, 0, "Chartreuse", "127, 255, 0");
    assoc!(CHOCOLATE, 210, 105, 30, "Chocolate", "210, 105, 30");
    assoc!(CORAL, 255, 127, 80, "Coral", "255, 127, 80");
    assoc!(CORNFLOWER_BLUE, 100, 149, 237, "Cornflower Blue", "100, 149, 237");
    assoc!(CORNSILK, 255, 248, 220, "Cornsilk", "255, 248, 220");
    assoc!(CRIMSON, 220, 20, 60, "Crimson", "220, 20, 60");
    assoc!(CYAN, 0, 255, 255, "Cyan", "0, 255, 255");
    assoc!(DARK_BLUE, 0, 0, 139, "Dark Blue", "0, 0, 139");
    assoc!(DARK_CYAN, 0, 139, 139, "Dark Cyan", "0, 139, 139");
    assoc!(DARK_GOLDENROD, 184, 134, 11, "Dark Goldenrod", "184, 134, 11");
    assoc!(DARK_GRAY, 169, 169, 169, "Dark Gray", "169, 169, 169");
    assoc!(DARK_GREEN, 0, 100, 0, "Dark Green", "0, 100, 0");
    assoc!(DARK_KHAKI, 189, 183, 107, "Dark Khaki", "189, 183, 107");
    assoc!(DARK_MAGENTA, 139, 0, 139, "Dark Magenta", "139, 0, 139");
    assoc!(DARK_OLIVE_GREEN, 85, 107, 47, "Dark Olive Green", "85, 107, 47");
    assoc!(DARK_ORANGE, 255, 140, 0, "Dark Orange", "255, 140, 0");
    assoc!(DARK_ORCHID, 153, 50, 204, "Dark Orchid", "153, 50, 204");
    assoc!(DARK_RED, 139, 0, 0, "Dark Red", "139, 0, 0");
    assoc!(DARK_SALMON, 233, 150, 122, "Dark Salmon", "233, 150, 122");
    assoc!(DARK_SEA_GREEN, 143, 188, 143, "Dark Sea Green", "143, 188, 143");
    assoc!(DARK_SLATE_BLUE, 72, 61, 139, "Dark Slate Blue", "72, 61, 139");
    assoc!(DARK_SLATE_GRAY, 47, 79, 79, "Dark Slate Gray", "47, 79, 79");
    assoc!(DARK_TURQUOISE, 0, 206, 209, "Dark Turquoise", "0, 206, 209");
    assoc!(DARK_VIOLET, 148, 0, 211, "Dark Violet", "148, 0, 211");
    assoc!(DEEP_PINK, 255, 20, 147, "Deep Pink", "255, 20, 147");
    assoc!(DEEP_SKY_BLUE, 0, 191, 255, "Deep Sky Blue", "0, 191, 255");
    assoc!(DIM_GRAY, 105, 105, 105, "Dim Gray", "105, 105, 105");
    assoc!(DODGER_BLUE, 30, 144, 255, "Dodger Blue", "30, 144, 255");
    assoc!(FIRE_BRICK, 178, 34, 34, "Fire Brick", "178, 34, 34");
    assoc!(FLORAL_WHITE, 255, 250, 240, "Floral White", "255, 250, 240");
    assoc!(FOREST_GREEN, 34, 139, 34, "Forest Green", "34, 139, 34");
    assoc!(FUCHSIA, 255, 0, 255, "Fuchsia", "255, 0, 255");
    assoc!(GAINSBORO, 220, 220, 220, "Gainsboro", "220, 220, 220");
    assoc!(GHOST_WHITE, 248, 248, 255, "Ghost White", "248, 248, 255");
    assoc!(GOLD, 255, 215, 0, "Gold", "255, 215, 0");
    assoc!(GOLDENROD, 218, 165, 32, "Goldenrod", "218, 165, 32");
    assoc!(GRAY, 128, 128, 128, "Gray", "128, 128, 128");
    assoc!(GREEN, 0, 128, 0, "Green", "0, 128, 0");
    assoc!(GREEN_YELLOW, 173, 255, 47, "Green Yellow", "173, 255, 47");
    assoc!(HONEYDEW, 240, 255, 240, "Honeydew", "240, 255, 240");
    assoc!(HOT_PINK, 255, 105, 180, "Hot Pink", "255, 105, 180");
    assoc!(INDIAN_RED, 205, 92, 92, "Indian Red", "205, 92, 92");
    assoc!(INDIGO, 75, 0, 130, "Indigo", "75, 0, 130");
    assoc!(IVORY, 255, 255, 240, "Ivory", "255, 255, 240");
    assoc!(KHAKI, 240, 230, 140, "Khaki", "240, 230, 140");
    assoc!(LAVENDER, 230, 230, 250, "Lavender", "230, 230, 250");
    assoc!(LAVENDER_BLUSH, 255, 240, 245, "Lavender Blush", "255, 240, 245");
    assoc!(LAWN_GREEN, 124, 252, 0, "Lawn Green", "124, 252, 0");
    assoc!(LEMON_CHIFFON, 255, 250, 205, "Lemon Chiffon", "255, 250, 205");
    assoc!(LIGHT_BLUE, 173, 216, 230, "Light Blue", "173, 216, 230");
    assoc!(LIGHT_CORAL, 240, 128, 128, "Light Coral", "240, 128, 128");
    assoc!(LIGHT_CYAN, 224, 255, 255, "Light Cyan", "224, 255, 255");
    assoc!(LIGHT_GOLDENROD_YELLOW, 250, 250, 210, "Light Goldenrod Yellow", "250, 250, 210");
    assoc!(LIGHT_GRAY, 211, 211, 211, "Light Gray", "211, 211, 211");
    assoc!(LIGHT_GREEN, 144, 238, 144, "Light Green", "144, 238, 144");
    assoc!(LIGHT_PINK, 255, 182, 193, "Light Pink", "255, 182, 193");
    assoc!(LIGHT_SALMON, 255, 160, 122, "Light Salmon", "255, 160, 122");
    assoc!(LIGHT_SEA_GREEN, 32, 178, 170, "Light Sea Green", "32, 178, 170");
    assoc!(LIGHT_SKY_BLUE, 135, 206, 250, "Light Sky Blue", "135, 206, 250");
    assoc!(LIGHT_SLATE_GRAY, 119, 136, 153, "Light Slate Gray", "119, 136, 153");
    assoc!(LIGHT_STEEL_BLUE, 176, 196, 222, "Light Steel Blue", "176, 196, 222");
    assoc!(LIGHT_YELLOW, 255, 255, 224, "Light Yellow", "255, 255, 224");
    assoc!(LIME, 0, 255, 0, "Lime", "0, 255, 0");
    assoc!(LIME_GREEN, 50, 205, 50, "Lime Green", "50, 205, 50");
    assoc!(LINEN, 250, 240, 230, "Linen", "250, 240, 230");
    assoc!(MAGENTA, 255, 0, 255, "Magenta", "255, 0, 255");
    assoc!(MAROON, 128, 0, 0, "Maroon", "128, 0, 0");
    assoc!(MEDIUM_AQUAMARINE, 102, 205, 170, "Medium Aquamarine", "102, 205, 170");
    assoc!(MEDIUM_BLUE, 0, 0, 205, "Medium Blue", "0, 0, 205");
    assoc!(MEDIUM_ORCHID, 186, 85, 211, "Medium Orchid", "186, 85, 211");
    assoc!(MEDIUM_PURPLE, 147, 112, 219, "Medium Purple", "147, 112, 219");
    assoc!(MEDIUM_SEA_GREEN, 60, 179, 113, "Medium Sea Green", "60, 179, 113");
    assoc!(MEDIUM_SLATE_BLUE, 123, 104, 238, "Medium Slate Blue", "123, 104, 238");
    assoc!(MEDIUM_SPRING_GREEN, 0, 250, 154, "Medium Spring Green", "0, 250, 154");
    assoc!(MEDIUM_TURQUOISE, 72, 209, 204, "Medium Turquoise", "72, 209, 204");
    assoc!(MEDIUM_VIOLET_RED, 199, 21, 133, "Medium Violet Red", "199, 21, 133");
    assoc!(MIDNIGHT_BLUE, 25, 25, 112, "Midnight Blue", "25, 25, 112");
    assoc!(MINT_CREAM, 245, 255, 250, "Mint Cream", "245, 255, 250");
    assoc!(MISTY_ROSE, 255, 228, 225, "Misty Rose", "255, 228, 225");
    assoc!(MOCCASIN, 255, 228, 181, "Moccasin", "255, 228, 181");
    assoc!(NAVAJO_WHITE, 255, 222, 173, "Navajo White", "255, 222, 173");
    assoc!(NAVY, 0, 0, 128, "Navy", "0, 0, 128");
    assoc!(OLD_LACE, 253, 245, 230, "Old Lace", "253, 245, 230");
    assoc!(OLIVE, 128, 128, 0, "Olive", "128, 128, 0");
    assoc!(OLIVE_DRAB, 107, 142, 35, "Olive Drab", "107, 142, 35");
    assoc!(ORANGE, 255, 165, 0, "Orange", "255, 165, 0");
    assoc!(ORANGE_RED, 255, 69, 0, "Orange Red", "255, 69, 0");
    assoc!(ORCHID, 218, 112, 214, "Orchid", "218, 112, 214");
    assoc!(PALE_GOLDENROD, 238, 232, 170, "Pale Goldenrod", "238, 232, 170");
    assoc!(PALE_GREEN, 152, 251, 152, "Pale Green", "152, 251, 152");
    assoc!(PALE_TURQUOISE, 175, 238, 238, "Pale Turquoise", "175, 238, 238");
    assoc!(PALE_VIOLET_RED, 219, 112, 147, "Pale Violet Red", "219, 112, 147");
    assoc!(PAPAYA_WHIP, 255, 239, 213, "Papaya Whip", "255, 239, 213");
    assoc!(PEACH_PUFF, 255, 218, 185, "Peach Puff", "255, 218, 185");
    assoc!(PERU, 205, 133, 63, "Peru", "205, 133, 63");
    assoc!(PINK, 255, 192, 203, "Pink", "255, 192, 203");
    assoc!(PLUM, 221, 160, 221, "Plum", "221, 160, 221");
    assoc!(POWDER_BLUE, 176, 224, 230, "Powder Blue", "176, 224, 230");
    assoc!(PURPLE, 128, 0, 128, "Purple", "128, 0, 128");
    assoc!(REBECCAPURPLE, 102, 51, 153, "Rebeccapurple", "102, 51, 153");
    assoc!(RED, 255, 0, 0, "Red", "255, 0, 0");
    assoc!(ROSY_BROWN, 188, 143, 143, "Rosy Brown", "188, 143, 143");
    assoc!(ROYAL_BLUE, 65, 105, 225, "Royal Blue", "65, 105, 225");
    assoc!(SADDLE_BROWN, 139, 69, 19, "Saddle Brown", "139, 69, 19");
    assoc!(SALMON, 250, 128, 114, "Salmon", "250, 128, 114");
    assoc!(SANDY_BROWN, 244, 164, 96, "Sandy Brown", "244, 164, 96");
    assoc!(SEA_GREEN, 46, 139, 87, "Sea Green", "46, 139, 87");
    assoc!(SEASHELL, 255, 245, 238, "Seashell", "255, 245, 238");
    assoc!(SIENNA, 160, 82, 45, "Sienna", "160, 82, 45");
    assoc!(SILVER, 192, 192, 192, "Silver", "192, 192, 192");
    assoc!(SKY_BLUE, 135, 206, 235, "Sky Blue", "135, 206, 235");
    assoc!(SLATE_BLUE, 106, 90, 205, "Slate Blue", "106, 90, 205");
    assoc!(SLATE_GRAY, 112, 128, 144, "Slate Gray", "112, 128, 144");
    assoc!(SNOW, 255, 250, 250, "Snow", "255, 250, 250");
    assoc!(SPRING_GREEN, 0, 255, 127, "Spring Green", "0, 255, 127");
    assoc!(STEEL_BLUE, 70, 130, 180, "Steel Blue", "70, 130, 180");
    assoc!(TAN, 210, 180, 140, "Tan", "210, 180, 140");
    assoc!(TEAL, 0, 128, 128, "Teal", "0, 128, 128");
    assoc!(THISTLE, 216, 191, 216, "Thistle", "216, 191, 216");
    assoc!(TOMATO, 255, 99, 71, "Tomato", "255, 99, 71");
    assoc!(TURQUOISE, 64, 224, 208, "Turquoise", "64, 224, 208");
    assoc!(VIOLET, 238, 130, 238, "Violet", "238, 130, 238");
    assoc!(WHEAT, 245, 222, 179, "Wheat", "245, 222, 179");
    assoc!(WHITE, 255, 255, 255, "White", "255, 255, 255");
    assoc!(WHITE_SMOKE, 245, 245, 245, "White Smoke", "245, 245, 245");
    assoc!(YELLOW, 255, 255, 0, "Yellow", "255, 255, 0");
    assoc!(YELLOW_GREEN, 154, 205, 50, "Yellow Green", "154, 205, 50");
}

macro_rules! impl_web_colors {
    ($mod:ident, $container_str:expr, $container:ty) => {
        #[doc = "Named web colors for the"]
        #[doc = $container_str]
        #[doc = "color type."]
        impl WebColor for $container {
            color!(ALICE_BLUE, 240, 248, 255, "Alice Blue", "240, 248, 255");
            color!(ANTIQUE_WHITE, 250, 235, 215, "Antique White", "250, 235, 215");
            color!(AQUA, 0, 255, 255, "Aqua", "0, 255, 255");
            color!(AQUAMARINE, 127, 255, 212, "Aquamarine", "127, 255, 212");
            color!(AZURE, 240, 255, 255, "Azure", "240, 255, 255");
            color!(BEIGE, 245, 245, 220, "Beige", "245, 245, 220");
            color!(BISQUE, 255, 228, 196, "Bisque", "255, 228, 196");
            color!(BLACK, 0, 0, 0, "Black", "0, 0, 0");
            color!(BLANCHED_ALMOND, 255, 235, 205, "Blanched Almond", "255, 235, 205");
            color!(BLUE, 0, 0, 255, "Blue", "0, 0, 255");
            color!(BLUE_VIOLET, 138, 43, 226, "Blue Violet", "138, 43, 226");
            color!(BROWN, 165, 42, 42, "Brown", "165, 42, 42");
            color!(BURLY_WOOD, 222, 184, 135, "Burly Wood", "222, 184, 135");
            color!(CADET_BLUE, 95, 158, 160, "Cadet Blue", "95, 158, 160");
            color!(CHARTREUSE, 127, 255, 0, "Chartreuse", "127, 255, 0");
            color!(CHOCOLATE, 210, 105, 30, "Chocolate", "210, 105, 30");
            color!(CORAL, 255, 127, 80, "Coral", "255, 127, 80");
            color!(CORNFLOWER_BLUE, 100, 149, 237, "Cornflower Blue", "100, 149, 237");
            color!(CORNSILK, 255, 248, 220, "Cornsilk", "255, 248, 220");
            color!(CRIMSON, 220, 20, 60, "Crimson", "220, 20, 60");
            color!(CYAN, 0, 255, 255, "Cyan", "0, 255, 255");
            color!(DARK_BLUE, 0, 0, 139, "Dark Blue", "0, 0, 139");
            color!(DARK_CYAN, 0, 139, 139, "Dark Cyan", "0, 139, 139");
            color!(DARK_GOLDENROD, 184, 134, 11, "Dark Goldenrod", "184, 134, 11");
            color!(DARK_GRAY, 169, 169, 169, "Dark Gray", "169, 169, 169");
            color!(DARK_GREEN, 0, 100, 0, "Dark Green", "0, 100, 0");
            color!(DARK_KHAKI, 189, 183, 107, "Dark Khaki", "189, 183, 107");
            color!(DARK_MAGENTA, 139, 0, 139, "Dark Magenta", "139, 0, 139");
            color!(DARK_OLIVE_GREEN, 85, 107, 47, "Dark Olive Green", "85, 107, 47");
            color!(DARK_ORANGE, 255, 140, 0, "Dark Orange", "255, 140, 0");
            color!(DARK_ORCHID, 153, 50, 204, "Dark Orchid", "153, 50, 204");
            color!(DARK_RED, 139, 0, 0, "Dark Red", "139, 0, 0");
            color!(DARK_SALMON, 233, 150, 122, "Dark Salmon", "233, 150, 122");
            color!(DARK_SEA_GREEN, 143, 188, 143, "Dark Sea Green", "143, 188, 143");
            color!(DARK_SLATE_BLUE, 72, 61, 139, "Dark Slate Blue", "72, 61, 139");
            color!(DARK_SLATE_GRAY, 47, 79, 79, "Dark Slate Gray", "47, 79, 79");
            color!(DARK_TURQUOISE, 0, 206, 209, "Dark Turquoise", "0, 206, 209");
            color!(DARK_VIOLET, 148, 0, 211, "Dark Violet", "148, 0, 211");
            color!(DEEP_PINK, 255, 20, 147, "Deep Pink", "255, 20, 147");
            color!(DEEP_SKY_BLUE, 0, 191, 255, "Deep Sky Blue", "0, 191, 255");
            color!(DIM_GRAY, 105, 105, 105, "Dim Gray", "105, 105, 105");
            color!(DODGER_BLUE, 30, 144, 255, "Dodger Blue", "30, 144, 255");
            color!(FIRE_BRICK, 178, 34, 34, "Fire Brick", "178, 34, 34");
            color!(FLORAL_WHITE, 255, 250, 240, "Floral White", "255, 250, 240");
            color!(FOREST_GREEN, 34, 139, 34, "Forest Green", "34, 139, 34");
            color!(FUCHSIA, 255, 0, 255, "Fuchsia", "255, 0, 255");
            color!(GAINSBORO, 220, 220, 220, "Gainsboro", "220, 220, 220");
            color!(GHOST_WHITE, 248, 248, 255, "Ghost White", "248, 248, 255");
            color!(GOLD, 255, 215, 0, "Gold", "255, 215, 0");
            color!(GOLDENROD, 218, 165, 32, "Goldenrod", "218, 165, 32");
            color!(GRAY, 128, 128, 128, "Gray", "128, 128, 128");
            color!(GREEN, 0, 128, 0, "Green", "0, 128, 0");
            color!(GREEN_YELLOW, 173, 255, 47, "Green Yellow", "173, 255, 47");
            color!(HONEYDEW, 240, 255, 240, "Honeydew", "240, 255, 240");
            color!(HOT_PINK, 255, 105, 180, "Hot Pink", "255, 105, 180");
            color!(INDIAN_RED, 205, 92, 92, "Indian Red", "205, 92, 92");
            color!(INDIGO, 75, 0, 130, "Indigo", "75, 0, 130");
            color!(IVORY, 255, 255, 240, "Ivory", "255, 255, 240");
            color!(KHAKI, 240, 230, 140, "Khaki", "240, 230, 140");
            color!(LAVENDER, 230, 230, 250, "Lavender", "230, 230, 250");
            color!(LAVENDER_BLUSH, 255, 240, 245, "Lavender Blush", "255, 240, 245");
            color!(LAWN_GREEN, 124, 252, 0, "Lawn Green", "124, 252, 0");
            color!(LEMON_CHIFFON, 255, 250, 205, "Lemon Chiffon", "255, 250, 205");
            color!(LIGHT_BLUE, 173, 216, 230, "Light Blue", "173, 216, 230");
            color!(LIGHT_CORAL, 240, 128, 128, "Light Coral", "240, 128, 128");
            color!(LIGHT_CYAN, 224, 255, 255, "Light Cyan", "224, 255, 255");
            color!(LIGHT_GOLDENROD_YELLOW, 250, 250, 210, "Light Goldenrod Yellow", "250, 250, 210");
            color!(LIGHT_GRAY, 211, 211, 211, "Light Gray", "211, 211, 211");
            color!(LIGHT_GREEN, 144, 238, 144, "Light Green", "144, 238, 144");
            color!(LIGHT_PINK, 255, 182, 193, "Light Pink", "255, 182, 193");
            color!(LIGHT_SALMON, 255, 160, 122, "Light Salmon", "255, 160, 122");
            color!(LIGHT_SEA_GREEN, 32, 178, 170, "Light Sea Green", "32, 178, 170");
            color!(LIGHT_SKY_BLUE, 135, 206, 250, "Light Sky Blue", "135, 206, 250");
            color!(LIGHT_SLATE_GRAY, 119, 136, 153, "Light Slate Gray", "119, 136, 153");
            color!(LIGHT_STEEL_BLUE, 176, 196, 222, "Light Steel Blue", "176, 196, 222");
            color!(LIGHT_YELLOW, 255, 255, 224, "Light Yellow", "255, 255, 224");
            color!(LIME, 0, 255, 0, "Lime", "0, 255, 0");
            color!(LIME_GREEN, 50, 205, 50, "Lime Green", "50, 205, 50");
            color!(LINEN, 250, 240, 230, "Linen", "250, 240, 230");
            color!(MAGENTA, 255, 0, 255, "Magenta", "255, 0, 255");
            color!(MAROON, 128, 0, 0, "Maroon", "128, 0, 0");
            color!(MEDIUM_AQUAMARINE, 102, 205, 170, "Medium Aquamarine", "102, 205, 170");
            color!(MEDIUM_BLUE, 0, 0, 205, "Medium Blue", "0, 0, 205");
            color!(MEDIUM_ORCHID, 186, 85, 211, "Medium Orchid", "186, 85, 211");
            color!(MEDIUM_PURPLE, 147, 112, 219, "Medium Purple", "147, 112, 219");
            color!(MEDIUM_SEA_GREEN, 60, 179, 113, "Medium Sea Green", "60, 179, 113");
            color!(MEDIUM_SLATE_BLUE, 123, 104, 238, "Medium Slate Blue", "123, 104, 238");
            color!(MEDIUM_SPRING_GREEN, 0, 250, 154, "Medium Spring Green", "0, 250, 154");
            color!(MEDIUM_TURQUOISE, 72, 209, 204, "Medium Turquoise", "72, 209, 204");
            color!(MEDIUM_VIOLET_RED, 199, 21, 133, "Medium Violet Red", "199, 21, 133");
            color!(MIDNIGHT_BLUE, 25, 25, 112, "Midnight Blue", "25, 25, 112");
            color!(MINT_CREAM, 245, 255, 250, "Mint Cream", "245, 255, 250");
            color!(MISTY_ROSE, 255, 228, 225, "Misty Rose", "255, 228, 225");
            color!(MOCCASIN, 255, 228, 181, "Moccasin", "255, 228, 181");
            color!(NAVAJO_WHITE, 255, 222, 173, "Navajo White", "255, 222, 173");
            color!(NAVY, 0, 0, 128, "Navy", "0, 0, 128");
            color!(OLD_LACE, 253, 245, 230, "Old Lace", "253, 245, 230");
            color!(OLIVE, 128, 128, 0, "Olive", "128, 128, 0");
            color!(OLIVE_DRAB, 107, 142, 35, "Olive Drab", "107, 142, 35");
            color!(ORANGE, 255, 165, 0, "Orange", "255, 165, 0");
            color!(ORANGE_RED, 255, 69, 0, "Orange Red", "255, 69, 0");
            color!(ORCHID, 218, 112, 214, "Orchid", "218, 112, 214");
            color!(PALE_GOLDENROD, 238, 232, 170, "Pale Goldenrod", "238, 232, 170");
            color!(PALE_GREEN, 152, 251, 152, "Pale Green", "152, 251, 152");
            color!(PALE_TURQUOISE, 175, 238, 238, "Pale Turquoise", "175, 238, 238");
            color!(PALE_VIOLET_RED, 219, 112, 147, "Pale Violet Red", "219, 112, 147");
            color!(PAPAYA_WHIP, 255, 239, 213, "Papaya Whip", "255, 239, 213");
            color!(PEACH_PUFF, 255, 218, 185, "Peach Puff", "255, 218, 185");
            color!(PERU, 205, 133, 63, "Peru", "205, 133, 63");
            color!(PINK, 255, 192, 203, "Pink", "255, 192, 203");
            color!(PLUM, 221, 160, 221, "Plum", "221, 160, 221");
            color!(POWDER_BLUE, 176, 224, 230, "Powder Blue", "176, 224, 230");
            color!(PURPLE, 128, 0, 128, "Purple", "128, 0, 128");
            color!(REBECCAPURPLE, 102, 51, 153, "Rebeccapurple", "102, 51, 153");
            color!(RED, 255, 0, 0, "Red", "255, 0, 0");
            color!(ROSY_BROWN, 188, 143, 143, "Rosy Brown", "188, 143, 143");
            color!(ROYAL_BLUE, 65, 105, 225, "Royal Blue", "65, 105, 225");
            color!(SADDLE_BROWN, 139, 69, 19, "Saddle Brown", "139, 69, 19");
            color!(SALMON, 250, 128, 114, "Salmon", "250, 128, 114");
            color!(SANDY_BROWN, 244, 164, 96, "Sandy Brown", "244, 164, 96");
            color!(SEA_GREEN, 46, 139, 87, "Sea Green", "46, 139, 87");
            color!(SEASHELL, 255, 245, 238, "Seashell", "255, 245, 238");
            color!(SIENNA, 160, 82, 45, "Sienna", "160, 82, 45");
            color!(SILVER, 192, 192, 192, "Silver", "192, 192, 192");
            color!(SKY_BLUE, 135, 206, 235, "Sky Blue", "135, 206, 235");
            color!(SLATE_BLUE, 106, 90, 205, "Slate Blue", "106, 90, 205");
            color!(SLATE_GRAY, 112, 128, 144, "Slate Gray", "112, 128, 144");
            color!(SNOW, 255, 250, 250, "Snow", "255, 250, 250");
            color!(SPRING_GREEN, 0, 255, 127, "Spring Green", "0, 255, 127");
            color!(STEEL_BLUE, 70, 130, 180, "Steel Blue", "70, 130, 180");
            color!(TAN, 210, 180, 140, "Tan", "210, 180, 140");
            color!(TEAL, 0, 128, 128, "Teal", "0, 128, 128");
            color!(THISTLE, 216, 191, 216, "Thistle", "216, 191, 216");
            color!(TOMATO, 255, 99, 71, "Tomato", "255, 99, 71");
            color!(TURQUOISE, 64, 224, 208, "Turquoise", "64, 224, 208");
            color!(VIOLET, 238, 130, 238, "Violet", "238, 130, 238");
            color!(WHEAT, 245, 222, 179, "Wheat", "245, 222, 179");
            color!(WHITE, 255, 255, 255, "White", "255, 255, 255");
            color!(WHITE_SMOKE, 245, 245, 245, "White Smoke", "245, 245, 245");
            color!(YELLOW, 255, 255, 0, "Yellow", "255, 255, 0");
            color!(YELLOW_GREEN, 154, 205, 50, "Yellow Green", "154, 205, 50");
        }
    };
}

impl_web_colors!(rgb555, "Rgb555", crate::pixelcolor::Rgb555);
impl_web_colors!(bgr555, "Bgr555", crate::pixelcolor::Bgr555);
impl_web_colors!(rgb565, "Rgb565", crate::pixelcolor::Rgb565);
impl_web_colors!(bgr565, "Bgr565", crate::pixelcolor::Bgr565);
impl_web_colors!(rgb888, "Rgb888", crate::pixelcolor::Rgb888);
impl_web_colors!(bgr888, "Bgr888", crate::pixelcolor::Bgr888);
