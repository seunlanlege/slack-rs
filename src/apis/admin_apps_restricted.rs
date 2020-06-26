/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method `apps_restricted_list`
#[derive(Clone, Debug)]
pub struct AppsRestrictedListParams {
    /// Authentication token. Requires scope: `admin.apps:read`
    pub token: String,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    pub cursor: Option<String>,
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    pub limit: Option<i32>,
    pub team_id: Option<String>,
    pub enterprise_id: Option<String>
}


/// struct for typed errors of method `apps_restricted_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsRestrictedListError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// List restricted apps for an org or workspace.
pub async fn apps_restricted_list(configuration: &configuration::Configuration, params: AppsRestrictedListParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<AppsRestrictedListError>> {
    // unbox the parameters
    let token = params.token;
    let cursor = params.cursor;
    let limit = params.limit;
    let team_id = params.team_id;
    let enterprise_id = params.enterprise_id;


    let client = &configuration.client;

    let uri_str = format!("{}/admin.apps.restricted.list", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = cursor {
        req_builder = req_builder.query(&[("cursor", &s.to_string())]);
    }
    req_builder = req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref s) = limit {
        req_builder = req_builder.query(&[("limit", &s.to_string())]);
    }
    if let Some(ref s) = team_id {
        req_builder = req_builder.query(&[("team_id", &s.to_string())]);
    }
    if let Some(ref s) = enterprise_id {
        req_builder = req_builder.query(&[("enterprise_id", &s.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<::std::collections::HashMap<String, serde_json::Value>> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<AppsRestrictedListError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

