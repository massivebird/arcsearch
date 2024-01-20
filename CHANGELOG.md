# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

I'm new at this, so expect imperfection ;_; I'm trying!

## [0.2.4] - 2023-01-19

### Added

+ Added `--all` flag that displays all games (#17)
+ Added long help messages invoked with `--help`
+ Added some short option invocations
  + `-r` for `--archive-root`
  + `-s` for `--systems`

### Changed

+ Changed file navigation strategy for improved performance (#16)

## [0.2.3] - 2023-01-17

### Changed

+ Updated arcconfig => 0.2.1

## [0.2.2] - 2023-01-16

### Added

+ Added support for system paths containing `/` characters (#11)
+ Added basic GitHub Actions for automated testing

### Documentation

+ Added note about avoiding nested system directories

## [0.2.1] - 2023-12-18

### Added

+ Ability to query specific systems via `--systems` argument

## [0.2.0] - 2023-12-18

### Changed

+ Query argument is now required and no longer defaults to `.*` regex pattern

### Documentation

+ Updates query argument's brackets to convey that it is (now) required
