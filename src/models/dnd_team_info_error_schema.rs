/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DndTeamInfoErrorSchema : Schema for error response from dnd.teamInfo method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DndTeamInfoErrorSchema {
    #[serde(rename = "error")]
    pub error: Error,
    #[serde(rename = "needed", skip_serializing_if = "Option::is_none")]
    pub needed: Option<String>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkFalse,
    #[serde(rename = "provided", skip_serializing_if = "Option::is_none")]
    pub provided: Option<String>,
}

impl DndTeamInfoErrorSchema {
    /// Schema for error response from dnd.teamInfo method
    pub fn new(error: Error, ok: crate::models::DefsOkFalse) -> DndTeamInfoErrorSchema {
        DndTeamInfoErrorSchema {
            error,
            needed: None,
            ok,
            provided: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Error {
    #[serde(rename = "not_authed")]
    NotAuthed,
    #[serde(rename = "invalid_auth")]
    InvalidAuth,
    #[serde(rename = "account_inactive")]
    AccountInactive,
    #[serde(rename = "invalid_arg_name")]
    InvalidArgName,
    #[serde(rename = "invalid_array_arg")]
    InvalidArrayArg,
    #[serde(rename = "invalid_charset")]
    InvalidCharset,
    #[serde(rename = "invalid_form_data")]
    InvalidFormData,
    #[serde(rename = "invalid_post_type")]
    InvalidPostType,
    #[serde(rename = "missing_post_type")]
    MissingPostType,
    #[serde(rename = "team_added_to_org")]
    TeamAddedToOrg,
    #[serde(rename = "request_timeout")]
    RequestTimeout,
    #[serde(rename = "upgrade_required")]
    UpgradeRequired,
}

