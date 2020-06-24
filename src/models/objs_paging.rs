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
pub struct ObjsPaging {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "pages", skip_serializing_if = "Option::is_none")]
    pub pages: Option<i32>,
    #[serde(rename = "per_page", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(rename = "spill", skip_serializing_if = "Option::is_none")]
    pub spill: Option<i32>,
    #[serde(rename = "total")]
    pub total: i32,
}

impl ObjsPaging {
    pub fn new(page: i32, total: i32) -> ObjsPaging {
        ObjsPaging {
            count: None,
            page,
            pages: None,
            per_page: None,
            spill: None,
            total,
        }
    }
}


