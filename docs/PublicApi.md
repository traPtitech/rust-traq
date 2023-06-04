# \PublicApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_public_user_icon**](PublicApi.md#get_public_user_icon) | **GET** /public/icon/{username} | ユーザーのアイコン画像を取得
[**get_server_version**](PublicApi.md#get_server_version) | **GET** /version | バージョンを取得



## get_public_user_icon

> std::path::PathBuf get_public_user_icon(username)
ユーザーのアイコン画像を取得

ユーザーのアイコン画像を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | ユーザー名 | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/gif, image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_version

> crate::models::Version get_server_version()
バージョンを取得

サーバーバージョン及びサーバーフラグ情報を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Version**](Version.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

