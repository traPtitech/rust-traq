# MyUserDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ユーザーUUID | 
**bio** | **String** | 自己紹介(biography) | 
**groups** | **Vec<uuid::Uuid>** | 所属グループのUUIDの配列 | 
**tags** | [**Vec<models::UserTag>**](UserTag.md) | タグリスト | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | 更新日時 | 
**last_online** | Option<**chrono::DateTime<chrono::FixedOffset>**> | 最後の接続の切断をサーバーが検出した日時 | 
**twitter_id** | **String** | Twitter ID | 
**name** | **String** | ユーザー名 | 
**display_name** | **String** | ユーザー表示名 | 
**icon_file_id** | **uuid::Uuid** | アイコンファイルUUID | 
**bot** | **bool** | BOTかどうか | 
**state** | [**models::UserAccountState**](UserAccountState.md) |  | 
**permissions** | [**Vec<models::UserPermission>**](UserPermission.md) | 所有している権限の配列 | 
**home_channel** | Option<**uuid::Uuid**> | ホームチャンネル | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


