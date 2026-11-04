# StampPalette

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | スタンプパレットUUID | 
**name** | **String** | パレット名 | 
**stamps** | **HashSet<uuid::Uuid>** | パレット内のスタンプのUUID配列 | 
**creator_id** | **uuid::Uuid** | 作成者UUID | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | パレット作成日時 | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | パレット更新日時 | 
**description** | **String** | パレット説明 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


