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
pub struct AppsPermissionsInfoSchemaInfoTeam {
    #[serde(rename = "resources")]
    pub resources: crate::models::ObjsResources,
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
}

impl AppsPermissionsInfoSchemaInfoTeam {
    pub fn new(resources: crate::models::ObjsResources, scopes: Vec<String>) -> AppsPermissionsInfoSchemaInfoTeam {
        AppsPermissionsInfoSchemaInfoTeam {
            resources,
            scopes,
        }
    }
}


