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

/// struct for passing parameters to the method `connect`
#[derive(Clone, Debug)]
pub struct ConnectParams {
    /// Authentication token. Requires scope: `rtm:stream`
    pub token: String,
    /// Only deliver presence events when requested by subscription. See [presence subscriptions](/docs/presence-and-status#subscriptions).
    pub presence_sub: Option<bool>,
    /// Batch presence deliveries via subscription. Enabling changes the shape of `presence_change` events. See [batch presence](/docs/presence-and-status#batching).
    pub batch_presence_aware: Option<bool>
}


/// struct for typed errors of method `connect`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConnectError {
    DefaultResponse(crate::models::RtmConnectErrorSchema),
    UnknownValue(serde_json::Value),
}


/// Starts a Real Time Messaging session.
pub async fn connect(configuration: &configuration::Configuration, params: ConnectParams) -> Result<crate::models::RtmConnectSchema, Error<ConnectError>> {
    // unbox the parameters
    let token = params.token;
    let presence_sub = params.presence_sub;
    let batch_presence_aware = params.batch_presence_aware;


    let client = &configuration.client;

    let uri_str = format!("{}/rtm.connect", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = presence_sub {
        req_builder = req_builder.query(&[("presence_sub", &s.to_string())]);
    }
    req_builder = req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref s) = batch_presence_aware {
        req_builder = req_builder.query(&[("batch_presence_aware", &s.to_string())]);
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

    let data: Option<crate::models::RtmConnectSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<ConnectError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

