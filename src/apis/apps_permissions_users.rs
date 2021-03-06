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

/// struct for passing parameters to the method `permissions_users_list`
#[derive(Clone, Debug)]
pub struct PermissionsUsersListParams {
    /// Authentication token. Requires scope: `none`
    pub token: String,
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<String>,
    /// The maximum number of items to return.
    pub limit: Option<i32>
}

/// struct for passing parameters to the method `permissions_users_request`
#[derive(Clone, Debug)]
pub struct PermissionsUsersRequestParams {
    /// A comma separated list of user scopes to request for
    pub scopes: String,
    /// Authentication token. Requires scope: `none`
    pub token: String,
    /// The user this scope is being requested for
    pub user: String,
    /// Token used to trigger the request
    pub trigger_id: String
}


/// struct for typed errors of method `permissions_users_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PermissionsUsersListError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `permissions_users_request`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PermissionsUsersRequestError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Returns list of user grants and corresponding scopes this app has on a team.
pub async fn permissions_users_list(configuration: &configuration::Configuration, params: PermissionsUsersListParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<PermissionsUsersListError>> {
    // unbox the parameters
    let token = params.token;
    let cursor = params.cursor;
    let limit = params.limit;


    let client = &configuration.client;

    let uri_str = format!("{}/apps.permissions.users.list", configuration.base_path);
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
        let entity: Option<PermissionsUsersListError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Enables an app to trigger a permissions modal to grant an app access to a user access scope.
pub async fn permissions_users_request(configuration: &configuration::Configuration, params: PermissionsUsersRequestParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<PermissionsUsersRequestError>> {
    // unbox the parameters
    let scopes = params.scopes;
    let token = params.token;
    let user = params.user;
    let trigger_id = params.trigger_id;


    let client = &configuration.client;

    let uri_str = format!("{}/apps.permissions.users.request", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    req_builder = req_builder.query(&[("scopes", &scopes.to_string())]);
    req_builder = req_builder.query(&[("token", &token.to_string())]);
    req_builder = req_builder.query(&[("user", &user.to_string())]);
    req_builder = req_builder.query(&[("trigger_id", &trigger_id.to_string())]);
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
        let entity: Option<PermissionsUsersRequestError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

