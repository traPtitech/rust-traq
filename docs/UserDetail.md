# UserDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | ユーザーUUID | 
**state** | [**crate::models::UserAccountState**](UserAccountState.md) |  | 
**bot** | **bool** | BOTかどうか | 
**icon_file_id** | [**uuid::Uuid**](uuid::Uuid.md) | アイコンファイルUUID | 
**display_name** | **String** | ユーザー表示名 | 
**name** | **String** | ユーザー名 | 
**twitter_id** | **String** | Twitter ID | 
**last_online** | Option<**String**> | 最終オンライン日時 | 
**updated_at** | **String** | 更新日時 | 
**tags** | [**Vec<crate::models::UserTag>**](UserTag.md) | タグリスト | 
**groups** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | 所属グループのUUIDの配列 | 
**bio** | **String** | 自己紹介(biography) | 
**home_channel** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | ホームチャンネル | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


