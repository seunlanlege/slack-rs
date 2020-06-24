/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChannelsRenameSchema : Schema for successful response from channels.rename method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelsRenameSchema {
    #[serde(rename = "channel")]
    pub channel: crate::models::ObjsChannel,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl ChannelsRenameSchema {
    /// Schema for successful response from channels.rename method
    pub fn new(channel: crate::models::ObjsChannel, ok: crate::models::DefsOkTrue) -> ChannelsRenameSchema {
        ChannelsRenameSchema {
            channel,
            ok,
        }
    }
}


