# ChannelEventDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | [**uuid::Uuid**](uuid::Uuid.md) | 作成者UUID | 
**before** | [**uuid::Uuid**](uuid::Uuid.md) | 変更前親チャンネルUUID | 
**after** | [**uuid::Uuid**](uuid::Uuid.md) | 変更後親チャンネルUUID | 
**on** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | オンにされたユーザーのUUID配列 | 
**off** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | オフにされたユーザーのUUID配列 | 
**message_id** | [**uuid::Uuid**](uuid::Uuid.md) | メッセージUUID | 
**visibility** | **bool** | 変更後可視状態 | 
**force** | **bool** | 変更後強制通知状態 | 
**channel_id** | [**uuid::Uuid**](uuid::Uuid.md) | チャンネルUUID | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


