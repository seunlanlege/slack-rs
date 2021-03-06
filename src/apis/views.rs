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

/// struct for passing parameters to the method `open`
#[derive(Clone, Debug)]
pub struct OpenParams {
    /// Authentication token. Requires scope: `none`
    pub token: String,
    /// Exchange a trigger to post to the user.
    pub trigger_id: String,
    /// A [view payload](/reference/surfaces/views). This must be a JSON-encoded string.
    pub view: String
}

/// struct for passing parameters to the method `publish`
#[derive(Clone, Debug)]
pub struct PublishParams {
    /// Authentication token. Requires scope: `none`
    pub token: String,
    /// `id` of the user you want publish a view to.
    pub user_id: String,
    /// A [view payload](/reference/surfaces/views). This must be a JSON-encoded string.
    pub view: String,
    /// A string that represents view state to protect against possible race conditions.
    pub hash: Option<String>
}

/// struct for passing parameters to the method `push`
#[derive(Clone, Debug)]
pub struct PushParams {
    /// Authentication token. Requires scope: `none`
    pub token: String,
    /// Exchange a trigger to post to the user.
    pub trigger_id: String,
    /// A [view payload](/reference/surfaces/views). This must be a JSON-encoded string.
    pub view: String
}

/// struct for passing parameters to the method `update`
#[derive(Clone, Debug)]
pub struct UpdateParams {
    /// Authentication token. Requires scope: `none`
    pub token: String,
    /// A string that represents view state to protect against possible race conditions.
    pub hash: Option<String>,
    /// A unique identifier of the view to be updated. Either `view_id` or `external_id` is required.
    pub view_id: Option<String>,
    /// A unique identifier of the view set by the developer. Must be unique for all views on a team. Max length of 255 characters. Either `view_id` or `external_id` is required.
    pub external_id: Option<String>,
    /// A [view payload](/reference/surfaces/views) This must be a JSON-encoded string.
    pub view: Option<String>
}


/// struct for typed errors of method `open`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `publish`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublishError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `push`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PushError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Open a view for a user.
pub async fn open(configuration: &configuration::Configuration, params: OpenParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<OpenError>> {
    // unbox the parameters
    let token = params.token;
    let trigger_id = params.trigger_id;
    let view = params.view;


    let client = &configuration.client;

    let uri_str = format!("{}/views.open", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    req_builder = req_builder.query(&[("token", &token.to_string())]);
    req_builder = req_builder.query(&[("trigger_id", &trigger_id.to_string())]);
    req_builder = req_builder.query(&[("view", &view.to_string())]);
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
        let entity: Option<OpenError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Publish a static view for a User.
pub async fn publish(configuration: &configuration::Configuration, params: PublishParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<PublishError>> {
    // unbox the parameters
    let token = params.token;
    let user_id = params.user_id;
    let view = params.view;
    let hash = params.hash;


    let client = &configuration.client;

    let uri_str = format!("{}/views.publish", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = hash {
        req_builder = req_builder.query(&[("hash", &s.to_string())]);
    }
    req_builder = req_builder.query(&[("token", &token.to_string())]);
    req_builder = req_builder.query(&[("user_id", &user_id.to_string())]);
    req_builder = req_builder.query(&[("view", &view.to_string())]);
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
        let entity: Option<PublishError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Push a view onto the stack of a root view.
pub async fn push(configuration: &configuration::Configuration, params: PushParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<PushError>> {
    // unbox the parameters
    let token = params.token;
    let trigger_id = params.trigger_id;
    let view = params.view;


    let client = &configuration.client;

    let uri_str = format!("{}/views.push", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    req_builder = req_builder.query(&[("token", &token.to_string())]);
    req_builder = req_builder.query(&[("trigger_id", &trigger_id.to_string())]);
    req_builder = req_builder.query(&[("view", &view.to_string())]);
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
        let entity: Option<PushError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Update an existing view.
pub async fn update(configuration: &configuration::Configuration, params: UpdateParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UpdateError>> {
    // unbox the parameters
    let token = params.token;
    let hash = params.hash;
    let view_id = params.view_id;
    let external_id = params.external_id;
    let view = params.view;


    let client = &configuration.client;

    let uri_str = format!("{}/views.update", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = hash {
        req_builder = req_builder.query(&[("hash", &s.to_string())]);
    }
    if let Some(ref s) = view_id {
        req_builder = req_builder.query(&[("view_id", &s.to_string())]);
    }
    req_builder = req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref s) = external_id {
        req_builder = req_builder.query(&[("external_id", &s.to_string())]);
    }
    if let Some(ref s) = view {
        req_builder = req_builder.query(&[("view", &s.to_string())]);
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
        let entity: Option<UpdateError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

