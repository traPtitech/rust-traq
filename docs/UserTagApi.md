# \UserTagApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_my_user_tag**](UserTagApi.md#add_my_user_tag) | **POST** /users/me/tags | 自分にタグを追加
[**add_user_tag**](UserTagApi.md#add_user_tag) | **POST** /users/{userId}/tags | ユーザーにタグを追加
[**edit_my_user_tag**](UserTagApi.md#edit_my_user_tag) | **PATCH** /users/me/tags/{tagId} | 自分のタグを編集
[**edit_user_tag**](UserTagApi.md#edit_user_tag) | **PATCH** /users/{userId}/tags/{tagId} | ユーザーのタグを編集
[**get_my_user_tags**](UserTagApi.md#get_my_user_tags) | **GET** /users/me/tags | 自分のタグリストを取得
[**get_tag**](UserTagApi.md#get_tag) | **GET** /tags/{tagId} | タグ情報を取得
[**get_user_tags**](UserTagApi.md#get_user_tags) | **GET** /users/{userId}/tags | ユーザーのタグリストを取得
[**remove_my_user_tag**](UserTagApi.md#remove_my_user_tag) | **DELETE** /users/me/tags/{tagId} | 自分からタグを削除します
[**remove_user_tag**](UserTagApi.md#remove_user_tag) | **DELETE** /users/{userId}/tags/{tagId} | ユーザーからタグを削除します



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


## add_user_tag

> crate::models::UserTag add_user_tag(user_id, post_user_tag_request)
ユーザーにタグを追加

指定したユーザーに指定したタグを追加します。 Webhookユーザーにタグを追加することは出来ません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | ユーザーUUID | [required] |
**post_user_tag_request** | Option<[**PostUserTagRequest**](PostUserTagRequest.md)> |  |  |

### Return type

[**crate::models::UserTag**](UserTag.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

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


## edit_user_tag

> edit_user_tag(user_id, tag_id, patch_user_tag_request)
ユーザーのタグを編集

指定したユーザーの指定したタグの状態を変更します。 他人の状態は変更できません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | ユーザーUUID | [required] |
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


## get_tag

> crate::models::Tag get_tag(tag_id)
タグ情報を取得

指定したタグの情報を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **uuid::Uuid** | タグUUID | [required] |

### Return type

[**crate::models::Tag**](Tag.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_tags

> Vec<crate::models::UserTag> get_user_tags(user_id)
ユーザーのタグリストを取得

指定したユーザーのタグリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | ユーザーUUID | [required] |

### Return type

[**Vec<crate::models::UserTag>**](UserTag.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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


## remove_user_tag

> remove_user_tag(user_id, tag_id)
ユーザーからタグを削除します

既に存在しないタグを削除しようとした場合は204を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | ユーザーUUID | [required] |
**tag_id** | **uuid::Uuid** | タグUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

