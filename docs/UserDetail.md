# UserDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ユーザーUUID | 
**state** | [**models::UserAccountState**](UserAccountState.md) |  | 
**bot** | **bool** | BOTかどうか | 
**icon_file_id** | **uuid::Uuid** | アイコンファイルUUID | 
**display_name** | **String** | ユーザー表示名 | 
**name** | **String** | ユーザー名 | 
**twitter_id** | **String** | Twitter ID | 
**last_online** | Option<**chrono::DateTime<chrono::FixedOffset>**> | 最後の接続の切断をサーバーが検出した日時 | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | 更新日時 | 
**tags** | [**Vec<models::UserTag>**](UserTag.md) | タグリスト | 
**groups** | **Vec<uuid::Uuid>** | 所属グループのUUIDの配列 | 
**bio** | **String** | 自己紹介(biography) | 
**home_channel** | Option<**uuid::Uuid**> | ホームチャンネル | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


