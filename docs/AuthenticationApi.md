# \AuthenticationApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_my_external_accounts**](AuthenticationApi.md#get_my_external_accounts) | **GET** /users/me/ex-accounts | 外部ログインアカウント一覧を取得
[**get_my_sessions**](AuthenticationApi.md#get_my_sessions) | **GET** /users/me/sessions | 自分のログインセッションリストを取得
[**link_external_account**](AuthenticationApi.md#link_external_account) | **POST** /users/me/ex-accounts/link | 外部ログインアカウントを紐付ける
[**login**](AuthenticationApi.md#login) | **POST** /login | ログイン
[**logout**](AuthenticationApi.md#logout) | **POST** /logout | ログアウト
[**revoke_my_session**](AuthenticationApi.md#revoke_my_session) | **DELETE** /users/me/sessions/{sessionId} | セッションを無効化
[**unlink_external_account**](AuthenticationApi.md#unlink_external_account) | **POST** /users/me/ex-accounts/unlink | 外部ログインアカウントの紐付けを解除



## get_my_external_accounts

> Vec<crate::models::ExternalProviderUser> get_my_external_accounts()
外部ログインアカウント一覧を取得

自分に紐付けられている外部ログインアカウント一覧を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ExternalProviderUser>**](ExternalProviderUser.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_sessions

> Vec<crate::models::LoginSession> get_my_sessions()
自分のログインセッションリストを取得

自分のログインセッションのリストを取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LoginSession>**](LoginSession.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_external_account

> link_external_account(post_link_external_account)
外部ログインアカウントを紐付ける

自分に外部ログインアカウントを紐付けます。 指定した`providerName`がサーバー側で有効である必要があります。 リクエストが受理された場合、外部サービスの認証画面にリダイレクトされ、認証される必要があります。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_link_external_account** | Option<[**PostLinkExternalAccount**](PostLinkExternalAccount.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> login(redirect, post_login_request)
ログイン

ログインします。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**redirect** | Option<**String**> | リダイレクト先 |  |
**post_login_request** | Option<[**PostLoginRequest**](PostLoginRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> logout(redirect, all)
ログアウト

ログアウトします。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**redirect** | Option<**String**> | リダイレクト先 |  |
**all** | Option<**bool**> | 全てのセッションでログアウトするかどうか |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_my_session

> revoke_my_session(session_id)
セッションを無効化

指定した自分のセッションを無効化(ログアウト)します。 既に存在しない・無効化されているセッションを指定した場合も`204`を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **uuid::Uuid** | セッションUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_external_account

> unlink_external_account(post_unlink_external_account)
外部ログインアカウントの紐付けを解除

自分に紐付けられている外部ログインアカウントの紐付けを解除します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_unlink_external_account** | Option<[**PostUnlinkExternalAccount**](PostUnlinkExternalAccount.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

