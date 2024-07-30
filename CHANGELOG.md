# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

## [0.1.5] - 2024-07-30

### Added

- Add API for OIDC implementation ([traQ#1887](https://github.com/traPtitech/traQ/pull/1887))
    - Add API [`traq::apis::me_api::get_oidc_user_info`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/apis/me_api.rs#L1154-L1195)
    - Add variant `Openid`, `Profile` in enum [`traq::models::OAuth2Scope`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/o_auth2_scope.rs#L13-L26)
    - Add struct [`traq::models::OidcTraqUserInfo`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/oidc_traq_user_info.rs#L13-L47)
    - Add struct [`traq::models::OidcUserInfo`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/oidc_user_info.rs#L13-L32)
- Add field `confidential` in [`traq::models::GetClient200Response`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/get_client_200_response.rs#L11-L37), [`traq::models::OAuth2ClientDetail`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/o_auth2_client_detail.rs#L13-L39), [`traq::models::OAuth2Client`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/o_auth2_client.rs#L13-L33), [`traq::models::PatchClientRequest`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/patch_client_request.rs#L13-L30), [`traq::models::PostClientRequest`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/post_client_request.rs#L13-L30) ([traQ#2426](https://github.com/traPtitech/traQ/pull/2426))
- Add field `bio` in [`traq::models::PatchBotRequest`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/patch_bot_request.rs#L13-L38) [traQ#2483](https://github.com/traPtitech/traQ/pull/2483)
- Add variant `DeleteMyStamp` in enum [`traq::models::UserPermission`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/user_permission.rs#L13-L182) ([traQ#2474](https://github.com/traPtitech/traQ/pull/2474))
- Add struct [`traq::models::Session`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/session.rs#L11-L19), which is used in [`traq::models::WebRtcUserState`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/models/web_rtc_user_state.rs#L13-L24) ([traQ#2475](https://github.com/traPtitech/traQ/pull/2475))
- Add API [`traq::apis::oauth2_api::revoke_client_tokens`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/apis/oauth2_api.rs#L711-L757) ([traQ#2477](https://github.com/traPtitech/traQ/pull/2477))
- Add feature flags to select TLS backend ([#26](https://github.com/traPtitech/rust-traq/pull/26))

### Changed

- Change the format of the parameter `user_id` of [`traq::apis::channel_api::get_user_dm_channel`](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/src/apis/channel_api.rs#L900-L946) to UUID in [documentation](https://github.com/traPtitech/rust-traq/blob/f53385bfd7a9cce3bf4070958a01acdb8a930bd1/docs/ChannelApi.md#L474) ([traQ#2485](https://github.com/traPtitech/traQ/pull/2485))

### Fixed

- Fix example code in README

## [0.1.4] - 2024-03-21

### Changed

- Update dependencies

## [0.1.3] - 2023-12-27

### Added

- Add struct [`traq::models::StampWithThumbnail`](https://github.com/traPtitech/rust-traq/blob/a798e6acc5652062330da5395e75dc142a94582d/src/models/stamp_with_thumbnail.rs#L13-L39)

### Changed

- Change return type of [`traq::apis::stamp_api::get_stamps`](https://github.com/traPtitech/rust-traq/blob/a798e6acc5652062330da5395e75dc142a94582d/src/apis/stamp_api.rs#L889-L939),
  from `Result<Vec<traq::models::Stamp>, traq::apis::Error<traq::apis::stamp_api::GetStampsError>>`
  to `Result<Vec<traq::models::StampWithThumbnail>, traq::apis::Error<traq::apis::stamp_api::GetStampsError>>`
    - [`traq::models::Stamp`](https://github.com/traPtitech/rust-traq/blob/a798e6acc5652062330da5395e75dc142a94582d/src/models/stamp.rs#L13-L36)
    - [`traq::models::StampWithThumbnail`](https://github.com/traPtitech/rust-traq/blob/a798e6acc5652062330da5395e75dc142a94582d/src/models/stamp_with_thumbnail.rs#L13-L39)

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
[0.1.5]: https://github.com/traPtitech/rust-traq/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/traPtitech/rust-traq/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/traPtitech/rust-traq/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/traPtitech/rust-traq/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/traPtitech/rust-traq/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/traPtitech/rust-traq/releases/tag/v0.1.0
