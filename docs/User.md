# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ユーザーUUID | 
**name** | **String** | ユーザー名 | 
**display_name** | **String** | ユーザー表示名 | 
**icon_file_id** | **uuid::Uuid** | アイコンファイルUUID | 
**bot** | **bool** | BOTかどうか | 
**state** | [**models::UserAccountState**](UserAccountState.md) |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | 更新日時 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


