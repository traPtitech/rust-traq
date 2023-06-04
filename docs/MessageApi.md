# \MessageApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_message_stamp**](MessageApi.md#add_message_stamp) | **POST** /messages/{messageId}/stamps/{stampId} | スタンプを押す
[**create_pin**](MessageApi.md#create_pin) | **POST** /messages/{messageId}/pin | ピン留めする
[**delete_message**](MessageApi.md#delete_message) | **DELETE** /messages/{messageId} | メッセージを削除
[**edit_message**](MessageApi.md#edit_message) | **PUT** /messages/{messageId} | メッセージを編集
[**get_direct_messages**](MessageApi.md#get_direct_messages) | **GET** /users/{userId}/messages | ダイレクトメッセージのリストを取得
[**get_message**](MessageApi.md#get_message) | **GET** /messages/{messageId} | メッセージを取得
[**get_message_clips**](MessageApi.md#get_message_clips) | **GET** /messages/{messageId}/clips | 自分のクリップを取得
[**get_message_stamps**](MessageApi.md#get_message_stamps) | **GET** /messages/{messageId}/stamps | メッセージのスタンプリストを取得
[**get_messages**](MessageApi.md#get_messages) | **GET** /channels/{channelId}/messages | チャンネルメッセージのリストを取得
[**get_pin**](MessageApi.md#get_pin) | **GET** /messages/{messageId}/pin | ピン留めを取得
[**post_direct_message**](MessageApi.md#post_direct_message) | **POST** /users/{userId}/messages | ダイレクトメッセージを送信
[**post_message**](MessageApi.md#post_message) | **POST** /channels/{channelId}/messages | チャンネルにメッセージを投稿
[**remove_message_stamp**](MessageApi.md#remove_message_stamp) | **DELETE** /messages/{messageId}/stamps/{stampId} | スタンプを消す
[**remove_pin**](MessageApi.md#remove_pin) | **DELETE** /messages/{messageId}/pin | ピン留めを外す
[**search_messages**](MessageApi.md#search_messages) | **GET** /messages | メッセージを検索



## add_message_stamp

> add_message_stamp(message_id, stamp_id, post_message_stamp_request)
スタンプを押す

指定したメッセージに指定したスタンプを押します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | メッセージUUID | [required] |
**stamp_id** | **uuid::Uuid** | スタンプUUID | [required] |
**post_message_stamp_request** | Option<[**PostMessageStampRequest**](PostMessageStampRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pin

> crate::models::MessagePin create_pin(message_id)
ピン留めする

指定したメッセージをピン留めします。 アーカイブされているチャンネルのメッセージ・存在しないメッセージ・チャンネル当たりの上限数を超えたメッセージのピン留めはできません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | メッセージUUID | [required] |

### Return type

[**crate::models::MessagePin**](MessagePin.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_message

> delete_message(message_id)
メッセージを削除

指定したメッセージを削除します。 自身が投稿したメッセージと自身が管理権限を持つWebhookとBOTが投稿したメッセージのみ削除することができます。 アーカイブされているチャンネルのメッセージを編集することは出来ません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | メッセージUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_message

> edit_message(message_id, post_message_request)
メッセージを編集

指定したメッセージを編集します。 自身が投稿したメッセージのみ編集することができます。 アーカイブされているチャンネルのメッセージを編集することは出来ません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | メッセージUUID | [required] |
**post_message_request** | Option<[**PostMessageRequest**](PostMessageRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_direct_messages

> Vec<crate::models::Message> get_direct_messages(user_id, limit, offset, since, until, inclusive, order)
ダイレクトメッセージのリストを取得

指定したユーザーとのダイレクトメッセージのリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | ユーザーUUID | [required] |
**limit** | Option<**i32**> | 取得する件数 |  |
**offset** | Option<**i32**> | 取得するオフセット |  |[default to 0]
**since** | Option<**String**> | 取得する時間範囲の開始日時 |  |[default to 0000-01-01T00:00Z]
**until** | Option<**String**> | 取得する時間範囲の終了日時 |  |
**inclusive** | Option<**bool**> | 範囲の端を含めるかどうか |  |[default to false]
**order** | Option<**String**> | 昇順か降順か |  |[default to desc]

### Return type

[**Vec<crate::models::Message>**](Message.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message

> crate::models::Message get_message(message_id)
メッセージを取得

指定したメッセージを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | メッセージUUID | [required] |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message_clips

> Vec<crate::models::MessageClip> get_message_clips(message_id)
自分のクリップを取得

対象のメッセージの自分のクリップの一覧を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | メッセージUUID | [required] |

### Return type

[**Vec<crate::models::MessageClip>**](MessageClip.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message_stamps

> Vec<crate::models::MessageStamp> get_message_stamps(message_id)
メッセージのスタンプリストを取得

指定したメッセージに押されているスタンプのリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | メッセージUUID | [required] |

### Return type

[**Vec<crate::models::MessageStamp>**](MessageStamp.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_messages

> Vec<crate::models::Message> get_messages(channel_id, limit, offset, since, until, inclusive, order)
チャンネルメッセージのリストを取得

指定したチャンネルのメッセージのリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **uuid::Uuid** | チャンネルUUID | [required] |
**limit** | Option<**i32**> | 取得する件数 |  |
**offset** | Option<**i32**> | 取得するオフセット |  |[default to 0]
**since** | Option<**String**> | 取得する時間範囲の開始日時 |  |[default to 0000-01-01T00:00Z]
**until** | Option<**String**> | 取得する時間範囲の終了日時 |  |
**inclusive** | Option<**bool**> | 範囲の端を含めるかどうか |  |[default to false]
**order** | Option<**String**> | 昇順か降順か |  |[default to desc]

### Return type

[**Vec<crate::models::Message>**](Message.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pin

> crate::models::MessagePin get_pin(message_id)
ピン留めを取得

指定したメッセージのピン留め情報を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | メッセージUUID | [required] |

### Return type

[**crate::models::MessagePin**](MessagePin.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_direct_message

> crate::models::Message post_direct_message(user_id, post_message_request)
ダイレクトメッセージを送信

指定したユーザーにダイレクトメッセージを送信します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | ユーザーUUID | [required] |
**post_message_request** | Option<[**PostMessageRequest**](PostMessageRequest.md)> |  |  |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_message

> crate::models::Message post_message(channel_id, post_message_request)
チャンネルにメッセージを投稿

指定したチャンネルにメッセージを投稿します。 embedをtrueに指定すると、メッセージ埋め込みが自動で行われます。 アーカイブされているチャンネルに投稿することはできません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **uuid::Uuid** | チャンネルUUID | [required] |
**post_message_request** | Option<[**PostMessageRequest**](PostMessageRequest.md)> |  |  |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_message_stamp

> remove_message_stamp(message_id, stamp_id)
スタンプを消す

指定したメッセージから指定した自身が押したスタンプを削除します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | メッセージUUID | [required] |
**stamp_id** | **uuid::Uuid** | スタンプUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_pin

> remove_pin(message_id)
ピン留めを外す

指定したメッセージのピン留めを外します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** | メッセージUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_messages

> crate::models::MessageSearchResult search_messages(word, after, before, r#in, to, from, citation, bot, has_url, has_attachments, has_image, has_video, has_audio, limit, offset, sort)
メッセージを検索

メッセージを検索します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**word** | Option<**String**> | 検索ワード Simple-Query-String-Syntaxをパースして検索します  |  |
**after** | Option<**String**> | 投稿日時が指定日時より後 |  |
**before** | Option<**String**> | 投稿日時が指定日時より前 |  |
**r#in** | Option<**uuid::Uuid**> | メッセージが投稿されたチャンネル |  |
**to** | Option<**uuid::Uuid**> | メンションされたユーザー |  |
**from** | Option<**uuid::Uuid**> | メッセージを投稿したユーザー |  |
**citation** | Option<**uuid::Uuid**> | 引用しているメッセージ |  |
**bot** | Option<**bool**> | メッセージを投稿したユーザーがBotかどうか |  |
**has_url** | Option<**bool**> | メッセージがURLを含むか |  |
**has_attachments** | Option<**bool**> | メッセージが添付ファイルを含むか |  |
**has_image** | Option<**bool**> | メッセージが画像を含むか |  |
**has_video** | Option<**bool**> | メッセージが動画を含むか |  |
**has_audio** | Option<**bool**> | メッセージが音声ファイルを含むか |  |
**limit** | Option<**i32**> | 検索結果から取得するメッセージの最大件数 |  |
**offset** | Option<**i32**> | 検索結果から取得するメッセージのオフセット |  |
**sort** | Option<**String**> | ソート順 (作成日時が新しい `createdAt`, 作成日時が古い `-createdAt`, 更新日時が新しい `updatedAt`, 更新日時が古い `-updatedAt`) |  |[default to -createdAt]

### Return type

[**crate::models::MessageSearchResult**](MessageSearchResult.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

