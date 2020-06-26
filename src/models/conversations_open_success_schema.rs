/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConversationsOpenSuccessSchema : Schema for successful response from conversations.open method when opening channels, ims, mpims



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversationsOpenSuccessSchema {
    #[serde(rename = "already_open", skip_serializing_if = "Option::is_none")]
    pub already_open: Option<bool>,
    #[serde(rename = "channel")]
    pub channel: crate::models::ObjsConversationIm,
    #[serde(rename = "no_op", skip_serializing_if = "Option::is_none")]
    pub no_op: Option<bool>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl ConversationsOpenSuccessSchema {
    /// Schema for successful response from conversations.open method when opening channels, ims, mpims
    pub fn new(channel: crate::models::ObjsConversationIm, ok: crate::models::DefsOkTrue) -> ConversationsOpenSuccessSchema {
        ConversationsOpenSuccessSchema {
            already_open: None,
            channel,
            no_op: None,
            ok,
        }
    }
}


