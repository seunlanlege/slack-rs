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

/// struct for passing parameters to the method `apps_approve`
#[derive(Clone, Debug)]
pub struct AppsApproveParams {
    /// Authentication token. Requires scope: `admin.apps:write`
    pub token: String,
    pub team_id: Option<String>,
    /// The id of the app to approve.
    pub app_id: Option<String>,
    /// The id of the request to approve.
    pub request_id: Option<String>
}

/// struct for passing parameters to the method `apps_restrict`
#[derive(Clone, Debug)]
pub struct AppsRestrictParams {
    /// Authentication token. Requires scope: `admin.apps:write`
    pub token: String,
    pub team_id: Option<String>,
    /// The id of the app to restrict.
    pub app_id: Option<String>,
    /// The id of the request to restrict.
    pub request_id: Option<String>
}


/// struct for typed errors of method `apps_approve`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsApproveError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `apps_restrict`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsRestrictError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Approve an app for installation on a workspace.
pub async fn apps_approve(configuration: &configuration::Configuration, params: AppsApproveParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<AppsApproveError>> {
    // unbox the parameters
    let token = params.token;
    let team_id = params.team_id;
    let app_id = params.app_id;
    let request_id = params.request_id;


    let client = &configuration.client;

    let uri_str = format!("{}/admin.apps.approve", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = team_id {
        form_params.insert("team_id", param_value.to_string());
    }
    form_params.insert("token", token.to_string());
    if let Some(param_value) = app_id {
        form_params.insert("app_id", param_value.to_string());
    }
    if let Some(param_value) = request_id {
        form_params.insert("request_id", param_value.to_string());
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
        let entity: Option<AppsApproveError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Restrict an app for installation on a workspace.
pub async fn apps_restrict(configuration: &configuration::Configuration, params: AppsRestrictParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<AppsRestrictError>> {
    // unbox the parameters
    let token = params.token;
    let team_id = params.team_id;
    let app_id = params.app_id;
    let request_id = params.request_id;


    let client = &configuration.client;

    let uri_str = format!("{}/admin.apps.restrict", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = team_id {
        form_params.insert("team_id", param_value.to_string());
    }
    form_params.insert("token", token.to_string());
    if let Some(param_value) = app_id {
        form_params.insert("app_id", param_value.to_string());
    }
    if let Some(param_value) = request_id {
        form_params.insert("request_id", param_value.to_string());
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
        let entity: Option<AppsRestrictError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

