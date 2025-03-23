# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

I'm new at this, so expect imperfection ;_; I'm trying!

## [0.2.7] - 2025-03-23

### Changes

+ Massive performance improvements by implementing an async runtime

> Average runtime reduced from 571.6ms to 218.3ms!!!

## [0.2.6] - 2024-02-24

### Updates

+ Updates arcconfig depenedency => 0.3.1
  + Versions will be more explicit from now on

## [0.2.5] - 2024-01-23

### Changes

+ Updates arcconfig dependency => 0.3.x

### Documentation

+ Fixed incorrect dates in changelog

## [0.2.4] - 2024-01-19

### Added

+ Added `--all` flag that displays all games (#17)
+ Added long help messages invoked with `--help`
+ Added some short option invocations
  + `-r` for `--archive-root`
  + `-s` for `--systems`

### Changed

+ Changed file navigation strategy for improved performance (#16)

## [0.2.3] - 2024-01-17

### Changed

+ Updated arcconfig => 0.2.1

## [0.2.2] - 2024-01-16

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
