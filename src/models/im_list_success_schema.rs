/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ImListSuccessSchema : Schema for successful response im.list method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImListSuccessSchema {
    #[serde(rename = "ims")]
    pub ims: Vec<crate::models::ObjsIm>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
    #[serde(rename = "response_metadata", skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<crate::models::ObjsResponseMetadata>,
}

impl ImListSuccessSchema {
    /// Schema for successful response im.list method
    pub fn new(ims: Vec<crate::models::ObjsIm>, ok: crate::models::DefsOkTrue) -> ImListSuccessSchema {
        ImListSuccessSchema {
            ims,
            ok,
            response_metadata: None,
        }
    }
}

