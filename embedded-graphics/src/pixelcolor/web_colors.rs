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
    ($ident:ident, $r:expr, $g:expr, $b:expr, $container:ty, $name:expr,  $value:expr) => {

        #[doc = "<span style=\"background:rgb("]
        #[doc = $value]
        #[doc = ");border-radius:3px;display:inline-block;width:0.9em;height:0.9em\"></span>"]
        #[doc = $name]
        pub const $ident: $container = <$container>::new($r, $g, $b);
    };
}

macro_rules! impl_web_colors {
    ($mod:ident, $container_str:expr, $container:ty) => {
        #[doc = "Named web colors for the"]
        #[doc = $container_str]
        #[doc = "color type."]
        pub mod $mod {
            color!(CSS_ALICE_BLUE, 240, 248, 255, $container, "Alice Blue", "240, 248, 255");
            color!(CSS_ANTIQUE_WHITE, 250, 235, 215, $container, "Antique White", "250, 235, 215");
            color!(CSS_AQUA, 0, 255, 255, $container, "Aqua", "0, 255, 255");
            color!(CSS_AQUAMARINE, 127, 255, 212, $container, "Aquamarine", "127, 255, 212");
            color!(CSS_AZURE, 240, 255, 255, $container, "Azure", "240, 255, 255");
            color!(CSS_BEIGE, 245, 245, 220, $container, "Beige", "245, 245, 220");
            color!(CSS_BISQUE, 255, 228, 196, $container, "Bisque", "255, 228, 196");
            color!(CSS_BLACK, 0, 0, 0, $container, "Black", "0, 0, 0");
            color!(CSS_BLANCHED_ALMOND, 255, 235, 205, $container, "Blanched Almond", "255, 235, 205");
            color!(CSS_BLUE, 0, 0, 255, $container, "Blue", "0, 0, 255");
            color!(CSS_BLUE_VIOLET, 138, 43, 226, $container, "Blue Violet", "138, 43, 226");
            color!(CSS_BROWN, 165, 42, 42, $container, "Brown", "165, 42, 42");
            color!(CSS_BURLY_WOOD, 222, 184, 135, $container, "Burly Wood", "222, 184, 135");
            color!(CSS_CADET_BLUE, 95, 158, 160, $container, "Cadet Blue", "95, 158, 160");
            color!(CSS_CHARTREUSE, 127, 255, 0, $container, "Chartreuse", "127, 255, 0");
            color!(CSS_CHOCOLATE, 210, 105, 30, $container, "Chocolate", "210, 105, 30");
            color!(CSS_CORAL, 255, 127, 80, $container, "Coral", "255, 127, 80");
            color!(CSS_CORNFLOWER_BLUE, 100, 149, 237, $container, "Cornflower Blue", "100, 149, 237");
            color!(CSS_CORNSILK, 255, 248, 220, $container, "Cornsilk", "255, 248, 220");
            color!(CSS_CRIMSON, 220, 20, 60, $container, "Crimson", "220, 20, 60");
            color!(CSS_CYAN, 0, 255, 255, $container, "Cyan", "0, 255, 255");
            color!(CSS_DARK_BLUE, 0, 0, 139, $container, "Dark Blue", "0, 0, 139");
            color!(CSS_DARK_CYAN, 0, 139, 139, $container, "Dark Cyan", "0, 139, 139");
            color!(CSS_DARK_GOLDENROD, 184, 134, 11, $container, "Dark Goldenrod", "184, 134, 11");
            color!(CSS_DARK_GRAY, 169, 169, 169, $container, "Dark Gray", "169, 169, 169");
            color!(CSS_DARK_GREEN, 0, 100, 0, $container, "Dark Green", "0, 100, 0");
            color!(CSS_DARK_KHAKI, 189, 183, 107, $container, "Dark Khaki", "189, 183, 107");
            color!(CSS_DARK_MAGENTA, 139, 0, 139, $container, "Dark Magenta", "139, 0, 139");
            color!(CSS_DARK_OLIVE_GREEN, 85, 107, 47, $container, "Dark Olive Green", "85, 107, 47");
            color!(CSS_DARK_ORANGE, 255, 140, 0, $container, "Dark Orange", "255, 140, 0");
            color!(CSS_DARK_ORCHID, 153, 50, 204, $container, "Dark Orchid", "153, 50, 204");
            color!(CSS_DARK_RED, 139, 0, 0, $container, "Dark Red", "139, 0, 0");
            color!(CSS_DARK_SALMON, 233, 150, 122, $container, "Dark Salmon", "233, 150, 122");
            color!(CSS_DARK_SEA_GREEN, 143, 188, 143, $container, "Dark Sea Green", "143, 188, 143");
            color!(CSS_DARK_SLATE_BLUE, 72, 61, 139, $container, "Dark Slate Blue", "72, 61, 139");
            color!(CSS_DARK_SLATE_GRAY, 47, 79, 79, $container, "Dark Slate Gray", "47, 79, 79");
            color!(CSS_DARK_TURQUOISE, 0, 206, 209, $container, "Dark Turquoise", "0, 206, 209");
            color!(CSS_DARK_VIOLET, 148, 0, 211, $container, "Dark Violet", "148, 0, 211");
            color!(CSS_DEEP_PINK, 255, 20, 147, $container, "Deep Pink", "255, 20, 147");
            color!(CSS_DEEP_SKY_BLUE, 0, 191, 255, $container, "Deep Sky Blue", "0, 191, 255");
            color!(CSS_DIM_GRAY, 105, 105, 105, $container, "Dim Gray", "105, 105, 105");
            color!(CSS_DODGER_BLUE, 30, 144, 255, $container, "Dodger Blue", "30, 144, 255");
            color!(CSS_FIRE_BRICK, 178, 34, 34, $container, "Fire Brick", "178, 34, 34");
            color!(CSS_FLORAL_WHITE, 255, 250, 240, $container, "Floral White", "255, 250, 240");
            color!(CSS_FOREST_GREEN, 34, 139, 34, $container, "Forest Green", "34, 139, 34");
            color!(CSS_FUCHSIA, 255, 0, 255, $container, "Fuchsia", "255, 0, 255");
            color!(CSS_GAINSBORO, 220, 220, 220, $container, "Gainsboro", "220, 220, 220");
            color!(CSS_GHOST_WHITE, 248, 248, 255, $container, "Ghost White", "248, 248, 255");
            color!(CSS_GOLD, 255, 215, 0, $container, "Gold", "255, 215, 0");
            color!(CSS_GOLDENROD, 218, 165, 32, $container, "Goldenrod", "218, 165, 32");
            color!(CSS_GRAY, 128, 128, 128, $container, "Gray", "128, 128, 128");
            color!(CSS_GREEN, 0, 128, 0, $container, "Green", "0, 128, 0");
            color!(CSS_GREEN_YELLOW, 173, 255, 47, $container, "Green Yellow", "173, 255, 47");
            color!(CSS_HONEYDEW, 240, 255, 240, $container, "Honeydew", "240, 255, 240");
            color!(CSS_HOT_PINK, 255, 105, 180, $container, "Hot Pink", "255, 105, 180");
            color!(CSS_INDIAN_RED, 205, 92, 92, $container, "Indian Red", "205, 92, 92");
            color!(CSS_INDIGO, 75, 0, 130, $container, "Indigo", "75, 0, 130");
            color!(CSS_IVORY, 255, 255, 240, $container, "Ivory", "255, 255, 240");
            color!(CSS_KHAKI, 240, 230, 140, $container, "Khaki", "240, 230, 140");
            color!(CSS_LAVENDER, 230, 230, 250, $container, "Lavender", "230, 230, 250");
            color!(CSS_LAVENDER_BLUSH, 255, 240, 245, $container, "Lavender Blush", "255, 240, 245");
            color!(CSS_LAWN_GREEN, 124, 252, 0, $container, "Lawn Green", "124, 252, 0");
            color!(CSS_LEMON_CHIFFON, 255, 250, 205, $container, "Lemon Chiffon", "255, 250, 205");
            color!(CSS_LIGHT_BLUE, 173, 216, 230, $container, "Light Blue", "173, 216, 230");
            color!(CSS_LIGHT_CORAL, 240, 128, 128, $container, "Light Coral", "240, 128, 128");
            color!(CSS_LIGHT_CYAN, 224, 255, 255, $container, "Light Cyan", "224, 255, 255");
            color!(CSS_LIGHT_GOLDENROD_YELLOW, 250, 250, 210, $container, "Light Goldenrod Yellow", "250, 250, 210");
            color!(CSS_LIGHT_GRAY, 211, 211, 211, $container, "Light Gray", "211, 211, 211");
            color!(CSS_LIGHT_GREEN, 144, 238, 144, $container, "Light Green", "144, 238, 144");
            color!(CSS_LIGHT_PINK, 255, 182, 193, $container, "Light Pink", "255, 182, 193");
            color!(CSS_LIGHT_SALMON, 255, 160, 122, $container, "Light Salmon", "255, 160, 122");
            color!(CSS_LIGHT_SEA_GREEN, 32, 178, 170, $container, "Light Sea Green", "32, 178, 170");
            color!(CSS_LIGHT_SKY_BLUE, 135, 206, 250, $container, "Light Sky Blue", "135, 206, 250");
            color!(CSS_LIGHT_SLATE_GRAY, 119, 136, 153, $container, "Light Slate Gray", "119, 136, 153");
            color!(CSS_LIGHT_STEEL_BLUE, 176, 196, 222, $container, "Light Steel Blue", "176, 196, 222");
            color!(CSS_LIGHT_YELLOW, 255, 255, 224, $container, "Light Yellow", "255, 255, 224");
            color!(CSS_LIME, 0, 255, 0, $container, "Lime", "0, 255, 0");
            color!(CSS_LIME_GREEN, 50, 205, 50, $container, "Lime Green", "50, 205, 50");
            color!(CSS_LINEN, 250, 240, 230, $container, "Linen", "250, 240, 230");
            color!(CSS_MAGENTA, 255, 0, 255, $container, "Magenta", "255, 0, 255");
            color!(CSS_MAROON, 128, 0, 0, $container, "Maroon", "128, 0, 0");
            color!(CSS_MEDIUM_AQUAMARINE, 102, 205, 170, $container, "Medium Aquamarine", "102, 205, 170");
            color!(CSS_MEDIUM_BLUE, 0, 0, 205, $container, "Medium Blue", "0, 0, 205");
            color!(CSS_MEDIUM_ORCHID, 186, 85, 211, $container, "Medium Orchid", "186, 85, 211");
            color!(CSS_MEDIUM_PURPLE, 147, 112, 219, $container, "Medium Purple", "147, 112, 219");
            color!(CSS_MEDIUM_SEA_GREEN, 60, 179, 113, $container, "Medium Sea Green", "60, 179, 113");
            color!(CSS_MEDIUM_SLATE_BLUE, 123, 104, 238, $container, "Medium Slate Blue", "123, 104, 238");
            color!(CSS_MEDIUM_SPRING_GREEN, 0, 250, 154, $container, "Medium Spring Green", "0, 250, 154");
            color!(CSS_MEDIUM_TURQUOISE, 72, 209, 204, $container, "Medium Turquoise", "72, 209, 204");
            color!(CSS_MEDIUM_VIOLET_RED, 199, 21, 133, $container, "Medium Violet Red", "199, 21, 133");
            color!(CSS_MIDNIGHT_BLUE, 25, 25, 112, $container, "Midnight Blue", "25, 25, 112");
            color!(CSS_MINT_CREAM, 245, 255, 250, $container, "Mint Cream", "245, 255, 250");
            color!(CSS_MISTY_ROSE, 255, 228, 225, $container, "Misty Rose", "255, 228, 225");
            color!(CSS_MOCCASIN, 255, 228, 181, $container, "Moccasin", "255, 228, 181");
            color!(CSS_NAVAJO_WHITE, 255, 222, 173, $container, "Navajo White", "255, 222, 173");
            color!(CSS_NAVY, 0, 0, 128, $container, "Navy", "0, 0, 128");
            color!(CSS_OLD_LACE, 253, 245, 230, $container, "Old Lace", "253, 245, 230");
            color!(CSS_OLIVE, 128, 128, 0, $container, "Olive", "128, 128, 0");
            color!(CSS_OLIVE_DRAB, 107, 142, 35, $container, "Olive Drab", "107, 142, 35");
            color!(CSS_ORANGE, 255, 165, 0, $container, "Orange", "255, 165, 0");
            color!(CSS_ORANGE_RED, 255, 69, 0, $container, "Orange Red", "255, 69, 0");
            color!(CSS_ORCHID, 218, 112, 214, $container, "Orchid", "218, 112, 214");
            color!(CSS_PALE_GOLDENROD, 238, 232, 170, $container, "Pale Goldenrod", "238, 232, 170");
            color!(CSS_PALE_GREEN, 152, 251, 152, $container, "Pale Green", "152, 251, 152");
            color!(CSS_PALE_TURQUOISE, 175, 238, 238, $container, "Pale Turquoise", "175, 238, 238");
            color!(CSS_PALE_VIOLET_RED, 219, 112, 147, $container, "Pale Violet Red", "219, 112, 147");
            color!(CSS_PAPAYA_WHIP, 255, 239, 213, $container, "Papaya Whip", "255, 239, 213");
            color!(CSS_PEACH_PUFF, 255, 218, 185, $container, "Peach Puff", "255, 218, 185");
            color!(CSS_PERU, 205, 133, 63, $container, "Peru", "205, 133, 63");
            color!(CSS_PINK, 255, 192, 203, $container, "Pink", "255, 192, 203");
            color!(CSS_PLUM, 221, 160, 221, $container, "Plum", "221, 160, 221");
            color!(CSS_POWDER_BLUE, 176, 224, 230, $container, "Powder Blue", "176, 224, 230");
            color!(CSS_PURPLE, 128, 0, 128, $container, "Purple", "128, 0, 128");
            color!(CSS_REBECCAPURPLE, 102, 51, 153, $container, "Rebeccapurple", "102, 51, 153");
            color!(CSS_RED, 255, 0, 0, $container, "Red", "255, 0, 0");
            color!(CSS_ROSY_BROWN, 188, 143, 143, $container, "Rosy Brown", "188, 143, 143");
            color!(CSS_ROYAL_BLUE, 65, 105, 225, $container, "Royal Blue", "65, 105, 225");
            color!(CSS_SADDLE_BROWN, 139, 69, 19, $container, "Saddle Brown", "139, 69, 19");
            color!(CSS_SALMON, 250, 128, 114, $container, "Salmon", "250, 128, 114");
            color!(CSS_SANDY_BROWN, 244, 164, 96, $container, "Sandy Brown", "244, 164, 96");
            color!(CSS_SEA_GREEN, 46, 139, 87, $container, "Sea Green", "46, 139, 87");
            color!(CSS_SEASHELL, 255, 245, 238, $container, "Seashell", "255, 245, 238");
            color!(CSS_SIENNA, 160, 82, 45, $container, "Sienna", "160, 82, 45");
            color!(CSS_SILVER, 192, 192, 192, $container, "Silver", "192, 192, 192");
            color!(CSS_SKY_BLUE, 135, 206, 235, $container, "Sky Blue", "135, 206, 235");
            color!(CSS_SLATE_BLUE, 106, 90, 205, $container, "Slate Blue", "106, 90, 205");
            color!(CSS_SLATE_GRAY, 112, 128, 144, $container, "Slate Gray", "112, 128, 144");
            color!(CSS_SNOW, 255, 250, 250, $container, "Snow", "255, 250, 250");
            color!(CSS_SPRING_GREEN, 0, 255, 127, $container, "Spring Green", "0, 255, 127");
            color!(CSS_STEEL_BLUE, 70, 130, 180, $container, "Steel Blue", "70, 130, 180");
            color!(CSS_TAN, 210, 180, 140, $container, "Tan", "210, 180, 140");
            color!(CSS_TEAL, 0, 128, 128, $container, "Teal", "0, 128, 128");
            color!(CSS_THISTLE, 216, 191, 216, $container, "Thistle", "216, 191, 216");
            color!(CSS_TOMATO, 255, 99, 71, $container, "Tomato", "255, 99, 71");
            color!(CSS_TURQUOISE, 64, 224, 208, $container, "Turquoise", "64, 224, 208");
            color!(CSS_VIOLET, 238, 130, 238, $container, "Violet", "238, 130, 238");
            color!(CSS_WHEAT, 245, 222, 179, $container, "Wheat", "245, 222, 179");
            color!(CSS_WHITE, 255, 255, 255, $container, "White", "255, 255, 255");
            color!(CSS_WHITE_SMOKE, 245, 245, 245, $container, "White Smoke", "245, 245, 245");
            color!(CSS_YELLOW, 255, 255, 0, $container, "Yellow", "255, 255, 0");
            color!(CSS_YELLOW_GREEN, 154, 205, 50, $container, "Yellow Green", "154, 205, 50");
        }
    };
}

impl_web_colors!(rgb555, "Rgb555", crate::pixelcolor::Rgb555);
impl_web_colors!(bgr555, "Bgr555", crate::pixelcolor::Bgr555);
impl_web_colors!(rgb565, "Rgb565", crate::pixelcolor::Rgb565);
impl_web_colors!(bgr565, "Bgr565", crate::pixelcolor::Bgr565);
impl_web_colors!(rgb888, "Rgb888", crate::pixelcolor::Rgb888);
impl_web_colors!(bgr888, "Bgr888", crate::pixelcolor::Bgr888);
