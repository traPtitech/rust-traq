# MyUserDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | ユーザーUUID | 
**bio** | **String** | 自己紹介(biography) | 
**groups** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | 所属グループのUUIDの配列 | 
**tags** | [**Vec<crate::models::UserTag>**](UserTag.md) | タグリスト | 
**updated_at** | **String** | 更新日時 | 
**last_online** | Option<**String**> | 最終オンライン日時 | 
**twitter_id** | **String** | Twitter ID | 
**name** | **String** | ユーザー名 | 
**display_name** | **String** | ユーザー表示名 | 
**icon_file_id** | [**uuid::Uuid**](uuid::Uuid.md) | アイコンファイルUUID | 
**bot** | **bool** | BOTかどうか | 
**state** | [**crate::models::UserAccountState**](UserAccountState.md) |  | 
**permissions** | [**Vec<crate::models::UserPermission>**](UserPermission.md) | 所有している権限の配列 | 
**home_channel** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | ホームチャンネル | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


