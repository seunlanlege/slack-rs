/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChannelsRepliesSchema : Schema for successful response from channels.replies method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelsRepliesSchema {
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "messages")]
    pub messages: Vec<serde_json::Value>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl ChannelsRepliesSchema {
    /// Schema for successful response from channels.replies method
    pub fn new(has_more: bool, messages: Vec<serde_json::Value>, ok: crate::models::DefsOkTrue) -> ChannelsRepliesSchema {
        ChannelsRepliesSchema {
            has_more,
            messages,
            ok,
        }
    }
}


