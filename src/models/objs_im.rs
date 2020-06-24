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
pub struct ObjsIm {
    #[serde(rename = "created")]
    pub created: i32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "is_app_home", skip_serializing_if = "Option::is_none")]
    pub is_app_home: Option<bool>,
    #[serde(rename = "is_archived", skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "is_ext_shared", skip_serializing_if = "Option::is_none")]
    pub is_ext_shared: Option<bool>,
    #[serde(rename = "is_frozen", skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
    #[serde(rename = "is_im")]
    pub is_im: bool,
    #[serde(rename = "is_org_shared")]
    pub is_org_shared: bool,
    #[serde(rename = "is_shared", skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    #[serde(rename = "is_user_deleted")]
    pub is_user_deleted: bool,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<f32>,
    #[serde(rename = "user")]
    pub user: String,
}

impl ObjsIm {
    pub fn new(created: i32, id: String, is_im: bool, is_org_shared: bool, is_user_deleted: bool, user: String) -> ObjsIm {
        ObjsIm {
            created,
            id,
            is_app_home: None,
            is_archived: None,
            is_ext_shared: None,
            is_frozen: None,
            is_im,
            is_org_shared,
            is_shared: None,
            is_user_deleted,
            priority: None,
            user,
        }
    }
}

