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
pub struct ObjsSubteamPrefs {
    #[serde(rename = "channels")]
    pub channels: Vec<String>,
    #[serde(rename = "groups")]
    pub groups: Vec<String>,
}

impl ObjsSubteamPrefs {
    pub fn new(channels: Vec<String>, groups: Vec<String>) -> ObjsSubteamPrefs {
        ObjsSubteamPrefs {
            channels,
            groups,
        }
    }
}


