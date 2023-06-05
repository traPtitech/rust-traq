/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChannelSubscribeLevel : チャンネル購読レベル 0：無し 1：未読管理 2：未読管理+通知

/// チャンネル購読レベル 0：無し 1：未読管理 2：未読管理+通知
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChannelSubscribeLevel {
    #[serde(rename = "0")]
    None,
    #[serde(rename = "1")]
    Subscribed,
    #[serde(rename = "2")]
    Notified,
}

impl ToString for ChannelSubscribeLevel {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("0"),
            Self::Subscribed => String::from("1"),
            Self::Notified => String::from("2"),
        }
    }
}

impl Default for ChannelSubscribeLevel {
    fn default() -> ChannelSubscribeLevel {
        Self::None
    }
}
