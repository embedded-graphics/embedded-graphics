# Embedded graphics simulator

![It can display all sorts of embedded-graphics test code.](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/simulator-demo.png)

The simulator can be used to test and debug [embedded-graphics](https://crates.io/crates/embedded-graphics) code, or produce snazzy examples for people to try drivers out without needing physical hardware to run on.

# Examples

## Simulate a 128x64 SSD1306 OLED

```rust
use embedded_graphics::prelude::*;
use embedded_graphics::{icoord, circle, line, text_6x8};
use embedded_graphics_simulator::{DisplayBuilder, DisplayTheme};
use std::thread;
use std::time::Duration;

fn main() {
 let mut display = DisplayBuilder::new()
     .theme(DisplayTheme::OledBlue)
     .size(128, 64)
     .build();

 display.draw(text_6x8!("Hello World!"));

 display.draw(egcircle!((96, 32), 31, stroke = Some(1u8.into())));

 display.draw(egline!((32, 32), (1, 32), stroke = Some(1u8.into())).translate(icoord!(64, 0)));
 display.draw(egline!((32, 32), (40, 40), stroke = Some(1u8.into())).translate(icoord!(64, 0)));

 loop {
     let end = display.run_once();

     if end {
         break;
     }

     thread::sleep(Duration::from_millis(200));
 }
}
```
