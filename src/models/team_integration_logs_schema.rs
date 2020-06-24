/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TeamIntegrationLogsSchema : Schema for successful response from team.integrationLogs method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamIntegrationLogsSchema {
    #[serde(rename = "logs")]
    pub logs: Vec<crate::models::TeamIntegrationLogsSchemaLogs>,
    #[serde(rename = "ok")]
    pub ok: crate::models::DefsOkTrue,
    #[serde(rename = "paging")]
    pub paging: crate::models::ObjsPaging,
}

impl TeamIntegrationLogsSchema {
    /// Schema for successful response from team.integrationLogs method
    pub fn new(logs: Vec<crate::models::TeamIntegrationLogsSchemaLogs>, ok: crate::models::DefsOkTrue, paging: crate::models::ObjsPaging) -> TeamIntegrationLogsSchema {
        TeamIntegrationLogsSchema {
            logs,
            ok,
            paging,
        }
    }
}


