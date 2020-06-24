/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ImCloseSchema : Schema for successful response from im.close method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImCloseSchema {
    #[serde(rename = "already_closed", skip_serializing_if = "Option::is_none")]
    pub already_closed: Option<bool>,
    #[serde(rename = "no_op", skip_serializing_if = "Option::is_none")]
    pub no_op: Option<bool>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
}

impl ImCloseSchema {
    /// Schema for successful response from im.close method
    pub fn new(ok: crate::models::DefsOkTrue) -> ImCloseSchema {
        ImCloseSchema {
            already_closed: None,
            no_op: None,
            ok,
        }
    }
}


