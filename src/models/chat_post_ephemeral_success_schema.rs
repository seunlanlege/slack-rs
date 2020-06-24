/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChatPostEphemeralSuccessSchema : Schema for successful response from chat.postEphemeral method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatPostEphemeralSuccessSchema {
    #[serde(rename = "message_ts")]
    pub message_ts: String,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl ChatPostEphemeralSuccessSchema {
    /// Schema for successful response from chat.postEphemeral method
    pub fn new(message_ts: String, ok: crate::models::DefsOkTrue) -> ChatPostEphemeralSuccessSchema {
        ChatPostEphemeralSuccessSchema {
            message_ts,
            ok,
        }
    }
}

