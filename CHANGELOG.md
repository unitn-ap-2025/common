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
- Added `utils` module with the standardized `ID` type

### Changed
- **Breaking**: Removed generic `handle_orchestrator_msg()` handler from `PlanetAI` trait in favor of specialized handlers
- **Breaking**: Renamed `start()` to `on_start()` in `PlanetAI` trait with `generator` and `combinator` parameters, now has empty default implementation
- **Breaking**: Renamed `stop()` to `on_stop()` in `PlanetAI` trait with `generator` and `combinator` parameters, now has empty default implementation
- `Sunray` and `InternalStateRequest` messages are now handled by dedicated methods instead of generic handler
- **Breaking**: Added `explorer_id` to `IncomingExplorerResponse` and `OutgoingExplorerResponse` messages in `PlanetToOrchestrator` enum
- **Breaking**: Removed `protocol::messages` module replaced by `protocols::{orchestrator_explorer, orchestrator_planet, planet_explorer}`
- **Migration note**: update messages imports from `protocol::messages` to `protocols::{orchestrator_explorer, orchestrator_planet, planet_explorer}`
- Updated logging system to use `u64` for timestamps and the `ID` type for sender/receiver identifiers
- Replaced deprecated `lazy_static!` with `LazyLock` in `Forge` component for modern Rust patterns
- **Breaking**: Renamed `planet_type` to `type_` in `Planet` component for clarity
- Extracted `handle_orchestrator_msg()` helper method in `Planet` component for improved code organization
- Simplified pattern matches in protocol files to use fallthrough for `explorer_id()` and `planet_id()` extraction
- Enhanced documentation for logging channel and log levels with detailed descriptions

### Fixed
- Fixed doc comments in `EnergyCell` to satisfy Clippy pedantic warnings
- Resolved various Clippy pedantic warnings throughout codebase for improved code quality 
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
