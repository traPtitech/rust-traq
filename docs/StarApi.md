# \StarApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_my_star**](StarApi.md#add_my_star) | **POST** /users/me/stars | チャンネルをスターに追加
[**get_my_stars**](StarApi.md#get_my_stars) | **GET** /users/me/stars | スターチャンネルリストを取得
[**remove_my_star**](StarApi.md#remove_my_star) | **DELETE** /users/me/stars/{channelId} | チャンネルをスターから削除します



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

