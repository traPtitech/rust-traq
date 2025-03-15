# \QallApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_participant_role**](QallApi.md#change_participant_role) | **PATCH** /qall/rooms/{roomId}/participants | ルームでの発言権限を変更
[**get_endpoints**](QallApi.md#get_endpoints) | **GET** /qall/endpoints | LiveKitエンドポイントを取得
[**get_live_kit_token**](QallApi.md#get_live_kit_token) | **GET** /qall/token | LiveKitトークンを取得
[**get_room_metadata**](QallApi.md#get_room_metadata) | **GET** /qall/rooms/{roomId}/metadata | ルームのメタデータを取得
[**get_rooms**](QallApi.md#get_rooms) | **GET** /qall/rooms | ルームと参加者の一覧を取得
[**get_soundboard_list**](QallApi.md#get_soundboard_list) | **GET** /qall/soundboard | サウンドボード用の音声一覧を取得
[**live_kit_webhook**](QallApi.md#live_kit_webhook) | **POST** /qall/webhook | LiveKit Webhook受信
[**post_soundboard**](QallApi.md#post_soundboard) | **POST** /qall/soundboard | サウンドボード用の短い音声ファイルをアップロード
[**post_soundboard_play**](QallApi.md#post_soundboard_play) | **POST** /qall/soundboard/play | アップロード済み音声を LiveKit ルームで再生
[**update_room_metadata**](QallApi.md#update_room_metadata) | **PATCH** /qall/rooms/{roomId}/metadata | ルームのメタデータを更新



## change_participant_role

> crate::models::QallParticipantResponse change_participant_role(room_id, qall_participant_request)
ルームでの発言権限を変更

ルーム内の参加者の発言権限を変更します。 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **uuid::Uuid** | ルームUUID | [required] |
**qall_participant_request** | [**Vec<crate::models::QallParticipantRequest>**](qallParticipantRequest.md) | 発言権限を変更する参加者の情報 | [required] |

### Return type

[**crate::models::QallParticipantResponse**](qallParticipantResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_endpoints

> crate::models::QallEndpointResponse get_endpoints()
LiveKitエンドポイントを取得

接続可能なLiveKitエンドポイントを取得します。 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::QallEndpointResponse**](qallEndpointResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_live_kit_token

> crate::models::QallTokenResponse get_live_kit_token(room_id, is_webinar)
LiveKitトークンを取得

指定したルームに参加するためのLiveKitトークンを取得します。   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | Option<**uuid::Uuid**> | ルームUUID |  |
**is_webinar** | Option<**bool**> | ウェビナールームかどうか(デフォルト false) |  |

### Return type

[**crate::models::QallTokenResponse**](qallTokenResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_room_metadata

> crate::models::QallMetadataResponse get_room_metadata(room_id)
ルームのメタデータを取得

ルームのメタデータを取得します。 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **uuid::Uuid** | ルームUUID | [required] |

### Return type

[**crate::models::QallMetadataResponse**](qallMetadataResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rooms

> Vec<crate::models::QallRoomWithParticipants> get_rooms()
ルームと参加者の一覧を取得

現在存在する(またはアクティブな)ルームと、そのルームに所属している参加者情報を取得します。 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::QallRoomWithParticipants>**](qallRoomWithParticipants.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_soundboard_list

> Vec<crate::models::SoundboardItem> get_soundboard_list()
サウンドボード用の音声一覧を取得

DBに保存されたサウンドボード情報を取得します。   各アイテムには soundId, soundName, stampId が含まれます。 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SoundboardItem>**](soundboardItem.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## live_kit_webhook

> live_kit_webhook(body)
LiveKit Webhook受信

LiveKit側で設定したWebhookから呼び出されるエンドポイントです。   参加者の入室・退出などのイベントを受け取り、サーバ内で処理を行います。 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/webhook+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_soundboard

> crate::models::SoundboardUploadResponse post_soundboard(audio, sound_name, stamp_id)
サウンドボード用の短い音声ファイルをアップロード

15秒程度の短い音声ファイルを multipart/form-data で送信し、S3(互換ストレージ)にアップロードします。   クライアントは「soundName」というフィールドを送信し、それをDBに保存して関連付けを行います。   また、サーバ側で soundId を自動生成し、S3のファイル名に使用します。 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audio** | **std::path::PathBuf** | アップロードする音声ファイル(20秒以内) | [required] |
**sound_name** | **String** | ユーザが自由につけるサウンド名 | [required] |
**stamp_id** | Option<**String**> | アイコンスタンプID |  |

### Return type

[**crate::models::SoundboardUploadResponse**](soundboardUploadResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_soundboard_play

> crate::models::SoundboardPlayResponse post_soundboard_play(soundboard_play_request)
アップロード済み音声を LiveKit ルームで再生

S3上にある音声ファイルの署名付きURLを生成し、   Ingressを介して指定ルームに音声を流します。     該当ルームに参加しているユーザであれば再生可能とします。 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**soundboard_play_request** | [**SoundboardPlayRequest**](SoundboardPlayRequest.md) |  | [required] |

### Return type

[**crate::models::SoundboardPlayResponse**](soundboardPlayResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_room_metadata

> update_room_metadata(room_id, qall_metadata_request)
ルームのメタデータを更新

ルームのメタデータを更新します。 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **uuid::Uuid** | ルームUUID | [required] |
**qall_metadata_request** | [**QallMetadataRequest**](QallMetadataRequest.md) | ルームのメタデータ | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

