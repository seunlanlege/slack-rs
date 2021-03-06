/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MpimListSchema : Schema for successful response from mpim.list method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpimListSchema {
    #[serde(rename = "groups")]
    pub groups: Vec<crate::models::ObjsGroup>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
    #[serde(rename = "response_metadata", skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<crate::models::ObjsResponseMetadata>,
}

impl MpimListSchema {
    /// Schema for successful response from mpim.list method
    pub fn new(groups: Vec<crate::models::ObjsGroup>, ok: crate::models::DefsOkTrue) -> MpimListSchema {
        MpimListSchema {
            groups,
            ok,
            response_metadata: None,
        }
    }
}


