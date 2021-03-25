# Changelog

`embedded-graphics-core` changelog.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- **(breaking)** [#552](https://github.com/embedded-graphics/embedded-graphics/pull/552) Added the `Output` associated type to `Drawable` to allow returning non-`()` values from drawing operations.
- [#563](https://github.com/embedded-graphics/embedded-graphics/pull/563) Added `is_none`, `is_text_color` and `is_custom` methods to `DecorationColor`.

### Removed

- **(breaking)** [#569](https://github.com/embedded-graphics/embedded-graphics/pull/569) Removed text renderer API. The text renderer API will be added back when it has stabilized.

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

[unreleased]: https://github.com/embedded-graphics/embedded-graphics-core/compare/embedded-graphics-core-v0.2.0...HEAD
[0.2.0]: https://github.com/embedded-graphics/embedded-graphics-core/compare/embedded-graphics-core-v0.1.1...embedded-graphics-core-v0.2.0
[0.1.1]: https://github.com/embedded-graphics/embedded-graphics-core/compare/embedded-graphics-core-v0.1.0...embedded-graphics-core-v0.1.1
[0.1.0]: https://github.com/embedded-graphics/embedded-graphics/compare/embedded-graphics-v0.7.0-alpha.1...embedded-graphics-core-v0.1.0
