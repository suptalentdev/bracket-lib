use crate::named::*;
use crate::prelude::RGBA;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref PALETTE: Mutex<HashMap<String, RGBA>> = Mutex::new(HashMap::new());
}

/// Register a palette color by name with the global registry.
pub fn register_palette_color<S: ToString, COLOR: Into<RGBA>>(name: S, color: COLOR) {
    PALETTE
        .lock()
        .unwrap()
        .insert(name.to_string(), color.into());
}

/// Retrieve a palette color by name from the global registry.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::needless_pass_by_value)]
pub fn palette_color<S: ToString>(name: &S) -> Option<RGBA> {
    let plock = PALETTE.lock().unwrap();
    if let Some(col) = plock.get(&name.to_string()) {
        Some(*col)
    } else {
        None
    }
}

/// Empties the palette
#[allow(clippy::module_name_repetitions)]
pub fn clear_palette() {
    PALETTE.lock().unwrap().clear();
}

macro_rules! w3c_color_helper {
    ( $( $n:literal, $name:expr ),* ) => {
        let mut plock = PALETTE.lock().unwrap();
        $(
            plock.insert($n.to_string(), RGBA::named($name));
        )*
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn add_check_color() {
        register_palette_color("red", RGB::from_f32(1.0, 0.0, 0.0));
        let red = palette_color(&"red").unwrap();
        assert!(red == RGBA::from_f32(1.0, 0.0, 0.0, 1.0))
    }

    #[test]
    fn clear_test() {
        add_named_colors_to_palette();
        clear_palette();
        assert!(palette_color(&"snow").is_none())
    }

    #[test]
    fn no_such_color() {
        add_named_colors_to_palette();
        assert!(palette_color(&"i like fish").is_none())
    }
}

// --- Below here was generated with a script ---

/// Insert all named W3C colors into the palette
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::too_many_lines)]
pub fn add_named_colors_to_palette() {
    w3c_color_helper!(
        "snow",
        SNOW,
        "ghost_white",
        GHOST_WHITE,
        "ghostwhite",
        GHOSTWHITE,
        "white_smoke",
        WHITE_SMOKE,
        "whitesmoke",
        WHITESMOKE,
        "gainsboro",
        GAINSBORO,
        "floral_white",
        FLORAL_WHITE,
        "floralwhite",
        FLORALWHITE,
        "old_lace",
        OLD_LACE,
        "oldlace",
        OLDLACE,
        "linen",
        LINEN,
        "antique_white",
        ANTIQUE_WHITE,
        "antiquewhite",
        ANTIQUEWHITE,
        "papaya_whip",
        PAPAYA_WHIP,
        "papayawhip",
        PAPAYAWHIP,
        "blanched_almond",
        BLANCHED_ALMOND,
        "blanchedalmond",
        BLANCHEDALMOND,
        "bisque",
        BISQUE,
        "peach_puff",
        PEACH_PUFF,
        "peachpuff",
        PEACHPUFF,
        "navajo_white",
        NAVAJO_WHITE,
        "navajowhite",
        NAVAJOWHITE,
        "moccasin",
        MOCCASIN,
        "cornsilk",
        CORNSILK,
        "ivory",
        IVORY,
        "lemon_chiffon",
        LEMON_CHIFFON,
        "lemonchiffon",
        LEMONCHIFFON,
        "seashell",
        SEASHELL,
        "honeydew",
        HONEYDEW,
        "mint_cream",
        MINT_CREAM,
        "mintcream",
        MINTCREAM,
        "azure",
        AZURE,
        "alice_blue",
        ALICE_BLUE,
        "aliceblue",
        ALICEBLUE,
        "lavender",
        LAVENDER,
        "lavender_blush",
        LAVENDER_BLUSH,
        "lavenderblush",
        LAVENDERBLUSH,
        "misty_rose",
        MISTY_ROSE,
        "mistyrose",
        MISTYROSE,
        "white",
        WHITE,
        "black",
        BLACK,
        "dark_slate",
        DARK_SLATE,
        "darkslategray",
        DARKSLATEGRAY,
        "darkslategrey",
        DARKSLATEGREY,
        "dim_gray",
        DIM_GRAY,
        "dimgray",
        DIMGRAY,
        "dim_grey",
        DIM_GREY,
        "dimgrey",
        DIMGREY,
        "slate_gray",
        SLATE_GRAY,
        "slategray",
        SLATEGRAY,
        "slate_grey",
        SLATE_GREY,
        "slategrey",
        SLATEGREY,
        "light_slate",
        LIGHT_SLATE,
        "lightslategray",
        LIGHTSLATEGRAY,
        "lightslategrey",
        LIGHTSLATEGREY,
        "gray",
        GRAY,
        "grey",
        GREY,
        "x11_gray",
        X11_GRAY,
        "x11gray",
        X11GRAY,
        "x11_grey",
        X11_GREY,
        "x11grey",
        X11GREY,
        "web_gray",
        WEB_GRAY,
        "webgray",
        WEBGRAY,
        "web_grey",
        WEB_GREY,
        "webgrey",
        WEBGREY,
        "light_grey",
        LIGHT_GREY,
        "lightgrey",
        LIGHTGREY,
        "light_gray",
        LIGHT_GRAY,
        "lightgray",
        LIGHTGRAY,
        "midnight_blue",
        MIDNIGHT_BLUE,
        "midnightblue",
        MIDNIGHTBLUE,
        "navy",
        NAVY,
        "navy_blue",
        NAVY_BLUE,
        "navyblue",
        NAVYBLUE,
        "cornflower_blue",
        CORNFLOWER_BLUE,
        "cornflowerblue",
        CORNFLOWERBLUE,
        "darkslateblue",
        DARKSLATEBLUE,
        "slate_blue",
        SLATE_BLUE,
        "slateblue",
        SLATEBLUE,
        "medium_slate",
        MEDIUM_SLATE,
        "mediumslateblue",
        MEDIUMSLATEBLUE,
        "lightslateblue",
        LIGHTSLATEBLUE,
        "medium_blue",
        MEDIUM_BLUE,
        "mediumblue",
        MEDIUMBLUE,
        "royal_blue",
        ROYAL_BLUE,
        "royalblue",
        ROYALBLUE,
        "blue",
        BLUE,
        "dodger_blue",
        DODGER_BLUE,
        "dodgerblue",
        DODGERBLUE,
        "deep_sky",
        DEEP_SKY,
        "deepskyblue",
        DEEPSKYBLUE,
        "sky_blue",
        SKY_BLUE,
        "skyblue",
        SKYBLUE,
        "light_sky",
        LIGHT_SKY,
        "lightskyblue",
        LIGHTSKYBLUE,
        "steel_blue",
        STEEL_BLUE,
        "steelblue",
        STEELBLUE,
        "light_steel",
        LIGHT_STEEL,
        "lightsteelblue",
        LIGHTSTEELBLUE,
        "light_blue",
        LIGHT_BLUE,
        "lightblue",
        LIGHTBLUE,
        "powder_blue",
        POWDER_BLUE,
        "powderblue",
        POWDERBLUE,
        "pale_turquoise",
        PALE_TURQUOISE,
        "paleturquoise",
        PALETURQUOISE,
        "dark_turquoise",
        DARK_TURQUOISE,
        "darkturquoise",
        DARKTURQUOISE,
        "medium_turquoise",
        MEDIUM_TURQUOISE,
        "mediumturquoise",
        MEDIUMTURQUOISE,
        "turquoise",
        TURQUOISE,
        "cyan",
        CYAN,
        "aqua",
        AQUA,
        "light_cyan",
        LIGHT_CYAN,
        "lightcyan",
        LIGHTCYAN,
        "cadet_blue",
        CADET_BLUE,
        "cadetblue",
        CADETBLUE,
        "medium_aquamarine",
        MEDIUM_AQUAMARINE,
        "mediumaquamarine",
        MEDIUMAQUAMARINE,
        "aquamarine",
        AQUAMARINE,
        "dark_green",
        DARK_GREEN,
        "darkgreen",
        DARKGREEN,
        "dark_olive",
        DARK_OLIVE,
        "darkolivegreen",
        DARKOLIVEGREEN,
        "dark_sea",
        DARK_SEA,
        "darkseagreen",
        DARKSEAGREEN,
        "sea_green",
        SEA_GREEN,
        "seagreen",
        SEAGREEN,
        "medium_sea",
        MEDIUM_SEA,
        "mediumseagreen",
        MEDIUMSEAGREEN,
        "light_sea",
        LIGHT_SEA,
        "lightseagreen",
        LIGHTSEAGREEN,
        "pale_green",
        PALE_GREEN,
        "palegreen",
        PALEGREEN,
        "spring_green",
        SPRING_GREEN,
        "springgreen",
        SPRINGGREEN,
        "lawn_green",
        LAWN_GREEN,
        "lawngreen",
        LAWNGREEN,
        "green",
        GREEN,
        "lime",
        LIME,
        "x11_green",
        X11_GREEN,
        "x11green",
        X11GREEN,
        "web_green",
        WEB_GREEN,
        "webgreen",
        WEBGREEN,
        "chartreuse",
        CHARTREUSE,
        "medium_spring",
        MEDIUM_SPRING,
        "mediumspringgreen",
        MEDIUMSPRINGGREEN,
        "green_yellow",
        GREEN_YELLOW,
        "greenyellow",
        GREENYELLOW,
        "lime_green",
        LIME_GREEN,
        "limegreen",
        LIMEGREEN,
        "yellow_green",
        YELLOW_GREEN,
        "yellowgreen",
        YELLOWGREEN,
        "forest_green",
        FOREST_GREEN,
        "forestgreen",
        FORESTGREEN,
        "olive_drab",
        OLIVE_DRAB,
        "olivedrab",
        OLIVEDRAB,
        "dark_khaki",
        DARK_KHAKI,
        "darkkhaki",
        DARKKHAKI,
        "khaki",
        KHAKI,
        "pale_goldenrod",
        PALE_GOLDENROD,
        "palegoldenrod",
        PALEGOLDENROD,
        "light_goldenrod",
        LIGHT_GOLDENROD,
        "lightgoldenrodyellow",
        LIGHTGOLDENRODYELLOW,
        "light_yellow",
        LIGHT_YELLOW,
        "lightyellow",
        LIGHTYELLOW,
        "yellow",
        YELLOW,
        "gold",
        GOLD,
        "lightgoldenrod",
        LIGHTGOLDENROD,
        "goldenrod",
        GOLDENROD,
        "dark_goldenrod",
        DARK_GOLDENROD,
        "darkgoldenrod",
        DARKGOLDENROD,
        "rosy_brown",
        ROSY_BROWN,
        "rosybrown",
        ROSYBROWN,
        "indian_red",
        INDIAN_RED,
        "indianred",
        INDIANRED,
        "saddle_brown",
        SADDLE_BROWN,
        "saddlebrown",
        SADDLEBROWN,
        "sienna",
        SIENNA,
        "peru",
        PERU,
        "burlywood",
        BURLYWOOD,
        "beige",
        BEIGE,
        "wheat",
        WHEAT,
        "sandy_brown",
        SANDY_BROWN,
        "sandybrown",
        SANDYBROWN,
        "tan",
        TAN,
        "chocolate",
        CHOCOLATE,
        "firebrick_34",
        FIREBRICK_34,
        "brown_42",
        BROWN_42,
        "dark_salmon",
        DARK_SALMON,
        "darksalmon",
        DARKSALMON,
        "salmon",
        SALMON,
        "light_salmon",
        LIGHT_SALMON,
        "lightsalmon",
        LIGHTSALMON,
        "orange",
        ORANGE,
        "dark_orange",
        DARK_ORANGE,
        "darkorange",
        DARKORANGE,
        "coral",
        CORAL,
        "light_coral",
        LIGHT_CORAL,
        "lightcoral",
        LIGHTCORAL,
        "tomato",
        TOMATO,
        "orange_red",
        ORANGE_RED,
        "orangered",
        ORANGERED,
        "red",
        RED,
        "hot_pink",
        HOT_PINK,
        "hotpink",
        HOTPINK,
        "deep_pink",
        DEEP_PINK,
        "deeppink",
        DEEPPINK,
        "pink",
        PINK,
        "light_pink",
        LIGHT_PINK,
        "lightpink",
        LIGHTPINK,
        "pale_violet",
        PALE_VIOLET,
        "palevioletred",
        PALEVIOLETRED,
        "maroon",
        MAROON,
        "x11_maroon",
        X11_MAROON,
        "x11maroon",
        X11MAROON,
        "web_maroon",
        WEB_MAROON,
        "webmaroon",
        WEBMAROON,
        "medium_violet",
        MEDIUM_VIOLET,
        "mediumvioletred",
        MEDIUMVIOLETRED,
        "violet_red",
        VIOLET_RED,
        "violetred",
        VIOLETRED,
        "magenta",
        MAGENTA,
        "fuchsia",
        FUCHSIA,
        "violet",
        VIOLET,
        "plum",
        PLUM,
        "orchid",
        ORCHID,
        "medium_orchid",
        MEDIUM_ORCHID,
        "mediumorchid",
        MEDIUMORCHID,
        "dark_orchid",
        DARK_ORCHID,
        "darkorchid",
        DARKORCHID,
        "dark_violet",
        DARK_VIOLET,
        "darkviolet",
        DARKVIOLET,
        "blue_violet",
        BLUE_VIOLET,
        "blueviolet",
        BLUEVIOLET,
        "purple",
        PURPLE,
        "x11_purple",
        X11_PURPLE,
        "x11purple",
        X11PURPLE,
        "web_purple",
        WEB_PURPLE,
        "webpurple",
        WEBPURPLE,
        "medium_purple",
        MEDIUM_PURPLE,
        "mediumpurple",
        MEDIUMPURPLE,
        "thistle",
        THISTLE,
        "snow1",
        SNOW1,
        "snow2",
        SNOW2,
        "snow3",
        SNOW3,
        "snow4",
        SNOW4,
        "seashell1",
        SEASHELL1,
        "seashell2",
        SEASHELL2,
        "seashell3",
        SEASHELL3,
        "seashell4",
        SEASHELL4,
        "antiquewhite1",
        ANTIQUEWHITE1,
        "antiquewhite2",
        ANTIQUEWHITE2,
        "antiquewhite3",
        ANTIQUEWHITE3,
        "antiquewhite4",
        ANTIQUEWHITE4,
        "bisque1",
        BISQUE1,
        "bisque2",
        BISQUE2,
        "bisque3",
        BISQUE3,
        "bisque4",
        BISQUE4,
        "peachpuff1",
        PEACHPUFF1,
        "peachpuff2",
        PEACHPUFF2,
        "peachpuff3",
        PEACHPUFF3,
        "peachpuff4",
        PEACHPUFF4,
        "navajowhite1",
        NAVAJOWHITE1,
        "navajowhite2",
        NAVAJOWHITE2,
        "navajowhite3",
        NAVAJOWHITE3,
        "navajowhite4",
        NAVAJOWHITE4,
        "lemonchiffon1",
        LEMONCHIFFON1,
        "lemonchiffon2",
        LEMONCHIFFON2,
        "lemonchiffon3",
        LEMONCHIFFON3,
        "lemonchiffon4",
        LEMONCHIFFON4,
        "cornsilk1",
        CORNSILK1,
        "cornsilk2",
        CORNSILK2,
        "cornsilk3",
        CORNSILK3,
        "cornsilk4",
        CORNSILK4,
        "ivory1",
        IVORY1,
        "ivory2",
        IVORY2,
        "ivory3",
        IVORY3,
        "ivory4",
        IVORY4,
        "honeydew1",
        HONEYDEW1,
        "honeydew2",
        HONEYDEW2,
        "honeydew3",
        HONEYDEW3,
        "honeydew4",
        HONEYDEW4,
        "lavenderblush1",
        LAVENDERBLUSH1,
        "lavenderblush2",
        LAVENDERBLUSH2,
        "lavenderblush3",
        LAVENDERBLUSH3,
        "lavenderblush4",
        LAVENDERBLUSH4,
        "mistyrose1",
        MISTYROSE1,
        "mistyrose2",
        MISTYROSE2,
        "mistyrose3",
        MISTYROSE3,
        "mistyrose4",
        MISTYROSE4,
        "azure1",
        AZURE1,
        "azure2",
        AZURE2,
        "azure3",
        AZURE3,
        "azure4",
        AZURE4,
        "slateblue1",
        SLATEBLUE1,
        "slateblue2",
        SLATEBLUE2,
        "slateblue3",
        SLATEBLUE3,
        "slateblue4",
        SLATEBLUE4,
        "royalblue1",
        ROYALBLUE1,
        "royalblue2",
        ROYALBLUE2,
        "royalblue3",
        ROYALBLUE3,
        "royalblue4",
        ROYALBLUE4,
        "blue1",
        BLUE1,
        "blue2",
        BLUE2,
        "blue3",
        BLUE3,
        "blue4",
        BLUE4,
        "dodgerblue1",
        DODGERBLUE1,
        "dodgerblue2",
        DODGERBLUE2,
        "dodgerblue3",
        DODGERBLUE3,
        "dodgerblue4",
        DODGERBLUE4,
        "steelblue1",
        STEELBLUE1,
        "steelblue2",
        STEELBLUE2,
        "steelblue3",
        STEELBLUE3,
        "steelblue4",
        STEELBLUE4,
        "deepskyblue1",
        DEEPSKYBLUE1,
        "deepskyblue2",
        DEEPSKYBLUE2,
        "deepskyblue3",
        DEEPSKYBLUE3,
        "deepskyblue4",
        DEEPSKYBLUE4,
        "skyblue1",
        SKYBLUE1,
        "skyblue2",
        SKYBLUE2,
        "skyblue3",
        SKYBLUE3,
        "skyblue4",
        SKYBLUE4,
        "lightskyblue1",
        LIGHTSKYBLUE1,
        "lightskyblue2",
        LIGHTSKYBLUE2,
        "lightskyblue3",
        LIGHTSKYBLUE3,
        "lightskyblue4",
        LIGHTSKYBLUE4,
        "slategray1",
        SLATEGRAY1,
        "slategray2",
        SLATEGRAY2,
        "slategray3",
        SLATEGRAY3,
        "slategray4",
        SLATEGRAY4,
        "lightsteelblue1",
        LIGHTSTEELBLUE1,
        "lightsteelblue2",
        LIGHTSTEELBLUE2,
        "lightsteelblue3",
        LIGHTSTEELBLUE3,
        "lightsteelblue4",
        LIGHTSTEELBLUE4,
        "lightblue1",
        LIGHTBLUE1,
        "lightblue2",
        LIGHTBLUE2,
        "lightblue3",
        LIGHTBLUE3,
        "lightblue4",
        LIGHTBLUE4,
        "lightcyan1",
        LIGHTCYAN1,
        "lightcyan2",
        LIGHTCYAN2,
        "lightcyan3",
        LIGHTCYAN3,
        "lightcyan4",
        LIGHTCYAN4,
        "paleturquoise1",
        PALETURQUOISE1,
        "paleturquoise2",
        PALETURQUOISE2,
        "paleturquoise3",
        PALETURQUOISE3,
        "paleturquoise4",
        PALETURQUOISE4,
        "cadetblue1",
        CADETBLUE1,
        "cadetblue2",
        CADETBLUE2,
        "cadetblue3",
        CADETBLUE3,
        "cadetblue4",
        CADETBLUE4,
        "turquoise1",
        TURQUOISE1,
        "turquoise2",
        TURQUOISE2,
        "turquoise3",
        TURQUOISE3,
        "turquoise4",
        TURQUOISE4,
        "cyan1",
        CYAN1,
        "cyan2",
        CYAN2,
        "cyan3",
        CYAN3,
        "cyan4",
        CYAN4,
        "darkslategray1",
        DARKSLATEGRAY1,
        "darkslategray2",
        DARKSLATEGRAY2,
        "darkslategray3",
        DARKSLATEGRAY3,
        "darkslategray4",
        DARKSLATEGRAY4,
        "aquamarine1",
        AQUAMARINE1,
        "aquamarine2",
        AQUAMARINE2,
        "aquamarine3",
        AQUAMARINE3,
        "aquamarine4",
        AQUAMARINE4,
        "darkseagreen1",
        DARKSEAGREEN1,
        "darkseagreen2",
        DARKSEAGREEN2,
        "darkseagreen3",
        DARKSEAGREEN3,
        "darkseagreen4",
        DARKSEAGREEN4,
        "seagreen1",
        SEAGREEN1,
        "seagreen2",
        SEAGREEN2,
        "seagreen3",
        SEAGREEN3,
        "seagreen4",
        SEAGREEN4,
        "palegreen1",
        PALEGREEN1,
        "palegreen2",
        PALEGREEN2,
        "palegreen3",
        PALEGREEN3,
        "palegreen4",
        PALEGREEN4,
        "springgreen1",
        SPRINGGREEN1,
        "springgreen2",
        SPRINGGREEN2,
        "springgreen3",
        SPRINGGREEN3,
        "springgreen4",
        SPRINGGREEN4,
        "green1",
        GREEN1,
        "green2",
        GREEN2,
        "green3",
        GREEN3,
        "green4",
        GREEN4,
        "chartreuse1",
        CHARTREUSE1,
        "chartreuse2",
        CHARTREUSE2,
        "chartreuse3",
        CHARTREUSE3,
        "chartreuse4",
        CHARTREUSE4,
        "olivedrab1",
        OLIVEDRAB1,
        "olivedrab2",
        OLIVEDRAB2,
        "olivedrab3",
        OLIVEDRAB3,
        "olivedrab4",
        OLIVEDRAB4,
        "darkolivegreen1",
        DARKOLIVEGREEN1,
        "darkolivegreen2",
        DARKOLIVEGREEN2,
        "darkolivegreen3",
        DARKOLIVEGREEN3,
        "darkolivegreen4",
        DARKOLIVEGREEN4,
        "khaki1",
        KHAKI1,
        "khaki2",
        KHAKI2,
        "khaki3",
        KHAKI3,
        "khaki4",
        KHAKI4,
        "lightgoldenrod1",
        LIGHTGOLDENROD1,
        "lightgoldenrod2",
        LIGHTGOLDENROD2,
        "lightgoldenrod3",
        LIGHTGOLDENROD3,
        "lightgoldenrod4",
        LIGHTGOLDENROD4,
        "lightyellow1",
        LIGHTYELLOW1,
        "lightyellow2",
        LIGHTYELLOW2,
        "lightyellow3",
        LIGHTYELLOW3,
        "lightyellow4",
        LIGHTYELLOW4,
        "yellow1",
        YELLOW1,
        "yellow2",
        YELLOW2,
        "yellow3",
        YELLOW3,
        "yellow4",
        YELLOW4,
        "gold1",
        GOLD1,
        "gold2",
        GOLD2,
        "gold3",
        GOLD3,
        "gold4",
        GOLD4,
        "goldenrod1",
        GOLDENROD1,
        "goldenrod2",
        GOLDENROD2,
        "goldenrod3",
        GOLDENROD3,
        "goldenrod4",
        GOLDENROD4,
        "darkgoldenrod1",
        DARKGOLDENROD1,
        "darkgoldenrod2",
        DARKGOLDENROD2,
        "darkgoldenrod3",
        DARKGOLDENROD3,
        "darkgoldenrod4",
        DARKGOLDENROD4,
        "rosybrown1",
        ROSYBROWN1,
        "rosybrown2",
        ROSYBROWN2,
        "rosybrown3",
        ROSYBROWN3,
        "rosybrown4",
        ROSYBROWN4,
        "indianred1",
        INDIANRED1,
        "indianred2",
        INDIANRED2,
        "indianred3",
        INDIANRED3,
        "indianred4",
        INDIANRED4,
        "sienna1",
        SIENNA1,
        "sienna2",
        SIENNA2,
        "sienna3",
        SIENNA3,
        "sienna4",
        SIENNA4,
        "burlywood1",
        BURLYWOOD1,
        "burlywood2",
        BURLYWOOD2,
        "burlywood3",
        BURLYWOOD3,
        "burlywood4",
        BURLYWOOD4,
        "wheat1",
        WHEAT1,
        "wheat2",
        WHEAT2,
        "wheat3",
        WHEAT3,
        "wheat4",
        WHEAT4,
        "tan1",
        TAN1,
        "tan2",
        TAN2,
        "tan3",
        TAN3,
        "tan4",
        TAN4,
        "chocolate1",
        CHOCOLATE1,
        "chocolate2",
        CHOCOLATE2,
        "chocolate3",
        CHOCOLATE3,
        "chocolate4",
        CHOCOLATE4,
        "firebrick1",
        FIREBRICK1,
        "firebrick2",
        FIREBRICK2,
        "firebrick3",
        FIREBRICK3,
        "firebrick4",
        FIREBRICK4,
        "brown1",
        BROWN1,
        "brown2",
        BROWN2,
        "brown3",
        BROWN3,
        "brown4",
        BROWN4,
        "salmon1",
        SALMON1,
        "salmon2",
        SALMON2,
        "salmon3",
        SALMON3,
        "salmon4",
        SALMON4,
        "lightsalmon1",
        LIGHTSALMON1,
        "lightsalmon2",
        LIGHTSALMON2,
        "lightsalmon3",
        LIGHTSALMON3,
        "lightsalmon4",
        LIGHTSALMON4,
        "orange1",
        ORANGE1,
        "orange2",
        ORANGE2,
        "orange3",
        ORANGE3,
        "orange4",
        ORANGE4,
        "darkorange1",
        DARKORANGE1,
        "darkorange2",
        DARKORANGE2,
        "darkorange3",
        DARKORANGE3,
        "darkorange4",
        DARKORANGE4,
        "coral1",
        CORAL1,
        "coral2",
        CORAL2,
        "coral3",
        CORAL3,
        "coral4",
        CORAL4,
        "tomato1",
        TOMATO1,
        "tomato2",
        TOMATO2,
        "tomato3",
        TOMATO3,
        "tomato4",
        TOMATO4,
        "orangered1",
        ORANGERED1,
        "orangered2",
        ORANGERED2,
        "orangered3",
        ORANGERED3,
        "orangered4",
        ORANGERED4,
        "red1",
        RED1,
        "red2",
        RED2,
        "red3",
        RED3,
        "red4",
        RED4,
        "deeppink1",
        DEEPPINK1,
        "deeppink2",
        DEEPPINK2,
        "deeppink3",
        DEEPPINK3,
        "deeppink4",
        DEEPPINK4,
        "hotpink1",
        HOTPINK1,
        "hotpink2",
        HOTPINK2,
        "hotpink3",
        HOTPINK3,
        "hotpink4",
        HOTPINK4,
        "pink1",
        PINK1,
        "pink2",
        PINK2,
        "pink3",
        PINK3,
        "pink4",
        PINK4,
        "lightpink1",
        LIGHTPINK1,
        "lightpink2",
        LIGHTPINK2,
        "lightpink3",
        LIGHTPINK3,
        "lightpink4",
        LIGHTPINK4,
        "palevioletred1",
        PALEVIOLETRED1,
        "palevioletred2",
        PALEVIOLETRED2,
        "palevioletred3",
        PALEVIOLETRED3,
        "palevioletred4",
        PALEVIOLETRED4,
        "maroon1",
        MAROON1,
        "maroon2",
        MAROON2,
        "maroon3",
        MAROON3,
        "maroon4",
        MAROON4,
        "violetred1",
        VIOLETRED1,
        "violetred2",
        VIOLETRED2,
        "violetred3",
        VIOLETRED3,
        "violetred4",
        VIOLETRED4,
        "magenta1",
        MAGENTA1,
        "magenta2",
        MAGENTA2,
        "magenta3",
        MAGENTA3,
        "magenta4",
        MAGENTA4,
        "orchid1",
        ORCHID1,
        "orchid2",
        ORCHID2,
        "orchid3",
        ORCHID3,
        "orchid4",
        ORCHID4,
        "plum1",
        PLUM1,
        "plum2",
        PLUM2,
        "plum3",
        PLUM3,
        "plum4",
        PLUM4,
        "mediumorchid1",
        MEDIUMORCHID1,
        "mediumorchid2",
        MEDIUMORCHID2,
        "mediumorchid3",
        MEDIUMORCHID3,
        "mediumorchid4",
        MEDIUMORCHID4,
        "darkorchid1",
        DARKORCHID1,
        "darkorchid2",
        DARKORCHID2,
        "darkorchid3",
        DARKORCHID3,
        "darkorchid4",
        DARKORCHID4,
        "purple1",
        PURPLE1,
        "purple2",
        PURPLE2,
        "purple3",
        PURPLE3,
        "purple4",
        PURPLE4,
        "mediumpurple1",
        MEDIUMPURPLE1,
        "mediumpurple2",
        MEDIUMPURPLE2,
        "mediumpurple3",
        MEDIUMPURPLE3,
        "mediumpurple4",
        MEDIUMPURPLE4,
        "thistle1",
        THISTLE1,
        "thistle2",
        THISTLE2,
        "thistle3",
        THISTLE3,
        "thistle4",
        THISTLE4,
        "gray0",
        GRAY0,
        "grey0",
        GREY0,
        "gray1",
        GRAY1,
        "grey1",
        GREY1,
        "gray2",
        GRAY2,
        "grey2",
        GREY2,
        "gray3",
        GRAY3,
        "grey3",
        GREY3,
        "gray4",
        GRAY4,
        "grey4",
        GREY4,
        "gray5",
        GRAY5,
        "grey5",
        GREY5,
        "gray6",
        GRAY6,
        "grey6",
        GREY6,
        "gray7",
        GRAY7,
        "grey7",
        GREY7,
        "gray8",
        GRAY8,
        "grey8",
        GREY8,
        "gray9",
        GRAY9,
        "grey9",
        GREY9,
        "gray10",
        GRAY10,
        "grey10",
        GREY10,
        "gray11",
        GRAY11,
        "grey11",
        GREY11,
        "gray12",
        GRAY12,
        "grey12",
        GREY12,
        "gray13",
        GRAY13,
        "grey13",
        GREY13,
        "gray14",
        GRAY14,
        "grey14",
        GREY14,
        "gray15",
        GRAY15,
        "grey15",
        GREY15,
        "gray16",
        GRAY16,
        "grey16",
        GREY16,
        "gray17",
        GRAY17,
        "grey17",
        GREY17,
        "gray18",
        GRAY18,
        "grey18",
        GREY18,
        "gray19",
        GRAY19,
        "grey19",
        GREY19,
        "gray20",
        GRAY20,
        "grey20",
        GREY20,
        "gray21",
        GRAY21,
        "grey21",
        GREY21,
        "gray22",
        GRAY22,
        "grey22",
        GREY22,
        "gray23",
        GRAY23,
        "grey23",
        GREY23,
        "gray24",
        GRAY24,
        "grey24",
        GREY24,
        "gray25",
        GRAY25,
        "grey25",
        GREY25,
        "gray26",
        GRAY26,
        "grey26",
        GREY26,
        "gray27",
        GRAY27,
        "grey27",
        GREY27,
        "gray28",
        GRAY28,
        "grey28",
        GREY28,
        "gray29",
        GRAY29,
        "grey29",
        GREY29,
        "gray30",
        GRAY30,
        "grey30",
        GREY30,
        "gray31",
        GRAY31,
        "grey31",
        GREY31,
        "gray32",
        GRAY32,
        "grey32",
        GREY32,
        "gray33",
        GRAY33,
        "grey33",
        GREY33,
        "gray34",
        GRAY34,
        "grey34",
        GREY34,
        "gray35",
        GRAY35,
        "grey35",
        GREY35,
        "gray36",
        GRAY36,
        "grey36",
        GREY36,
        "gray37",
        GRAY37,
        "grey37",
        GREY37,
        "gray38",
        GRAY38,
        "grey38",
        GREY38,
        "gray39",
        GRAY39,
        "grey39",
        GREY39,
        "gray40",
        GRAY40,
        "grey40",
        GREY40,
        "gray41",
        GRAY41,
        "grey41",
        GREY41,
        "gray42",
        GRAY42,
        "grey42",
        GREY42,
        "gray43",
        GRAY43,
        "grey43",
        GREY43,
        "gray44",
        GRAY44,
        "grey44",
        GREY44,
        "gray45",
        GRAY45,
        "grey45",
        GREY45,
        "gray46",
        GRAY46,
        "grey46",
        GREY46,
        "gray47",
        GRAY47,
        "grey47",
        GREY47,
        "gray48",
        GRAY48,
        "grey48",
        GREY48,
        "gray49",
        GRAY49,
        "grey49",
        GREY49,
        "gray50",
        GRAY50,
        "grey50",
        GREY50,
        "gray51",
        GRAY51,
        "grey51",
        GREY51,
        "gray52",
        GRAY52,
        "grey52",
        GREY52,
        "gray53",
        GRAY53,
        "grey53",
        GREY53,
        "gray54",
        GRAY54,
        "grey54",
        GREY54,
        "gray55",
        GRAY55,
        "grey55",
        GREY55,
        "gray56",
        GRAY56,
        "grey56",
        GREY56,
        "gray57",
        GRAY57,
        "grey57",
        GREY57,
        "gray58",
        GRAY58,
        "grey58",
        GREY58,
        "gray59",
        GRAY59,
        "grey59",
        GREY59,
        "gray60",
        GRAY60,
        "grey60",
        GREY60,
        "gray61",
        GRAY61,
        "grey61",
        GREY61,
        "gray62",
        GRAY62,
        "grey62",
        GREY62,
        "gray63",
        GRAY63,
        "grey63",
        GREY63,
        "gray64",
        GRAY64,
        "grey64",
        GREY64,
        "gray65",
        GRAY65,
        "grey65",
        GREY65,
        "gray66",
        GRAY66,
        "grey66",
        GREY66,
        "gray67",
        GRAY67,
        "grey67",
        GREY67,
        "gray68",
        GRAY68,
        "grey68",
        GREY68,
        "gray69",
        GRAY69,
        "grey69",
        GREY69,
        "gray70",
        GRAY70,
        "grey70",
        GREY70,
        "gray71",
        GRAY71,
        "grey71",
        GREY71,
        "gray72",
        GRAY72,
        "grey72",
        GREY72,
        "gray73",
        GRAY73,
        "grey73",
        GREY73,
        "gray74",
        GRAY74,
        "grey74",
        GREY74,
        "gray75",
        GRAY75,
        "grey75",
        GREY75,
        "gray76",
        GRAY76,
        "grey76",
        GREY76,
        "gray77",
        GRAY77,
        "grey77",
        GREY77,
        "gray78",
        GRAY78,
        "grey78",
        GREY78,
        "gray79",
        GRAY79,
        "grey79",
        GREY79,
        "gray80",
        GRAY80,
        "grey80",
        GREY80,
        "gray81",
        GRAY81,
        "grey81",
        GREY81,
        "gray82",
        GRAY82,
        "grey82",
        GREY82,
        "gray83",
        GRAY83,
        "grey83",
        GREY83,
        "gray84",
        GRAY84,
        "grey84",
        GREY84,
        "gray85",
        GRAY85,
        "grey85",
        GREY85,
        "gray86",
        GRAY86,
        "grey86",
        GREY86,
        "gray87",
        GRAY87,
        "grey87",
        GREY87,
        "gray88",
        GRAY88,
        "grey88",
        GREY88,
        "gray89",
        GRAY89,
        "grey89",
        GREY89,
        "gray90",
        GRAY90,
        "grey90",
        GREY90,
        "gray91",
        GRAY91,
        "grey91",
        GREY91,
        "gray92",
        GRAY92,
        "grey92",
        GREY92,
        "gray93",
        GRAY93,
        "grey93",
        GREY93,
        "gray94",
        GRAY94,
        "grey94",
        GREY94,
        "gray95",
        GRAY95,
        "grey95",
        GREY95,
        "gray96",
        GRAY96,
        "grey96",
        GREY96,
        "gray97",
        GRAY97,
        "grey97",
        GREY97,
        "gray98",
        GRAY98,
        "grey98",
        GREY98,
        "gray99",
        GRAY99,
        "grey99",
        GREY99,
        "gray100",
        GRAY100,
        "grey100",
        GREY100,
        "dark_grey",
        DARK_GREY,
        "darkgrey",
        DARKGREY,
        "dark_gray",
        DARK_GRAY,
        "darkgray",
        DARKGRAY,
        "dark_blue",
        DARK_BLUE,
        "darkblue",
        DARKBLUE,
        "dark_cyan",
        DARK_CYAN,
        "darkcyan",
        DARKCYAN,
        "dark_magenta",
        DARK_MAGENTA,
        "darkmagenta",
        DARKMAGENTA,
        "dark_red",
        DARK_RED,
        "darkred",
        DARKRED,
        "light_green",
        LIGHT_GREEN,
        "lightgreen",
        LIGHTGREEN,
        "crimson",
        CRIMSON,
        "indigo",
        INDIGO,
        "olive",
        OLIVE,
        "rebecca_purple",
        REBECCA_PURPLE,
        "rebeccapurple",
        REBECCAPURPLE,
        "silver",
        SILVER,
        "teal",
        TEAL
    );
}
