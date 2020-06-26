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

/// struct for passing parameters to the method `test`
#[derive(Clone, Debug)]
pub struct TestParams {
    /// example property to return
    pub foo: Option<String>,
    /// Error response to return
    pub error: Option<String>
}


/// struct for typed errors of method `test`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Checks API calling code.
pub async fn test(configuration: &configuration::Configuration, params: TestParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<TestError>> {
    // unbox the parameters
    let foo = params.foo;
    let error = params.error;


    let client = &configuration.client;

    let uri_str = format!("{}/api.test", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = foo {
        req_builder = req_builder.query(&[("foo", &s.to_string())]);
    }
    if let Some(ref s) = error {
        req_builder = req_builder.query(&[("error", &s.to_string())]);
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
        let entity: Option<TestError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}
