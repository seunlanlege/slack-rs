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

/// struct for passing parameters to the method `invite_requests_approve`
#[derive(Clone, Debug)]
pub struct InviteRequestsApproveParams {
    /// Authentication token. Requires scope: `admin.invites:write`
    pub token: String,
    /// ID of the request to invite.
    pub invite_request_id: String,
    /// ID for the workspace where the invite request was made.
    pub team_id: Option<String>
}

/// struct for passing parameters to the method `invite_requests_deny`
#[derive(Clone, Debug)]
pub struct InviteRequestsDenyParams {
    /// Authentication token. Requires scope: `admin.invites:write`
    pub token: String,
    /// ID of the request to invite.
    pub invite_request_id: String,
    /// ID for the workspace where the invite request was made.
    pub team_id: Option<String>
}

/// struct for passing parameters to the method `invite_requests_list`
#[derive(Clone, Debug)]
pub struct InviteRequestsListParams {
    /// Authentication token. Requires scope: `admin.invites:read`
    pub token: String,
    /// Value of the `next_cursor` field sent as part of the previous API response
    pub cursor: Option<String>,
    /// The number of results that will be returned by the API on each invocation. Must be between 1 - 1000, both inclusive
    pub limit: Option<i32>,
    /// ID for the workspace where the invite requests were made.
    pub team_id: Option<String>
}


/// struct for typed errors of method `invite_requests_approve`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteRequestsApproveError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `invite_requests_deny`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteRequestsDenyError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `invite_requests_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteRequestsListError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Approve a workspace invite request.
pub async fn invite_requests_approve(configuration: &configuration::Configuration, params: InviteRequestsApproveParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<InviteRequestsApproveError>> {
    // unbox the parameters
    let token = params.token;
    let invite_request_id = params.invite_request_id;
    let team_id = params.team_id;


    let client = &configuration.client;

    let uri_str = format!("{}/admin.inviteRequests.approve", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    form_params.insert("token", token.to_string());
    if let Some(param_value) = team_id {
        form_params.insert("team_id", param_value.to_string());
    }
    form_params.insert("invite_request_id", invite_request_id.to_string());
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<::std::collections::HashMap<String, serde_json::Value>> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<InviteRequestsApproveError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Deny a workspace invite request.
pub async fn invite_requests_deny(configuration: &configuration::Configuration, params: InviteRequestsDenyParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<InviteRequestsDenyError>> {
    // unbox the parameters
    let token = params.token;
    let invite_request_id = params.invite_request_id;
    let team_id = params.team_id;


    let client = &configuration.client;

    let uri_str = format!("{}/admin.inviteRequests.deny", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    form_params.insert("token", token.to_string());
    if let Some(param_value) = team_id {
        form_params.insert("team_id", param_value.to_string());
    }
    form_params.insert("invite_request_id", invite_request_id.to_string());
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<::std::collections::HashMap<String, serde_json::Value>> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<InviteRequestsDenyError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// List all pending workspace invite requests.
pub async fn invite_requests_list(configuration: &configuration::Configuration, params: InviteRequestsListParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<InviteRequestsListError>> {
    // unbox the parameters
    let token = params.token;
    let cursor = params.cursor;
    let limit = params.limit;
    let team_id = params.team_id;


    let client = &configuration.client;

    let uri_str = format!("{}/admin.inviteRequests.list", configuration.base_path);
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
        let entity: Option<InviteRequestsListError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

