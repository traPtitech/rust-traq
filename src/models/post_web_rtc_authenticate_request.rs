/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostWebRtcAuthenticateRequest : skyway用認証リクエスト

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostWebRtcAuthenticateRequest {
    /// ピアID
    #[serde(rename = "peerId")]
    pub peer_id: String,
}

impl PostWebRtcAuthenticateRequest {
    /// skyway用認証リクエスト
    pub fn new(peer_id: String) -> PostWebRtcAuthenticateRequest {
        PostWebRtcAuthenticateRequest { peer_id }
    }
}