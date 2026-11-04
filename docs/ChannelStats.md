# ChannelStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_message_count** | **i64** | チャンネルの総投稿メッセージ数(削除されたものも含む) | 
**stamps** | [**Vec<models::ChannelStatsStamp>**](ChannelStatsStamp.md) | チャンネル上のスタンプ統計情報 | 
**users** | [**Vec<models::ChannelStatsUser>**](ChannelStatsUser.md) | チャンネル上のユーザー統計情報 | 
**datetime** | **chrono::DateTime<chrono::FixedOffset>** | 統計情報日時 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


