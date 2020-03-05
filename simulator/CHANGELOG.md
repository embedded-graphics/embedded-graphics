# Changelog

[`embedded-graphics-simulator`](https://crates.io/crates/embedded-graphics-simulator) is an SDL-based simulator for testing, debugging and developing [`embedded-graphics`](https://crates.io/crates/embedded-graphics) applications.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- **(breaking)** #266 Added [image](https://crates.io/crates/image) support and PNG export. See the `README.md` for information about how to use these features. The API for creating windows was changed to make the output settings independent of the `Window` type. The pixel scaling and theme settings were moved to a new `OutputSettings` struct, that can be built using the `OutputSettingsBuilder`. `WindowBuilder` was removed and replaced by a `Window::new(title, &output_settings)` function.

### Changed

## [0.2.0-beta.2] - 2020-02-17

### Added

- #183 Added limited mouse and keyboard event handling to the simulator in order to simulate input devices such as touch screens, buttons, or rotary encoders.
- #171 Added a more complex `analog-clock` example to the simulator - [check it out](https://github.com/jamwaffles/embedded-graphics/tree/embedded-graphics-v0.6.0-alpha.3/simulator/examples/analog-clock.rs) for some more in-depth usage of Embedded Graphics.

### Fixed

- #192 Performance of drawing in the simulator is increased.
- #218 Test README examples in CI and update them to work with latest crate versions.

### Changed

- **(breaking)** The simulator API changed.
- #203 updated simulator screenshots and added them to README

## 0.2.0-alpha.1

### Fixed

- The TGA example in the simulator now draws the image correctly

## 0.1.0

### Changed

- The simulator is now [available on crates.io](https://crates.io/crates/embedded-graphics-simulator) as a standalone crate. You can now create simulated displays for testing out embedded_graphics code or showing off cool examples.
- The builtin simulator now supports colour pixel types, like `RGB565`.

<!-- next-url -->
[unreleased]: https://github.com/jamwaffles/embedded-graphics-simulator/compare/embedded-graphics-simulator-v0.2.0-beta.2...HEAD

[0.2.0-beta.2]: https://github.com/jamwaffles/embedded-graphics/compare/embedded-graphics-simulator-v0.2.0-alpha.1...embedded-graphics-simulator-v0.2.0-beta.2
