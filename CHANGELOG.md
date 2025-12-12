# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added `GUIDELINES.md` defining library maintenance standards for v2.0+
- Added comprehensive documentation to `Forge` component with module-level and item-level doc comments
- Added proper error handling in `Forge::new()` to catch mutex poisoning

### Changed
- **Breaking**: Changed `OrchestratorToExplorer::CombineResourceRequest` from containing `ComplexResourceRequest` to containing `ComplexResourceType` via named field `to_generate`
- Refactored `Forge` internal state management - moved `ALREADY_CREATED` mutex into `pub(crate) mod internal` for better encapsulation
- Improved `Forge` test organization with helper function `reset_flag()` and clearer test names
- Enhanced documentation throughout `Forge` module following new guidelines

### Fixed
- Fixed `Forge::new()` to properly handle mutex poisoning instead of using `unwrap()`

## [1.1.0] - 2024-12-10

### Added
- Derived `Debug` trait for `Sunray`, `Rocket`, `Asteroid`, and all message enums
- Implemented `Debug` trait for `EnergyCell`

## [1.0.0] - 2024-12-08

Initial stable release.

[Unreleased]: https://github.com/unitn-ap-2025/common/compare/v1.1.0...2.0-development
[1.1.0]: https://github.com/unitn-ap-2025/common/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/unitn-ap-2025/common/releases/tag/v1.0.0
