/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConversationsCreateErrorSchema : Schema for error response from conversations.create method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversationsCreateErrorSchema {
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "error")]
    pub error: Error,
    #[serde(rename = "needed", skip_serializing_if = "Option::is_none")]
    pub needed: Option<String>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkFalse,
    #[serde(rename = "provided", skip_serializing_if = "Option::is_none")]
    pub provided: Option<String>,
}

impl ConversationsCreateErrorSchema {
    /// Schema for error response from conversations.create method
    pub fn new(error: Error, ok: crate::models::DefsOkFalse) -> ConversationsCreateErrorSchema {
        ConversationsCreateErrorSchema {
            detail: None,
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
    #[serde(rename = "method_not_supported_for_channel_type")]
    MethodNotSupportedForChannelType,
    #[serde(rename = "missing_scope")]
    MissingScope,
    #[serde(rename = "name_taken")]
    NameTaken,
    #[serde(rename = "restricted_action")]
    RestrictedAction,
    #[serde(rename = "no_channel")]
    NoChannel,
    #[serde(rename = "invalid_name_required")]
    InvalidNameRequired,
    #[serde(rename = "invalid_name_punctuation")]
    InvalidNamePunctuation,
    #[serde(rename = "invalid_name_maxlength")]
    InvalidNameMaxlength,
    #[serde(rename = "invalid_name_specials")]
    InvalidNameSpecials,
    #[serde(rename = "invalid_name")]
    InvalidName,
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
    #[serde(rename = "invalid_json")]
    InvalidJson,
    #[serde(rename = "json_not_object")]
    JsonNotObject,
    #[serde(rename = "request_timeout")]
    RequestTimeout,
    #[serde(rename = "upgrade_required")]
    UpgradeRequired,
}

