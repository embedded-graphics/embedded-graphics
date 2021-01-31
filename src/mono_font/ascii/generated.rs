// GENERATED CODE DO NOT MODIFY!
// Any manual changes to this file will be overwritten!

use crate::{geometry::Size, mono_font::MonoFont};
/// 6x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAABOCAAAAAD6VUyiAAAEfUlEQVR4nO2V3XobNxBD6/d/aPccgH+7lmzViXuRL9gZADMckmtJbd7++WF854J3NpkvYc1lx/tbpIiFiIG6j/wc729rIKO9YDA1xocOBa3IZtfU5/jkgtDVo5OAykvAsQ/BwlrCD7CLffNQF5YX06L2EZT6EVhYS/iJsYtbUkwamD4qMdhRcHoqHFFYFHMo9xDDTUw7FRz2RNpmkKoY50UGcastQRFMBYc9kbZ5Rw/FwFgfDHVRWz6x3wGPNT1joOUJOyThXqWoK584OrxdN6r/He40r7h0drHMT+GPuKCfV7hfDgahWh4JDWGxfAMbgqGZYUN+IPxOoJGDIQLIpqxOpnFiN+Ik/mdX8VGbg7gzZjDAEGS5eeVC37yNVk0IgdY1AEOQZd6B1cmkNkUrZY4mgcKU7FKPp4HakuLhCFk2T6SGPEVNTlZxhIiHrzl54Vqmki6bS4JDYd8xHiWJaHLyhvXcpS+7TUWaZVMiyLIRJctNmGNAC7GdK1a9uWxBiyZcP5iE6w8eC4+V+BV4ROkZPlv7LfgTLji+KKweSXvw9cvPHKLHIVR0ZNUsZ3EuExBMQHADApvbmQkQgpRV058tHD+k3IAEMv2dZwJkBeDoBrWdXkANUzQggUx/55kA6ZQpn4810seCq3I3ky15l3TK6espRqXwfjWAHcdjjeRhkTl6eBYAMv1izoJ1SYB0yiynXpMIHmPRgAQy/eZ2ZgJkBZAzw6EwPqJJJiC4AYHN7RzZIGXVPJl1+qOIxcjQ4K4CCjp4Mh6WkjRoDR9mlWXsz+LvBSKf9cDpg3w5tJB8K7BSz6rlc2bEUYqSiAkFc/QDn/mM2S8T0ULTgSCu9clCbZ5MQIPV5uQokRSZ6/TJQm2eTECD1ebkrQP5HN3lSr0fP0y9kqUSgrJ4cuguE9nSrttVoR6ZBWnImCgNnlpe6HS62UkI5cix5BMZE6XBU8sLnabbp05+lH10O8GQYUIb2QDyuevJ+Djo9A4TO2myrA1iQhtHdVt5Hdyzth4eSzGrH8P/ccG3P5qX4OHrhmUGdp1/uoLdmxidPXLBpTlGF1aNIYKbIkQw9YpL8z6y6mUudmB2pl4xm/ymeBcMDaSMwHoS1vMfA4yHKiwSeIgaWTMwOkGb6iGz6rb66WhG6MXLep7MtIaCnNGWrJq2M41IdMJNgHicwJJ96mRItNismjKISO3MBEh7sWSfOhkSFLwIjPvIBvDPmZ0kQBqxZJ86GQpyfE7QD0ZgPRkwgyXTVxPm7JDldrCvgi3JDSric3y1fiJvdAF/N/wpvhz4VfyeC9ZfsszCvf4W+kUcDBHBkAeYEy+go5v9HaliyAPMiRfQ0c0kURy/fTxVPRM2Mkf3GTPDEA698MYcLTMNxxtR8mQiFB9yl8CKWQ44zdJm0JEUUvuyj+6aj3hjbNkcaMwQmSUJQZO+BiIqJ2/wsUD0lS7KZEJiuQYawoDTGlE9eWMNuTQ8l2k4I2TXhBAiKTOY0VlDgzfcM7Dd13DWHOA1YLDMwjcv4KQPRz3Gv0B4B4tTfE7OAAAAAElFTkSuQmCC" alt="Font6x13 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x13;

impl MonoFont for Font6x13 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x13.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 13);
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 7x14 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font7x14B font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font7x14B;

impl MonoFont for Font7x14B {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/7x14B.raw");
    const FONT_IMAGE_WIDTH: u32 = 112;

    const CHARACTER_SIZE: Size = Size::new(7, 14);
    const BASELINE: Option<i32> = Some(11);

    const UNDERLINE_OFFSET: i32 = 11 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 8x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAABOCAAAAADNt0HTAAAFUUlEQVR4nO2Wi3IcNwwE4///aKd7AILkPWKpyrZcKbeAmQHJ3ds7XeR8++eL+aMe4HsNGHXQk2YPe0iKAE59jO/fzpN9GUYdTuPfvzFRmxk6tHFBhx/zkQfICwtG4ev+maRDmc+bJ+75v+DEeYbxYN1oHoAlA0KdzNhhPWGP7/EANTgfMFL4nKqxlzczVkApL5sL5XFmxcleZGHIBdlfn0EpG+VDrx+heBgfqe1IUysDIwX9AAhl98LQU9vwON/0bulLOEHxhvuUL8sClT7ose0RbnBv5FwE2sJae6TWVTs3TNhkjLzg8Vfm9Sgtbb+Q9Uqba+XMX8LfB/AB1rekfzd8SVzQEsCdPZMwFDpg46/wBs3EOs/ECoU7YGoEyrNTESYAmbr8FddyDzH/MepAIRCNLKsdW5aL2VaoeAss36GoMXo+QEtRcZSi+czirBWJS6j+qHJse3ZQuldqQd49QCUuwc3pHtaDABMwZsbzUfXyeNvJWsK9YaUWyEsVLFHpQMgu3lIageUXT4trQeeG6CFjgcwbg15zNm4pjcDym6zu+2QUA4VRcGp7pMxWqHgaYpFlpb5iBeIKcg05kg8VBUJ8BUxyzZrHFZi57PBYWGfhIVI/He+79Id87NQv5O8D8AB8aTSDdowJbXHT8gSNVhydGej9N4mSMpSK59j1h4imrlaoabkcoeItwXQ2lKHU6ROsZ4c4QoWXviWYaCou+XgV20VKT+DDQGrBmpY4QoWXviWY+DcD1RGtfhIQy1UDscoFn4bHSQxxZNbpuALO90qlvFMwA3ZUJKsGfji8buTGaokjVDicSxDnLcHEXVEdiU1xYTmSkLOUC9Z2iCNUuByh4i3BRFNxKUOp0ztAHKGmFWpaTj/eyZLoY0MZSp2O8kFyHzKO9EimCW1xE0cKFq5PDmOb3NbCCJ4ElIpnw78DX8zfB/gDHsDvAra+FDE9obcTsjZfpnfeFzAZGEmhRpkl6RMI9dJbSvcLvXMEdHtpUXmdLfYLUc8eKkURyqYApa6WeGRMEhW7qRso1LOHSlGEsilQ1ztabkJat59pw51QrHaRNSfoLkIUoeypTOHB216Fg74HxJH9RiAxUqrUAbqKVuG28Z2Wn9Q9JPeZapIipUoOorvSt8KEiW0XdQPBc2tq+pBSJaei9cMy05JWmACVS2/qBUP/zjMrPROij7NKUxCN5FgOGrDA1jkePCxx8Dfh84D/HH8tf8QD/L7P/QV57esJrmFxLfLrO8Zr62DW/f+29zzvzYUn56LZbna8k13s9ILnvZfHz8Uzw8M47PWdXnDs8ck68rclvmY9f27wmhVgdv04r0oSW+lWD/W6sVZw0wFn+5Q17pqueB2jsXoNJcFEUxJDuvo8patN37k3rBBHss2+WbTVhwQT51FxUqbWpKtFL1jbIY5QxQr9Chm3BBNNSQyZWpOuFi748dgO1LhresSGtvItwbS6FZlak642viNWH74kcdf1mhXoOSPiuObVSq+TAKXi2Xj6En4C71Fy45L9QT5+8ol6B8+w/mr5DZ84+mv4fz9A/zLqG9EDob+IxU5vOU5/jnXh8gr8V8YPodjpLcfpz7EuXF4BoQan/nA0ts5Z1+J3U4BSt2O5IJO0jV94AevsUZNtKp4G7WgKVJpKl7TlZpJJZuWgLlaoeFrW8Z61o6eYcrCi0jreNn7BPWj3qMlNxjVrNqLRVcQ5tpM8+g4H3oPfmjuaR2yIKd1UGtFUf0wWfl+/w/IdDvrihYMt3JBtIOha9iLR1SWAcwwyE3UWoYdHuOBcvobPkUsjn+LnPYBvkdt9ln8BkyUJi1BNi6wAAAAASUVORK5CYII=" alt="Font8x13 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font8x13;

impl MonoFont for Font8x13 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/8x13.raw");
    const FONT_IMAGE_WIDTH: u32 = 128;

    const CHARACTER_SIZE: Size = Size::new(8, 13);
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 9x18 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAJAAAABsCAAAAACgfWQGAAAG3klEQVR4nO2Yi3LcNhAEk///aKd7dvEg70GdZKlUlRtjZ3oXIAk7iZLKv//8Mr0vdKXDhf50S7KAD2kdber4nA6PjjeRrB0knCDlQMk2uoVXxYOHJ+nxJGsDPo+8Bv0ENtQTeFE+d3jSgSJZC6htPbrQSDRw5IKn8tThnINdfhz5bRbEpAhjMdi0DYJYrTG4Ug6xljLZxYBlzpPpjcpNvaMKOYMkMPFUdcaaqtEmBqzAOFo9QTa3tq6RQ4KhwKEebRNnttZDbY+JlbkiNSZTzPCo0BN9pgZP1UdiD8UhFjDIbwgsODtTWx/Eao3BU40T5a0xnGLA6mBBPAJkgdQSQxw9hUFDPRknyltjOMWAZSrvY2HltptmO+FWee6gmsxHOi41H3imPtRxX0830fPdv6+r+1xt/7zeF7rS+0JX+pUXqh9NaP4z2RNCiTbkNhFJA0GiscFLqqdYfIk1J4iWBaim2uwGLUJpMFZ1LOCujlurg1j80mN8Ekc2ZaizN7vbQKWZZpXd03lj9ElsPV+LrVLzClaSDk1Q1eCsZBueOEDl0j6B14Wqx0vF/SIbC0PkApQdxCxA39ixgC3cjNKMWdH8ENYeTQxoVhTQLNVJsAAIhU7q/ZPGNNmvjbWrRcF8wcNRTWhi7RUxNeGoe+Mxq7z36g3Q4JFFVll7RUxNOKnn2++wJwMogAVL+DGtKKBZIRYg4UkWgBr4bnIB1NA693Z1Go/8YtQzIqon1wQaUAkUkQXSCcRBausKrZ9Qfe0D+uCxr+rD9/nwuR/T+0JXel/oSr/xQvzQSqI/iQxqIgKGWSBOyKwC1+gRDZKd0GlWFNCsYejm3/Yd+LS9ytQEiZVsa4+CmlWm/JIFMNEsjXEalr1O4JoVBTQrOoJmlbVHQYw1OsWXlE/FqjRP/vFX9TqBa1YU0KzoAcTaoyDvNYREoL6ZrQK2MBu2DFuWYxXAVEDLFryDjjI5zRrZwlE6FdAodgsyofU6Bi2qcZkVBTQrOgAPakxi7VGQfUNINGhVsZp4E65j2LIyLrOigGZFR9CssvYoiLFGpwKaFbO0nOSK8RqlYlYU0KzoCJpV1p6wYlaZCmhWzNKaVECzlu1VpiZIrGRbe8JaZkUBzYpZWhwl2zIgkQgYZoHouRYzORONHtGEKnHNARLmpAFD/mD8ZXpf6ErvC13pfaEr/cyF+mfMJieHwZA/mlDtgQKBJgHJgjYrZsUsjcVZvZ4CIGxGaTQjWz5WlS1Wsswqa6/QrJgVszTWOKgCMTRSNXvrQGu9KBWzYlarMaFZMWsaK5kqhWJopCrWraluNStmxaxWY2IYK9mGJ8wCNHZ0NKEx1l6qr1rLrGGIlFYMY1UWwXGyWtTUge7RQX4euZcTWiZkycnwimGsyiI4Tla7YOR9OsjP90ZAo1hMosLyihjeMS/UE+GYCzaccFQ+TyKAV9IBqVJReUUMH6FAxixYwkegRYtHnnT6NCtZZpW1J1kAlDQsgDbWPgItWjzypH5RiRfPSRsipYQwqdJIOY+1ExHcg1I3HWfdDB+c+5pytcN7newDe+V/U/8uvS90pd94oW/52/jT8i6/6ka5ym+60b2bPLjfacxPjlM/2zPGFM8MfKR72/P5o45jOhYwtHVnjJU2vK972w8eOo7pWMDQsTto29rwvvZt/0AdJIWiCVKABHSUCc3IABHBoGctVMFYgfQ0gGiyDuJAKmbFrJi1GTKtzdqjIMaiC5bXgmIMMRa/jnJmxawooFkxXoSjasBl7VFwHhYFvJaUNqXhUz7nk4xtGlRAqyMAhCHT2qw9CvoUodLrtaS0KQ0foiuzYlYU0KzSpPG1DGLtURBj0QXLa0lpUxo+RFdmxayYFbM2QyMLYu1RULNQBV5LShtj8WvJ36u/25E9mSAFSEBHY5KBlgEZOVg2tuQxgWK4wfqa8rLYrRizgBf04vFb5YuxWzFmAS/oxeN3xJ/0w9ew92DnoV49/+16X+hK/9cL8Xd3far/qaNP3urB+C+rr4EmNeRmzdGi59qfeV3r6UkFeC24tOi59mde13p6UoGeP6Sp9IZq5BTkiAgkC9osjcWObsWs2HwK0tWkCSf5WFWdsbD6klXWXqFZGmsbs5JtKBkrLfQjxFnr1anN0EjUmBjG6mBlM0H2IIiPQJMmnJRX1ra1meITHkE9SgxjdbDYzP+3QnQsQMJXbjThpLyytq3Nhqopr4jhBlxrTsw50NGERRNOqntYdcYqQ8mYzjLCmRh4FMwRxNDsDt2hCSf5DVSbA+fhMUCgQCCp/wJ51toMH4GkYENH5a36bVPH7pN68pInW6VvuND4/+if0zdc6Gvv+MKj36P/APg6gKlAtrpAAAAAAElFTkSuQmCC" alt="Font9x18B font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font9x18B;

impl MonoFont for Font9x18B {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/9x18B.raw");
    const FONT_IMAGE_WIDTH: u32 = 144;

    const CHARACTER_SIZE: Size = Size::new(9, 18);
    const BASELINE: Option<i32> = Some(13);

    const UNDERLINE_OFFSET: i32 = 13 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 6x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAABOCAAAAAD6VUyiAAAEeUlEQVR4nO2U6VrdRhBEzfs/NDmnqmc0ulyWYJMf/lLqWrpnkQwkL79+GN96wSun5FdwbeuJ15e6SEKohcTI6Gc4djXmBZFfr7QEHxoINAlq1fdxrCeu29DI5FFsCcAdYzj9M7y+XCvXJj+9347TYrsxIoLQnYlPwXyvkDd6HadRGiUxSC8M2YOgxZnpnr7APc6ZtIyxYCfCbCU9hXOqsBnMdbERCgkOd4jh9G+RsXxETvlxKNGHdu+kE7HI4NiSufSOQdsTTiDlWa1oqp44JnxdD+r/Hp6Ud9wmV7PDT+HveEF/YNH+dggY3c5YZIzF6gM4EIy7h5Q/QxplOIpQQJWqvpTBiWuQpEAK+ujlCO9MGAUEClbLuxbmIbUJYiMYsl8DCBSs8g2sLoXGNO1i1OYou7Dc0us7cATxGppJVZ5Iz/+u9cWlgEu9mUT2KvPJpRfufTrEe/SwIrgf9eJk/JFLL9ivU/mmTPxMHSurUqFg1YrDaolyDWgjruSKXd9ctWHEEG0ehWjzobPw3KnfgVdU3sNHa38Ef8ULjt8U0YzlVz16/+1nH2YmYXRMVF1Ws4hOs6dXoBBwaSeLAKOgqkv/btFkC1ZbiMBWftRFgO0CXN2idwIJaaehEIGt/KiLAOsuqZ7P1RNs+IB5GcBYIuDVzM0002l+YALgxPEcPWtsY0RmDrCVt3IVagoB1l2ymn7v1FoyhQhs5Us7WQTYLqCuPdWYIUwhaAsBl3ZysAVVXZ7KOvNpEgkqMtpVQMOEDJNRJWTAaHKUVZaJP4v/XxDkhz04s8ivAtXe5vtv7Ymyxa00FZEQEV1l6xs9+Z5ygUrFC0M3iK62P1Xo5akUMqqXS+NUCFbzqEIvT6WQUd0Sd1tgucqYM8mMEAwtUcVn5qdKcLeFHsk0xymhHUz1ob9efwioVzdypNMcp4R2cJZ8MCs8RCREDngEeNSnSb0xQ8Unk82lwiRP5ADgp+8VBMychJzZzdRFhvsGYJInjvZx6avgNfvkQ6bb7U/hP3nBd382X0Lu3m/YYXD188cvruHCTI49F+6j2bmxewJVTDhMiuUn7qPHHbvfAZy5WJPlJ/aIPyq+ksAEq2KoGaJm/mtAyUiNRYqM0GN7z/3PlDHtU2XVY80rMYwxS1bNPNnTngXhFV7CpDMVZo7RRbq8CDCvE0TYp0llAehroupSBTGlk0WAdZYI+zSpiKDhO1DSW7WA/5o1CQHWSoR9mlQkyPW5wTyKoWYYsIcIM9dTck1gtRPiV8GR8AId9TE+Wz+RL7qBfzf6IT7d8Lv4My+4/iVXGjy038P8ImpRhBLV55gtn2M21lT/kHRQfY7Z8jlmY02FVECK1njv5DV3H9P3lD2zCdSqG11lazV9sxWHp1KRZMVDQXps9cFsxVFPipo6pGAfkxUeMrpsYU5cGhhkBKb6pJd+lEwjatUL/AMR5loXVZhSWG5AxtjgbgO4WfXC2gSuA4xjKOJUIhgVqmx0a9ux6gXPFDt8BWymFvgKVFxp4ZsveHLTO/gH7YIHgEKxuM8AAAAASUVORK5CYII=" alt="Font6x13O font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x13O;

impl MonoFont for Font6x13O {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x13O.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 13);
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

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
    const BASELINE: Option<i32> = Some(13);

    const UNDERLINE_OFFSET: i32 = 13 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 7x14 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHAAAABUCAAAAAB/tuy3AAAFsklEQVR4nO2VUXIbOQxEk/sf2vteAyQx0iiRsy7vx6YFdDdAkJyxVOWfP74ZTxd+/Px46j1iT2CIT+Dj5xivrX1hCyrxXBpV6EU09C5+d+EHLnelrADFD/IOGB3DVCBHtyT8cDNNFYkRv5J7sDiWqTZ8tR+5KYSn1qmN7TXOqRjoBVwjFiw3ansxKMl7blQPxPRDVfECrhELlht9VR8TT3F5xW3LhJmvHcG1osYTC5YLe7QupsxdVTW2jckISHGPLJl3yNEJibEYmLXG9hqXVQx0jyyZjdQL2Z6oT0xuPqAh7uQ6WM3Q7FejkfPZ5wQuB0j4jfYtV1yaHENZrdAvUFMvkMXQE65dKxMU/yk4hLjBTbdbxd+I/+JCfwsi78z3SymrgO6uMKdSq6e8gZpkY8IaLie12I0BS0H9iI9AjzjNcmH+PbUSEFwVCKdLiBaBJdSWRQezjG+62xETglH/FuNFgZasiHUIRmQUoQUwqQTqgTFNIMowgq3Q8klTDzGCaGPi4PguDqqWOaZdERhaEXgdNuUighv6jImnRnWkGj60RaM1RVVI5aAtA+nwLEBXdfgcU1Tc1DtWVVK5qGURl0S6WhhV/i4ECIeKQzKhthggXE2TmzgLpABLxfIqUQ/FBqowtWqXPtLVEhKsKgxWu2SY4zTEl8Izf4nfrX85/hcX9tee3w5eaS2OYFpSwixIVpR4hBpTogm1OMAgSk3uMAlVIQkxpdaQRQAlTEL4CmSqvUHlasXytWiglm4eAmhFEnCZSeDXBhQXslRIQkSqCW7kEEB7NqRwVygeRlBcyJKXrWW05TRbSGCVejXUnB0n3MnBfvQ2x4VU7MdTGCLCmqKBIgxX8xBAezbUkpaEWKG4kOUUTSiigVq6eQigFUkQqcm8aip1PPBr0UAt3dzUSZiEiNTkuJDEFPdi965VnFDO/qZOQo1oQleBecG9P54sLY5gMgIo8akgKmqFBrVEAdZIiyPMWHwr/l74WeSL4gsLUpD8xuBCfcHUan2xSLQrQINvPcdABCkjBlmTsWiooCWJxtp4K00m1GKQMmIkgWIuFjgiWlgbb6XJhFoMs2Jnc6hZHNdYx9xIoDOhFsOs0EZQqHnJMAs13Zu28O1pAD4pVZysmd6EQs1Lhlmor7tOgroC+7hKqeJkTcQvfpBhFvrx2M12LpQoRYxkShWkH8QA4RB4pRtuFYjHIKgRLTJD9U5OUVWAcAi80g23CoXzFHxLkxliIK4+sU1m8GhaDmoj4G/KcamgqgAeIktlenQJXZMTmbEI2rQczPpp8Q9wzihXPHCpn1Y/D1+1TuGwUwALyl1/F7iQR/lGeNm4cdiF2epfjTjt7fJjKozBB6S/54bbGC0sUdjmuHEhHrrDQ/9mbrSGvfiN0zzuAbvvj8gXQOnBLT41YsUqZEWTD5pSBhgHzObqrEkYwQQsSIxE4BY6ihXMshVJP9okGKFfVcQGn5rsjryxuxFNqKUyRBLRJuHkKqKWftpG5KCev7sRTShL9ZioRBLRJpHJLtKkkU/biCy6nKIJRUSZOjX+kMgkISI2/LSNyKLL699boaNQVV74kOgNZLMNP20jcsBz8+T04apKaClWjIco6ZXPWpeswMjr5vmVvg+2Nk3YMH+Ld2YeUE/6CLrPzRu8NfSV+PoL+ZXJodjyC9Pf4DL7FnrHkPxsiMLSF9hzb6N3DCFzZ2HpC+y5t9E7rrKVC7kcpgXjT4XICg2JMAlSHgI7qoceZYBReuyQLxVSuYkwCVI+Al34QQbWjqsYQXQRYToV6UizqflZN9xqe4rIqyLyplqXo45oQqZovcpAttKfUoi9Uq3L6I5xM2i9ysD81rAKiEjkXusWVJ9YFrKLKLS5ygDDbmEJxuMElU0NQlUrIRgldJDCBLDaLWjLgHs3pn8T2RJqtL/KwL+90LfPm23kb1JHlb3iH6dBwJEPFqb8AAAAAElFTkSuQmCC" alt="Font7x14 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font7x14;

impl MonoFont for Font7x14 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/7x14.raw");
    const FONT_IMAGE_WIDTH: u32 = 112;

    const CHARACTER_SIZE: Size = Size::new(7, 14);
    const BASELINE: Option<i32> = Some(11);

    const UNDERLINE_OFFSET: i32 = 11 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 7x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHAAAABOCAAAAADd+81KAAAFB0lEQVR4nO2U0XYbOQxDt///0dl7QVKSx5PGOcmmD12YBAiKkuyZtL/++WH8yQvfUr/9Kj1hxxxMHQ29irdfx3Tt7AtbUBMmFrp+lJfw4YVvZC4jMJDQgFMgAhS/AxaPZdyCF7HW1+IRkthYJsV8m5j3wOKxjNvoq7hYwSaJjWUomAOt0784PDUx0A7WaF1swuksVBOkmMWYe2TJbMQP+iIqha/eAS10HYEIUHyLLJl3mKNlUoug9AZtWh7ANz27GQmd7WrcwIUk4+pCTOiKecAFN8Jkiv8AdfbG4Xf1Q/grLpwXnAfN+8XKKqC7HMV2avWUZ7Cx0VVNsjGhh6uSWuymAKOg/pFsgS44e6mb1o7aaxYVp0uIFkFJqC1DV2noTOh5RxdDCZ6GQiewMiHFpwDJS0Jk2fLg9sIow3UFWQcymSagB5Eowgp1qCX0gOrAx3CIAyKACyhNgXNFax605MC1U17OoakOKniFqA7OQjIPWnLC1t6uAwoPLAFkE0RCMqG2GCAcemSPFGULZ02ZRwWD9ZtUu/SRdiMkGBdeWoeJHgOrsiC+CZ5W9Ft8OPDd+Bsu5G3DPH5ZM1pLI9c/qLwtSIelRvAUJRahFgcY3DbRKSMGKYNTurkJoIRJCH8CGbc2jLYYpCzU5W5kE0ArkoDLTIK6TqkL+a2K1iBloS53I5sA2rMhhbsgPtY2cyFVSOvlfCGKQGWNyqKFBLr4aahcQGEl3MnBOAmxhfKhj3O/bRqwUJfbwnA1NwG0Z0Mt1fIGBKfaQ7QGKQt1uVO6uQmgFUkQWZMtpVZFBCkjBimDQ/YXbuokTEJEevsSVlG4rAY3TXikrwBY6jgIh1do4CUMmJEWR5jR/Cj+v/CTqPc0eHSFesF01XqxSLSFESre+q0YZAbZoQMtoKriYDbeSpMJ3YqRBAghWkCqng6q7M6jBFYm9CgdlhGLSjGaYjKo6d70KIGVKa04nGUEHQKjuxjU0997YVxeHkIBm9JE/z+D0Ecj5aoGU4wuZCuIcBYfiiCVZEoTnB01wDOLrlo23Co4xgNCWJFCMkPcVJRMgHBoCeiqZcMrBOJ1CGpEi8yQ94RIfZohU4yCKosP1EZQby0OKgeoIbIUrk7tW+SEM9aNKosPPPin1U/Di/chD06DXf6n8Ccu/PqD/Ay87LjxKAdHi9ewzNHeWM38ed3i0l87NnbLygxWsav+0w12dcGlfzO3W7sCD2awm7u6YPV5XBi+pNJO8VsjcSTA0eSDxsqAwgmzuTozCSMUGy7RQOAWOooOptKR9KNNghH65SI2+NRkd+TA4/bZikUoS+yKDZNEtElksk2aNPLpMiKLtqdYhCKgtU6N2SQySYiIDT9dRmSh5bcT6CwqdBScAYpLNoneQDbb8NNlRA783nbVuBZaShwJysVAurZshpH3m5c/mpfA1qYTNswP8crMBfVNr6D73LzBS0PfiW+/8PylZz14anwR8x4fFCIKo/dYYy9jdpzqH6oajN5jjb2M2XEqSTS4up60TL0dUlzTEGESpHwInFlq6CoHGKXHDpkyGocYIAwRJmGuiAOyRwQasRqDObvFED1XLgwRpsNIzVgc9I4cuPuFgaZdWOoxODoTqwAtT7rAVp4/XZlVAoQhggxnpkoYrWDh2Id9RxccXqAmBMd4g4rIWQjBKGEFKUwAXHeedMG9C2f9IrIl1ODq5c568NULPfT51N/gX+RYBYsUE1p4AAAAAElFTkSuQmCC" alt="Font7x13 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font7x13;

impl MonoFont for Font7x13 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/7x13.raw");
    const FONT_IMAGE_WIDTH: u32 = 112;

    const CHARACTER_SIZE: Size = Size::new(7, 13);
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 6x9 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAAA2CAAAAAAdJMMOAAADSElEQVR4nN2UUWLbMAxD1/sfunsPkGTZddJsa34GgyBIUZLrdvv49RCfrkX+BWt/Tvr8MA11UUvxiQU4kRT5Fp8faywbclpVSYKRLCNDTXDFPW4v4NV5+VhBKmNBNWkMRe7AwlrCT3Qb96AUSuxAOkmMMFQlwO6pcPCKOZR7QsVOQSVMWTIh+0h92kaQqsg2ZSamsBPDkuAKMJJwS2vjCo6kzQ+CAgrm0YnYyBk3rXnGLW7nn+B+/rb5U+AH50lClXo+jimt4fd+/NCnYL//DpLGvGZkxFbcUIDZ2ZmhM22GmczBhDBnYP9VV0Fbytilrw4ByUg0czABVsJAQSpjDchm9m6wt8PayDRMTDVzgKuAXplQIFB9x6FDQDISkFOwUiWheCp0+MykgdZveloA09iGPwBPeoDHK3+AJ+f/zAXPsL64b4GtVymqpCpJoYRpNFmG1Agx6OL8c+TPZporwdLFBDBlL2rJgZxKzCapjwVXsoohq7KdKoESmBZJHDcc4mPyCmsSnhmAtyUBWoKpGQ0TwEQ/mtKHBBE0SWNUrwRLF2dAo4w1QgTF8eoYCywep1JUSS6D9lPS1yspIW4cOkmD5834Hy7Il+LLqZvHWvgpB5GLZijaEtSPQrD8gAllEkFhJB6gEFSbM1Nk+cLEUGSyT1pKErkBjgQTgkpB3VFPpU6RONZ90gKbRkBzdYHNabC3fjGhQG4gc4cpt5CmDAUxkQNuINHH6mF9HFqPWvf0GbRzgiLoLT+xVSy/AW85tOCH4fw3XlBwwfZpNiu28rDDbclAPesrbHZAHC44yv5yg2UWRmekC1aTL7b+BpqmImNz/N5uGrcr1E32Scd/17R3AjR0qhtSDiaAyQkQy/nkjTSDFAcBKo/NymICmHKmyfAhLdIs8pKR4c9KoHqs1AKsIe2QjZLW8YlehNs25HWf47v1E3yjM749/8uOv8Nxz+EGrvWGL7MP0cldD1zKHdfRx+jkWZu9IL99WqbRr1foLCLomQyjLIAtdTjI8sH8Q0RZsY5M9qHlMgYmDznpgcztDMYZlGUa1tNTyMSdHmBOYcHU1ViQqsU41A4B6dBCUIqz7GBotpZ5BQzDE651sc0t8xLyM72A33N2slXU8ReCAAAAAElFTkSuQmCC" alt="Font6x9 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x9;

impl MonoFont for Font6x9 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x9.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 9);
    const BASELINE: Option<i32> = Some(6);

    const UNDERLINE_OFFSET: i32 = 6 + 2;

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
    const BASELINE: Option<i32> = Some(11);

    const UNDERLINE_OFFSET: i32 = 11 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 9x15 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font9x15B font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font9x15B;

impl MonoFont for Font9x15B {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/9x15B.raw");
    const FONT_IMAGE_WIDTH: u32 = 144;

    const CHARACTER_SIZE: Size = Size::new(9, 15);
    const BASELINE: Option<i32> = Some(11);

    const UNDERLINE_OFFSET: i32 = 11 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 6x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAABOCAAAAAD6VUyiAAAEhElEQVR4nO2VUXIbORBDN/c/dPY9oDnkyEqkdeL9SAVhA2h0c8aWUuVv/3wxPvOC79yy3sK15g2KQ7NBz8GIupNf4djSUhwkL142TWBy51c4trQUB/GZd85eJARQjpJ6CgbXBA8Ln1kkYWASL+jh6gy0T8HgmuDhoNfKyGULA4GaL6xQ1Nt949/AplhLKs8wLhfLq1mpfYLEnCJd0WsHSwgF1iLKUVJPkJjzAT6P3LlW5FmD2vIJdq/IqfUQfQQpBwFrbrD4xJG4n4sI9V/hTeuOW7KbpV+GP+IF/bzC8+WIdJdHl1cSwA/gAnxpdtYqLLWcwRCHTgcD1rCLWxd2EydRHESorUWd6mSA4SCMYBqNDlMu9FMcJSVUC+KyJNMLDAdJGMcEh1zWBgNFOEoKKG7BjrCWYEwg6WWkMLdOpIe43qGVBFa5Po2+zVnm8oV7m066XSyJPBTY6jPn2MWTydaC3t3ZRmW9CaLFhC2Jg7iA4yRMhTpSYEQW22nsWLnAgyBqDTY3rYdsNj/Xul+Dz/khfjL6PfgTXsDXBSO1I/mmhwdpIBcQvLeEiWy/PC4G3s2D4eh7rdykzsy0iWxXv+7Sc5Dd1IPDY2+8SmSHhoOzC+ghegwHnI2rbeuxN14l3HGrfbuFpCu6GsjV1YIqbB6Pazc9tn26C9k6IxoOSq2ZHls2wONSIhG+vWxybNZh2nBoqbb12OEmdWamJDQcnDwJ0OP2wMbJ3XdablK3qol83ylbwYPJw4eBl1fuAoLHwHQxRb3MAJObX4m/LxD5rClx+iDfBJHy0YNpf8CCVR4ZEjGhIFNXH/msJh/Zh8CcKB3QdCHItP3JohPLZDEXauS9VCdQjkLDkqbbJ4tOLJPFXKiR91KdWDrIvuxeUMNSs1QSCbFHDw49ygLX7OX1ANH+KkehEZZIh4aXli+wby/npgYoR81IRNghHRpeWr7Avr23hI8rP6tCtwuMjAltzAuIx1PxRqHLt2zrLME+GyImtHE0D5P34XvW1cNrr+br8D+84NOfzHvg4fsN2xW75/Mcu7JRhIPBVR9wy2b1wtVjOJhY+FKEg4mFH3HLHleu/jI3O1jJ0jtWxi+Yn4VTD9tcHtUDMhMokS1Hn3gnNucffXsnUHzseEDbY5nmKkISj6UGk5oV9sBINqgXbBNBPboUQNhalirq5Iz2cPGZABsZ0a4CCHE8lirqZCcAY0KmOIAlPGA8FF4FkJ5YqqiTGQS4RCgeA9tcHo3XUsv3nAkeA9NgvPgmuJLaoONgfoIX4xvyE1EbJPfgI17Nfxm/5QX7F9lu4aH9FOZ7qISh74gYeQKW4HcwmxWZ3+N68sgTuPkeZrMi89NzCv+fccx9L6SPAfC0P2Ch83Hh0AkekSqN5Aqn+yspk2NQfRYIMLHhyAXGrmKGAQZ1EKIdFrp7dWs4dIIXpEpr6poVopyFEHuLlmLHBh4pb7jDMecGpitUDglklUZmQQPw8Eh5g7UVHBdikHlQhADdHVwkApXyxpqC7V7DXavgVWO3W/jkC1jmLvUS/wKDbB9rbji23AAAAABJRU5ErkJggg==" alt="Font6x13B font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x13B;

impl MonoFont for Font6x13B {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x13B.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 13);
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 5x7 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAFAAAAAqCAAAAAABw4PWAAACqUlEQVR4nK2SAXIbMQwD4/8/2t0FKMk+x85MGhwJgiSkOze9fRV3xP0mL7xoyfyMZdCZC0ssLAkgm+VYYNor7rc1da2RqwAChhiMSg4fuoDZmiKLXMG1kKNQUCW7JWHSrgJJJZ7gmiHXlnkYiIrwA+EKFJmYIo3IFVxrr7ItFCZkMUvCC4Mpv8S+74BJppJv4dtg26ozU5Xeg2MY/QEaLdqhaaODmkrZlijhqRpYw2oGssaM+K6hzJG0DVSpXTwocLkwBI9H5V6xoosFJhtqEo+n4J2EmQgZJtHKqVK7vAQVUJRM57u2sapEjjpEgilT4dRfgKPfYz5J4gsfFfT41dxg2GLqvwWkJkJs+ULrO4o7lJZIyj4UNcmL6Wn5K1NtkGws7KHVhjIjqdEwhyxq0paZVtgmQ3sIz6OStDhTM4CZWKIJnrHIe3ilLEJpCTPhQznbfSGfTBp8Joa0KigegFQZKIg06GYRyiIH/hK+hUu9+yiIBZMrETweiwUBqlgGGNAXyjpxIQJa66SgzuL1QgPAiUN8CJs0cLO0S+APwGxEzRZKlBhQySySpScuPMLE6uOOMCe4Jheq9wKUW0IDjSDnGih6KEqiWS8zc79MK7ZCfPH/cIDzT8Cr1k1biN0sMbUlHwz8/gsYdHdqsJp1FAEdTDvlATPg5/vHpMtLI6tI1NOWwmu6waY6WwpwlLm0W/YxTgtlBWiZA1os+KiljKkY0kO2BJNKCIkgkrJPROOQU2DHq+g8DMFVk5KRhWZLWkuJdv3kH+A5ygZn3+L71acTG5pWQin/c2E8i/Z9/BOQzGDzQeFI+KRs8hg+zHRS5GA8h7JEmgkfSo5ZunqhhboXiTloJniYosi0EIPyoQXd9HLmihTORTJFZEuYaUnkzgONwaqf8bPrH4J0dUm42AffAAAAAElFTkSuQmCC" alt="Font5x7 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font5x7;

impl MonoFont for Font5x7 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/5x7.raw");
    const FONT_IMAGE_WIDTH: u32 = 80;

    const CHARACTER_SIZE: Size = Size::new(5, 7);
    const BASELINE: Option<i32> = Some(5);

    const UNDERLINE_OFFSET: i32 = 5 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 7x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHAAAABOCAAAAADd+81KAAAFAklEQVR4nO3Wi3LcthIE0Jv//+jc0z0ACXFXjlxRnEol7Zl+DABiJUoq//a/X4y/8cLf67Giv/2OE0NYrEKHUC19FTbfu4VhRXuhK4R4wymT4Yd8Cfbem4VhRWDdg8Jm3NqDP0rWq02fIIv3ctIFd/SWPYxqtcJmGDO85T2yeC8n3XCXmHsJHa4szAxiLIEPFKz5C7IxtdC40ZOyoYuxiOdjDEQMMd1vHREdfExibPoNutfTbXJFlEkdB2T8mYyemKX0YPKGO0TnTK0kRqQLK4zYrWgTGR10WDrHa7Dg+Ykdcn1AiL8wi6EXfJg6J65R6duxHr5xxtv9IvwrLlzf4ZH9goNk1s9mMNPhrXYseYFzONhudjrYSixLGkYyHseea+oQs484Z+PDep/wa8DEtceFP164wKpDOrLwkIWmkH45sUwp3IK1q4hbrSI6W25BOBgXrl4nhiJRTDLSagSsZZUkLhlFS0of0Imt3mFsemhxwKmIEFGR+qE0LDnwnEwuv3nMAusjQSdNz51pWFI4ktARDzVowanWmpZG0kDUIe3O0yoyGVyytFJsX1VVG8H5IM8IB1ld6RIUWekUGobL3K5GfQ/6tMU/wh+tfzv+DReu1z0SJtUrVZ4/UHlbOiERmrj1VzKmlDSdjf6IGhJeq/cCh2gVmVwwagtEtDrk8wshopUQh09JDwXMVJxMApN0yiQXGlxRbYGIVkIcPiU9FDB9VB1Dggx1yqQXxpvCSBdoRV9DiGSQBttkBhjDBA5XtCGQpl53R7UFIloJcbiiVWRywfRRdUxFt9JNLrR0Lm6BiFZCHK5oFZlcMFNxckW30k29kKofChyiVWRySastENHqkJDrcTuBYyPhLVmzAI1NuqF22JZ5SWKSVhFt5t8vxn8Xfju8UnBtdF4sqa6ULbFPsas/H5MEO9nhLFrWcTsHtnaDepW1NaTVp+JJlfS4Qd2+t9gn1EOKmpBWh/jqUqyH6DgTDUvG7B7s8+ohRU2oPRXXdjqLlRhSumSMM0wo8GwwnV6JrVAcak/F6aGHDL/qDVd0FNH7IpAOms5ieKtFevOWyyw54IqOHI+dkjOhF6U3BSatZFyCrdssObCfTzxJe8wlHQ+lb9IqMnkItm43fKIHA9dxTT74JHbxkayGlugOheyJipDxnSBBvDJkz1+HNxf+EvwNF/6138gXuOy88fSDc+I1XOGan0Yxscu8IvO9DQ67cExYxQSXO41i4DIveMxfNx6Twz7CxjW8zAuuue+CECbVJTTShCK+FF8MyBQHllWkvqQINEaYC1bVDyQH4tIqMrlgpsaHgk60ipgPurqnSopLd2ksDWsVmVww2dnEhwLD7lQR84JNqy0Q0UqYCHlO5+2hgJkl4Ie0OsS8YNPqU2nPToikhwJmavyQVoeYD/LhMgWz6BIaaUKRpASdkAhN55CbVNFm/v0s5uzjXAbpP8ZX9jwwn/SBt8N3+NKm78Q//kLf2jxy3ucR8n+DYut7ZOtPYR841bUw+db3yP6fwj5wqi8usuC3BMTR4UnZB5G2+oEEPYiecsLWdeKNtOeYVp+KXZV08NQb+4TaEiwdCWt1iHYaT8+uzQ85sc+rLYO6Unna/vLW8N2w5EVvuAJMR9eGyOqpOG1Hmc4gDU3BZ3rj2gqsYmItaUrwUtSfOsWngXRHEoPJcOUDbL1mh/0ycib9ZfzZCx3yEP1V/B+Ad9t8dkKewwAAAABJRU5ErkJggg==" alt="Font7x13B font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font7x13B;

impl MonoFont for Font7x13B {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/7x13B.raw");
    const FONT_IMAGE_WIDTH: u32 = 112;

    const CHARACTER_SIZE: Size = Size::new(7, 13);
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 10x20 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font10x20 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font10x20;

impl MonoFont for Font10x20 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/10x20.raw");
    const FONT_IMAGE_WIDTH: u32 = 160;

    const CHARACTER_SIZE: Size = Size::new(10, 20);
    const BASELINE: Option<i32> = Some(15);

    const UNDERLINE_OFFSET: i32 = 15 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 4x6 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAAAkCAAAAAAcZ2NOAAAB9UlEQVR4nKXQC1IbUQxE0bD/RTvnSm/wGExVKrR+rZaeBvPxJzw++BqSjzYhVSbzVzw+VjFo4YECDWdK9DVecDsQevTY9xuhKhw3a6g9NX3JgQHBEepmqNzjhloOMeiRIxoCigwtpuUTQeUtf0HqC+bpJ+487JkTfb31quazNntim3Kj1i22M5KcxWzJl6G1E3sYlzx+NuP6OSjqN2oWtQuMuy3ziUkba8vKhgu0j1TDkISNRE+q6RW82EZs2ixV/hn2v2HPm1Suun/NED+AxQ3XlpVz9J3N+ITMl2WYg7GMswQfmeH0eDF8yB44NG2CcYL8amnFGsIve+qZyV5SmtSKtD5aZfqZHS6z5v4Hyq/gohsuba0X74w+812TBBDuRpFiX+0M8kMlcHmbKjKhWzPXjPYtDqzVr8XWWbOCEfIZT+IXGh+N+SDiNyLVNIYcTWimHBxm4z/QUVf37Slwsa2TT6rcvx50ZCpHpYl4cfSDOTHO0nG5etmNOy6vzdqplu4zStplh59Fm3Ve4FKm8Kc1MbM4j6TP0Etjvnjn8hipwc/ozoU7v+O76urPMFyPTvEnKdL01bR3RvfI7gYFIUbGmgsT/jQNl8pnvouLEY2YDls/Zq4hX/UWB9ZkC1WlxjJSoWZpz2MnDmb8orxB9xa7t3nxF3CEGEO2QhWnAAAAAElFTkSuQmCC" alt="Font4x6 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font4x6;

impl MonoFont for Font4x6 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/4x6.raw");
    const FONT_IMAGE_WIDTH: u32 = 64;

    const CHARACTER_SIZE: Size = Size::new(4, 6);
    const BASELINE: Option<i32> = Some(4);

    const UNDERLINE_OFFSET: i32 = 4 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 8x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAABOCAAAAADNt0HTAAAFdUlEQVR4nO2WAXIbOQwEk/8/2tc9AEiutE6iqyR2pdwGZgYkd7WSFd99//bBfK4HeMuEUic1qvZiD5VKX+N6TU0otf3bG21mouCNEWqACeMvcL2kJpQ6PYZbgFLxMN4BpaD0h7x9v5y5XOH79p2zOuvte6HYU6X+gHr6EZw4zzCeMFLAo5RH8RqHWQYjp4FrUWzxOLvCSC28fJHztd3vOAKOk2XnSrkv1PQ+tR9pamVgoqRej6bi6c2aEhAKSt+lt0vv4ACF0zh5P4gLG1bC+CPc4bKTcxFoC7P2SNYVG+WiSkNNpTd4wUE9z5xu+5PMSy0uC2f+EL4ewAdYX5L65fAlcUFLEHeygBswFDpgy2/w8sUauCB/CVmgoB4kOQLt2ekctwWnLn7Dw2qPmv8t0CP1zxwbOcxgQ1sw2woVh9LlbZsstNxeOKlUS+Ujw1kqEkeo+qTQ2LgbwASdy/S6sKQVZxnG6kwmgjciAQM4Z4GQT4oUbY880GtaXRJp5Ubl4Mp04GMwbimNwPjJzVovxY4bXrQw88aAAMxUvKU0AuMXavG4TS20546UxCIQj5TZChVPQyzyZN5deh6uI0N/5Jx3jOEJmOSSmZcrsOayHfq+MEfDHpLs301u3PpTfu3UH+TrAXgAvjSaQTvGhLa4aTxBoyPOLiRwLhGxoez4Q1LHrgepSyvUark4QsWhFAzTJWP7ASh9BevZIY5Q4dYjrWCgKSjlDeMIhfdmPwAfBlIL1mqJI1S49UgrGPq9GhBt/zBYLhqIVS74NDxOYogja52OK+CcBWJZhbxTcQAstSWLRBKHcx+uYVotcYQKh3MJ4hxpBUPuKxerVV6OlJEXJjObI9R2iCNUuDhCxaEUDDQFpW31AJgVrwWII9RqhVotp/vkekmpcnbJ2M0DcBc/SXOkRzJNaIubOFKw4N0oMsLMShzNUoQZPAr7AbKOUR/L1wN8ggfwy4Ctb4WmJ/R2QtbqS4RbsBY4AQx1AZNBo0PPhGjDBY4Idestpev1eh9t505QQ7kNbSF5Lin6Rgj17KFStC9W7ChlU2mJR2AcjNNNXahQzx4qRRFqdel6qhiUl8IKR1pwQ1RzT+k5QSNoS2hqnOIgAmZpb4OVVthcnxrZ7wMSI6UKTdlVtAq3JhPHT+ZiNkmrmqRIqZKD2q50ybLl0rntQt0N8NyaWn1IqZJTZfXDwFQ9CitIDaVX+gbAr9zfZWalZ0L0ecZoClQbpXPQgBXsXebNdYlzfwufB/hfsg/mUzzAX/zgn8lrX57gMgyXRX59l/EcDtb6/KG65XlrXXdyLprtYedrsoudnnneuj19Lp4ZHsbN3tjpmWOLT9aRvy3xmfX8tcBrVoDZ9XWehRgksZdu9ZTr6zyGmw446qE4plCrFa/rI9UzjIrJV8LASUllMSmLanHeGIlBXJn9DKX2Ia1iyiuJQzSFzKCroWdrO8QRqpjAA5kyRlrFRFMSQ1LIDHEtkLknG7QDtdw1PWJDW3mkVUzTrUgKmSGuFb4jFjhCqHncdb1mBXrOqDjPwrTS6yTg7rriOkb9P7xHyQOu2b/Ir598pN/BM2zcrt/zwtE/wz/9AOt3UV+JGZmoYad3OU6/xL5uJYP/EPRmp3c5Tr/Evm4lA00tiDxQOcrWjM66prORplarcYQqx/YFYcL4CUv+nWPP3Y5pKp4WnaZWR6vSJWW5V+Eox9KClWzXXkUF2tsq0FV2FdFrKyk1Y1GYMH6Be9DsuduR5ZBxzXq1inUdx3aCNlhphQPvwW+NHdUTNsSU51Zj/hAsFq7XLz/SCgd9dWO2hRuyC4Tlbtqg0VQ84JwCZ5JGiK3xSm45nPlVcm3kNX7bA/gOb9/iT/gPh0EGicE6xj0AAAAASUVORK5CYII=" alt="Font8x13O font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font8x13O;

impl MonoFont for Font8x13O {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/8x13O.raw");
    const FONT_IMAGE_WIDTH: u32 = 128;

    const CHARACTER_SIZE: Size = Size::new(8, 13);
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 6x12 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAABICAAAAAAsDK+/AAADyElEQVR4nO2VAXLbOhBDf+5/6Pz3AFKkFDlOW6cznQm0C2CXS9KOPO3bf9+Mv3zB+9v7amx2YHQU80s45rJjv+DdxXczISof+THe346BjPaCsMSTC3xYxUEb26/SuAMLxxJ+gI/Oh+9WgBxeTKsyCsfegoVjCT/R8/bNoYHpoxKDHQW7p8IRhcVAL6gQdTFiGpXz4NgbpG1ewC76HulDGV6DtbLrVRvbWfVpm0GqYuxyEeMwiRuo2zlgBi7iuxa6Rwdu4IJ5Qs68wX33hfi54Cm4oK8nb62/HAxCtbwzwyNpyJ8jM2xo4NRk2ZYKhmi0k2nsWI04yX/sUCIpolJO4UOsa+wZZJlldLA1FqgkQBhKJEHEHRhCcBwcb4eijDioD0sHUkkMJZKTVU9gFeDRa05esJ679OWcTiQXmRJBls8ZGp89jFbAci5lnas5jkaFDj24vpzG8Nl04hs4T/wpHp4ffLb2Evxc8BTzNxOGYGV6mf/NoBaQA4geh1DRkVWznMV9mUxAcAMCi9uZCZAjAL8oAz8nES9q0YAEMv2VZwKkU6ach08O0+kFPizaDbEAkPbRcvp6ilEpHhcDZHaNR4+wBbZoQAKZ/mDOgnVJgHTKLKc+JiOaZAKCGxBY3M6WewB5zpRZ4QtgLLD40YmX+1kABR08GQ9LSRq0hg+zyjL2e/FzwVO84oK8zIHhecUDeftUSF47rNSzavmYGXGUoiQ0LMFBXOsz7/mI2S8ToBwlkmKMXlmozZ0JaLDanLx0IF/TXa7U+9eBqWfu3Dxx6CoT2WZ3PhRA3TIL0pAxURo8tXyg03b3uGYOlXzSORIMGVo+kA2Af1WOzduf6OwdIFbSZFkbxIQWztXvgXuOY3YfnKuXgrs4/xsvKP7CBZeX8mpw+rphuWLV/MhgMXtLh1sjJ9gcE5sZOGoMEUxdmJ2pZ8wmr5xDMDSQMgLrSVgPwQgEUBePTMm3cpnR0//JtKlu2UnULDdFOoTOzMM4nBoKOCOX05JVM20NKtEJNwWdjOjMPnUyJFrcM4hI7cwU6RA6s0+dDAV8DL6BFF9GYD0Jk2ObHgPmQe1Y+awO9lfAth2UxKd4snxCPtEZvs7P8Wz9j/GPXJAXsWjHtd7wYfYhOrnzwqXccR19jE7uvLD99vFUh4dgy8fMSDaQJ4aIYI6WmYbxQm3uTITiSyZY4ggSxLQsAz6Yxqo52Ud3zo9MEsXYsrjQjTy+ljTEhTKzU3Ze4M8C0Ve6GEtiOK5dE0KIlTSZiT/zgnsGlvsCzsOpFu34zQu+Pvs/KpcwdgSR2pYAAAAASUVORK5CYII=" alt="Font6x12 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x12;

impl MonoFont for Font6x12 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x12.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 12);
    const BASELINE: Option<i32> = Some(9);

    const UNDERLINE_OFFSET: i32 = 9 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 7x13 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHAAAABOCAAAAADd+81KAAAFI0lEQVR4nO2WW3YjOQxDk/0vOnMvSEoqPybOJCf90QOTAEhRku1y+vT72y/jT174Ef/xXnqBLbMxNhp6FR/vx3Tt7Atb0BBCvH3UG9SCq7yETy/8IFE8YRJRUDwKE9HnYPFYplrwItb6WhCFiMGyZa6f/iFcIxpWG30VFyuU0KqCagkdSy4qVA2ro8wg0bAarMm5V4JLC8uXefED1lSQclBney9CtQIajI1CRPQZail0jzk5DFkjKHWjfcsVvNWz7YwJikU3HsCVJOPqID50h/6CG26cweKfRh++cNTb/RL+jgvnCeeb5gFTyiqwm7L0nQRVIMUP4L5G2ZrEUhJoF00tu7sE4AjU1bI3uPQsTGg2qhBcFQgf3WahM88LQ6C0eIGSUJcZEjFDCb4OhY6IKaI9JI9mUQ/KDvcxmxCazXSI8e7H4CCagHUojRBQzSvSkY7hEAdEADdgzaAWKAkQDjWfuOukEcqhZTcVuBBmhEylgQgQDjVfkNbenrI4+wkgmyASkgn1FM2Fmr0kpsvCWWD7uyL5vBCSyi59pKsSG6IrCWyNYFrB2Kj5I8hpzf+GTwd+Gn/FhTxvBYUtRvOTWHL7i8rj4kcLUwFc/YRpKppQiwNvxz9tFZ0yYpAyOKWbkhlgCJMQfgQy1d5AqC0GKQt1VQ9EMgNMRRJwmUng6xTI4KMiU5KyUFf1QCQzwPRsSOEuiJfeJhQXqhKBrYDKGk7TQgIry6ljuACjE+7kYCoJScsXY73fLg1YqKvawmw1JTPA9GyopVrcoFCp9hBLg5SFuqpTuimZAaYiCSJrsqVUV0SQMmKQMjhkv+HKEEmYhIj09iWsonCVFlTThEf6CkCJT4XAjKQJkxAFWCMljjBj8av4/8Kvoh7U4FoF9SyR6JOKJ24nPNX8FhQyk0xgQXEQGyr0SG28k8omZuHLBpJICpQAxYG2hgs90q2riJgiZuNMqYKUhWqCFoAjkkGV0AMRMUXMxplST7VohpaA7QrsgxUX2AxT2YVRuclXmfqjNsFFirdu08hOEOEoXphAZy6qRbh2YWAVujAYM7rQm1jANFEKjbmIizRwMkEtNY9sM7rgFQLxOgQ1oslFXoOBkxSTRWB0uZaNXCF4aH2KVFUs6Mpr4KKWSpYE+yyCdi0bl/pu9cvw4n3IpbKgXPVv4U9c+P0v8ivwsvPG0zd2i8ewPNXhB6tXP7AHuG2vHRurpTEby45BicZ2V9y2H8yt1jLBtWrs5nZX7DbfFxV/YEpXin9xCBUmoKLJC00plzpiFtOJ9CR8+2fhEh0EbiEJlHQxFdmT3RAoW2wDCtLXTOIUKOjz0sgEqIImQgsH5MmDADo79ZIvpH0EEjGrGdGEIhoIcD0u1SaAVgjFnShRPiILS94egc6iQkfpCpyyCaBEstgtKFE+Ige+b7tqqhZaChUmOCrIKkMpEOlp87/8z5utTQeszc/x0tAV9U5vQPOu9xCvTf0gfvzC86OefnDX+CbmQV4UIgqjT7DmXsVsOJWfPi8KMfoEa+5VzIZTSaKhr69a9q8coY+gxYygzCEkYRLqCLxnb+UAG0kWXMJGaCISAcI9Ur3OilgAcwIG4AXvo8wCM3Z6sA4VpcUlWSNIX3bYVCZEBUu3coLvgmadgFUKFBDZDE34sjMTywzf6wZ762PLeByIQAQZZqYdXGqw4L5Y0HKnGw4v1EmCY7ghisiumLJqFilMgHXvnW5k8+D0LyJbQgVuXsXpF757oac+OPY5/gEEB/p2xvUjJAAAAABJRU5ErkJggg==" alt="Font7x13O font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font7x13O;

impl MonoFont for Font7x13O {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/7x13O.raw");
    const FONT_IMAGE_WIDTH: u32 = 112;

    const CHARACTER_SIZE: Size = Size::new(7, 13);
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 5x8 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAFAAAAAwCAAAAACjjqIrAAAC5UlEQVR4nK2TC1YrMQxDYf+L7rtXcpJpKeUdqMaW5U88mXL4/HjA7fNGTQ+2+F+seQ9mV8kGEZcJQ/or3D7XgJOeZhVATMpDLunhWvwLvi4cYqd5vayXbzajHkFtVZEBq1zm2vVQEBWwI/2SVvAIJBG7R1YwZENNVlTIlxtWClKs50QS0RV60DRQjGPxkvAKSfTfIyuKJSe8DyzMbskfhKvDplWnpipdwWDQCDPotztooBxvipBAh0rpUpLJBioHaMNqirJzKXGvodSRpDXUzolnBrKojKeBPDOUMX2sjQUqG2qcGU/B2o54LKTpWCOnEHKyvAQVEJRU516Zm9omfNSmFodXhBN/A84+BeW+8F3gGzWX8sls3gq6/gwZQykhtcSBURrNSAbIVfeU6VBSLC7tgqfZgvFQgXm6kAaBPrTSUGo4sZraSXCHIcprISCSQ8xclcRxFBaXqEknWSPlbyjToaSYPjp0unshvorcEglFQZkBqdWoGfWcyAhKossfhZ70NrxzV+C1Weotj4JoUHlC9ugakqZArEYiMniltGNfCY5jcUFsDxiTDGkAjh3iInSSwPXSDkE/mWFGVdOFYiUKRDyNeOmOC49YYRxFlCwsY00WqncDlBtCAwdBztVQ5FCURHL3Jvph0slVgy3p/BVs/uA/5c3gxmvnFmInS0xsyC8A/EEeQKG9E4OVrKMI6GDSCRdMgc/nD53Mt0YqekKVEgwRfA2mUyLdXQKwThnSdFP6GZwUSguQ2lBgPCSwZHXXIU0nZSgS4jQCi8tETFc7C0sURY7gkdSVCiYoGRBaGiRqjFTtaNL1yT/AwSs4+y1etH7AesuKg7tk49UVBh0JS3w2DMqPmOZLZEaqy4Lf06XyFPkEM2ZqPqlGSwxBFCnLo4uZCTWxS9Rj+6GRQloX0g+YY5A1KKIgcpyHgO1nzVHrFAiHFpiEASeico4MpwvHfRIk+hZDFIwHzgQrvsbD6Sf4B4gljU/isuhYAAAAAElFTkSuQmCC" alt="Font5x8 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font5x8;

impl MonoFont for Font5x8 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/5x8.raw");
    const FONT_IMAGE_WIDTH: u32 = 80;

    const CHARACTER_SIZE: Size = Size::new(5, 8);
    const BASELINE: Option<i32> = Some(6);

    const UNDERLINE_OFFSET: i32 = 6 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 6x10 pixel monospace font.
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAAA8CAAAAAC8v+BoAAADpklEQVR4nN2S4VobMQwEy/s/NJ3ZtXxOIISvDf3RjTVayTr7Qnj79cPaF7zr3t9MJanoUkcyvqH3tz2XJ3r0ordr+2GXChw843N9egEnc3bsdAQKA2ReG8FnYmNv4Ud9jHsgxWBpPJkRhkoCnZ4Kx6osqhnKPRukaox5tYOPStu4Vx/jnjmakjWqlWdA5kd2DM9YahnlRFOC5bOmJa1xp6O1Xg5HfpV63tJVbPNT+hcX9OuE/dthSFSX50cgUFIa8k48UNVkhgfy+3kCtlHaMqOVNNphYzhJaRtr1A99lCzSAtc19lwEzIbvuEiXIO8idNScgN2F1K44PmREh5aeRgpiMbhkaewxo1AeiVLqvUZ3xfCS9X7KIvBJM6lRGoJFlK7kxAGObQqjy2IsenNpQYsmrF8kYL2ATQJNHmNi/Y08onikr/Zeon9wwfF3xOpJaS/e/jaZI+lxJCo60myU2ZxtFoAsALsAutjOBCLtheTMlCb+IyBFF1Ck8fecQKROGZCjsqjt5IJ+LNjPFpMtGU2nTF9PsSqTx8Ugnjg+1iQH1mP0qNlApPGbjEJdApE6ZZSp92SSJpEFYBdAF9s54lxIzkzJznp1AotfnXjZd0EUdPBEPBQJGrSWD9llG/uz+g8u4A/lLaT80aCpnl3Lx2Sko4AOiieJaEY/8IxH5HnJSq40HYjiWp9U5sZJFlg0N4bJrITK15xW/fk1V7BVkMhsngzu0yiPtOvjZmU+IhtipTVRLE4utzpNtx+M6z66xTCrnR1opWWCS3kA5c+iJ+LjwOUb5Qp22CZRIHrbj46K7ZeKuzj/xYd+FBe8+s1v5eH7hm2WrppfF6rpXXm5a+RGN801urVrDCuafGk6k281TX4RD8lrxEPfbXkCEvTY0ANEdnNHyj0DyaO2Ib2QEurZpVx+HE1Ep43Aik9mWoMoZ7QlzYbtTJMEnbCh6DiB7FDlUyeBanHeDCmhauKkdiZUJy2ElZ86CaL1MGfGh9QwnoiYwBK7f9shE2U72O+KRxKHKFlf6sn2jfJGt+Jrwq/0bP+v9fiCZ9/9VL5IHojTsqKVPtFMfEMdLU3+zKQIk0sBbjwE0pL+AzLDkHYQEqyKIStqDKteJYl2hqwgvvAxpEWrGjnN1kXEi2FSCCJbgkRO5CCySgoWL11vjYFLuCyxB8BKDGSapZrKSSPGmKUdh9HSJmgFdg1AYiWkkw45BYi6Uz6zdLnnctZ4rj+8gDe9f9UH+g231TdqFMbfogAAAABJRU5ErkJggg==" alt="Font6x10 font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font6x10;

impl MonoFont for Font6x10 {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/6x10.raw");
    const FONT_IMAGE_WIDTH: u32 = 96;

    const CHARACTER_SIZE: Size = Size::new(6, 10);
    const BASELINE: Option<i32> = Some(7);

    const UNDERLINE_OFFSET: i32 = 7 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
/// 8x13 pixel monospace font.
///
/// <img src="data:image/png;base64,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" alt="Font8x13B font">
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Font8x13B;

impl MonoFont for Font8x13B {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../../../fonts/ascii/raw/8x13B.raw");
    const FONT_IMAGE_WIDTH: u32 = 128;

    const CHARACTER_SIZE: Size = Size::new(8, 13);
    const BASELINE: Option<i32> = Some(10);

    const UNDERLINE_OFFSET: i32 = 10 + 2;

    fn char_offset(c: char) -> u32 {
        super::char_offset(c)
    }
}
