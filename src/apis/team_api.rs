/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;

use std::option::Option;

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method `access_logs`
#[derive(Clone, Debug)]
pub struct AccessLogsParams {
    /// Authentication token. Requires scope: `admin`
    pub token: String,
    pub count: Option<i32>,
    pub page: Option<i32>,
    /// End of time range of logs to include in results (inclusive).
    pub before: Option<String>
}

/// struct for passing parameters to the method `billable_info`
#[derive(Clone, Debug)]
pub struct BillableInfoParams {
    /// Authentication token. Requires scope: `admin`
    pub token: String,
    /// A user to retrieve the billable information for. Defaults to all users.
    pub user: Option<String>
}

/// struct for passing parameters to the method `info`
#[derive(Clone, Debug)]
pub struct InfoParams {
    /// Authentication token. Requires scope: `team:read`
    pub token: String,
    /// Team to get info on, if omitted, will return information about the current team. Will only return team that the authenticated token is allowed to see through external shared channels
    pub team: Option<String>
}

/// struct for passing parameters to the method `integration_logs`
#[derive(Clone, Debug)]
pub struct IntegrationLogsParams {
    /// Authentication token. Requires scope: `admin`
    pub token: String,
    pub count: Option<i32>,
    /// Filter logs with this change type. Defaults to all logs.
    pub change_type: Option<String>,
    /// Filter logs to this Slack app. Defaults to all logs.
    pub app_id: Option<i32>,
    /// Filter logs generated by this user’s actions. Defaults to all logs.
    pub user: Option<String>,
    /// Filter logs to this service. Defaults to all logs.
    pub service_id: Option<i32>,
    pub page: Option<String>
}

/// struct for passing parameters to the method `profile_get`
#[derive(Clone, Debug)]
pub struct ProfileGetParams {
    /// Authentication token. Requires scope: `users.profile:read`
    pub token: String,
    /// Filter by visibility.
    pub visibility: Option<String>
}


/// struct for typed errors of method `access_logs`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccessLogsError {
    DefaultResponse(crate::models::TeamAccessLogsErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `billable_info`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BillableInfoError {
    DefaultResponse(crate::models::TeamBillableInfoErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `info`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfoError {
    DefaultResponse(crate::models::TeamInfoErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `integration_logs`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntegrationLogsError {
    DefaultResponse(crate::models::TeamIntegrationLogsErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `profile_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileGetError {
    DefaultResponse(crate::models::TeamProfileGetErrorSchema),
    UnknownValue(serde_json::Value),
}


    pub async fn access_logs(configuration: &configuration::Configuration, params: AccessLogsParams) -> Result<crate::models::TeamAccessLogsSchema, Error<AccessLogsError>> {
        // unbox the parameters
        let token = params.token;
        let count = params.count;
        let page = params.page;
        let before = params.before;

        let client = &configuration.client;

        let uri_str = format!("{}/team.accessLogs", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = count {
            req_builder = req_builder.query(&[("count", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("token", &token.to_string())]);
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = before {
            req_builder = req_builder.query(&[("before", &s.to_string())]);
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

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<AccessLogsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn billable_info(configuration: &configuration::Configuration, params: BillableInfoParams) -> Result<crate::models::TeamBillableInfoSchema, Error<BillableInfoError>> {
        // unbox the parameters
        let token = params.token;
        let user = params.user;

        let client = &configuration.client;

        let uri_str = format!("{}/team.billableInfo", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("token", &token.to_string())]);
        if let Some(ref s) = user {
            req_builder = req_builder.query(&[("user", &s.to_string())]);
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

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<BillableInfoError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn info(configuration: &configuration::Configuration, params: InfoParams) -> Result<crate::models::TeamInfoSchema, Error<InfoError>> {
        // unbox the parameters
        let token = params.token;
        let team = params.team;

        let client = &configuration.client;

        let uri_str = format!("{}/team.info", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("token", &token.to_string())]);
        if let Some(ref s) = team {
            req_builder = req_builder.query(&[("team", &s.to_string())]);
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

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<InfoError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn integration_logs(configuration: &configuration::Configuration, params: IntegrationLogsParams) -> Result<crate::models::TeamIntegrationLogsSchema, Error<IntegrationLogsError>> {
        // unbox the parameters
        let token = params.token;
        let count = params.count;
        let change_type = params.change_type;
        let app_id = params.app_id;
        let user = params.user;
        let service_id = params.service_id;
        let page = params.page;

        let client = &configuration.client;

        let uri_str = format!("{}/team.integrationLogs", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = count {
            req_builder = req_builder.query(&[("count", &s.to_string())]);
        }
        if let Some(ref s) = change_type {
            req_builder = req_builder.query(&[("change_type", &s.to_string())]);
        }
        if let Some(ref s) = app_id {
            req_builder = req_builder.query(&[("app_id", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("token", &token.to_string())]);
        if let Some(ref s) = user {
            req_builder = req_builder.query(&[("user", &s.to_string())]);
        }
        if let Some(ref s) = service_id {
            req_builder = req_builder.query(&[("service_id", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
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

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<IntegrationLogsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn profile_get(configuration: &configuration::Configuration, params: ProfileGetParams) -> Result<crate::models::TeamProfileGetSuccessSchema, Error<ProfileGetError>> {
        // unbox the parameters
        let token = params.token;
        let visibility = params.visibility;

        let client = &configuration.client;

        let uri_str = format!("{}/team.profile.get", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("token", &token.to_string())]);
        if let Some(ref s) = visibility {
            req_builder = req_builder.query(&[("visibility", &s.to_string())]);
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

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ProfileGetError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

