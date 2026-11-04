# Webhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | WebhookUUID | 
**bot_user_id** | **uuid::Uuid** | WebhookユーザーUUID | 
**display_name** | **String** | Webhookユーザー表示名 | 
**description** | **String** | 説明 | 
**secure** | **bool** | セキュアWebhookかどうか | 
**channel_id** | **uuid::Uuid** | デフォルトの投稿先チャンネルUUID | 
**owner_id** | **uuid::Uuid** | オーナーUUID | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | 作成日時 | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | 更新日時 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


