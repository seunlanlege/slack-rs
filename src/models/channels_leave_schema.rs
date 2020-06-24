/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChannelsLeaveSchema : Schema for successful response from channels.leave method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelsLeaveSchema {
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl ChannelsLeaveSchema {
    /// Schema for successful response from channels.leave method
    pub fn new(ok: crate::models::DefsOkTrue) -> ChannelsLeaveSchema {
        ChannelsLeaveSchema {
            ok,
        }
    }
}


