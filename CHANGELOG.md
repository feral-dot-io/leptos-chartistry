# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

<!-- Added for new features. -->
<!-- Changed for changes in existing functionality. -->
<!-- Deprecated for soon-to-be removed features. -->
<!-- Removed for now removed features. -->
<!-- Fixed for any bug fixes. -->
<!-- Security in case of vulnerabilities. -->

## [0.1.6] - 2024-06-15
### Fixed
- Panic if Tick::position returns f64::NaN.
- Compile errors with Leptos nightly.

## [0.1.5] - 2024-02-23
### Added
- [Bar charts](https://feral-dot-io.github.io/leptos-chartistry/examples.html#bar-chart) in [#15].

## [0.1.4] - 2024-02-16
### Added
- Line interpolation: [linear and monotone](https://feral-dot-io.github.io/leptos-chartistry/examples.html#linear-and-monotone) and [stepped](https://feral-dot-io.github.io/leptos-chartistry/examples.html#stepped) in [#12].
### Changed
- Default line interpolation is now [`Interpolation::monotone`](https://docs.rs/leptos-chartistry/latest/leptos_chartistry/enum.Interpolation.html#variant.Monotone).

## [0.1.3] - 2024-02-15
### Added
- Application of [CSS styles](https://feral-dot-io.github.io/leptos-chartistry/examples.html#css-styles) in [#10].

## [0.1.2] - 2024-02-11
### Added
- [Interpolated line gradients](https://feral-dot-io.github.io/leptos-chartistry/examples.html#line-colour-scheme) in [#5].
- [Line point markers](https://feral-dot-io.github.io/leptos-chartistry/examples.html#point-markers) ([another example](https://feral-dot-io.github.io/leptos-chartistry/examples.html#point-markers-2)) in [#1].

## [0.1.1] - 2024-02-11
### Fixed
- Fix missing crates.io README.

## [0.1.0] - 2024-02-11
Initial release!

### Added
- Aspect ratio on inner, outer, or from the env chart.
- Debug (draw bounding boxes, print to console).

Edge layout options:
- Legends.
- Rotated labels.
- Tick labels (aligned floats and periodic timestamps, custom formatting).

Inner layout options:
- Axis markers (on edges and zero).
- Grid lines (aligned to ticks).
- Guide lines (aligned to mouse or data).
- Inset legends.

Overlay options:
- Tooltips (with sorting and formatting).

Series options:
- Line charts.
- Stacked line charts.
- X and Y ranges.
- Colour scheme.


[#1]: https://github.com/feral-dot-io/leptos-chartistry/pull/1
[#5]: https://github.com/feral-dot-io/leptos-chartistry/pull/5
[#10]: https://github.com/feral-dot-io/leptos-chartistry/pull/10
[#12]: https://github.com/feral-dot-io/leptos-chartistry/pull/12
[#15]: https://github.com/feral-dot-io/leptos-chartistry/pull/15
[0.1.0]: https://github.com/feral-dot-io/leptos-chartistry/releases/tag/v0.1.0
[0.1.1]: https://github.com/feral-dot-io/leptos-chartistry/compare/v0.1.0...v0.1.1
[0.1.2]: https://github.com/feral-dot-io/leptos-chartistry/compare/v0.1.1...v0.1.2
[0.1.3]: https://github.com/feral-dot-io/leptos-chartistry/compare/v0.1.2...v0.1.3
[0.1.4]: https://github.com/feral-dot-io/leptos-chartistry/compare/v0.1.3...v0.1.4
[0.1.5]: https://github.com/feral-dot-io/leptos-chartistry/compare/v0.1.4...v0.1.5
[0.1.6]: https://github.com/feral-dot-io/leptos-chartistry/compare/v0.1.5...v0.1.6
[unreleased]: https://github.com/feral-dot-io/leptos-chartistry/compare/v1.1.5...HEAD
