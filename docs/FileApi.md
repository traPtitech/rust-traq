# \FileApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_file**](FileApi.md#delete_file) | **DELETE** /files/{fileId} | ファイルを削除
[**get_file**](FileApi.md#get_file) | **GET** /files/{fileId} | ファイルをダウンロード
[**get_file_meta**](FileApi.md#get_file_meta) | **GET** /files/{fileId}/meta | ファイルメタを取得
[**get_files**](FileApi.md#get_files) | **GET** /files | ファイルメタのリストを取得
[**get_thumbnail_image**](FileApi.md#get_thumbnail_image) | **GET** /files/{fileId}/thumbnail | サムネイル画像を取得
[**post_file**](FileApi.md#post_file) | **POST** /files | ファイルをアップロード



## delete_file

> delete_file(file_id)
ファイルを削除

指定したファイルを削除します。 指定したファイルの削除権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **uuid::Uuid** | ファイルUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file

> std::path::PathBuf get_file(file_id, dl)
ファイルをダウンロード

指定したファイル本体を取得します。 指定したファイルへのアクセス権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **uuid::Uuid** | ファイルUUID | [required] |
**dl** | Option<**i32**> | 1を指定するとレスポンスにContent-Dispositionヘッダーが付与されます |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_meta

> crate::models::FileInfo get_file_meta(file_id)
ファイルメタを取得

指定したファイルのメタ情報を取得します。 指定したファイルへのアクセス権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **uuid::Uuid** | ファイルUUID | [required] |

### Return type

[**crate::models::FileInfo**](FileInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files

> Vec<crate::models::FileInfo> get_files(channel_id, limit, offset, since, until, inclusive, order, mine)
ファイルメタのリストを取得

指定したクエリでファイルメタのリストを取得します。 クエリパラメータ`channelId`, `mine`の少なくともいずれかが必須です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | Option<**uuid::Uuid**> | アップロード先チャンネルUUID |  |
**limit** | Option<**i32**> | 取得する件数 |  |
**offset** | Option<**i32**> | 取得するオフセット |  |[default to 0]
**since** | Option<**String**> | 取得する時間範囲の開始日時 |  |[default to 0000-01-01T00:00Z]
**until** | Option<**String**> | 取得する時間範囲の終了日時 |  |
**inclusive** | Option<**bool**> | 範囲の端を含めるかどうか |  |[default to false]
**order** | Option<**String**> | 昇順か降順か |  |[default to desc]
**mine** | Option<**bool**> | アップロード者が自分のファイルのみを取得するか |  |[default to false]

### Return type

[**Vec<crate::models::FileInfo>**](FileInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thumbnail_image

> std::path::PathBuf get_thumbnail_image(file_id, r#type)
サムネイル画像を取得

指定したファイルのサムネイル画像を取得します。 指定したファイルへのアクセス権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **uuid::Uuid** | ファイルUUID | [required] |
**r#type** | Option<[**ThumbnailType**](.md)> | 取得するサムネイルのタイプ |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_file

> crate::models::FileInfo post_file(file, channel_id)
ファイルをアップロード

指定したチャンネルにファイルをアップロードします。 アーカイブされているチャンネルにはアップロード出来ません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | ファイル本体 | [required] |
**channel_id** | **uuid::Uuid** | アップロード先チャンネルUUID | [required] |

### Return type

[**crate::models::FileInfo**](FileInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

