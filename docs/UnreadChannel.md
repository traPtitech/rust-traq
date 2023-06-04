# UnreadChannel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel_id** | [**uuid::Uuid**](uuid::Uuid.md) | チャンネルUUID | 
**count** | **i32** | 未読メッセージ数 | 
**noticeable** | **bool** | 自分宛てメッセージが含まれているかどうか | 
**since** | **String** | チャンネルの最古の未読メッセージの日時 | 
**updated_at** | **String** | チャンネルの最新の未読メッセージの日時 | 
**oldest_message_id** | [**uuid::Uuid**](uuid::Uuid.md) | そのチャンネルの未読の中で最も古いメッセージのid | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


