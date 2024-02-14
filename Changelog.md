# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].


## [Unreleased]


## [0.3.0] - 2024-02-14

### Added
- add optional dependencies: `memchr`, `wide`.
- add crate manifest categories.

### Removed
- remove features: `safest`, `unsafest`.

### Changed
- rename feature `full` to `all`.
- refactor manifest, update comments.


## [0.2.0] - 2023-12-11

### Added
- add optional dependency: `unicode_width`.

### Removed
- remove dependency `devela_macros`.

### Changed
- rename feature `nightly_docs` to `docsrs`.


## [0.1.1] - 2023-10-19

### Added
- add optional dependency: `hashbrown`.

### Fixed
- new private helper macro `reexport` for reexported items.


## [0.1.0] - 2023-10-07

First release.

### Added
- add optional dependencies:
  `atomic`, `az`, `bytemuck`, `const-str`, `devela_macros`, `portable-atomic`, `unicode-segmentation`.

[unreleased]: https://github.com/andamira/devela_macros/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/andamira/devela_macros/releases/tag/v0.3.0
[0.2.0]: https://github.com/andamira/devela_macros/releases/tag/v0.2.0
[0.1.1]: https://github.com/andamira/devela_macros/releases/tag/v0.1.1
[0.1.0]: https://github.com/andamira/devela_macros/releases/tag/v0.1.0

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
