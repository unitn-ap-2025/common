# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added `handle_sunray()` method to `PlanetAI` trait for handling sunray messages
- Added `handle_internal_state_req()` method to `PlanetAI` trait for handling internal state requests
- Added `on_explorer_arrival()` optional listener method with empty default implementation
- Added `on_explorer_departure()` optional listener method with empty default implementation
- Added `try_make()` optional methods for generator and combinator to try build a generic resource
- Added `StopExplorerAI` message that was missing
- Added `EnumAsInner` and `EnumDiscriminants` to the message enums to provide useful features

### Changed
- **Breaking**: Removed generic `handle_orchestrator_msg()` handler from `PlanetAI` trait in favor of specialized handlers
- **Breaking**: Renamed `start()` to `on_start()` in `PlanetAI` trait with `generator` and `combinator` parameters, now has empty default implementation
- **Breaking**: Renamed `stop()` to `on_stop()` in `PlanetAI` trait with `generator` and `combinator` parameters, now has empty default implementation
- `Sunray` and `InternalStateRequest` messages are now handled by dedicated methods instead of generic handler
- **Breaking**: Added `explorer_id` to `IncomingExplorerResponse` and `OutgoingExplorerResponse` messages in `PlanetToOrchestrator` enum
- **Breaking**: Changed messages module structure, now divided in three modules for readability purposes
## [2.0.0-beta.1] - 2025-12-12

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

[Unreleased]: https://github.com/unitn-ap-2025/common/compare/v2.0.0-beta.1...beta
[2.0.0-beta.1]: https://github.com/unitn-ap-2025/common/compare/v1.1.0...v2.0.0-beta.1
[1.1.0]: https://github.com/unitn-ap-2025/common/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/unitn-ap-2025/common/releases/tag/v1.0.0
