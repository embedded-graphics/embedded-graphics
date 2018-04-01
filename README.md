# Embedded graphics

[![Build Status](https://travis-ci.org/jamwaffles/embedded-graphics.svg?branch=master)](https://travis-ci.org/jamwaffles/embedded-graphics)

A small 2D graphics library to (eventually) use for drawing things on small graphical LCDs, like the SSD1306 OLED display.

It currently only supports monochrome displays. Contributions to support full colour as well are very welcome!

Examples for the STM32F103C "Blue Pill" board can be found in the [`examples/`](examples) folder.

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
