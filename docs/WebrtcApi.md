# \WebrtcApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_web_rtc_state**](WebrtcApi.md#get_web_rtc_state) | **GET** /webrtc/state | WebRTC状態を取得
[**post_web_rtc_authenticate**](WebrtcApi.md#post_web_rtc_authenticate) | **POST** /webrtc/authenticate | Skyway用認証API



## get_web_rtc_state

> Vec<crate::models::WebRtcUserState> get_web_rtc_state()
WebRTC状態を取得

現在のWebRTC状態を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::WebRtcUserState>**](WebRTCUserState.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_web_rtc_authenticate

> crate::models::WebRtcAuthenticateResult post_web_rtc_authenticate(post_web_rtc_authenticate_request)
Skyway用認証API

Skyway WebRTC用の認証API

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_web_rtc_authenticate_request** | Option<[**PostWebRtcAuthenticateRequest**](PostWebRtcAuthenticateRequest.md)> |  |  |

### Return type

[**crate::models::WebRtcAuthenticateResult**](WebRTCAuthenticateResult.md)

### Authorization

[OAuth2](../README.md#OAuth2), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

