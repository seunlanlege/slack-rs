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

/// struct for passing parameters to the method `info`
#[derive(Clone, Debug)]
pub struct InfoParams {
    /// Authentication token. Requires scope: `users:read`
    pub token: String,
    /// Bot user to get info on
    pub bot: Option<String>
}


/// struct for typed errors of method `info`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfoError {
    DefaultResponse(crate::models::BotsInfoErrorSchema),
    UnknownValue(serde_json::Value),
}


/// Gets information about a bot user.
pub async fn info(configuration: &configuration::Configuration, params: InfoParams) -> Result<crate::models::BotsInfoSchema, Error<InfoError>> {
    // unbox the parameters
    let token = params.token;
    let bot = params.bot;


    let client = &configuration.client;

    let uri_str = format!("{}/bots.info", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    req_builder = req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref s) = bot {
        req_builder = req_builder.query(&[("bot", &s.to_string())]);
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

    let data: Option<crate::models::BotsInfoSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<InfoError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

