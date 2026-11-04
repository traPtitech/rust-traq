# Bot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | BOT UUID | 
**bot_user_id** | **uuid::Uuid** | BOTユーザーUUID | 
**description** | **String** | 説明 | 
**developer_id** | **uuid::Uuid** | BOT開発者UUID | 
**subscribe_events** | **Vec<String>** | BOTが購読しているイベントの配列 | 
**mode** | [**models::BotMode**](BotMode.md) |  | 
**state** | [**models::BotState**](BotState.md) |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | 作成日時 | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | 更新日時 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


