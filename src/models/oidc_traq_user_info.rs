/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// OidcTraqUserInfo : traQ特有のユーザー詳細情報

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OidcTraqUserInfo {
    /// 自己紹介(biography)
    #[serde(rename = "bio")]
    pub bio: String,
    /// 所属グループのUUIDの配列
    #[serde(rename = "groups")]
    pub groups: Vec<uuid::Uuid>,
    /// タグリスト
    #[serde(rename = "tags")]
    pub tags: Vec<crate::models::UserTag>,
    /// 最終オンライン日時
    #[serde(rename = "last_online", deserialize_with = "Option::deserialize")]
    pub last_online: Option<String>,
    /// Twitter ID
    #[serde(rename = "twitter_id")]
    pub twitter_id: String,
    /// ユーザー表示名
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// アイコンファイルUUID
    #[serde(rename = "icon_file_id")]
    pub icon_file_id: uuid::Uuid,
    /// BOTかどうか
    #[serde(rename = "bot")]
    pub bot: bool,
    #[serde(rename = "state")]
    pub state: crate::models::UserAccountState,
    /// 所有している権限の配列
    #[serde(rename = "permissions")]
    pub permissions: Vec<crate::models::UserPermission>,
    /// ホームチャンネル
    #[serde(rename = "home_channel", deserialize_with = "Option::deserialize")]
    pub home_channel: Option<uuid::Uuid>,
}

impl OidcTraqUserInfo {
    /// traQ特有のユーザー詳細情報
    pub fn new(
        bio: String,
        groups: Vec<uuid::Uuid>,
        tags: Vec<crate::models::UserTag>,
        last_online: Option<String>,
        twitter_id: String,
        display_name: String,
        icon_file_id: uuid::Uuid,
        bot: bool,
        state: crate::models::UserAccountState,
        permissions: Vec<crate::models::UserPermission>,
        home_channel: Option<uuid::Uuid>,
    ) -> OidcTraqUserInfo {
        OidcTraqUserInfo {
            bio,
            groups,
            tags,
            last_online,
            twitter_id,
            display_name,
            icon_file_id,
            bot,
            state,
            permissions,
            home_channel,
        }
    }
}
