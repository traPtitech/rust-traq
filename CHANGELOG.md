# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

## [0.2.0] - 2026-11-04

### Added

- Add Qall (LiveKit) APIs, soundboard support, and related event models.

    - Add API [`traq::apis::qall_api::change_participant_role`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L112) and its error enum [`traq::apis::qall_api::ChangeParticipantRoleError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L21).
    - Add API [`traq::apis::qall_api::get_live_kit_token`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L171) and its error enum [`traq::apis::qall_api::GetLiveKitTokenError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L31).
    - Add API [`traq::apis::qall_api::get_qall_endpoints`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L229) and its error enum [`traq::apis::qall_api::GetQallEndpointsError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L41).
    - Add API [`traq::apis::qall_api::get_room_metadata`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L275) and its error enum [`traq::apis::qall_api::GetRoomMetadataError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L50).
    - Add API [`traq::apis::qall_api::get_rooms`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L329) and its error enum [`traq::apis::qall_api::GetRoomsError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L59).
    - Add API [`traq::apis::qall_api::get_soundboard_list`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L375) and its error enum [`traq::apis::qall_api::GetSoundboardListError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L68).
    - Add API [`traq::apis::qall_api::live_kit_webhook`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L421) and its error enum [`traq::apis::qall_api::LiveKitWebhookError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L76).
    - Add API [`traq::apis::qall_api::post_soundboard`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L463) and its error enum [`traq::apis::qall_api::PostSoundboardError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L85).
    - Add API [`traq::apis::qall_api::post_soundboard_play`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L534) and its error enum [`traq::apis::qall_api::PostSoundboardPlayError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L94).
    - Add API [`traq::apis::qall_api::update_room_metadata`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L587) and its error enum [`traq::apis::qall_api::UpdateRoomMetadataError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/qall_api.rs#L104).
    - Add struct [`traq::models::QallEndpointResponse`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_endpoint_response.rs#L15).
    - Add struct [`traq::models::QallMetadataRequest`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_metadata_request.rs#L15).
    - Add struct [`traq::models::QallMetadataResponse`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_metadata_response.rs#L15).
    - Add struct [`traq::models::QallParticipant`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_participant.rs#L16).
    - Add struct [`traq::models::QallParticipantRequest`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_participant_request.rs#L15).
    - Add struct [`traq::models::QallParticipantRequestUsersInner`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_participant_request_users_inner.rs#L15).
    - Add struct [`traq::models::QallParticipantResponse`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_participant_response.rs#L15).
    - Add struct [`traq::models::QallParticipantResponseResultsInner`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_participant_response_results_inner.rs#L15).
    - Add struct [`traq::models::QallRoomStateChangedEvent`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_room_state_changed_event.rs#L16).
    - Add struct [`traq::models::QallRoomStateChangedEventRoomStatesInner`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_room_state_changed_event_room_states_inner.rs#L15).
    - Add struct [`traq::models::QallRoomStateChangedEventRoomStatesInnerParticipantsInner`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_room_state_changed_event_room_states_inner_participants_inner.rs#L15).
    - Add struct [`traq::models::QallRoomWithParticipants`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_room_with_participants.rs#L15).
    - Add struct [`traq::models::QallSoundboardItemCreatedEvent`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_soundboard_item_created_event.rs#L16).
    - Add struct [`traq::models::QallSoundboardItemDeletedEvent`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_soundboard_item_deleted_event.rs#L16).
    - Add struct [`traq::models::QallTokenResponse`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/qall_token_response.rs#L15).
    - Add struct [`traq::models::SoundboardItem`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/soundboard_item.rs#L15).
    - Add struct [`traq::models::SoundboardPlayRequest`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/soundboard_play_request.rs#L15).
    - Add struct [`traq::models::SoundboardPlayResponse`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/soundboard_play_response.rs#L15).
    - Add struct [`traq::models::SoundboardUploadResponse`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/soundboard_upload_response.rs#L15).
    - Add variant [`traq::models::OAuth2Scope::Qall`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/o_auth2_scope.rs#L29).

- Add channel path retrieval.

    - Add API [`traq::apis::channel_api::get_channel_path`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/channel_api.rs#L562) and its error enum [`traq::apis::channel_api::GetChannelPathError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/channel_api.rs#L84).
    - Add struct [`traq::models::ChannelPath`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/channel_path.rs#L16).

- Add personal stamp recommendations.

    - Add API [`traq::apis::me_api::get_my_stamp_recommendations`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/me_api.rs#L944) and its error enum [`traq::apis::me_api::GetMyStampRecommendationsError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/me_api.rs#L138).
    - Add API [`traq::apis::stamp_api::get_my_stamp_recommendations`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/stamp_api.rs#L690) and its error enum [`traq::apis::stamp_api::GetMyStampRecommendationsError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/stamp_api.rs#L112).
    - Add struct [`traq::models::GetMyStampRecommendations200ResponseInner`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/get_my_stamp_recommendations_200_response_inner.rs#L15).
    - Add variant [`traq::models::UserPermission::GetMyStampRecommendations`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/user_permission.rs#L121).

- Add public metadata retrieval.

    - Add API [`traq::apis::public_api::get_meta`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/public_api.rs#L39) and its error enum [`traq::apis::public_api::GetMetaError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/public_api.rs#L19).
    - Add struct [`traq::models::Meta`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/meta.rs#L16).

- Add bulk user-group member operations.

    - Add API [`traq::apis::group_api::remove_user_group_members`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/group_api.rs#L801) and its error enum [`traq::apis::group_api::RemoveUserGroupMembersError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/group_api.rs#L141).
    - Add enum [`traq::models::AddUserGroupMemberRequest`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/add_user_group_member_request.rs#L16), with `UserGroupMember` and `UserGroupMembers` variants for a single member and a list of members.

- Add deletion of messages posted by a webhook.

    - Add API [`traq::apis::webhook_api::delete_webhook_message`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/webhook_api.rs#L259) and its error enum [`traq::apis::webhook_api::DeleteWebhookMessageError`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/webhook_api.rs#L47).

- Add an optional nonce to identify a posted message.

    - Add field [`traq::models::PostMessageRequest::nonce`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/post_message_request.rs#L25).
    - Add field [`traq::models::Message::nonce`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/message.rs#L46).

- Add the revocation status of a bot access token.

    - Add field [`traq::models::BotTokens::access_token_revoked`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/bot_tokens.rs#L25).

- Add a stale-viewing channel state.

    - Add variant [`traq::models::ChannelViewState::StaleViewing`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/channel_view_state.rs#L21).

- Add a user-offline event model.

    - Add struct [`traq::models::UserOfflineEvent`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/user_offline_event.rs#L16).

- Add a typed HTTP 400 error for channel topic retrieval.

    - Add variant [`traq::apis::channel_api::GetChannelTopicError::Status400`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/channel_api.rs#L118).

- Re-export `reqwest::Client` as [`traq::apis::configuration::Client`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/configuration.rs#L11).

### Changed

- Update OpenAPI Generator version to `7.27.0` and Cargo dependencies for multipart upload support.
- Update rust toolchain version to `1.99.1`.
- **Breaking:** Update `reqwest` from `0.12` to `0.13`. Custom clients assigned to `Configuration::client` must use `reqwest` `0.13`; use the re-exported `traq::apis::configuration::Client` to construct a compatible client.
- **Breaking:** Change date-time model fields and API parameters from strings to `chrono::DateTime<chrono::FixedOffset>` (preserving their optionality). Parse RFC 3339 strings with `chrono::DateTime::parse_from_rfc3339`, and use `.to_rfc3339()` when a string is needed. The affected API parameters are:

    - `since` and `until` in [`traq::apis::channel_api::get_channel_events`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/channel_api.rs#L478).
    - `since` and `until` in [`traq::apis::channel_api::get_messages`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/channel_api.rs#L949).
    - `since` and `until` in [`traq::apis::file_api::get_files`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/file_api.rs#L220).
    - `since` and `until` in [`traq::apis::message_api::get_direct_messages`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/message_api.rs#L346).
    - `since` and `until` in [`traq::apis::message_api::get_messages`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/message_api.rs#L592).
    - `since` and `until` in [`traq::apis::user_api::get_direct_messages`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/user_api.rs#L468).
    - `since` and `until` in [`traq::apis::webhook_api::get_webhook_messages`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/webhook_api.rs#L452).
    - `after` and `before` in [`traq::apis::message_api::search_messages`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/message_api.rs#L941).

- **Breaking:** Change the success type of binary-download APIs from `std::path::PathBuf` to `reqwest::Response`. Read the body with `response.bytes().await?` or stream it, and explicitly save it when a local file is needed. This affects:

    - [`traq::apis::bot_api::get_bot_icon`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/bot_api.rs#L494).
    - [`traq::apis::file_api::get_file`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/file_api.rs#L118).
    - [`traq::apis::file_api::get_thumbnail_image`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/file_api.rs#L308).
    - [`traq::apis::me_api::get_my_icon`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/me_api.rs#L719).
    - [`traq::apis::me_api::get_my_qr_code`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/me_api.rs#L803).
    - [`traq::apis::public_api::get_public_user_icon`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/public_api.rs#L79).
    - [`traq::apis::stamp_api::get_stamp_image`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/stamp_api.rs#L800).
    - [`traq::apis::user_api::get_user_icon`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/user_api.rs#L660).
    - [`traq::apis::webhook_api::get_webhook_icon`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/webhook_api.rs#L409).

- **Breaking:** Rename numeric enum variants to match the OpenAPI definition and change their Rust representation from `u8` to `i64`. Update variant references and explicit numeric casts; serialized JSON values remain `0`, `1`, and `2`.

    - [`traq::models::BotState`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/bot_state.rs#L21): `Deactivated`, `Active`, `Suspended` become `deactivated`, `active`, `suspended`.
    - [`traq::models::UserAccountState`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/user_account_state.rs#L21): `Deactivated`, `Active`, `Suspended` become `deactivated`, `active`, `suspended`.
    - [`traq::models::ChannelSubscribeLevel`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/channel_subscribe_level.rs#L21): `None`, `Subscribed`, `Notified` become `none`, `subscribed`, `notified`.

- **Breaking:** Rename `traq::models::channel_event::RHashType` to [`traq::models::channel_event::Type`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/channel_event.rs#L43). Update imports and variant references used with `ChannelEvent::r#type`.
- **Breaking:** Replace the following structs with untagged enums. Replace direct field access with matching on the variant and reading its inner model; replace the removed `new` constructors with enum variant constructors.

    - [`traq::models::GetBot200Response`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/get_bot_200_response.rs#L16): `Bot` and `BotDetail`.
    - [`traq::models::GetClient200Response`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/get_client_200_response.rs#L16): `OAuth2Client` and `OAuth2ClientDetail`.
    - [`traq::models::ChannelEventDetail`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/channel_event_detail.rs#L18): `TopicChangedEvent`, `SubscribersChangedEvent`, `PinAddedEvent`, `PinRemovedEvent`, `NameChangedEvent`, `ParentChangedEvent`, `VisibilityChangedEvent`, `ForcedNotificationChangedEvent`, and `ChildCreatedEvent`.

- **Breaking:** Change existing API arguments as follows:

    - Add `path: Option<&str>` to [`traq::apis::channel_api::get_channels`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/channel_api.rs#L891). Pass `None` to leave the filter unset.
    - Add `exclude_deleted_messages: Option<bool>` to [`traq::apis::channel_api::get_channel_stats`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/channel_api.rs#L670). Pass `None` to use the server default.
    - Change `to` and `from` in [`traq::apis::message_api::search_messages`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/message_api.rs#L941) from `Option<&str>` to `Option<Vec<uuid::Uuid>>`. Parse user IDs as UUIDs and pass them as a vector.
    - Change the body of [`traq::apis::group_api::add_user_group_member`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/apis/group_api.rs#L196) from `Option<UserGroupMember>` to `Option<AddUserGroupMemberRequest>`. Wrap a single member with `AddUserGroupMemberRequest::UserGroupMember(Box::new(member))`, or use `UserGroupMembers` for a vector of members.

- **Breaking:** Add the required `access_token_revoked: bool` argument to [`traq::models::BotTokens::new`](https://github.com/traPtitech/rust-traq/blob/75be94309e3be189e6b520b0b0a2957957612cf0/src/models/bot_tokens.rs#L30). JSON deserialized as `BotTokens` must also include `accessTokenRevoked`.
- **Breaking:** Update exhaustive struct literals for the new `Message::nonce`, `PostMessageRequest::nonce`, and `BotTokens::access_token_revoked` fields. Update exhaustive matches for the new `OAuth2Scope::Qall`, `ChannelViewState::StaleViewing`, `UserPermission::GetMyStampRecommendations`, and `GetChannelTopicError::Status400` variants listed under Added.

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
[0.2.0]: https://github.com/traPtitech/rust-traq/compare/v0.1.5..v0.2.0
[0.1.5]: https://github.com/traPtitech/rust-traq/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/traPtitech/rust-traq/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/traPtitech/rust-traq/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/traPtitech/rust-traq/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/traPtitech/rust-traq/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/traPtitech/rust-traq/releases/tag/v0.1.0
