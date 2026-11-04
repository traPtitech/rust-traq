# BotEventLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bot_id** | **uuid::Uuid** | BOT UUID | 
**request_id** | **uuid::Uuid** | リクエストUUID | 
**event** | **String** | イベントタイプ | 
**result** | Option<[**models::BotEventResult**](BotEventResult.md)> |  | [optional]
**code** | **i32** | ステータスコード | 
**datetime** | **chrono::DateTime<chrono::FixedOffset>** | イベント日時 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


