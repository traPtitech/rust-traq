# BotDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | BOT UUID | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | 更新日時 | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | 作成日時 | 
**mode** | [**models::BotMode**](BotMode.md) |  | 
**state** | [**models::BotState**](BotState.md) |  | 
**subscribe_events** | **Vec<String>** | BOTが購読しているイベントの配列 | 
**developer_id** | **uuid::Uuid** | BOT開発者UUID | 
**description** | **String** | 説明 | 
**bot_user_id** | **uuid::Uuid** | BOTユーザーUUID | 
**tokens** | [**models::BotTokens**](BotTokens.md) |  | 
**endpoint** | **String** | BOTサーバーエンドポイント | 
**privileged** | **bool** | 特権BOTかどうか | 
**channels** | **Vec<uuid::Uuid>** | BOTが参加しているチャンネルのUUID配列 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


