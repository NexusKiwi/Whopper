# Changelog

All notable changes to this burger will be documented in this file.

## [1.0.0] - 2026-11-21

### Added
- Coloured, animated terminal assembly
- `--receipt` flag with itemised billing
- `NO_COLOR` support

### Changed
- Cheese is now an upgrade. We are a business.
- Energy is now reported in kJ instead of calories

### Fixed
- Pickle race condition when two pickles were added at the same time
- Top bun occasionally rendered *under* the patty
- `--no-onions` was being ignored on Tuesdays

## [0.2.0] - 2026-11-21

### Added
- Structural integrity validation (`MAX_LAYERS = 24`)
- `WhopperError::PickleOverflow`

### Security
- Patched onion injection vulnerability (CVE-2026-WOPR)

## [0.1.0] - 2026-11-21

### Added
- Initial burger
