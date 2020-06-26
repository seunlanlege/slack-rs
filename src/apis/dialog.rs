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
    /// The dialog definition. This must be a JSON-encoded string.
    pub dialog: String
}


/// struct for typed errors of method `open`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenError {
    DefaultResponse(crate::models::DialogOpenErrorSchema),
    UnknownValue(serde_json::Value),
}


/// Open a dialog with a user
pub async fn open(configuration: &configuration::Configuration, params: OpenParams) -> Result<crate::models::DialogOpenSchema, Error<OpenError>> {
    // unbox the parameters
    let token = params.token;
    let trigger_id = params.trigger_id;
    let dialog = params.dialog;


    let client = &configuration.client;

    let uri_str = format!("{}/dialog.open", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    req_builder = req_builder.query(&[("token", &token.to_string())]);
    req_builder = req_builder.query(&[("trigger_id", &trigger_id.to_string())]);
    req_builder = req_builder.query(&[("dialog", &dialog.to_string())]);
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

    let data: Option<crate::models::DialogOpenSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<OpenError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

