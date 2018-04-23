# Embedded graphics

[![Build Status](https://travis-ci.org/jamwaffles/embedded-graphics.svg?branch=master)](https://travis-ci.org/jamwaffles/embedded-graphics)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg?longCache=true)](https://crates.io/crates/embedded-graphics)

A small 2D graphics library to draw things on embedded graphical LCDs, like the SSD1306 OLED display.

It currently only supports monochrome displays. Contributions to support full colour as well are very welcome!

Example from the [SSD1306 driver](https://github.com/jamwaffles/ssd1306):

```rust
#![no_std]

extern crate cortex_m;
extern crate embedded_graphics;
extern crate embedded_hal as hal;
extern crate panic_abort;
extern crate ssd1306;
extern crate stm32f103xx_hal as blue_pill;

use blue_pill::i2c::{DutyCycle, I2c, Mode};
use blue_pill::prelude::*;
use embedded_graphics::image::Image1BPP;
use embedded_graphics::prelude::*;
use ssd1306::{mode::GraphicsMode, Builder};

fn main() {
    let dp = blue_pill::stm32f103xx::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = I2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio1to1,
        },
        clocks,
        &mut rcc.apb1,
    );

    let im = Image1BPP::new(include_bytes!("./rust.raw"), 64, 64).translate((32, 0));
    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().unwrap();
    disp.flush().unwrap();
    disp.draw(im.into_iter());
    disp.flush().unwrap();
}
```

## [Documentation](https://jamwaffles.github.io/embedded-graphics)

## Features

* Primitives
	* Lines
	* Squares/rects
	* Circles
* Images
	* 1BPP images as `&[u8]`s
     * 8BPP images as `&[u8]`s (downsampled badly to 1BPP)
* Text
	* 6x8 bitmap font
* Translations: move an object around the screen

## TODO

* [ ] General matrix transforms
* [ ] Full colour support

## Attribution

All source font PNGs are taken from the excellent [Uzebox Wiki page](http://uzebox.org/wiki/Font_Bitmaps).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.