# \ClipApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clip_message**](ClipApi.md#clip_message) | **POST** /clip-folders/{folderId}/messages | メッセージをクリップフォルダに追加
[**create_clip_folder**](ClipApi.md#create_clip_folder) | **POST** /clip-folders | クリップフォルダを作成
[**delete_clip_folder**](ClipApi.md#delete_clip_folder) | **DELETE** /clip-folders/{folderId} | クリップフォルダを削除
[**edit_clip_folder**](ClipApi.md#edit_clip_folder) | **PATCH** /clip-folders/{folderId} | クリップフォルダ情報を編集
[**get_clip_folder**](ClipApi.md#get_clip_folder) | **GET** /clip-folders/{folderId} | クリップフォルダ情報を取得
[**get_clip_folders**](ClipApi.md#get_clip_folders) | **GET** /clip-folders | クリップフォルダのリストを取得
[**get_clips**](ClipApi.md#get_clips) | **GET** /clip-folders/{folderId}/messages | フォルダ内のクリップのリストを取得
[**get_message_clips**](ClipApi.md#get_message_clips) | **GET** /messages/{messageId}/clips | 自分のクリップを取得
[**unclip_message**](ClipApi.md#unclip_message) | **DELETE** /clip-folders/{folderId}/messages/{messageId} | メッセージをクリップフォルダから除外



## clip_message

> crate::models::ClippedMessage clip_message(folder_id, post_clip_folder_message_request)
メッセージをクリップフォルダに追加

指定したメッセージを指定したクリップフォルダに追加します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **uuid::Uuid** | クリップフォルダUUID | [required] |
**post_clip_folder_message_request** | Option<[**PostClipFolderMessageRequest**](PostClipFolderMessageRequest.md)> |  |  |

### Return type

[**crate::models::ClippedMessage**](ClippedMessage.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_clip_folder

> crate::models::ClipFolder create_clip_folder(post_clip_folder_request)
クリップフォルダを作成

クリップフォルダを作成します。 既にあるフォルダと同名のフォルダを作成することは可能です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_clip_folder_request** | Option<[**PostClipFolderRequest**](PostClipFolderRequest.md)> |  |  |

### Return type

[**crate::models::ClipFolder**](ClipFolder.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_clip_folder

> delete_clip_folder(folder_id)
クリップフォルダを削除

指定したクリップフォルダを削除します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **uuid::Uuid** | クリップフォルダUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_clip_folder

> edit_clip_folder(folder_id, patch_clip_folder_request)
クリップフォルダ情報を編集

指定したクリップフォルダの情報を編集します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **uuid::Uuid** | クリップフォルダUUID | [required] |
**patch_clip_folder_request** | Option<[**PatchClipFolderRequest**](PatchClipFolderRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clip_folder

> crate::models::ClipFolder get_clip_folder(folder_id)
クリップフォルダ情報を取得

指定したクリップフォルダの情報を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **uuid::Uuid** | クリップフォルダUUID | [required] |

### Return type

[**crate::models::ClipFolder**](ClipFolder.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clip_folders

> Vec<crate::models::ClipFolder> get_clip_folders()
クリップフォルダのリストを取得

自身が所有するクリップフォルダのリストを取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ClipFolder>**](ClipFolder.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clips

> Vec<crate::models::ClippedMessage> get_clips(folder_id, limit, offset, order)
フォルダ内のクリップのリストを取得

指定したフォルダ内のクリップのリストを取得します。 `order`を指定しない場合、クリップした日時の新しい順で返されます。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **uuid::Uuid** | クリップフォルダUUID | [required] |
**limit** | Option<**i32**> | 取得する件数 |  |
**offset** | Option<**i32**> | 取得するオフセット |  |[default to 0]
**order** | Option<**String**> | 昇順か降順か |  |[default to desc]

### Return type

[**Vec<crate::models::ClippedMessage>**](ClippedMessage.md)

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


## unclip_message

> unclip_message(folder_id, message_id)
メッセージをクリップフォルダから除外

指定したフォルダから指定したメッセージのクリップを除外します。 既に外されているメッセージを指定した場合は204を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **uuid::Uuid** | クリップフォルダUUID | [required] |
**message_id** | **uuid::Uuid** | メッセージUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

