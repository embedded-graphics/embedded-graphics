// GENERATED CODE DO NOT MODIFY!
// Any manual changes to this file will be overwritten!

use crate::{geometry::Size, mono_font::MonoFont};
/// 10x20 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font10x20 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font10x20;

impl MonoFont for Font10x20 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/10x20.raw");
    const FONT_IMAGE_WIDTH: u32 = 160;

    const CHARACTER_SIZE: Size = Size::new(10, 20);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(15);

    const UNDERLINE_OFFSET: i32 = 15 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 4x6 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAAAkCAAAAAAcZ2NOAAAB90lEQVR4nKXQC1YbQQxE0bD/RZP7pB57jCEnJyn9SiW1xvDxK3x+8DUkH21Cqkzmr/j8WMWghU8UaDhToq/xgtuB0KPPfb8RqsJxs4baU9OXHBgQHKFuhso9bqjlEIMeOaIhoMjQYlo+EVTe8hekvmCePnDnYc+c6OutVzWP2uyJbcqNWrfYzkhyFrMlX4bWTuxhXPL42Yzr56Co36hZ1C4w7rbMJyZtrC0rGy7QPlINQxI2Ej2pplfwYhuxabNU+WvYf8OeN6lcdX/NEH8AixuuLSvn6Hc24xMyX5ZhDsYyzhJ8ZIbT48XwIXvg0LQJxgnyq6UVawi/7KlnJntJaVIr0vpolelndrjMmvsfKP8FF91waWu9+M7oM981SQDhbhQp9tXOID9UApe3qSITujVzzWhvcWCtfi22zpoVjJDPeBK/0PhozAcRfyNSTWPI0YRmysFhNv4BHXV1354CF9s6+aTK/etBR6ZyVJqIF0c/mBPjLB2Xq5fduOPy2qydauk+o6RddvhZtFnnBS5lCn9aEzOL80h6hF4a88U7l8dIDX5Gdy7c+R3vqqs/w3A92o/xc6TO601UavzN6B7Z3aAMUY41FzT+NA2Xyme+i4CPaMQI2Pqxx1eveosDa7KFqlJjGalQs7TnsRNBnfGwP6B7i93bvPgNgJ0bQ4gf6zQAAAAASUVORK5CYII=" alt="Font4x6 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font4x6;

impl MonoFont for Font4x6 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/4x6.raw");
    const FONT_IMAGE_WIDTH: u32 = 64;

    const CHARACTER_SIZE: Size = Size::new(4, 6);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(4);

    const UNDERLINE_OFFSET: i32 = 4 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 5x7 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAFAAAAAqCAAAAAABw4PWAAACrUlEQVR4nK2SCXLDMAwDm/8/Ot0FKNlxrplOYRIED9Fy09tPcUfcb/LCk5b0z1gDTmZhiYYhBmS9nBGY9Ir7bVVtO8gqgIAhCqPiwwddQG1VkUVWsBayFAqqZLs4jJtVIInYA2xTZG2Zh4KoCJ+IqUCRii6SiKxgrbnKtFDokEEvCRcGE/6Ive8AlVQl38LdYNOqo6YqvQfHGPQDHDQ4Dk0aHXSolG6JEJ7oAG1YTUF2MCXuNZQ6krSGKjXLDApcFobgmVHZVyxrY4HKhhpnxlPwdkyPhTQda+RUqVleggoISqpzrz1YVcJHHYSDCRPhxD+Ao68xV5K44VlB51uzQTNlqH8LSI2F6HJD4zvKdCgpFpd9CGqcF5OT8isTTZB0DPShlYZSw4nRMIcMatyUmqOwSYrmEDNnJTliTU0BpmKIxnhmRN7FK6URSorpMR/C0d0LuTKucU0GkqqgzACkSkNBuEY2jVAaOfCf8C0sdfehIBpUroTxeCwjCFBFM2AAfaG0YxfCoNWOC+I0nhdqAI4dxEXoJIHrpR0CP4BhLWq6UKxEgYinES89cOERKkYfe5g+xposVO8GKDeEBg6CnKuhyKEoiWS9TM9+mVRshfjh/3DA5L+AV61NW4idLDGxIRcG3v8CCu0dMVjJOoqADkw64YQp8Pn+mGR5aWQVjnroEnhNO4ypji4BWEpd2in9DE4KpQVIqQNSRpgjllImMpAcMsWoVEJIBBaXfSJqB1kFZryKzMMQXDUuaWk4bEhqKJGuT/4CzxE2OPsWr1ufTmw4tBwy4PAL0PiKzCxa+/gb9nNk/aSYiPkkbGIHEWc6FBm4kN6Z0kTqMR9CjhnaeqIA1elFYg7qMR6qKDwpRKF80ILT5HLqigTORVJFpIvpSXHk9gHSwWDFz/g+9Qsr5ntJ1JEU8wAAAABJRU5ErkJggg==" alt="Font5x7 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font5x7;

impl MonoFont for Font5x7 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/5x7.raw");
    const FONT_IMAGE_WIDTH: u32 = 80;

    const CHARACTER_SIZE: Size = Size::new(5, 7);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(5);

    const UNDERLINE_OFFSET: i32 = 5 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 5x8 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAFAAAAAwCAAAAACjjqIrAAAC5ElEQVR4nK2TCXLDMAwDm/8/Ot0FKNk5mnbSwCQIHqLldHr5usP1cqWmB1v8FWveg9lVskHEZcKQ/grXyxpw0tOsAohJecglPVyLP+Bx4RA7zetlvXy1GXUPaquKDFjlMteuh4KogB3pl7SCRyCJ2C2ygiEbarKiQj7dsFKQYj0nkoiu0IOmgWIci5eEV0iiv4+sKJac8DmwMLslfxCuDptWHTVV6QwGg0aYQb/dQQPleFOEBDpUSpeSTDZQOUAbVlOUnUuJew2ljiStoXZOPGYgi8p4GshjhjKmj7WxQGVDjTPjKVjbEY+FNB1r5BRCTpaXoAKCkurcK3NT24SP2tTi8Ipw4jvg7FNQ7gs/Bb5RcymfzOatoPPPkDGUElJLHBil0YxkgFx1S5kOJcXi0i54mi0YDxWYpwtpEOhDKw2lhhOrqR0J7jBEeS0ERHKImbOSOI7C4hI16UjWSPkHynQoKaaPDh3dvRBfRW6JhKKgzIDUatSMek5kBCXR5Y9CT/oYPrkr8Nos9ZaHgmhQeUL26BqSpkCsRiIyeKa0Y48Ex7G4ILYHjEmGNADHDuIidJLA9dIOQT+ZYUZV04ViJQpEPI146YYLj1hhHEWULCxjTRaqdwOUG0IDB0HO1VDkUJREcvMm+mHSyVWDLen8F2z+4j/lw+DGa+cWYidLTGzILwD8Qe5Aob0jBitZRxHQgUknnDAFPp8/dDLfGqnoCVVKMETwNZhOiXR3CcA6ZUjTTelncFIoLUBqQ4HxkMCS1V2HNJ2UoUiI0wgsLhMxXe0sLFEUOYJHUlcqmKBkQGhpkKgxUrWjSdcn/wIHz+Dsj3jR+gXrLSsObpKNV1cYdCQs8dnh9xd2Rqov5vd0qWzqDIqMbs0n1WiJIYgiZXm0cGFmQowR7RL12H5opJDWifQBkjkGWYMiCiLHeQjYftYctU6BcGiBSRhwIirnyHC6cNwnQaJvMUTBOCB1JljxNc6nn+MbMZeTT0clq8oAAAAASUVORK5CYII=" alt="Font5x8 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font5x8;

impl MonoFont for Font5x8 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/5x8.raw");
    const FONT_IMAGE_WIDTH: u32 = 80;

    const CHARACTER_SIZE: Size = Size::new(5, 8);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(6);

    const UNDERLINE_OFFSET: i32 = 6 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 6x10 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAAA8CAAAAAC8v+BoAAADqUlEQVR4nN2S4VobMQwEy/s/NJ3ZtXxOIISvDf3RjTVayTr7Qnj79cPaF7zr3t9MJanoUkcyvqH3tz2XJ3r0ordr+2GXChw843N9egEnc3bsdAQKA2ReG8FnYmNv4Ud9jHsgxWBpPJkRhkoCnZ4Kx6osqhnKPRukaox5tYOPStu4Vx/jnjmakjWqlWdA5kd2DM9YahnlRFOC5bOmJa1xp6O1Xg5HfpV63tJVbPNT+hcX9OuE/dthSFSX50cgUFIa8k48UNVkhgfy+3kCtlHaMqOVNNphYzhJaRtr1A99lCzSAtc19lwEzIbvuEiXIO8idNScgN2F1K44PmREh5aeRgpiMbhkaewxo1AeiVLqvUZ3xfCS9X7KIvBJM6lRGoJFlK7kxAGObQqjy2IsenNpQYsmrF8kYL2ATQJNHmNi/Y08onikr/Zeon9wwfF3xOpJaS/e/jaZI+lxJCo60myU2ZxtFoAsALsAutjOBCLtheTMlCb+IyBFF1Ck8fecQKROGZCjsqjt5IJ+LNjPFpMtGU2nTF9PsSqTx8Ugnjg+1iQH1mP0qNlApPGbjEJdApE6ZZSp92SSJpEFYBdAF9s54lxIzkzJznp1AotfnXjZd0EUdPBEPBQJGrSWD9llG/uz+g8u4A/lLaT80aCpnl3Lx2Sko4AOiieJaEY/8IxH5HnJSq40HYjiWp9U5sZJFlg0N4bJrITK15xW/fk1V7BVkMhsngzu0yiPtOvjZmU+IhtipTVRLE4utzpNtx+M6z66xTCrnR1opWWCS3kA5c+iJ+LjwOUb5Qp22CZRIHrbj46K7ZeKuzj/xYd+FBe8+s1v5eH7hm2WrppfF6rpXXm5a+RGN801urVrDCuafGk6k281TX4RD8lrxEPfbXkCEvTY0ANEdnNHyj0DyaO2Ib2QEurZpVx+HE1Ep43Aik9mWoMoZ7QlzYbtTJMEnbCh6DiB7FDlUyeBanHeDCmhauKkdiZUJy2ElZ86CaL1MGfGh9QwnoiYwBK7f9shE2U72O+KRxKHKFlf6sn2jfJGt+Jrwq/0bP+v9fiCZ9/9VL5IHojTujze8nMx8V11tDT5M/uxIOVSgBsPgbSk/4DMMKQdhAQLQYascBhWvUoS7QxZQXzhY0iLVqXsOI25iHgxTApBZEuQyIkcRFZJwWKlv94aA5dwWWIPgJUYyDRLNZWTRowxSzsOo6VN0ArsGoDESkgnHXIKEHUjOz6zdLnnctZ4rj+8gDc9X/UL/QabrT5qizRq3gAAAABJRU5ErkJggg==" alt="Font6x10 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x10;

impl MonoFont for Font6x10 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x10.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 10);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(7);

    const UNDERLINE_OFFSET: i32 = 7 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 6x12 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAABICAAAAAAsDK+/AAADz0lEQVR4nO2VgXLbMAxD1///6Ow9QLLk1Gm6Le3d7gqTAEhRUlpn69uvL8Y3X3B7u63GZgdGRzE/hWMuO/YLbi7ezISovOfHuL0dAxntBWGJJxf4sIqDNrZfpXEFFo4l/AAfnQ/frQA5vJhWZRSOvQQLxxJ+ouftm0MD00clBjsKdk+FIwqLgV5QIepixDQq58GxF0jbvAO76HukD2V4DdbKrldtbGfVp20GqYqxy0WMwyRuoG7ngBm4iO9a6BoduIAL5gk58wLX3Rfi54Kn4IK+nry1fnMwCNXyzgyPpCF/jMywoYFTk2VbKhii0U6msWM14iT/s0OJpIhKOYUPsa6xZ5BlltHB1ligkgBhKJEEEXdgCMFxcLwdijLioD4sHUglMZRITlY9gVWAR+9z8oL13KUv53QiuciUCLJ8ztD47GG0ApZzKetczXE0KnTowfXlNIbPphNfwHniX/Hw/OCjtZfg54KnmN+ZMAQr08v8NYNaQA4gehxCRUdWzXIW92UyAcENCCxuZyZAjgB8owz8nES8qEUDEsj09zwTIJ0y5Tx8cphOL/Bh0W6IBYC0j5bT11OMSvG4GCCzazx6hC2wRQMSyPQHcxasSwKkU2Y59TEZ0SQTENyAwOJ2ttwDyHOmzAo/AMYCix+deLmfBVDQwZPxsJSkQWv4MKssY78WPxc8xSsuyMscGJ5XPJC3T4XktcNKPauWj5kRRylKQsMSHMS1PvOej5j9MgHKUSIpxug9C7W5MwENVpuTlw7kx3SXK/X+dmDqmTs3Txy6l4lsszsfCqBumQVpyJgoDZ5aPtBpu3vcZw6VfNI5EgwZWj6QDYD/VY7N26/o7B0gVtJkWRvEhBbO1d+Be45jdh+cq5eCuzj/Cy8ovuGCu5fyanD6umG5YtV8yWAxe0uHWyMn2BwTmxk4agwRTF2YnalnzCavnEMwNJAyAutJWA/BCARQF49MyU/lMqOnv8m0qS7ZSdQsN0U6hM7MwzicGgo4I5fTklUzbQ0q0Qk3BZ2M6Mw+dTIkWlwziEjtzBTpEDqzT50MBXwMfgIpvozAehImxzY9BsyD2rHyWR3sn4BtOyiJD/Fk+YR8ojN8nR/j2fo/4z+5IC9i0Y77esO72Yfo5M5B3tkq32EbfYJO7ix0/DvgGm+J8A1eHoItHzMj2UCeGCKic7TMNIwXanNnIhRfMsESRxCU+6VZyoAPprFqTvbRnfM9k0T08gKgG3n8WNIQF8rMTtl5gV8LRF/pYiyJ4bh2TQghVtJkJv7Mhd49A8t9AufhVIt2/OUFn5/9DQ5vN3Yi37XAAAAAAElFTkSuQmCC" alt="Font6x12 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x12;

impl MonoFont for Font6x12 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x12.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 12);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(9);

    const UNDERLINE_OFFSET: i32 = 9 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 6x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAABOCAAAAAD6VUyiAAAEhElEQVR4nO2V3XobNxBD6/d/aPccgH+7lmzViXuRL9gZADMckmtJad/++WF854J3NpkvYc1lx/tbpIiFiIG6j/wc729rIKO9YDA1xocOBa3IZtfU5/jkgtDVo5OAykvAsQ/BwlrCD7CLffNQF5YX06L2EZT6EVhYS/iJsYtbUkwamD4qMdhRcHoqHFFYFHMo9xDDTUw7FRz2RNpmkKoY50UGcastQRFMBYc9kbZ5Rw/FwFgfDHVRWz6x3wGPNT1joOUJOyThXqWoK584OrxdN6r/He40r7h0drHMT+GPuKCfV7hfDgahWh4JDWGxfAMbgqGZYUN+IPxOoJGDIQLIpqxOpnFiN+Ik/mNX8VGbg7gzZjDAEGS5eeVC37yNVk0IgdY1AEOQZd6B1cmkNkUrZY4mgcKU7FKPp4HakuLhCFk2T6SGPEVNTlZxhIiHrzl54Vqmki6bS4JDYd8xHiWJaHLyhvXcpS+7TUWaZVMiyLIRJctNmGNAC7GdK1a9uWxBiyZcP5iE6w8eC4+V+BV4ROkZPlv7LfgTLji+KKweSXvw9cvPHKLHIVR0ZNUsZ3EuExBMQHADApvbmQkQgpRV058tHD+k3IAEMv2dZwJkBeDoBrWdXkANUzQggUx/55kA6ZQpn4810seCq3I3ky15l3TK6espRqXwfjWAHcdjjeRhkTl6eBYAMv1izoJ1SYB0yiynXpMIHmPRgAQy/eZ2ZgJkBZAzw6EwPqJJJiC4AYHN7RzZIGXVPJl1+qOIxcjQ4K4CCjp4Mh6WkjRoDR9mlWXsz+LvBSKf9cDpg3w5tJB8K7BSz6rlc2bEUYqSiAkFc/QDn/mM2S8T0ULTgSCu9clCbZ5MQIPV5uQokRSZ6/TJQm2eTECD1ebkrQP5HN3lSr0fP0y9kqUSgrJ4cuguE9nSrttVoR6ZBWnImCgNnlpe6HS62UkI5cix5BMZE6XBU8sLnabbp05+lH10O8GQYUIb2QDyuevJ+Djo9A4TO2myrA1iQhtHdVt5Hdyzth4eSzGrH8P/ccG3P5qX4OHrhmUGdp3/dQW7NzE6e+SCS3OMLqwaQwQ3RYhg6hWX5n1k1ctc7MDsTL1iNvlN8S4YGkgZgfUkrOcfA4yHKiwSeIgaWTMwOkGb6iGz6rb66WhG6MXLep7MtIaCnNGWrJq2M41IdMJNgHicwJJ96mRItNismjKISO3MBEh7sWSfOhkSFLwIjPvIBvDPmZ0kQBqxZJ86GQpyfE7QD0ZgPRkwgyXTVxPm7JDldrCvgi3JDSric3y1fiJvdAF/N/wpvhz4VfyeC9ZfsszCvf4W+kUcDBkeb/kYTLyKjm72d+Sjg57A9RfR0c1kgzx++/2XUM+yDQYo6T9hZhjCoRee8AIajJaZhuONKHkyEYoPuUtgxSwFLafRzaAjKaT2ZR/dNR9xoR9bNgcaM0RmSULQpK+BiMrJhZ6PBcIrNrAwmZBYroGGMOC0RlRP3lhDLg3PZRrOCNk1IYRIygxmdNbQ4ELvnoHtvoaz5gCvAYNlFr55ASd9OOox/gUr8hGL8VUvvQAAAABJRU5ErkJggg==" alt="Font6x13 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x13;

impl MonoFont for Font6x13 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x13.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 13);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 6x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAABOCAAAAAD6VUyiAAAEhklEQVR4nO2V0XYcNwxDm///6PReQJRm1muv68R9yAkqAiBIaRz7oT/++WZ85QM/uWV9CnvNGxSH5oCegxF1V36Fy5aW4iD58Ng0gcmdX+GypaU4iG/eOXuREEA5SuopGOwJHha+WSRhYBIv6OHqGmifgsGe4OGg18rItoWBQM0HE4p6ux/8t2BTzJLKG8blYryaldonSMwp0hW9dmEJocAsohwl9QSJOW/ge+TOtSJvLdSWr2B3R06th+gtSDkImLnB8BWXxP1cRKj/Cm9ad9yS04x+G/6ID/T3FV5/HJFue3S8kgB+ABfgrdmZVVhqOYMhDp0OBqxhh1sbp4mTKA4i1NZQpzoZYDgII5hGo8OUC/0qjpISqgVxWZLpBYaDJIxjgkO2tcFAEY6SAopbsCOsJRgTSHoZKcytK9JDXO/QSgKrXF+Nvs21zOWNe5tOul0siTwKbPWZc+ziyWRroHd3baOy3gTRYsKWxEFcwHESpkIdKTAii+M0dqxs8BBEzeBw03rI5vBzrfs1+M67+GD0e/AnfIA/F4zULslfevFCGsgFBO8tYSLbj8fFwKd5MBx9r5Wb1JmZNpHt6ucuPQc5TT24eOyNp0R2aDg4u4AeosdwwLVxtW099sZTwh232rcbJJ1oN5Cr04IqbB6Pa7d6bPt0G9m6RjQclJqZHls2wONSIhG+vWxy2azDtOHQUm3rsYub1JmZktBwcPJKgB53BjZO7r7TcpO6qSbyfadsBQ8mjy8GXp7cBQSPgeliinqZASY3vxN/PyDyu6bE1Qf5SxApbz1Y7TssWOXJkIgJBZm6+sjXavKWfQTmROmApgtBpu2vLDqxTIa5UCOfpTqBchQaljTdvrLoxDIZ5kKNfJbqxOhC9mX3ghqWmqWSSIg9euHQowy4Zi/PA6L9LkehJSyRLlo8Wt5g317OTQ1QLrVGIsIO6aLFo+UN9u29JXyu/KwK3SmwZJnQwfoA8fJUvFFo+5ZtnSXYZ0PEhA4uzcPk8/A7c/Xitbv5PvwPH/jyb+Zz4PHzheOK0/P7XHaypQgHg6s+4Jat1Y3dYziYWHgrwsHEwo+4ZY8ru9/mZhcmGb1jMv6B+Vk49bDN9qgekJlAiWw5+sQnsbn+T9/eCRQfuzyg7bFMcxUhicdSCys1K+yBkWxQL9gmgnp0KYCwNZYq6uSMznD4mgAbGdFOAYQ4HksVdbITgDEhUxzAEh4wXhSeAkhPLFXUyQwCXCIUj4FttkfjtdT4nmuCx8A0GC9+ElxJHdBxMB/gxfiG/ETUAck9eItX81/Gb/nA+YccN3hov4T1d6iEoZ/+wXif7h2wBH8Ga7Mi+zKfiDN7jo9md6zNiuxPz9HlO5y6/dkYAK/2HRa6ebp0QMuaVVqSK5wsUE3K5BhUnwUCTGw4UhAxdtW0DDCogxDtYqG7V7cWhwY2fCBVIgicWCHKWQixt2gpdmzgJeVC7w5Hzw1MYyqHBLJKS9aCBuDhJeUD1ia4XIhB1kMRAvR0cJEIVMqFfqbguNdw1yr41LLHDb74AZa5S73Ev2XEKWgJ4V5MAAAAAElFTkSuQmCC" alt="Font6x13Bold font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x13Bold;

impl MonoFont for Font6x13Bold {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x13Bold.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 13);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 6x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAABOCAAAAAD6VUyiAAAEg0lEQVR4nO2UW3bbOBBEk/0v2nNvVQMEZcnxOPF85Eyx69ENEKQlJT9/fDO+9IA37pKfwbWtd7z9rIskhFpIjIz+Cseuxjwg8uONluBFA4EmQa36Gsd64joNjUwexZYA3DGG0z/D289r5drkq/fdcVpsN0ZEELoz8SmY7xXyRo/jbpRGSQzSC0P2IGhxZrqnD3CPcyYtYyzYiTBbSU/hnCpsBnNcbIRCgsMdYjj9e2QsH5G7fDmU6EW7d9KJWGRwbMlcesag7QknkPJerWiqnjgmvF1v1P89vFPecZtczQ7fhb/jAf3Aov12CBjdzlhkjMXqA7ghGHcPKT9DGmU4ilBAlaq+lMGJa5CkQAp66eUIz0wYBQQKVsu7FuYhtQliIxiyHwMIFKzyDqwuhcY07WLU5ii7sJzS4ztwBPEamklVnkjPf9f64lLAoZ5MInuU+eTSC/c+HeI5elgRnI96cDL+yKUX7NddeadMfE0dK6tSoWDVisNqiXIMaCOu5Ipdn1y1YcQQbR6FaPOhs/Dcqd+BR1Re4aO1P4K/4gHHN0U0Y/mqR+/ffvZhZhJGx0TVZTWL6DR7egUKAZd2sggwCqq69HeLJluw2kIEtvKjLgJsF+DoFr0TSEg7DYUIbOVHXQRYd0n1vK6eYMMLzMMAxhIBr2ZupplO8wUTAHcc19GzxjZGZOYAW3krR6GmEGDdJavp906tJVOIwFa+tJNFgO0C6tpTjRnCFIK2EHBpJwdbUNXlqawznyaRoCKjXQU0TMgwGVVCBowmR1llmfi9+P8BQT7swZlFvgpUe5/v39oTZYtbaSoiISK6ytZ3evKVcoBKxQtDN4iutj9V6OWpFDKql0vjVAhW86hCL0+lkFHdEndbYLnKmHuSGSEYWqKK18xPleBuC70l09xOCe1gqhf99fhDQL26kVs6ze2U0A7OkhdmhYeIhMgBbwHe6tWk3pih4pXJ5lJhkidyA+DT9wgCZk5Czuxm6iLDfQIwyRNH+7j0WfCYfedDptvtd+E/ecBXP5tPIWfvJ+wwuPr58YtruDCTY8+F+2h2buyeQBUTDpNi+Yn76HHH7ncAZy7WZPmJPeJHxVsSmGBVDDVD1My/BpSM1FikyAg9tvfcf6aMaZ8qq97WvBLDGLNk1cyVPe1ZEB7hIUw6U2HmGF2ky4sA8zhBhL2aVBaAviaqLlUQUzpZBFhnibBXk4oIGt4DJb1XC/jXrEkIsFYi7NWkIkGOzwnmUQw1w4A9RJi5npJrAqudED8Lbgkv0FEf41frJ/JGN/B3ox/ilxt+F3/mAddfcqXBQ/s1zBdRiyIW59u9Als+h9lYU/0heZkYvcKHiydmY02FLcnf4QcV46mT15wN8KWyZzaBWnWBjoKj2d1sxeGpVCRZ8aYgPbZ64L9kx52xVzMg0SEFe5ms8JDRZQF57rg0MMgITPVKL30pmUbUqoWZPxAhaw6IKEwpLDcgY2xwtwHcrHphbQLXDYxjKOJUIhgVqmx0a9uxamH2nmKHz4DN1AJvgYorLXzxAU9OeoF/AMfkEX2qDT1oAAAAAElFTkSuQmCC" alt="Font6x13Italic font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x13Italic;

impl MonoFont for Font6x13Italic {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x13Italic.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 13);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 6x9 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAAA2CAAAAAAdJMMOAAADTUlEQVR4nN2UUWLbMAxD1/sfunsPkGTZddJsa34GgyBIUZLrdvv49RCfrkX+BWt/Tvr8MA11UUvxiQU4kRT5Fp8faywbclpVSYKRLCNDTXDFPW4v4NV5+VhBKmNBNWkMRe7AwlrCT3Qb96AUSuxAOkmMMFQlwO6pcPCKOZR7QsVOQSVMWTIh+0h92kaQqsg2ZSamsBPDkuAKMJJwS2vjCo6kzQ+CAgrm0YnYyBk3rXnGLW7nn+B+/rb5U+AH50lClXo+jimt4fd+/NCnYL//DpLGvGZkxFbcUIDZ2ZmhM22GmczBhDBnYP9VV0Fbytilrw4ByUg0czABVsJAQSpjDchm9m6wt8PayDRMTDVzgKuAXplQIFB9x6FDQDISkFOwUiWheCp0+MykgdZveloA09iGPwBPeoDHK3+AJ+f/zAXPsL64b4GtVymqpCpJoYRpNFmG1Agx6OL8c+TPZporwdLFBDBlL2rJgZxKzCapjwVXsoohq7KdKoESmBZJHDcc4mPyCmsSnhmAtyUBWoKpGQ0TwEQ/mtKHBBE0SWNUrwRLF2dAo4w1QgTF8eoYCywep1JUSS6D9lPS1yspIW4cOkmD5834Hy7Il+LLqZvHWvgpB5GLZijaEtSPQrD8gAllEkFhJB6gEFSbM1Nk+cLEUGSyT1pKErkBjgQTgkpB3VFPpU6RONZ90gKbRkBzdYHNabC3fjGhQG4gc4cpt5CmDAUxkQNuINHH6mF9HFqPWvf0GbRzgiLoLT+xVSy/AW85tOCH4fw3XlBwwfZpNiu28rDDbclAPesrbHZAHC44yv5yg2UWRmekC1aTL7b+BpqmImNz/N5uGrcr1E32Scd/17R3AjR0qhtSDiaAyQkQy/nkjTSDFAcBKo/NymICmHKmyfAhLdIs8pKR4c9KoHqs1AKsIe2QjZLW8YlehNs25HWf47v1E3yjM749/8uOv8Nxz+GAL7TXF5xmn6KTuwbY7Zf8Fay/iE6edWYu6XdKGv16hc4igp7JMMoC2FKHWfYTYQ/mHyLKqnVksg8tlzEwechJC33mdgbjDMoyDevpKWTiTg8wp7Bg6mosSNViHGqHgHRoISjFWSYsGEKDZV4Bw/CEa11sc8u8hPxML+A3VE65VaGGhCYAAAAASUVORK5CYII=" alt="Font6x9 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x9;

impl MonoFont for Font6x9 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x9.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 9);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(6);

    const UNDERLINE_OFFSET: i32 = 6 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 7x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHAAAABOCAAAAADd+81KAAAFDUlEQVR4nO2U3XrbOAxEt+//0NlzBgBJy0rjfMmmF90xMD8QKdpS2l///DD+5IFv8W+/Sk84sQfjo6FX8fbrWF07+8AW1IaphfaP8hI+PPCNzmEUARIGcApEgeJ3wMXjMmnBg7jWx5IRmtpYIWa+TcJ74OJxmbTRR3GwQkxTGytgWAdaZ35JZDw1MA7W0jrYhjNZqCGImYsJ98glu5E86INwCl+9C1poH4EoUHyLXLLvMLeWaSOCMht0aHkA3/ScZknoHNfgBl5Is1xdSAhdMQ+44EaYjvkPUPfeOPJ2P4S/4sB5wXnQvF+irAKmK2F2UmumPIONjXa1ko0pM1xOanEaA0ZB/SPZAl1wzuKb1o7aaxcVZ0qJFoGl1JahqzRMNvS8o81QiqehMAl0NqT4FCB5SYiuWBncHhhlcR1B1w1ZmSFgBtEowhV8qCX0gJrAx+IQN4gADsDaguQVo33QkgPXSWU5N407qOARoiYkjWQftOSEo73dBBQeWArINoiEZEptsUA49MjeUlQsnB6bRwWD9ZtUp8yRTiM0mBReWjcTvQwsp6G+Cd6t6Lf4cMF34284kLcN8/hlw2hdGrn+QeVtQSYiHiFjSjShFhewcMdUt4xYtAxO6eEmgFI2JfwJdNLaMNpi0bJQV7qRTQCtSgMOsyl83aUO5LcqRouWhbrSjWwCaK8NKZwF8dE7zIG4kNHD+UKYQOUaTtNCA1PyDFQOwOiEO7kxSUIcoXyYk9zvmAEs1JW2sLiGmwDaa0MtNfIEhKQ6Q4wWLQt1pVN6uAmgVWkQWStbSnVFFC0jFi2DQ/YXbuqmbEpEevsSrqJwRQNphvBIHwGI+CSIRFYYkCUCmCUtLmGN4Ufx/4GfRL2nwWMq1AtmqtaLRaItLMHx1m/ForOQHSbQAsoVB7PxVpps6FasNEAo0QLienVQtiePEuhs6FG6tBFNtRiNmQ5qdW96lEBnS6uOpI2gQ2B0m0E9/b0XJuXlIRjYlqb6/xmEORqpVB6MGV3IVhDhXnwwQZxkS1PcO2qBZxbtWjbcKriNNwgRRYxkhzipKJ0C4dAS0K5lwyME4nEIakWL7JDnhGhzhiFbjIKyxQdqI6i3lgRVAniILoVrUvsWucI1+kbZ4gMP+enqp+HB+yYPyUBc+afwJw78+oP8DDzsOPGwg2PEa1jhGG+sYf68bnGZrx0be6Szg2W26z/dYLsLLvObdXu0HXgIgz3c7oI153ER+JJKJ8VvjSTRgMSQD5ooA4wr7OaazEoYwWx4iQECtzBRTDDORDOPNgmWMK8UccCnVvZEDrzdvreiCeUSuxLDNBVtElnZIUMG+bSNyKLjKZpQBLTWXRM2iaykRMSBn7YRWRj57RQ6FxUmCskCxSWbRG+gmx34aRuRA7+3UzWphZGSRINKCZCpI5th5P3h5Y/mJbC16YQD+0O8suaC+qZXMH0e3uClRd+Jbz/w/KWnHzwNvoh5jw8KWbiP/mhY9knMjlP9Q/WTKL+PLPoUZsepdJWWg/mpeBm/E1LMKhSibIqWD4GzFg9dZcEDXcYOGRtNQiwQhiibslclAdlbBAaxBgHTuXeLJXpdpTBE2S5Gao3moHekYbr7hYGhU1jqZXB0ViwDWp40MLCV54+RGVAgDFF0OGvKwmgVF459xHd0wcULeEpwG09QETkXQjBK6SCFFYDUkycNDO5dOP2LyJZQg6NXOv3gqwd60+e7/gb/AhMcD4tQjviZAAAAAElFTkSuQmCC" alt="Font7x13 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font7x13;

impl MonoFont for Font7x13 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/7x13.raw");
    const FONT_IMAGE_WIDTH: u32 = 112;

    const CHARACTER_SIZE: Size = Size::new(7, 13);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 7x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHAAAABOCAAAAADd+81KAAAFCklEQVR4nO3WjVIcNxAE4OT9H9r5ukfaE3uHjSsYV8ppz/TPSFodLFD++68vxm+88Fs9VvTvbzgxhMUqdAjV0kdh82O3MKxoL3SFEG84ZTJ8kw/B3sdmYVgRWPegsBm39uC3kvVq0zvI4mM56YI7esseRrVaYTOMGd7yGll8LCc94C4x9xI6XFmYGcRYAh8oWPMnZGNqoXGjJ2VDF2MRz8cYiBhiut86Ijp4m8TY9At0r6fb5IookzoOyPg9GT0xS+nB5A13iM6ZWkmMSBdWGLFb0SYyOuiwdI7XYMHzEzvk+oAQf2EWQ094M3VOXKPSp2M9fOOMD/dF+CMuXN/hkf2Cg2TWz2Yw0+Gtdix5gnM42G52OthKLEsaRjIex55r6hCztzhn48N6n/BrwMS1x4XfXrjAqkM6snCThaaQfjqxTCncgrWriFutIjpbHoJwMC5cvU4MRaKYZKTVCFjLKklcMoqWlN6gE1u9w9j00OKAUxEhoiL1Q2lYcuA+mVx+8ZgF1keCTpruO9OwpHAkoSMeatCCU601LY2kgahD2p2nVWQyuGRppdi+qqo2gvNBnhEOsrrSJSiy0ik0DJd5uBr1OejTFn8PP1r/dPwJF67XPRIm1StV7j9QeVs6IRGauPVXMqaUNJ2N/ogaEl6r1wKHaBWZXDBqC0S0OuT9CyGilRCHT0kPBcxUnEwCk3TKJBcaXFFtgYhWQhw+JT0UMH1UHUOCDHXKpBfGm8JIF2hFX0OIZJAG22QGGMMEDle0IZCmXveIagtEtBLicEWryOSC6aPqmIpupZtcaOlc3AIRrYQ4XNEqMrlgpuLkim6lm3ohVd8VOESryOSSVlsgotUhIdfjdgLHRsJbsmYBGpt0Q+2wLfOSxCStItrMvy/G/xd+OrxScG10XiyprpQtsXexqz8fkwQ72eEsWtZxOwe2doN6lrU1pNW74kmV9LhB3b632CfUTYqakFaH+OpSrIfoOBMNS8bsHuzz6iZFTag9Fdd2OouVGFK6ZIwzTCjwbDCdXomtUBxqT8XpoZsMP+sDrugoovdFIB00ncXwVov0wVsus+SAKzpyPHZKzoRelN4UmLSScQm2brPkwH4+8STtMZd0PJR+kFaRyUOwdbvhEz0YuI5r8sEnsYuPZDW0RHcoZE9UhIwfCRLEK0P2/Dq8uPBL8Bsu/LXfyCe47Lzx9INz4jVc4ZqfRjGxyzwj870NDrtwTFjFBJc7jWLgMk+4zZ83HpPD3sLGNbzME66574IQJtUlNNKEIr4UXwzIFAeWVaS+pAg0RpgLVtV3JAfi0ioyuWCmxoeCTrSKmA+6uqdKikt3aSwNaxWZXDDZ2cSHAsPuVBHzgk2rLRDRSpgIeU7n7aGAmSXgh7Q6xLxg0+pdac9OiKSHAmZq/JBWh5gP8uEyBbPoEhppQpGkBJ2QCE3nkJtU0Wb+/Szm7O1cBukf4yN7bphPesPL4St8aNNn4j9/oW9tHjnv8wj+byAxGbyPbP0p7AOn5iJXjX38WryEDfgnsA+c2i/uaheD1dHhSVmHSFt9R4IeRHe5INu6TryQdrel1btiVyUd3LXwqbzDjLTaEiwdCWt1iM75cXvX5pssNO3zasugrlSetr+8NfxoWPKkD7gCTEfXhsjqqThtR5nOIA1NwXtaNFxbgVVMrCVNCV6K+lOn+DSQ7khiMBmubHRoKx4c9sPImfSH8W8vdMhD9EfxD0Yi5XxsYAd2AAAAAElFTkSuQmCC" alt="Font7x13Bold font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font7x13Bold;

impl MonoFont for Font7x13Bold {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/7x13Bold.raw");
    const FONT_IMAGE_WIDTH: u32 = 112;

    const CHARACTER_SIZE: Size = Size::new(7, 13);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 7x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHAAAABOCAAAAADd+81KAAAFJklEQVR4nO2W21ZbMQxE4f8/mu49kmyfJJRQWPShnUhzke1jQkJXX19+GH/zwrf4t9fSCxzZjbHR0LN4ez1218m+sAUNIdTLW/2AWnCVp/DhhW80iqdsKgqKR2Eq+j5YPJZJC17EWl8LohA1WLbM9d0/hGtUw7TRV3GxQoRWCmokdCy5qJAapiNmI9UwDdbOuVeCSwvLl3nyDdauIHFQz/ZehLQKGoyNQlT0PdRS6B7z5DBkRlByo33LFfyo59g9NigWPXgAV9JsVwfxoTv0L7jhwdlY/N3ohy8cebsfwr9x4XzC+U3zARNlFThNLH2lQQWk+AE81yhbO7FECu3Q1LKnSwCOQl0te4PLzGBDc1CF4EogfEybhc4+LwyB0uIFIqUuMyRihlL8OhQmIqaI8ZA8mkU9KDvcj9mEMGxmQo33PAYHMQSsQxmEgGpfkYl0bA7xgAjgBqwd1AKRAuFQ84m7SQahPLTspgIXwmyhkzQQBcKh5gsy2scTi3OeArINIiGZUk/RXKjZS2I6Fs6A7d8VzfuFkCSnzJFOJQ5EJwlsjWBawdio/S3I05p/hw83fDf+iQv5vBUUNozmK7Hk9huVj4svLUwCuPoKM1Q0oRY3vBz/tFV1y4hFy+CUHkp2gKFsSvgW6KR9gFJbLFoW6koPRLIDTFUacJlN4espkMVbRSbSslBXeiCSHWB6b0jhLoiX3iEUF6qIwCagsobTtNDAZJwcwwUYnfAkDyZJSEa+2NbnnTKAhbrSFvbWULIDTO8NtdSIGxSS6gwxWrQs1JVO6aFkB5iqNIisnS2luiKKlhGLlsEh+weuDtGUTYlIH1/CKgpXNJBmCI/0FYCIT0JgtmQI0xABrC0lbmGP4Ufx/8LPoj6owTUF9Vki0XcSn7iT8KT5Lih0drIDC4qD2FCht9TBO6luYi98OUBTaYFSoDjQ1uZCb+nRVURMEXvjbKmKloVqgxaAo9JBReiBiJgi9sbZUu9q0QwtAdsVOAcrLnAYJjmFUbnJV5n6o7bBRYq3btPISRDhUbwwgc5eVItwncLAKnRhMGZ0oQ+xgGkiCo29iIs0cDpFlppHthld8AqBeB2CWtH0Iq/BwGnCdBEYXa5lI1cIPrR+ilQpFnTyGriopZolwTlD0K5l45LvVj8NL94PuSQDceWfwt+48Ou/yM/Ay84bT9/YIz6G5UmHH6xZfcEe4Ha8TmyskcZuLDsGpRrbXXE7frBvjZYJrqmxh9tdscf8vkj8gSmdFP/iEBImIDHkhSbKpW6xi5lEeid8+2fhEhMEbqEplHYxie6dPRAoRxwDAu1rduIUKOjnZZAdoAJDhBEOyNMHAXRO6iVfSPsIJGLWMKIJRTQQ4Hpc0iaAVgnFkyhVPiILIz8ehc6iwkTpBE7ZBFAqXewRlCofkQN/bqdqUgsjhYQJjgSZsikBkd4d/sn/vDnadMBsf4ynNl1RP+kNGN7NHuK5Xd+Ib7/wfKunH9wNvoj5IC8KWbgPvzTs+xzmwKl89fNKlH+D7PoM5sCpdJVWw3slyP6VI67QrIXZgrIPoSmbUkfgvfdWBkQO0ji8NsIQkSgQ7i01666KBTBPwAC84OcoEzhkD8yCjE8CpcUlWaNoX044VCZEgqVbaSTxu0DqCVilQIDoZmjKl5PZsczwvQYJnK23LeNxIAJRdJg97eBSiwXPxYKWO91w80I9SfAYbogisiu2rNpFCjvAuvdOg4QcHpz+SeRIqMDNK5x+4asX+tQHj30fvwA3vASFbkQgowAAAABJRU5ErkJggg==" alt="Font7x13Italic font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font7x13Italic;

impl MonoFont for Font7x13Italic {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/7x13Italic.raw");
    const FONT_IMAGE_WIDTH: u32 = 112;

    const CHARACTER_SIZE: Size = Size::new(7, 13);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 7x14 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font7x14 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font7x14;

impl MonoFont for Font7x14 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/7x14.raw");
    const FONT_IMAGE_WIDTH: u32 = 112;

    const CHARACTER_SIZE: Size = Size::new(7, 14);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(11);

    const UNDERLINE_OFFSET: i32 = 11 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 7x14 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font7x14Bold font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font7x14Bold;

impl MonoFont for Font7x14Bold {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/7x14Bold.raw");
    const FONT_IMAGE_WIDTH: u32 = 112;

    const CHARACTER_SIZE: Size = Size::new(7, 14);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(11);

    const UNDERLINE_OFFSET: i32 = 11 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 8x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAABOCAAAAADNt0HTAAAFWUlEQVR4nO2Wi3IbRwwE4///aKd7AOyDpGLJZVuulFrAzGB373ikGDnf/vlk/qoH+F4DRh30pNmLPSRFAKfex/dv58m+DKMOp/Hv35iozRo6tHFBhx/zngfICwtG4XP/TNKhzOfNE/f8X3DiPMN4MDdaD8CSAaFO1thhnrDHt/EAtXA+YKTwdarGXt6ssQJKedm6UB5nVpzsIQuLXJD9+QxK2Shf9PoRiofxkdqONLWyYKSgHwCh7F5Y9NS2eJxverf0JZygeMN9ypdlgUof9Nj2CDe4N3IuAm1h1h6pddXODRM2GSMvePyVeT1KS9tvZF5pc62c+VP4egAfYL4l/bvhS+KClgDu7JmEodABW/4Kb9CsWOeZWKFwB0yNQHl2KsIKQKYuf8W13EPMf4w6UAhEI2O1Y8u4mG2FirfA+A5FjdHzAVqKikspms8szlqROEL1R5Vj27OD0r1SC/LWA1TiEtyc7mEeBJiAMTOej6qXl7edzBLuDSu1QF6qYIlKB0J28ZbSCIxfPC3Ogs4N0UOWBTJvDHrN2bilNALjN1nd98koBgqj4NT2SJmtUPE0xCJjpb5iBeIEuYYcyYeKAiE+AZNcM/NyBdZcdngszFl4iNQvx/uO/pD3nfqNfD0AD8CXRjNox5jQFjeNJ2i04ujMQO+/SZSUoVQ8x64/RDR1tUKtlssRKt4STGdDGUqdvoL17BBHqPDStwQTTcUlH69iu0jpCXwYSC1YqyWOUOGlbwkm/s1AdUSrnwTEctVArHLBp+FxEkMcWet0XAHne6VS3imYATsqklUDPxyeG7kxLXGECodzCeK8JZi4K6ojsVVcWI4k5CzlgrUd4ggVLkeoeEsw0VRcylDq9A4QR6jVCrVaTj/eyUj0saEMpU5H+SC5DxlHeiTThLa4iSMFC9cnh7FNbmthBE8CSsWz4d+BT+brAf6CB/C7gM2XIqYn9HZC1taX6S3vC5gMjKRQo6wl6RMI9dJbSvcLveUI6PZoUXnOFvuFqGcPlaIIZVOAUldLPLJMEhW7qRso1LOHSlGEsilQ5x2Nm5DW7WfacCcUq11k5gTdRYgilL0qU3jwtlfhoO8BcWS/EUiMlCp1gK6iVbht+U7jJ3UPyX1WNUmRUiUH0V3pW2GFFdsu6gaC59bU6kNKlZyK1g/LTCOtsAJULr2pFwz9O8+s9EyIPs4qTUE0kmM5aMACW+d48LDEwT+EzwP+c/y5/BUP8Oc+9xfkta8nuIbhWuTXd4zX1sFa9//b3uZ5b114ci6a7WbHO9nFTi943nt5/Fw8MzyMi72+0wuOPT5ZR/62xGfW8+cGr1kBZteP86oksZVu9VCvG2sFNx1wtk9Zy13TFa9jNFbPUBJMNCUxpKvPU7ra9J17wwpxJNvsm0WbPiSYOI+Kk7JqJl0tesHaDnGEKib0K2TcEkw0JTFk1Uy6Wrjgx2M7UMtd0yM2tJVvCabpVmTVTLra+I5YffiSxF3Xa1ag54yI48zTSq+TAKXi2Xj6En4A71Fy45L9Tt5/8ol6B8+w/mr5DT5w9Pfw/36A/mXUN6IHwnwR6Xd9CXP6Z5gLxyvwXxk/egb1B+TkzzAXjldAqLjixGeRAWP1nHUtfjcFKHU7lgsySdvygzyAh9mjVrapeBq0oylQaSpd0pabSSZZKwM7dbFCxdMyx3vWjl7FlIMVldblbcsXLnAP2kit3GScWbMRja4irmM7yaPv0Dh7D35rJK7Od4AFhjalm0ojmuqPycLv63cY3+GgLx4cbOGGbANB17IXiU6XAM4xyEzUWYQeTtzgAnRxDR8jl0Y+xK97AN8it/so/wICJROLe8uAfwAAAABJRU5ErkJggg==" alt="Font8x13 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font8x13;

impl MonoFont for Font8x13 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/8x13.raw");
    const FONT_IMAGE_WIDTH: u32 = 128;

    const CHARACTER_SIZE: Size = Size::new(8, 13);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 8x13 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font8x13Bold font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font8x13Bold;

impl MonoFont for Font8x13Bold {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/8x13Bold.raw");
    const FONT_IMAGE_WIDTH: u32 = 128;

    const CHARACTER_SIZE: Size = Size::new(8, 13);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 8x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAABOCAAAAADNt0HTAAAFeklEQVR4nO2WgVIcyQ4E7f//aC6zJHX37I4xOIwhLkhLVSVNzzAsHO/9/PHJfK0XeMmEUic1qvZiD5VK38f1nppQavuPF9rMRMELI9QAE8bfwfWWmlDq9BhuAUrFw3gHlILSV3n5eTlzucPv2++c7ezb96LYU6X+gHp6DU6cZxhPGCngVcqjeI3DrMHIaeBeFFs8zm4YqYW3L3K+Lvd3HAHHybJzpTwXavo1dT3S1GZgoqS+Hk3F05s1JSAUlP6Svlx6BwconMbJ+0VcbNiE8Ud4wuVKzkWgLczukewVG+WmSkNNpTd4w0G9z5xu+0jmSy0uizN/Ct8v4AusX5L64fBL4kJLEK9kgRswFDpgy2/w9sUauCF/CVlQUC+SHIH2XOkctwWnLn7Dw7ZHzf8t0CP1nzk2cpjBhrZgthUqDqXL2zZZtNzeOKlUS+Ujw1kViSNUfVJobNwLwASdy/S6saQVZw1jdSYTwQeRgAGcsyDkkyJF2yMP9E6rWyKtPKgc3EwHPgbjltIIjJ/c7HoVOx540cLMNwYEYKbiLaURGL9Qy+MxtWjPEymJRSAeKbMVKp6GWOTJfLr0PFxHhv7IOe8YwxMwyS0zL1dgzWU79HNhjoY9JNl/mzy49be87dQH8v0CvAC/NJpBO8aEtrhpPEGjI84uEjiXiNhQdvwhqWPXg9SlFWq1XByh4lAKhumSsf0ClL6C9ewQR6hw65FWMNAUlPIN4wiF98V+AT4MpBbWaokjVLj1SCsY+ns1INr+x2C5NBCrXPg2vE5iiCNrT8cVcM6CWFYh36k4AJbakiWRxOE8h3uYVkscocLh3II4R1rBkOfKxWrLlyNl5AuTmc0RajvEESpcHKHiUAoGmoLStnoBzIrXAuIItVqhVsvpvrleUqqcXTJ28wI8xU/SHOmRTBPa4iaOFCx8GkVGmNnE0awizOBR2C+QPUZ9Lt8v8AVewF8GbP1WaHpCX07Irn6JcAvWghPAUDcwGTQ69EyINtzgiFC33lK6vl5fR9t5EtRQbkNbSJ5bin4QQj17qBTtmxU7StlUWuIRGAfjdFM3KtSzh0pRhFpdut4qBuWlsMKRFjwQ1bym9JygEbQlNDVOcRABs7S3wUorbK5vjezvAxIjpQpN2VW0CrcmE8dP5mYuklY1SZFSJQe1XemSZculc9uFehrgeTS1+pBSJafK6h8DU/UorCA1lF7pBwA/cn+WmZWeCdHnGaMpUG2UzkEDVnDtMm+uK879K3wf4P+SfTJf4gX+4Qf/TL725Q0uw3BZ8uO7jOdwsPbzh+qW50vrvpNzabaHna/JLnZ65vnS7elzeWZ4GDf7wk7PHJf4ZB352xKfWc9fC7xmBZjdr/MsYpDEtXSrp9yv8xhuOuCoh+KYQq1WvK+PVM8wKia/EgZOSirLpCzV4nwwEoO4MtczlNqHtIopX0kcoilkBl0NPVvbIY5QxQReyJQx0iommpIYkkJmiGuBzDO5QDtQy93pERvayiOtYppuRVLIDHGt8DtiwRFCzePu9ZoV6Dmj4jyLaaX3JODpuuIeo/4Mn1HygDv7jbz95CP9HTzDhdv9Pe84+jH8r19g/SzqV2JGJgqySHqdOv1+9n0rGfwPQSfYyO/w4J+w71vJQFNxBeWF/CzyibCd0VnXdC6kqdVqHKHKsX1DmDC+YMHKv3NEholpKp4WnaZWR6vSJWV5VuEoxyq4p7S+VlGB9rYKdJVdRfTeSkrNWBQmjA+ZeQZNdurIOmRcs16tYl3HsZ2gDVZaocjoM/ipEVVXNsSU51Zj/iNYLK73Lz/SCgd9d2O2hQdyFQjLvWiDRlPxgHMKnEkaIbbGRfZ55HDm95J7I+/jr72A3+Hjt/gW/gP5MxCJGwHqEwAAAABJRU5ErkJggg==" alt="Font8x13Italic font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font8x13Italic;

impl MonoFont for Font8x13Italic {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/8x13Italic.raw");
    const FONT_IMAGE_WIDTH: u32 = 128;

    const CHARACTER_SIZE: Size = Size::new(8, 13);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 9x15 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font9x15 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font9x15;

impl MonoFont for Font9x15 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/9x15.raw");
    const FONT_IMAGE_WIDTH: u32 = 144;

    const CHARACTER_SIZE: Size = Size::new(9, 15);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(11);

    const UNDERLINE_OFFSET: i32 = 11 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 9x15 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font9x15Bold font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font9x15Bold;

impl MonoFont for Font9x15Bold {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/9x15Bold.raw");
    const FONT_IMAGE_WIDTH: u32 = 144;

    const CHARACTER_SIZE: Size = Size::new(9, 15);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(11);

    const UNDERLINE_OFFSET: i32 = 11 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 9x18 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font9x18 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font9x18;

impl MonoFont for Font9x18 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/9x18.raw");
    const FONT_IMAGE_WIDTH: u32 = 144;

    const CHARACTER_SIZE: Size = Size::new(9, 18);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(13);

    const UNDERLINE_OFFSET: i32 = 13 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 9x18 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font9x18Bold font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font9x18Bold;

impl MonoFont for Font9x18Bold {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/9x18Bold.raw");
    const FONT_IMAGE_WIDTH: u32 = 144;

    const CHARACTER_SIZE: Size = Size::new(9, 18);
    const CHARACTER_SPACING: u32 = 0;
    const BASELINE: Option<i32> = Some(13);

    const UNDERLINE_OFFSET: i32 = 13 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
