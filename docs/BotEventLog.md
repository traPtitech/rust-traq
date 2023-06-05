# BotEventLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bot_id** | [**uuid::Uuid**](uuid::Uuid.md) | BOT UUID | 
**request_id** | [**uuid::Uuid**](uuid::Uuid.md) | リクエストUUID | 
**event** | **String** | イベントタイプ | 
**result** | Option<[**crate::models::BotEventResult**](BotEventResult.md)> |  | [optional]
**code** | **i32** | ステータスコード | 
**datetime** | **String** | イベント日時 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


