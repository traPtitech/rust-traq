# Message

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | メッセージUUID | 
**user_id** | [**uuid::Uuid**](uuid::Uuid.md) | 投稿者UUID | 
**channel_id** | [**uuid::Uuid**](uuid::Uuid.md) | チャンネルUUID | 
**content** | **String** | メッセージ本文 | 
**created_at** | **String** | 投稿日時 | 
**updated_at** | **String** | 編集日時 | 
**pinned** | **bool** | ピン留めされているかどうか | 
**stamps** | [**Vec<crate::models::MessageStamp>**](MessageStamp.md) | 押されているスタンプの配列 | 
**thread_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | スレッドUUID | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


