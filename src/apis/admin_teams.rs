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

/// struct for passing parameters to the method `teams_create`
#[derive(Clone, Debug)]
pub struct TeamsCreateParams {
    /// Team domain (for example, slacksoftballteam).
    pub team_domain: String,
    /// Authentication token. Requires scope: `admin.teams:write`
    pub token: String,
    /// Team name (for example, Slack Softball Team).
    pub team_name: String,
    /// Description for the team.
    pub team_description: Option<String>,
    /// Who can join the team. A team's discoverability can be `open`, `closed`, `invite_only`, or `unlisted`.
    pub team_discoverability: Option<String>
}

/// struct for passing parameters to the method `teams_list`
#[derive(Clone, Debug)]
pub struct TeamsListParams {
    /// Authentication token. Requires scope: `admin.teams:read`
    pub token: String,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    pub cursor: Option<String>,
    /// The maximum number of items to return. Must be between 1 - 100 both inclusive.
    pub limit: Option<i32>
}


/// struct for typed errors of method `teams_create`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TeamsCreateError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `teams_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TeamsListError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Create an Enterprise team.
pub async fn teams_create(configuration: &configuration::Configuration, params: TeamsCreateParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<TeamsCreateError>> {
    // unbox the parameters
    let team_domain = params.team_domain;
    let token = params.token;
    let team_name = params.team_name;
    let team_description = params.team_description;
    let team_discoverability = params.team_discoverability;


    let client = &configuration.client;

    let uri_str = format!("{}/admin.teams.create", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    form_params.insert("team_domain", team_domain.to_string());
    form_params.insert("token", token.to_string());
    if let Some(param_value) = team_description {
        form_params.insert("team_description", param_value.to_string());
    }
    form_params.insert("team_name", team_name.to_string());
    if let Some(param_value) = team_discoverability {
        form_params.insert("team_discoverability", param_value.to_string());
    }
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<::std::collections::HashMap<String, serde_json::Value>> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<TeamsCreateError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// List all teams on an Enterprise organization
pub async fn teams_list(configuration: &configuration::Configuration, params: TeamsListParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<TeamsListError>> {
    // unbox the parameters
    let token = params.token;
    let cursor = params.cursor;
    let limit = params.limit;


    let client = &configuration.client;

    let uri_str = format!("{}/admin.teams.list", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = cursor {
        req_builder = req_builder.query(&[("cursor", &s.to_string())]);
    }
    req_builder = req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref s) = limit {
        req_builder = req_builder.query(&[("limit", &s.to_string())]);
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
        let entity: Option<TeamsListError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

