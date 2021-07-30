# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `derive` macro which automatically implements `if_empty` if the type has an `is_empty` function returning a `bool`

### Changed

### Removed

## [0.2.0] - 2021-06-19

### Added

- Implementations of the traits for `OsString` and `&OsStr`

## [0.1.0] - 2021-06-19

### Added

- `IfEmpty` and `IfEmptyBorrowed` traits

- Implementations of the traits for `String` and `&str`

[Unreleased]: https://github.com/cschlosser/ifempty/compare/0.2.0...HEAD
[0.2.0]: https://github.com/cschlosser/ifempty/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/cschlosser/IfEmpty/releases/tag/0.1.0

