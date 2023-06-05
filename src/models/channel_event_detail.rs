/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChannelEventDetail : イベント内容

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChannelEventDetail {
    /// 作成者UUID
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
    /// 変更前親チャンネルUUID
    #[serde(rename = "before")]
    pub before: uuid::Uuid,
    /// 変更後親チャンネルUUID
    #[serde(rename = "after")]
    pub after: uuid::Uuid,
    /// オンにされたユーザーのUUID配列
    #[serde(rename = "on")]
    pub on: Vec<uuid::Uuid>,
    /// オフにされたユーザーのUUID配列
    #[serde(rename = "off")]
    pub off: Vec<uuid::Uuid>,
    /// メッセージUUID
    #[serde(rename = "messageId")]
    pub message_id: uuid::Uuid,
    /// 変更後可視状態
    #[serde(rename = "visibility")]
    pub visibility: bool,
    /// 変更後強制通知状態
    #[serde(rename = "force")]
    pub force: bool,
    /// チャンネルUUID
    #[serde(rename = "channelId")]
    pub channel_id: uuid::Uuid,
}

impl ChannelEventDetail {
    /// イベント内容
    pub fn new(
        user_id: uuid::Uuid,
        before: uuid::Uuid,
        after: uuid::Uuid,
        on: Vec<uuid::Uuid>,
        off: Vec<uuid::Uuid>,
        message_id: uuid::Uuid,
        visibility: bool,
        force: bool,
        channel_id: uuid::Uuid,
    ) -> ChannelEventDetail {
        ChannelEventDetail {
            user_id,
            before,
            after,
            on,
            off,
            message_id,
            visibility,
            force,
            channel_id,
        }
    }
}