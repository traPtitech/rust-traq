# \Oauth2Api

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_client**](Oauth2Api.md#create_client) | **POST** /clients | OAuth2クライアントを作成
[**delete_client**](Oauth2Api.md#delete_client) | **DELETE** /clients/{clientId} | OAuth2クライアントを削除
[**edit_client**](Oauth2Api.md#edit_client) | **PATCH** /clients/{clientId} | OAuth2クライアント情報を変更
[**get_client**](Oauth2Api.md#get_client) | **GET** /clients/{clientId} | OAuth2クライアント情報を取得
[**get_clients**](Oauth2Api.md#get_clients) | **GET** /clients | OAuth2クライアントのリストを取得
[**get_my_tokens**](Oauth2Api.md#get_my_tokens) | **GET** /users/me/tokens | 有効トークンのリストを取得
[**get_o_auth2_authorize**](Oauth2Api.md#get_o_auth2_authorize) | **GET** /oauth2/authorize | OAuth2 認可エンドポイント
[**post_o_auth2_authorize**](Oauth2Api.md#post_o_auth2_authorize) | **POST** /oauth2/authorize | OAuth2 認可エンドポイント
[**post_o_auth2_authorize_decide**](Oauth2Api.md#post_o_auth2_authorize_decide) | **POST** /oauth2/authorize/decide | OAuth2 認可承諾API
[**post_o_auth2_token**](Oauth2Api.md#post_o_auth2_token) | **POST** /oauth2/token | OAuth2 トークンエンドポイント
[**revoke_my_token**](Oauth2Api.md#revoke_my_token) | **DELETE** /users/me/tokens/{tokenId} | トークンの認可を取り消す
[**revoke_o_auth2_token**](Oauth2Api.md#revoke_o_auth2_token) | **POST** /oauth2/revoke | OAuth2 トークン無効化エンドポイント



## create_client

> crate::models::OAuth2ClientDetail create_client(post_client_request)
OAuth2クライアントを作成

OAuth2クライアントを作成します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_client_request** | Option<[**PostClientRequest**](PostClientRequest.md)> |  |  |

### Return type

[**crate::models::OAuth2ClientDetail**](OAuth2ClientDetail.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_client

> delete_client(client_id)
OAuth2クライアントを削除

指定したOAuth2クライアントを削除します。 対象のクライアントの管理権限が必要です。正常に削除された場合、このクライアントに対する認可は全て取り消されます。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | OAuth2クライアントUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_client

> edit_client(client_id, patch_client_request)
OAuth2クライアント情報を変更

指定したOAuth2クライアントの情報を変更します。 対象のクライアントの管理権限が必要です。 クライアント開発者UUIDを変更した場合は、変更先ユーザーにクライアント管理権限が移譲され、自分自身は権限を失います。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | OAuth2クライアントUUID | [required] |
**patch_client_request** | Option<[**PatchClientRequest**](PatchClientRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client

> crate::models::GetClient200Response get_client(client_id, detail)
OAuth2クライアント情報を取得

指定したOAuth2クライアントの情報を取得します。 詳細情報の取得には対象のクライアントの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | OAuth2クライアントUUID | [required] |
**detail** | Option<**bool**> | 詳細情報を含めるかどうか |  |[default to false]

### Return type

[**crate::models::GetClient200Response**](getClient_200_response.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clients

> Vec<crate::models::OAuth2Client> get_clients(all)
OAuth2クライアントのリストを取得

自身が開発者のOAuth2クライアントのリストを取得します。 `all`が`true`の場合、全開発者の全クライアントのリストを返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | 全てのクライアントを取得するかどうか |  |[default to false]

### Return type

[**Vec<crate::models::OAuth2Client>**](OAuth2Client.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_tokens

> Vec<crate::models::ActiveOAuth2Token> get_my_tokens()
有効トークンのリストを取得

有効な自分に発行されたOAuth2トークンのリストを取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ActiveOAuth2Token>**](ActiveOAuth2Token.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_o_auth2_authorize

> get_o_auth2_authorize(client_id, response_type, redirect_uri, scope, state, code_challenge, code_challenge_method, nonce, prompt)
OAuth2 認可エンドポイント

OAuth2 認可エンドポイント

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** |  | [required] |
**response_type** | Option<[**OAuth2ResponseType**](.md)> |  |  |
**redirect_uri** | Option<**String**> |  |  |
**scope** | Option<**String**> |  |  |
**state** | Option<**String**> |  |  |
**code_challenge** | Option<**String**> |  |  |
**code_challenge_method** | Option<**String**> |  |  |
**nonce** | Option<**String**> |  |  |
**prompt** | Option<[**OAuth2Prompt**](.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_o_auth2_authorize

> post_o_auth2_authorize(client_id, response_type, redirect_uri, scope, state, code_challenge, code_challenge_method, nonce, prompt)
OAuth2 認可エンドポイント

OAuth2 認可エンドポイント

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** |  | [required] |
**response_type** | Option<[**crate::models::OAuth2ResponseType**](OAuth2ResponseType.md)> |  |  |
**redirect_uri** | Option<**String**> |  |  |
**scope** | Option<**String**> |  |  |
**state** | Option<**String**> |  |  |
**code_challenge** | Option<**String**> |  |  |
**code_challenge_method** | Option<**String**> |  |  |
**nonce** | Option<**String**> |  |  |
**prompt** | Option<[**crate::models::OAuth2Prompt**](OAuth2Prompt.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_o_auth2_authorize_decide

> post_o_auth2_authorize_decide(submit)
OAuth2 認可承諾API

OAuth2 認可承諾

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit** | **String** | 承諾する場合は\\\"approve\\\" | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_o_auth2_token

> crate::models::OAuth2Token post_o_auth2_token(grant_type, code, redirect_uri, client_id, code_verifier, username, password, scope, refresh_token, client_secret)
OAuth2 トークンエンドポイント

OAuth2 トークンエンドポイント

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** |  | [required] |
**code** | Option<**String**> |  |  |
**redirect_uri** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**code_verifier** | Option<**String**> |  |  |
**username** | Option<**String**> |  |  |
**password** | Option<**String**> |  |  |
**scope** | Option<**String**> |  |  |
**refresh_token** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |

### Return type

[**crate::models::OAuth2Token**](OAuth2Token.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_my_token

> revoke_my_token(token_id)
トークンの認可を取り消す

自分の指定したトークンの認可を取り消します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **uuid::Uuid** | OAuth2トークンUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_o_auth2_token

> revoke_o_auth2_token(token)
OAuth2 トークン無効化エンドポイント

OAuth2 トークン無効化エンドポイント

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | 無効化するOAuth2トークンまたはOAuth2リフレッシュトークン | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

