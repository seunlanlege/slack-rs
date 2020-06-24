/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthTestSuccessSchema : Schema for successful response auth.test method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthTestSuccessSchema {
    #[serde(rename = "is_enterprise_install", skip_serializing_if = "Option::is_none")]
    pub is_enterprise_install: Option<bool>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
    #[serde(rename = "team")]
    pub team: String,
    #[serde(rename = "team_id")]
    pub team_id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "user")]
    pub user: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl AuthTestSuccessSchema {
    /// Schema for successful response auth.test method
    pub fn new(ok: crate::models::DefsOkTrue, team: String, team_id: String, url: String, user: String, user_id: String) -> AuthTestSuccessSchema {
        AuthTestSuccessSchema {
            is_enterprise_install: None,
            ok,
            team,
            team_id,
            url,
            user,
            user_id,
        }
    }
}

