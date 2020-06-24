/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GroupsCreateChildSchema : Schema for successful response from groups.createChild method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupsCreateChildSchema {
    #[serde(rename = "group")]
    pub group: crate::models::ObjsGroup,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl GroupsCreateChildSchema {
    /// Schema for successful response from groups.createChild method
    pub fn new(group: crate::models::ObjsGroup, ok: crate::models::DefsOkTrue) -> GroupsCreateChildSchema {
        GroupsCreateChildSchema {
            group,
            ok,
        }
    }
}


