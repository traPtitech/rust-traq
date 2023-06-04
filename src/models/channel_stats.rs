/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChannelStats : チャンネル統計情報

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChannelStats {
    /// チャンネルの総投稿メッセージ数(削除されたものも含む)
    #[serde(rename = "totalMessageCount")]
    pub total_message_count: i64,
    /// チャンネル上のスタンプ統計情報
    #[serde(rename = "stamps")]
    pub stamps: Vec<crate::models::ChannelStatsStamp>,
    /// チャンネル上のユーザー統計情報
    #[serde(rename = "users")]
    pub users: Vec<crate::models::ChannelStatsUser>,
    /// 統計情報日時
    #[serde(rename = "datetime")]
    pub datetime: String,
}

impl ChannelStats {
    /// チャンネル統計情報
    pub fn new(
        total_message_count: i64,
        stamps: Vec<crate::models::ChannelStatsStamp>,
        users: Vec<crate::models::ChannelStatsUser>,
        datetime: String,
    ) -> ChannelStats {
        ChannelStats {
            total_message_count,
            stamps,
            users,
            datetime,
        }
    }
}