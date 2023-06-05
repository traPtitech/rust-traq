/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostMessageStampRequest : スタンプを押すリクエスト

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostMessageStampRequest {
    /// 押す数
    #[serde(rename = "count")]
    pub count: i32,
}

impl PostMessageStampRequest {
    /// スタンプを押すリクエスト
    pub fn new(count: i32) -> PostMessageStampRequest {
        PostMessageStampRequest { count }
    }
}