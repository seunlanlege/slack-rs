/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChannelsListSuccessSchema : Schema for successful response channels.list method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelsListSuccessSchema {
    #[serde(rename = "channels")]
    pub channels: Vec<crate::models::ObjsChannel>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
    #[serde(rename = "response_metadata", skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<crate::models::ObjsResponseMetadata>,
}

impl ChannelsListSuccessSchema {
    /// Schema for successful response channels.list method
    pub fn new(channels: Vec<crate::models::ObjsChannel>, ok: crate::models::DefsOkTrue) -> ChannelsListSuccessSchema {
        ChannelsListSuccessSchema {
            channels,
            ok,
            response_metadata: None,
        }
    }
}


