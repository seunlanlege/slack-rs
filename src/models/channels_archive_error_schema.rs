/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChannelsArchiveErrorSchema : Schema for error response channels.archive method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelsArchiveErrorSchema {
    #[serde(rename = "error")]
    pub error: Error,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkFalse,
}

impl ChannelsArchiveErrorSchema {
    /// Schema for error response channels.archive method
    pub fn new(error: Error, ok: crate::models::DefsOkFalse) -> ChannelsArchiveErrorSchema {
        ChannelsArchiveErrorSchema {
            error,
            ok,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Error {
    #[serde(rename = "channel_not_found")]
    ChannelNotFound,
    #[serde(rename = "already_archived")]
    AlreadyArchived,
    #[serde(rename = "cant_archive_general")]
    CantArchiveGeneral,
    #[serde(rename = "restricted_action")]
    RestrictedAction,
    #[serde(rename = "not_authed")]
    NotAuthed,
    #[serde(rename = "invalid_auth")]
    InvalidAuth,
    #[serde(rename = "account_inactive")]
    AccountInactive,
    #[serde(rename = "user_is_bot")]
    UserIsBot,
    #[serde(rename = "user_is_restricted")]
    UserIsRestricted,
    #[serde(rename = "user_is_ultra_restricted")]
    UserIsUltraRestricted,
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
    #[serde(rename = "invalid_json")]
    InvalidJson,
    #[serde(rename = "json_not_object")]
    JsonNotObject,
    #[serde(rename = "request_timeout")]
    RequestTimeout,
    #[serde(rename = "upgrade_required")]
    UpgradeRequired,
    #[serde(rename = "team_added_to_org")]
    TeamAddedToOrg,
    #[serde(rename = "missing_charset")]
    MissingCharset,
    #[serde(rename = "superfluous_charset")]
    SuperfluousCharset,
}

