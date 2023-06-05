# \ActivityApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_activity_timeline**](ActivityApi.md#get_activity_timeline) | **GET** /activity/timeline | アクテビティタイムラインを取得
[**get_online_users**](ActivityApi.md#get_online_users) | **GET** /activity/onlines | オンラインユーザーリストを取得



## get_activity_timeline

> Vec<crate::models::ActivityTimelineMessage> get_activity_timeline(limit, all, per_channel)
アクテビティタイムラインを取得

パブリックチャンネルの直近の投稿メッセージを作成日時の降順で取得します。 `all`が`true`でない場合、購読チャンネルのみのタイムラインを取得します

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | 取得する件数 |  |[default to 50]
**all** | Option<**bool**> | 全てのチャンネルのタイムラインを取得する |  |[default to false]
**per_channel** | Option<**bool**> | 同じチャンネルのメッセージは最新のもののみ取得するか |  |[default to false]

### Return type

[**Vec<crate::models::ActivityTimelineMessage>**](ActivityTimelineMessage.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_online_users

> Vec<String> get_online_users()
オンラインユーザーリストを取得

現在オンラインな(SSEまたはWSが接続中)ユーザーのUUIDのリストを返します。

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

