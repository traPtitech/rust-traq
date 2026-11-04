# Message

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | メッセージUUID | 
**user_id** | **uuid::Uuid** | 投稿者UUID | 
**channel_id** | **uuid::Uuid** | チャンネルUUID | 
**content** | **String** | メッセージ本文 | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | 投稿日時 | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | 編集日時 | 
**pinned** | **bool** | ピン留めされているかどうか | 
**stamps** | [**Vec<models::MessageStamp>**](MessageStamp.md) | 押されているスタンプの配列 | 
**thread_id** | Option<**uuid::Uuid**> | スレッドUUID | 
**nonce** | Option<**String**> | メッセージ送信の確認に使うことができる任意の識別子(投稿でのみ使用可) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


