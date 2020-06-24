/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GroupsListSuccessSchema : Schema for successful response groups.list method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupsListSuccessSchema {
    #[serde(rename = "groups")]
    pub groups: Vec<crate::models::ObjsGroup>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
    #[serde(rename = "response_metadata", skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<crate::models::ObjsResponseMetadata>,
}

impl GroupsListSuccessSchema {
    /// Schema for successful response groups.list method
    pub fn new(groups: Vec<crate::models::ObjsGroup>, ok: crate::models::DefsOkTrue) -> GroupsListSuccessSchema {
        GroupsListSuccessSchema {
            groups,
            ok,
            response_metadata: None,
        }
    }
}

