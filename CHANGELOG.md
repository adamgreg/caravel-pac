# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0](https://github.com/adamgreg/caravel-pac/compare/caravel-pac-v1.0.6...caravel-pac-v2.0.0) - 2026-06-22

### Breaking Changes

- `register_block` and `user_register_block` macros now require named arguments
  for base address / offset. `base` for `register_block`, and `offset` for
  `user_register_block` (relative to user area - 0x3000_0000)

### Added

- Add optional `stride` named argument to register block macros.

### Other

- _(deps)_ bump actions/checkout from 6 to 7

## [1.0.6](https://github.com/adamgreg/caravel-pac/compare/caravel-pac-v1.0.5...caravel-pac-v1.0.6) - 2026-04-28

### Other

- Add "interrupts" feature, to gate generation of interrupt code
- Remove unused dependencies

## [1.0.5](https://github.com/adamgreg/caravel-pac/compare/caravel-pac-v1.0.4...caravel-pac-v1.0.5) - 2026-04-28

### Other

- Add mock-user-registers feature, to mock only user (Wishbone) addresses
- Do not trigger CI workflow twice when pushing to a PR branch

## [1.0.4](https://github.com/adamgreg/caravel-pac/compare/caravel-pac-v1.0.3...caravel-pac-v1.0.4) - 2026-04-24

### Other

- Update CI to make sure the caravel-pac-macros crate is covered
- Add #[user_register_block] proc macro
- Fix mock-registers feature checking in register_block macro

## [1.0.3](https://github.com/adamgreg/caravel-pac/compare/caravel-pac-v1.0.2...caravel-pac-v1.0.3) - 2026-04-23

### Other

- Add "mock-registers" feature for use in unit tests on host platform

## [1.0.2](https://github.com/adamgreg/caravel-pac/compare/caravel-pac-v1.0.1...caravel-pac-v1.0.2) - 2026-04-23

### Other

- Add missing keywords for caravel-pac crate

## [1.0.1](https://github.com/adamgreg/caravel-pac/compare/caravel-pac-v1.0.0...caravel-pac-v1.0.1) - 2026-04-23

### Other

- Add release automation using release-plz
