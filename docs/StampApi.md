# \StampApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_message_stamp**](StampApi.md#add_message_stamp) | **POST** /messages/{messageId}/stamps/{stampId} | スタンプを押す
[**change_stamp_image**](StampApi.md#change_stamp_image) | **PUT** /stamps/{stampId}/image | スタンプ画像を変更
[**create_stamp**](StampApi.md#create_stamp) | **POST** /stamps | スタンプを作成
[**create_stamp_palette**](StampApi.md#create_stamp_palette) | **POST** /stamp-palettes | スタンプパレットを作成
[**delete_stamp**](StampApi.md#delete_stamp) | **DELETE** /stamps/{stampId} | スタンプを削除
[**delete_stamp_palette**](StampApi.md#delete_stamp_palette) | **DELETE** /stamp-palettes/{paletteId} | スタンプパレットを削除
[**edit_stamp**](StampApi.md#edit_stamp) | **PATCH** /stamps/{stampId} | スタンプ情報を変更
[**edit_stamp_palette**](StampApi.md#edit_stamp_palette) | **PATCH** /stamp-palettes/{paletteId} | スタンプパレットを編集
[**get_message_stamps**](StampApi.md#get_message_stamps) | **GET** /messages/{messageId}/stamps | メッセージのスタンプリストを取得
[**get_my_stamp_history**](StampApi.md#get_my_stamp_history) | **GET** /users/me/stamp-history | スタンプ履歴を取得
[**get_stamp**](StampApi.md#get_stamp) | **GET** /stamps/{stampId} | スタンプ情報を取得
[**get_stamp_image**](StampApi.md#get_stamp_image) | **GET** /stamps/{stampId}/image | スタンプ画像を取得
[**get_stamp_palette**](StampApi.md#get_stamp_palette) | **GET** /stamp-palettes/{paletteId} | スタンプパレットを取得
[**get_stamp_palettes**](StampApi.md#get_stamp_palettes) | **GET** /stamp-palettes | スタンプパレットのリストを取得
[**get_stamp_stats**](StampApi.md#get_stamp_stats) | **GET** /stamps/{stampId}/stats | スタンプ統計情報を取得
[**get_stamps**](StampApi.md#get_stamps) | **GET** /stamps | スタンプリストを取得
[**remove_message_stamp**](StampApi.md#remove_message_stamp) | **DELETE** /messages/{messageId}/stamps/{stampId} | スタンプを消す



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


## change_stamp_image

> change_stamp_image(stamp_id, file)
スタンプ画像を変更

指定したスタンプの画像を変更します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stamp_id** | **uuid::Uuid** | スタンプUUID | [required] |
**file** | **std::path::PathBuf** | スタンプ画像(1MBまでのpng, jpeg, gif) | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_stamp

> crate::models::Stamp create_stamp(name, file)
スタンプを作成

スタンプを新規作成します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | スタンプ名 | [required] |
**file** | **std::path::PathBuf** | スタンプ画像(1MBまでのpng, jpeg, gif) | [required] |

### Return type

[**crate::models::Stamp**](Stamp.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_stamp_palette

> crate::models::StampPalette create_stamp_palette(post_stamp_palette_request)
スタンプパレットを作成

スタンプパレットを作成します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_stamp_palette_request** | Option<[**PostStampPaletteRequest**](PostStampPaletteRequest.md)> |  |  |

### Return type

[**crate::models::StampPalette**](StampPalette.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_stamp

> delete_stamp(stamp_id)
スタンプを削除

指定したスタンプを削除します。 対象のスタンプの削除権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stamp_id** | **uuid::Uuid** | スタンプUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_stamp_palette

> delete_stamp_palette(palette_id)
スタンプパレットを削除

指定したスタンプパレットを削除します。 対象のスタンプパレットの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**palette_id** | **uuid::Uuid** | スタンプパレットUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_stamp

> edit_stamp(stamp_id, patch_stamp_request)
スタンプ情報を変更

指定したスタンプの情報を変更します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stamp_id** | **uuid::Uuid** | スタンプUUID | [required] |
**patch_stamp_request** | Option<[**PatchStampRequest**](PatchStampRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_stamp_palette

> edit_stamp_palette(palette_id, patch_stamp_palette_request)
スタンプパレットを編集

指定したスタンプパレットを編集します。 リクエストのスタンプの配列の順番は保存されて変更されます。 対象のスタンプパレットの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**palette_id** | **uuid::Uuid** | スタンプパレットUUID | [required] |
**patch_stamp_palette_request** | Option<[**PatchStampPaletteRequest**](PatchStampPaletteRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

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


## get_stamp

> crate::models::Stamp get_stamp(stamp_id)
スタンプ情報を取得

指定したスタンプの情報を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stamp_id** | **uuid::Uuid** | スタンプUUID | [required] |

### Return type

[**crate::models::Stamp**](Stamp.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stamp_image

> std::path::PathBuf get_stamp_image(stamp_id)
スタンプ画像を取得

指定したIDのスタンプ画像を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stamp_id** | **uuid::Uuid** | スタンプUUID | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, image/gif, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stamp_palette

> crate::models::StampPalette get_stamp_palette(palette_id)
スタンプパレットを取得

指定したスタンプパレットの情報を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**palette_id** | **uuid::Uuid** | スタンプパレットUUID | [required] |

### Return type

[**crate::models::StampPalette**](StampPalette.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stamp_palettes

> Vec<crate::models::StampPalette> get_stamp_palettes()
スタンプパレットのリストを取得

自身が所有しているスタンプパレットのリストを取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::StampPalette>**](StampPalette.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stamp_stats

> crate::models::StampStats get_stamp_stats(stamp_id)
スタンプ統計情報を取得

指定したスタンプの統計情報を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stamp_id** | **uuid::Uuid** | スタンプUUID | [required] |

### Return type

[**crate::models::StampStats**](StampStats.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stamps

> Vec<crate::models::Stamp> get_stamps(include_unicode, r#type)
スタンプリストを取得

スタンプのリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_unicode** | Option<**bool**> | Unicode絵文字を含ませるかどうか Deprecated: typeクエリを指定しなければ全てのスタンプを取得できるため、そちらを利用してください  |  |[default to true]
**r#type** | Option<**String**> | 取得するスタンプの種類 |  |

### Return type

[**Vec<crate::models::Stamp>**](Stamp.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
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

