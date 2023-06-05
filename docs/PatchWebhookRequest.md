# PatchWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Webhookユーザー表示名 | [optional]
**description** | Option<**String**> | 説明 | [optional]
**channel_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | デフォルトの投稿先チャンネルUUID | [optional]
**secret** | Option<**String**> | Webhookシークレット | [optional]
**owner_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | 移譲先のユーザーUUID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


