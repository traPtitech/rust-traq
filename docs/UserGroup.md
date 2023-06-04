# UserGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | グループUUID | 
**name** | **String** | グループ名 | 
**description** | **String** | グループ説明 | 
**r#type** | **String** | グループタイプ | 
**icon** | [**uuid::Uuid**](uuid::Uuid.md) | グループアイコンUUID | 
**members** | [**Vec<crate::models::UserGroupMember>**](UserGroupMember.md) | グループメンバーの配列 | 
**created_at** | **String** | 作成日時 | 
**updated_at** | **String** | 更新日時 | 
**admins** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | グループ管理者のUUIDの配列 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


