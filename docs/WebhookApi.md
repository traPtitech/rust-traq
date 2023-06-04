# \WebhookApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_webhook_icon**](WebhookApi.md#change_webhook_icon) | **PUT** /webhooks/{webhookId}/icon | Webhookのアイコンを変更
[**create_webhook**](WebhookApi.md#create_webhook) | **POST** /webhooks | Webhookを新規作成
[**delete_webhook**](WebhookApi.md#delete_webhook) | **DELETE** /webhooks/{webhookId} | Webhookを削除
[**edit_webhook**](WebhookApi.md#edit_webhook) | **PATCH** /webhooks/{webhookId} | Webhook情報を変更
[**get_webhook**](WebhookApi.md#get_webhook) | **GET** /webhooks/{webhookId} | Webhook情報を取得
[**get_webhook_icon**](WebhookApi.md#get_webhook_icon) | **GET** /webhooks/{webhookId}/icon | Webhookのアイコンを取得
[**get_webhook_messages**](WebhookApi.md#get_webhook_messages) | **GET** /webhooks/{webhookId}/messages | Webhookの投稿メッセージのリストを取得
[**get_webhooks**](WebhookApi.md#get_webhooks) | **GET** /webhooks | Webhook情報のリストを取得します
[**post_webhook**](WebhookApi.md#post_webhook) | **POST** /webhooks/{webhookId} | Webhookを送信



## change_webhook_icon

> change_webhook_icon(webhook_id, file)
Webhookのアイコンを変更

指定したWebhookのアイコン画像を変更します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | WebhookUUID | [required] |
**file** | **std::path::PathBuf** | アイコン画像(1MBまでのpng, jpeg, gif) | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_webhook

> crate::models::Webhook create_webhook(post_webhook_request)
Webhookを新規作成

Webhookを新規作成します。 `secret`が空文字の場合、insecureウェブフックが作成されます。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_webhook_request** | Option<[**PostWebhookRequest**](PostWebhookRequest.md)> |  |  |

### Return type

[**crate::models::Webhook**](Webhook.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook

> delete_webhook(webhook_id)
Webhookを削除

指定したWebhookを削除します。 Webhookによって投稿されたメッセージは削除されません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | WebhookUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_webhook

> edit_webhook(webhook_id, patch_webhook_request)
Webhook情報を変更

指定したWebhookの情報を変更します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | WebhookUUID | [required] |
**patch_webhook_request** | Option<[**PatchWebhookRequest**](PatchWebhookRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook

> crate::models::Webhook get_webhook(webhook_id)
Webhook情報を取得

指定したWebhookの詳細を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | WebhookUUID | [required] |

### Return type

[**crate::models::Webhook**](Webhook.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_icon

> std::path::PathBuf get_webhook_icon(webhook_id)
Webhookのアイコンを取得

指定したWebhookのアイコン画像を取得します

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | WebhookUUID | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/gif, image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_messages

> Vec<crate::models::Message> get_webhook_messages(webhook_id, limit, offset, since, until, inclusive, order)
Webhookの投稿メッセージのリストを取得

指定されたWebhookが投稿したメッセージのリストを返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | WebhookUUID | [required] |
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


## get_webhooks

> Vec<crate::models::Webhook> get_webhooks(all)
Webhook情報のリストを取得します

Webhookのリストを取得します。 allがtrueで無い場合は、自分がオーナーのWebhookのリストを返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | 全てのWebhookを取得します。権限が必要です。 |  |[default to false]

### Return type

[**Vec<crate::models::Webhook>**](Webhook.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_webhook

> post_webhook(webhook_id, x_traq_signature, x_traq_channel_id, embed, body)
Webhookを送信

Webhookにメッセージを投稿します。 secureなウェブフックに対しては`X-TRAQ-Signature`ヘッダーが必須です。 アーカイブされているチャンネルには投稿できません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | WebhookUUID | [required] |
**x_traq_signature** | Option<**String**> | リクエストボディシグネチャ(Secretが設定されている場合は必須) |  |
**x_traq_channel_id** | Option<**String**> | 投稿先のチャンネルID(変更する場合) |  |
**embed** | Option<**i32**> | メンション・チャンネルリンクを自動埋め込みする場合に1を指定する |  |[default to 0]
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

