# \OgpApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_ogp_cache**](OgpApi.md#delete_ogp_cache) | **DELETE** /ogp/cache | OGP情報のキャッシュを削除
[**get_ogp**](OgpApi.md#get_ogp) | **GET** /ogp | OGP情報を取得



## delete_ogp_cache

> delete_ogp_cache(url)
OGP情報のキャッシュを削除

指定されたURLのOGP情報のキャッシュを削除します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | OGPのキャッシュを削除したいURL | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ogp

> crate::models::Ogp get_ogp(url)
OGP情報を取得

指定されたURLのOGP情報を取得します。 指定されたURLに対するOGP情報が見つからなかった場合、typeがemptyに設定された空のOGP情報を返します。 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | OGPを取得したいURL | [required] |

### Return type

[**crate::models::Ogp**](Ogp.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

