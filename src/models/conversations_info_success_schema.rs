/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConversationsInfoSuccessSchema : Schema for successful response conversations.info



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversationsInfoSuccessSchema {
    #[serde(rename = "channel")]
    pub channel: crate::models::ObjsConversation,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl ConversationsInfoSuccessSchema {
    /// Schema for successful response conversations.info
    pub fn new(channel: crate::models::ObjsConversation, ok: crate::models::DefsOkTrue) -> ConversationsInfoSuccessSchema {
        ConversationsInfoSuccessSchema {
            channel,
            ok,
        }
    }
}


