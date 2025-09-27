# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - Unreleased

### Added

- RP2040 examples dir readme

### Changed

- Standardize formatting with `rustfmt`
- Mark functions as `const` where appropriate
- Clippy suggestions

## [0.4.0] - 2025-09-26

### Added

- Chip variants
- Constrain INT2 functionality to Pcf8523TS and Pcf8523U
- Disable CLKOUT when interrupts are enabled on INT1
- Battery low detection
- Battery switch-over
- Add reset, minute alarm and second timer examples

## [0.3.0] - 2025-09-03

### Added

- Standardize driver return types and datetime field names
- Add missing driver tests
- Enable/disable correction interrupt
- Enable/disable/clear second interrupt
- Clear alarm interrupt
- Enable second interrupt as pulsed or permanent
- Timer B
- Timer A

## [0.2.0] - 2025-08-23

### Added

- Calibration (i.e. offset)
- Power management
- Enable/disable minute, hour, day and weekday alarms
- Enable/disable alarm interrupt

## [0.1.0] - 2025-08-22

### Added

- Initial lib release