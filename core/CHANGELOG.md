# Changelog

`embedded-graphics-core` changelog.

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.4.0] - 2023-05-14

### Added

- [#612](https://github.com/embedded-graphics/embedded-graphics/pull/612), [#699](https://github.com/embedded-graphics/embedded-graphics/pull/699) Added `GetPixel` trait.
- [#625](https://github.com/embedded-graphics/embedded-graphics/pull/625) Added CSS colors and color conversions to `Rgb666` and `Bgr666`.
- [#656](https://github.com/embedded-graphics/embedded-graphics/pull/656) Added `Rgb666` and `Bgr666` conversions.
- [#710](https://github.com/embedded-graphics/embedded-graphics/pull/710) Added `AnchorX`, `AnchorY`, `AnchorPoint::x`, `AnchorPoint::y` and `AnchorPoint::from_xy`.
- [#710](https://github.com/embedded-graphics/embedded-graphics/pull/710) Added `Rectangle::resized_width`, `Rectangle::resized_height`, `Rectangle::anchor_x` and `Rectangle::anchor_y`.

### Changed

- **(breaking)** [#660](https://github.com/embedded-graphics/embedded-graphics/pull/660) Remove `RawU18` color storage type and use `RawU24` in is place for `Rgb666` and `Bgr666`.
- [#651](https://github.com/embedded-graphics/embedded-graphics/pull/651), [#652](https://github.com/embedded-graphics/embedded-graphics/pull/652) Improved performance of color conversions.
- **(breaking)** [#663](https://github.com/embedded-graphics/embedded-graphics/pull/663) Upgraded Cargo dependencies to their latest versions.
- **(breaking)** [#689](https://github.com/embedded-graphics/embedded-graphics/pull/689) Bump Minimum Supported Rust Version (MSRV) to 1.61.

## [0.3.3] - 2021-09-09

### Added

- [#621](https://github.com/embedded-graphics/embedded-graphics/pull/621) Added `Rgb666` and `Bgr666` color type support.

## [0.3.2] - 2021-06-05

### Added

- [#602](https://github.com/embedded-graphics/embedded-graphics/pull/602) Implemented `core::fmt::Display` for `Point` and `Size`.

## [0.3.1] - 2021-05-03

### Added

- [#589](https://github.com/embedded-graphics/embedded-graphics/pull/589) Implemented `From` trait to convert from RGB colors to grayscale colors, between different grayscale colors and from grayscale and RGB colors to `BinaryColor`.

## [0.3.0] - 2021-04-19

### Added

- **(breaking)** [#552](https://github.com/embedded-graphics/embedded-graphics/pull/552) Added the `Output` associated type to `Drawable` to allow returning non-`()` values from drawing operations.
- [#563](https://github.com/embedded-graphics/embedded-graphics/pull/563) Added `is_none`, `is_text_color` and `is_custom` methods to `DecorationColor`.

### Removed

- **(breaking)** [#569](https://github.com/embedded-graphics/embedded-graphics/pull/569) Removed text renderer API. The text renderer API will be added back when it has stabilized.
- **(breaking)** [#572](https://github.com/embedded-graphics/embedded-graphics/pull/572) Moved `pixelcolor::raw::RawDataIter` into `embedded-graphics` crate.

### Fixed

- [#571](https://github.com/embedded-graphics/embedded-graphics/pull/571) `Rectangle::points` did return a non empty iterator for rectangles with zero width.

## [0.2.0] - 2021-02-03

### Added

- [#523](https://github.com/embedded-graphics/embedded-graphics/pull/523) Added the external text renderer API.

### Removed

- [#522](https://github.com/embedded-graphics/embedded-graphics/pull/522) The `draw_target` module, expect the `DrawTarget` trait, was moved to `embedded-graphics`.
- [#522](https://github.com/embedded-graphics/embedded-graphics/pull/522) The `iterator` module was moved to `embedded-graphics`.

## [0.1.1] - 2020-12-07

### Added

- [#514](https://github.com/embedded-graphics/embedded-graphics/pull/514) Add `ImageDrawable` to the `prelude`.

## [0.1.0] - 2020-11-29

### Added

- [#498](https://github.com/embedded-graphics/embedded-graphics/pull/498) Split common functionality out of `embedded-graphics` into `embedded-graphics-core`.
- [#498](https://github.com/embedded-graphics/embedded-graphics/pull/498) Added `Size::saturating_add` and `Size::saturating_sub`.

<!-- next-url -->
[unreleased]: https://github.com/embedded-graphics/embedded-graphics-core/compare/embedded-graphics-core-v0.4.0...HEAD
[0.4.0]: https://github.com/embedded-graphics/embedded-graphics-core/compare/embedded-graphics-core-v0.3.3...embedded-graphics-core-v0.4.0
[0.3.3]: https://github.com/embedded-graphics/embedded-graphics-core/compare/embedded-graphics-core-v0.3.2...embedded-graphics-core-v0.3.3
[0.3.2]: https://github.com/embedded-graphics/embedded-graphics-core/compare/embedded-graphics-core-v0.3.1...embedded-graphics-core-v0.3.2
[0.3.1]: https://github.com/embedded-graphics/embedded-graphics-core/compare/embedded-graphics-core-v0.3.0...embedded-graphics-core-v0.3.1

[0.3.0]: https://github.com/embedded-graphics/embedded-graphics-core/compare/embedded-graphics-core-v0.2.0...embedded-graphics-core-v0.3.0
[0.2.0]: https://github.com/embedded-graphics/embedded-graphics-core/compare/embedded-graphics-core-v0.1.1...embedded-graphics-core-v0.2.0
[0.1.1]: https://github.com/embedded-graphics/embedded-graphics-core/compare/embedded-graphics-core-v0.1.0...embedded-graphics-core-v0.1.1
[0.1.0]: https://github.com/embedded-graphics/embedded-graphics/compare/embedded-graphics-v0.7.0-alpha.1...embedded-graphics-core-v0.1.0
