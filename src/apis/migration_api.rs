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

/// struct for passing parameters to the method `exchange`
#[derive(Clone, Debug)]
pub struct ExchangeParams {
    /// Authentication token. Requires scope: `tokens.basic`
    pub token: String,
    /// A comma-separated list of user ids, up to 400 per request
    pub users: String,
    /// Specify `true` to convert `W` global user IDs to workspace-specific `U` IDs. Defaults to `false`.
    pub to_old: Option<bool>
}


/// struct for typed errors of method `exchange`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExchangeError {
    DefaultResponse(crate::models::MigrationExchangeErrorSchema),
    UnknownValue(serde_json::Value),
}


    pub async fn exchange(configuration: &configuration::Configuration, params: ExchangeParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ExchangeError>> {
        // unbox the parameters
        let token = params.token;
        let users = params.users;
        let to_old = params.to_old;

        let client = &configuration.client;

        let uri_str = format!("{}/migration.exchange", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("token", &token.to_string())]);
        if let Some(ref s) = to_old {
            req_builder = req_builder.query(&[("to_old", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("users", &users.to_string())]);
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
            let entity: Option<ExchangeError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

