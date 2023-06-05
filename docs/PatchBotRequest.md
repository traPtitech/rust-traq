# PatchBotRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_name** | Option<**String**> | BOTユーザー表示名 | [optional]
**description** | Option<**String**> | BOTの説明 | [optional]
**privileged** | Option<**bool**> | 特権 | [optional]
**mode** | Option<[**crate::models::BotMode**](BotMode.md)> |  | [optional]
**endpoint** | Option<**String**> | BOTサーバーエンドポイント | [optional]
**developer_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | 移譲先の開発者UUID | [optional]
**subscribe_events** | Option<**Vec<String>**> | 購読するイベント | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


