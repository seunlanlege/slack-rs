/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConversationsCreateSuccessSchema : Schema for successful response conversations.create method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversationsCreateSuccessSchema {
    #[serde(rename = "channel")]
    pub channel: crate::models::ObjsConversation,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl ConversationsCreateSuccessSchema {
    /// Schema for successful response conversations.create method
    pub fn new(channel: crate::models::ObjsConversation, ok: crate::models::DefsOkTrue) -> ConversationsCreateSuccessSchema {
        ConversationsCreateSuccessSchema {
            channel,
            ok,
        }
    }
}


