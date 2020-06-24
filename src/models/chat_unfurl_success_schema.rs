/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChatUnfurlSuccessSchema : Schema for successful response from chat.unfurl method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatUnfurlSuccessSchema {
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl ChatUnfurlSuccessSchema {
    /// Schema for successful response from chat.unfurl method
    pub fn new(ok: crate::models::DefsOkTrue) -> ChatUnfurlSuccessSchema {
        ChatUnfurlSuccessSchema {
            ok,
        }
    }
}


