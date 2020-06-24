/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GroupsHistorySuccessSchema : Schema for successful response groups.history method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupsHistorySuccessSchema {
    #[serde(rename = "channel_actions_count")]
    pub channel_actions_count: i32,
    #[serde(rename = "channel_actions_ts")]
    pub channel_actions_ts: serde_json::Value,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "messages")]
    pub messages: Vec<crate::models::ObjsMessage>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl GroupsHistorySuccessSchema {
    /// Schema for successful response groups.history method
    pub fn new(channel_actions_count: i32, channel_actions_ts: serde_json::Value, has_more: bool, messages: Vec<crate::models::ObjsMessage>, ok: crate::models::DefsOkTrue) -> GroupsHistorySuccessSchema {
        GroupsHistorySuccessSchema {
            channel_actions_count,
            channel_actions_ts,
            has_more,
            messages,
            ok,
        }
    }
}


