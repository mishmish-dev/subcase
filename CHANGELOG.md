# Changelog

## [0.4.0] - 2023-04-10

### Changed
- **Breaking:** Raise mininum support Rust version to 1.59.0.

### Added
- Support annotating subcases with `&str` literals (resolve [issue #10]).

### Removed
- **Breaking:** Abandoned support for functions returning something except `()`,
  removed `ErrTestable` trait.

## [0.3.0] - 2023-04-07

### Added
- Inner `subcase!` macro can be used without double braces.
- Add `with_sections!` macro for using `section!` to define subcases.
- Add `def_custom_macro!` for customizing the inner macro name.

### Removed
- Abandon `#![no-std]`.

## [0.2.2] - 2023-04-02

### Added
- Support functions returning arbitrary values in `with_subcases!` macro
  (resolve [issue #09]).

## [0.2.1] - 2023-04-01

Minor documentation tweaks.

## [0.2.0] - 2023-04-01

### Changed
- **Breaking:** introduce custom container to avoid allocations during test case,
  placing hard limit 16 on subcase nesting (resolve [issue #08]).

## [0.1.1] - 2023-03-26

First proper release.

[0.4.0]: https://github.com/mishmish-dev/subcase/releases/tag/v0.4.0
[0.3.0]: https://github.com/mishmish-dev/subcase/releases/tag/v0.3.0
[0.2.2]: https://github.com/mishmish-dev/subcase/releases/tag/v0.2.2
[0.2.1]: https://github.com/mishmish-dev/subcase/releases/tag/v0.2.1
[0.2.0]: https://github.com/mishmish-dev/subcase/releases/tag/v0.2.0
[0.1.1]: https://github.com/mishmish-dev/subcase/releases/tag/v0.1.1

[issue #08]: https://github.com/mishmish-dev/subcase/issues/8
[issue #09]: https://github.com/mishmish-dev/subcase/issues/9
[issue #10]: https://github.com/mishmish-dev/subcase/issues/10
