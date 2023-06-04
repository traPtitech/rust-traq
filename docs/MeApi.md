# \MeApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_my_star**](MeApi.md#add_my_star) | **POST** /users/me/stars | チャンネルをスターに追加
[**add_my_user_tag**](MeApi.md#add_my_user_tag) | **POST** /users/me/tags | 自分にタグを追加
[**change_my_icon**](MeApi.md#change_my_icon) | **PUT** /users/me/icon | 自分のアイコン画像を変更
[**change_my_notify_citation**](MeApi.md#change_my_notify_citation) | **PUT** /users/me/settings/notify-citation | メッセージ引用通知の設定情報を変更
[**change_my_password**](MeApi.md#change_my_password) | **PUT** /users/me/password | 自分のパスワードを変更
[**edit_me**](MeApi.md#edit_me) | **PATCH** /users/me | 自分のユーザー情報を変更
[**edit_my_user_tag**](MeApi.md#edit_my_user_tag) | **PATCH** /users/me/tags/{tagId} | 自分のタグを編集
[**get_me**](MeApi.md#get_me) | **GET** /users/me | 自分のユーザー詳細を取得
[**get_my_channel_subscriptions**](MeApi.md#get_my_channel_subscriptions) | **GET** /users/me/subscriptions | 自分のチャンネル購読状態を取得
[**get_my_external_accounts**](MeApi.md#get_my_external_accounts) | **GET** /users/me/ex-accounts | 外部ログインアカウント一覧を取得
[**get_my_icon**](MeApi.md#get_my_icon) | **GET** /users/me/icon | 自分のアイコン画像を取得
[**get_my_notify_citation**](MeApi.md#get_my_notify_citation) | **GET** /users/me/settings/notify-citation | メッセージ引用通知の設定情報を取得
[**get_my_qr_code**](MeApi.md#get_my_qr_code) | **GET** /users/me/qr-code | QRコードを取得
[**get_my_sessions**](MeApi.md#get_my_sessions) | **GET** /users/me/sessions | 自分のログインセッションリストを取得
[**get_my_stamp_history**](MeApi.md#get_my_stamp_history) | **GET** /users/me/stamp-history | スタンプ履歴を取得
[**get_my_stars**](MeApi.md#get_my_stars) | **GET** /users/me/stars | スターチャンネルリストを取得
[**get_my_tokens**](MeApi.md#get_my_tokens) | **GET** /users/me/tokens | 有効トークンのリストを取得
[**get_my_unread_channels**](MeApi.md#get_my_unread_channels) | **GET** /users/me/unread | 未読チャンネルを取得
[**get_my_user_tags**](MeApi.md#get_my_user_tags) | **GET** /users/me/tags | 自分のタグリストを取得
[**get_my_view_states**](MeApi.md#get_my_view_states) | **GET** /users/me/view-states | 自身のチャンネル閲覧状態一覧を取得
[**get_user_settings**](MeApi.md#get_user_settings) | **GET** /users/me/settings | ユーザー設定を取得
[**link_external_account**](MeApi.md#link_external_account) | **POST** /users/me/ex-accounts/link | 外部ログインアカウントを紐付ける
[**read_channel**](MeApi.md#read_channel) | **DELETE** /users/me/unread/{channelId} | チャンネルを既読にする
[**register_fcm_device**](MeApi.md#register_fcm_device) | **POST** /users/me/fcm-device | FCMデバイスを登録
[**remove_my_star**](MeApi.md#remove_my_star) | **DELETE** /users/me/stars/{channelId} | チャンネルをスターから削除します
[**remove_my_user_tag**](MeApi.md#remove_my_user_tag) | **DELETE** /users/me/tags/{tagId} | 自分からタグを削除します
[**revoke_my_session**](MeApi.md#revoke_my_session) | **DELETE** /users/me/sessions/{sessionId} | セッションを無効化
[**revoke_my_token**](MeApi.md#revoke_my_token) | **DELETE** /users/me/tokens/{tokenId} | トークンの認可を取り消す
[**set_channel_subscribe_level**](MeApi.md#set_channel_subscribe_level) | **PUT** /users/me/subscriptions/{channelId} | チャンネル購読レベルを設定
[**unlink_external_account**](MeApi.md#unlink_external_account) | **POST** /users/me/ex-accounts/unlink | 外部ログインアカウントの紐付けを解除



## add_my_star

> add_my_star(post_star_request)
チャンネルをスターに追加

指定したチャンネルをスターチャンネルに追加します。 スター済みのチャンネルIDを指定した場合、204を返します。 不正なチャンネルIDを指定した場合、400を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_star_request** | Option<[**PostStarRequest**](PostStarRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_my_user_tag

> crate::models::UserTag add_my_user_tag(post_user_tag_request)
自分にタグを追加

自分に新しくタグを追加します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_user_tag_request** | Option<[**PostUserTagRequest**](PostUserTagRequest.md)> |  |  |

### Return type

[**crate::models::UserTag**](UserTag.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_my_icon

> change_my_icon(file)
自分のアイコン画像を変更

自分のアイコン画像を変更します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | アイコン画像(1MBまでのpng, jpeg, gif) | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_my_notify_citation

> change_my_notify_citation(put_notify_citation_request)
メッセージ引用通知の設定情報を変更

メッセージ引用通知の設定情報を変更します

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_notify_citation_request** | Option<[**PutNotifyCitationRequest**](PutNotifyCitationRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_my_password

> change_my_password(put_my_password_request)
自分のパスワードを変更

自身のパスワードを変更します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_my_password_request** | Option<[**PutMyPasswordRequest**](PutMyPasswordRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_me

> edit_me(patch_me_request)
自分のユーザー情報を変更

自身のユーザー情報を変更します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch_me_request** | Option<[**PatchMeRequest**](PatchMeRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_my_user_tag

> edit_my_user_tag(tag_id, patch_user_tag_request)
自分のタグを編集

自分の指定したタグの状態を変更します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **uuid::Uuid** | タグUUID | [required] |
**patch_user_tag_request** | Option<[**PatchUserTagRequest**](PatchUserTagRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_me

> crate::models::MyUserDetail get_me()
自分のユーザー詳細を取得

自身のユーザー詳細情報を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MyUserDetail**](MyUserDetail.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_channel_subscriptions

> Vec<crate::models::UserSubscribeState> get_my_channel_subscriptions()
自分のチャンネル購読状態を取得

自身のチャンネル購読状態を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::UserSubscribeState>**](UserSubscribeState.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_external_accounts

> Vec<crate::models::ExternalProviderUser> get_my_external_accounts()
外部ログインアカウント一覧を取得

自分に紐付けられている外部ログインアカウント一覧を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ExternalProviderUser>**](ExternalProviderUser.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_icon

> std::path::PathBuf get_my_icon()
自分のアイコン画像を取得

自分のアイコン画像を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/gif, image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_notify_citation

> crate::models::GetNotifyCitation get_my_notify_citation()
メッセージ引用通知の設定情報を取得

メッセージ引用通知の設定情報を変更します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetNotifyCitation**](GetNotifyCitation.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_qr_code

> std::path::PathBuf get_my_qr_code(token)
QRコードを取得

自身のQRコードを取得します。 返されたQRコードまたはトークンは、発行後の5分間のみ有効です

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**bool**> | 画像でなくトークン文字列で返すかどうか |  |[default to false]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_sessions

> Vec<crate::models::LoginSession> get_my_sessions()
自分のログインセッションリストを取得

自分のログインセッションのリストを取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LoginSession>**](LoginSession.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_stamp_history

> Vec<crate::models::StampHistoryEntry> get_my_stamp_history(limit)
スタンプ履歴を取得

自分のスタンプ履歴を最大100件まで取得します。 結果は降順で返されます。  このAPIが返すスタンプ履歴は厳密な履歴ではありません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | 件数 |  |[default to 100]

### Return type

[**Vec<crate::models::StampHistoryEntry>**](StampHistoryEntry.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_stars

> Vec<uuid::Uuid> get_my_stars()
スターチャンネルリストを取得

自分がスターしているチャンネルのUUIDの配列を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<uuid::Uuid>**](uuid::Uuid.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_tokens

> Vec<crate::models::ActiveOAuth2Token> get_my_tokens()
有効トークンのリストを取得

有効な自分に発行されたOAuth2トークンのリストを取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ActiveOAuth2Token>**](ActiveOAuth2Token.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_unread_channels

> Vec<crate::models::UnreadChannel> get_my_unread_channels()
未読チャンネルを取得

自分が現在未読のチャンネルの未読情報を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::UnreadChannel>**](UnreadChannel.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_user_tags

> Vec<crate::models::UserTag> get_my_user_tags()
自分のタグリストを取得

自分に付けられているタグの配列を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::UserTag>**](UserTag.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_view_states

> Vec<crate::models::MyChannelViewState> get_my_view_states()
自身のチャンネル閲覧状態一覧を取得

自身のチャンネル閲覧状態一覧を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::MyChannelViewState>**](MyChannelViewState.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_settings

> crate::models::UserSettings get_user_settings()
ユーザー設定を取得

ユーザー設定を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserSettings**](UserSettings.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_external_account

> link_external_account(post_link_external_account)
外部ログインアカウントを紐付ける

自分に外部ログインアカウントを紐付けます。 指定した`providerName`がサーバー側で有効である必要があります。 リクエストが受理された場合、外部サービスの認証画面にリダイレクトされ、認証される必要があります。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_link_external_account** | Option<[**PostLinkExternalAccount**](PostLinkExternalAccount.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_channel

> read_channel(channel_id)
チャンネルを既読にする

自分が未読のチャンネルを既読にします。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **uuid::Uuid** | チャンネルUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_fcm_device

> register_fcm_device(post_my_fcm_device_request)
FCMデバイスを登録

自身のFCMデバイスを登録します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_my_fcm_device_request** | Option<[**PostMyFcmDeviceRequest**](PostMyFcmDeviceRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_my_star

> remove_my_star(channel_id)
チャンネルをスターから削除します

既にスターから削除されているチャンネルを指定した場合は204を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **uuid::Uuid** | チャンネルUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_my_user_tag

> remove_my_user_tag(tag_id)
自分からタグを削除します

既に存在しないタグを削除しようとした場合は204を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **uuid::Uuid** | タグUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_my_session

> revoke_my_session(session_id)
セッションを無効化

指定した自分のセッションを無効化(ログアウト)します。 既に存在しない・無効化されているセッションを指定した場合も`204`を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **uuid::Uuid** | セッションUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_my_token

> revoke_my_token(token_id)
トークンの認可を取り消す

自分の指定したトークンの認可を取り消します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **uuid::Uuid** | OAuth2トークンUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_channel_subscribe_level

> set_channel_subscribe_level(channel_id, put_channel_subscribe_level_request)
チャンネル購読レベルを設定

自身の指定したチャンネルの購読レベルを設定します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **uuid::Uuid** | チャンネルUUID | [required] |
**put_channel_subscribe_level_request** | Option<[**PutChannelSubscribeLevelRequest**](PutChannelSubscribeLevelRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_external_account

> unlink_external_account(post_unlink_external_account)
外部ログインアカウントの紐付けを解除

自分に紐付けられている外部ログインアカウントの紐付けを解除します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_unlink_external_account** | Option<[**PostUnlinkExternalAccount**](PostUnlinkExternalAccount.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

