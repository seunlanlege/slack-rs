/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjsConversationMpimShares {
    #[serde(rename = "accepted_user", skip_serializing_if = "Option::is_none")]
    pub accepted_user: Option<String>,
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(rename = "team")]
    pub team: crate::models::ObjsTeam,
    #[serde(rename = "user")]
    pub user: String,
}

impl ObjsConversationMpimShares {
    pub fn new(is_active: bool, team: crate::models::ObjsTeam, user: String) -> ObjsConversationMpimShares {
        ObjsConversationMpimShares {
            accepted_user: None,
            is_active,
            team,
            user,
        }
    }
}


