# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

## [0.1.2] - 2023-07-21

### Fixed

- Fixed some JSON parsing failures ([#5](https://github.com/traPtitech/rust-traq/issues/5))

## [0.1.1] - 2023-06-16

### Added

- Add description of `GET /ogp?url=:url`

### Removed

- Remove variant `Status404()` from enum [`traq::apis::ogp_api::GetOgpError`](https://github.com/traPtitech/rust-traq/blob/116a053e3ab8e342cfcaf84e9a2ce3bdfe27706e/src/apis/ogp_api.rs#L16-L22)

## [0.1.0] - 2023-06-06

- initial release

<!-- Links -->
[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html

<!-- Versions -->
[0.1.2]: https://github.com/traPtitech/rust-traq/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/traPtitech/rust-traq/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/traPtitech/rust-traq/releases/tag/v0.1.0
