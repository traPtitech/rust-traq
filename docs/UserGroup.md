# UserGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | グループUUID | 
**name** | **String** | グループ名 | 
**description** | **String** | グループ説明 | 
**r#type** | **String** | グループタイプ | 
**icon** | **uuid::Uuid** | グループアイコンUUID | 
**members** | [**Vec<models::UserGroupMember>**](UserGroupMember.md) | グループメンバーの配列 | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | 作成日時 | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | 更新日時 | 
**admins** | **Vec<uuid::Uuid>** | グループ管理者のUUIDの配列 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


