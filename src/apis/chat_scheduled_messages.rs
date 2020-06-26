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

/// struct for passing parameters to the method `scheduled_messages_list`
#[derive(Clone, Debug)]
pub struct ScheduledMessagesListParams {
    /// For pagination purposes, this is the `cursor` value returned from a previous call to `chat.scheduledmessages.list` indicating where you want to start this call from.
    pub cursor: Option<String>,
    /// Authentication token. Requires scope: `none`
    pub token: Option<String>,
    /// Maximum number of original entries to return.
    pub limit: Option<i32>,
    /// A UNIX timestamp of the oldest value in the time range
    pub oldest: Option<f32>,
    /// The channel of the scheduled messages
    pub channel: Option<String>,
    /// A UNIX timestamp of the latest value in the time range
    pub latest: Option<f32>
}


/// struct for typed errors of method `scheduled_messages_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScheduledMessagesListError {
    DefaultResponse(crate::models::ChatScheduledMessagesListErrorSchema),
    UnknownValue(serde_json::Value),
}


/// Returns a list of scheduled messages.
pub async fn scheduled_messages_list(configuration: &configuration::Configuration, params: ScheduledMessagesListParams) -> Result<crate::models::ChatScheduledMessagesListSchema, Error<ScheduledMessagesListError>> {
    // unbox the parameters
    let cursor = params.cursor;
    let token = params.token;
    let limit = params.limit;
    let oldest = params.oldest;
    let channel = params.channel;
    let latest = params.latest;


    let client = &configuration.client;

    let uri_str = format!("{}/chat.scheduledMessages.list", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = cursor {
        req_builder = req_builder.query(&[("cursor", &s.to_string())]);
    }
    if let Some(ref s) = token {
        req_builder = req_builder.query(&[("token", &s.to_string())]);
    }
    if let Some(ref s) = limit {
        req_builder = req_builder.query(&[("limit", &s.to_string())]);
    }
    if let Some(ref s) = oldest {
        req_builder = req_builder.query(&[("oldest", &s.to_string())]);
    }
    if let Some(ref s) = channel {
        req_builder = req_builder.query(&[("channel", &s.to_string())]);
    }
    if let Some(ref s) = latest {
        req_builder = req_builder.query(&[("latest", &s.to_string())]);
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

    let data: Option<crate::models::ChatScheduledMessagesListSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<ScheduledMessagesListError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

