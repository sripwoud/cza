# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.4.0](https://github.com/sripwoud/cza/compare/v2.3.0...v2.4.0) - 2025-09-30

### Added

- add --json flag to list command ([#42](https://github.com/sripwoud/cza/pull/42))

## [2.3.0](https://github.com/sripwoud/cza/compare/v2.2.1...v2.3.0) - 2025-09-30

### Added

- add --dry-run flag to new command ([#39](https://github.com/sripwoud/cza/pull/39))
- unified release workflow with cargo-dist and release-plz ([#36](https://github.com/sripwoud/cza/pull/36))

### Other

- add comprehensive rustdoc documentation ([#37](https://github.com/sripwoud/cza/pull/37))
- add npm shield

## [2.2.1](https://github.com/sripwoud/cza/compare/v2.2.0...v2.2.1) - 2025-09-30

### Other

- collapse nested if block in git hooks setup

## [2.2.0](https://github.com/sripwoud/cza/compare/v2.1.0...v2.2.0) - 2025-09-30

### Added

- add --no-git flag to skip git initialization ([#32](https://github.com/sripwoud/cza/pull/32))

## [2.1.0](https://github.com/sripwoud/cza/compare/v2.0.0...v2.1.0) - 2025-09-28

### Added

- add template validation functionality ([#28](https://github.com/sripwoud/cza/pull/28))

### Other

- use shared utilities in new.rs ([#29](https://github.com/sripwoud/cza/pull/29))
- add utils module with command helpers ([#27](https://github.com/sripwoud/cza/pull/27))
- update CLI metadata in lib.rs ([#26](https://github.com/sripwoud/cza/pull/26))
- remove empty args.rs and utils.rs files ([#25](https://github.com/sripwoud/cza/pull/25))

## [2.0.0](https://github.com/sripwoud/cza/compare/v1.0.1...v2.0.0) - 2025-09-27

### Added

- _(update)_ implement self-updating CLI capability ([#24](https://github.com/sripwoud/cza/pull/24))
- _(output)_ integrate development.color config setting ([#21](https://github.com/sripwoud/cza/pull/21))
- _(new)_ [**breaking**] integrate config settings into new command ([#20](https://github.com/sripwoud/cza/pull/20))

### Fixed

- _(cli)_ restructure NewArgs to resolve clap validation error ([#22](https://github.com/sripwoud/cza/pull/22))

### Other

- update README

## [0.1.0-alpha.5](https://github.com/sripwoud/cza/compare/v0.1.0-alpha.4...v0.1.0-alpha.5) - 2025-09-10

### Added

- add comprehensive configuration system ([#11](https://github.com/sripwoud/cza/pull/11))

## [0.1.0-alpha.4](https://github.com/sripwoud/cza/compare/v0.1.0-alpha.3...v0.1.0-alpha.4) - 2025-09-09

### Added

- add cairo-vite template

### Other

- improve coverage

## [0.1.0-alpha.3](https://github.com/sripwoud/cza/compare/v0.1.0-alpha.2...v0.1.0-alpha.3) - 2025-08-07

### Other

- define template module ([#8](https://github.com/sripwoud/cza/pull/8))
- add badges to README
- _(templates)_ use templates monorepo with subfolders ([#7](https://github.com/sripwoud/cza/pull/7))
- add templates section

## [0.1.0-alpha.2](https://github.com/sripwoud/cza/compare/v0.1.0-alpha.1...v0.1.0-alpha.2) - 2025-08-06

### Fixed

- define binary in Cargo.toml

### Other

- add README, CONTRIBUTING and issue/pr templates
- release v0.1.0-alpha.1 ([#4](https://github.com/sripwoud/cza/pull/4))

## [0.1.0-alpha.1](https://github.com/sripwoud/cza/releases/tag/v0.1.0-alpha.1) - 2025-08-06

### Added

- run setup during new command
- format stdout/sterr
- define create and list commands
- scaffold clap cli ([#1](https://github.com/sripwoud/cza/pull/1))

### Other

- define unit and integrations tests
- update Execute trait
- Initial commit
