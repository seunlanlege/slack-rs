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

/// struct for passing parameters to the method `invite_requests_denied_list`
#[derive(Clone, Debug)]
pub struct InviteRequestsDeniedListParams {
    /// Authentication token. Requires scope: `admin.invites:read`
    pub token: String,
    /// Value of the `next_cursor` field sent as part of the previous api response
    pub cursor: Option<String>,
    /// The number of results that will be returned by the API on each invocation. Must be between 1 - 1000 both inclusive
    pub limit: Option<i32>,
    /// ID for the workspace where the invite requests were made.
    pub team_id: Option<String>
}


/// struct for typed errors of method `invite_requests_denied_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteRequestsDeniedListError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


    pub async fn invite_requests_denied_list(configuration: &configuration::Configuration, params: InviteRequestsDeniedListParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<InviteRequestsDeniedListError>> {
        // unbox the parameters
        let token = params.token;
        let cursor = params.cursor;
        let limit = params.limit;
        let team_id = params.team_id;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.inviteRequests.denied.list", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = cursor {
            req_builder = req_builder.query(&[("cursor", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("limit", &s.to_string())]);
        }
        if let Some(ref s) = team_id {
            req_builder = req_builder.query(&[("team_id", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("token", token.to_string());
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
            let entity: Option<InviteRequestsDeniedListError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

