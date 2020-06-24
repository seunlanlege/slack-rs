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

/// struct for passing parameters to the method `users_assign`
#[derive(Clone, Debug)]
pub struct UsersAssignParams {
    /// Authentication token. Requires scope: `admin.users:write`
    pub token: String,
    /// The ID of the user to add to the workspace.
    pub user_id: String,
    /// The ID (`T1234`) of the workspace.
    pub team_id: String,
    /// Comma separated values of channel IDs to add user in the new workspace.
    pub channel_ids: Option<String>,
    /// True if user should be added to the workspace as a single-channel guest.
    pub is_ultra_restricted: Option<bool>,
    /// True if user should be added to the workspace as a guest.
    pub is_restricted: Option<bool>
}

/// struct for passing parameters to the method `users_invite`
#[derive(Clone, Debug)]
pub struct UsersInviteParams {
    /// Authentication token. Requires scope: `admin.users:write`
    pub token: String,
    /// A comma-separated list of `channel_id`s for this user to join. At least one channel is required.
    pub channel_ids: String,
    /// The ID (`T1234`) of the workspace.
    pub team_id: String,
    /// The email address of the person to invite.
    pub email: String,
    /// Full name of the user.
    pub real_name: Option<String>,
    /// Is this user a single channel guest user? (default: false)
    pub is_ultra_restricted: Option<bool>,
    /// An optional message to send to the user in the invite email.
    pub custom_message: Option<String>,
    /// Is this user a multi-channel guest user? (default: false)
    pub is_restricted: Option<bool>,
    /// Timestamp when guest account should be disabled. Only include this timestamp if you are inviting a guest user and you want their account to expire on a certain date.
    pub guest_expiration_ts: Option<String>,
    /// Allow this invite to be resent in the future if a user has not signed up yet. (default: false)
    pub resend: Option<bool>
}

/// struct for passing parameters to the method `users_list`
#[derive(Clone, Debug)]
pub struct UsersListParams {
    /// Authentication token. Requires scope: `admin.users:read`
    pub token: String,
    /// The ID (`T1234`) of the workspace.
    pub team_id: String,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    pub cursor: Option<String>,
    /// Limit for how many users to be retrieved per page
    pub limit: Option<i32>
}

/// struct for passing parameters to the method `users_remove`
#[derive(Clone, Debug)]
pub struct UsersRemoveParams {
    /// Authentication token. Requires scope: `admin.users:write`
    pub token: String,
    /// The ID of the user to remove.
    pub user_id: String,
    /// The ID (`T1234`) of the workspace.
    pub team_id: String
}

/// struct for passing parameters to the method `users_set_admin`
#[derive(Clone, Debug)]
pub struct UsersSetAdminParams {
    /// Authentication token. Requires scope: `admin.users:write`
    pub token: String,
    /// The ID of the user to designate as an admin.
    pub user_id: String,
    /// The ID (`T1234`) of the workspace.
    pub team_id: String
}

/// struct for passing parameters to the method `users_set_expiration`
#[derive(Clone, Debug)]
pub struct UsersSetExpirationParams {
    /// Authentication token. Requires scope: `admin.users:write`
    pub token: String,
    /// Timestamp when guest account should be disabled.
    pub expiration_ts: i32,
    /// The ID of the user to set an expiration for.
    pub user_id: String,
    /// The ID (`T1234`) of the workspace.
    pub team_id: String
}

/// struct for passing parameters to the method `users_set_owner`
#[derive(Clone, Debug)]
pub struct UsersSetOwnerParams {
    /// Authentication token. Requires scope: `admin.users:write`
    pub token: String,
    /// Id of the user to promote to owner.
    pub user_id: String,
    /// The ID (`T1234`) of the workspace.
    pub team_id: String
}

/// struct for passing parameters to the method `users_set_regular`
#[derive(Clone, Debug)]
pub struct UsersSetRegularParams {
    /// Authentication token. Requires scope: `admin.users:write`
    pub token: String,
    /// The ID of the user to designate as a regular user.
    pub user_id: String,
    /// The ID (`T1234`) of the workspace.
    pub team_id: String
}


/// struct for typed errors of method `users_assign`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersAssignError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_invite`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersInviteError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersListError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_remove`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersRemoveError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_set_admin`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersSetAdminError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_set_expiration`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersSetExpirationError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_set_owner`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersSetOwnerError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_set_regular`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersSetRegularError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


    pub async fn users_assign(configuration: &configuration::Configuration, params: UsersAssignParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersAssignError>> {
        // unbox the parameters
        let token = params.token;
        let user_id = params.user_id;
        let team_id = params.team_id;
        let channel_ids = params.channel_ids;
        let is_ultra_restricted = params.is_ultra_restricted;
        let is_restricted = params.is_restricted;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.users.assign", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("token", token.to_string());
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("user_id", user_id.to_string());
        if let Some(param_value) = channel_ids {
            form_params.insert("channel_ids", param_value.to_string());
        }
        form_params.insert("team_id", team_id.to_string());
        if let Some(param_value) = is_ultra_restricted {
            form_params.insert("is_ultra_restricted", param_value.to_string());
        }
        if let Some(param_value) = is_restricted {
            form_params.insert("is_restricted", param_value.to_string());
        }
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UsersAssignError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_invite(configuration: &configuration::Configuration, params: UsersInviteParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersInviteError>> {
        // unbox the parameters
        let token = params.token;
        let channel_ids = params.channel_ids;
        let team_id = params.team_id;
        let email = params.email;
        let real_name = params.real_name;
        let is_ultra_restricted = params.is_ultra_restricted;
        let custom_message = params.custom_message;
        let is_restricted = params.is_restricted;
        let guest_expiration_ts = params.guest_expiration_ts;
        let resend = params.resend;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.users.invite", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("token", token.to_string());
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("channel_ids", channel_ids.to_string());
        form_params.insert("team_id", team_id.to_string());
        if let Some(param_value) = real_name {
            form_params.insert("real_name", param_value.to_string());
        }
        if let Some(param_value) = is_ultra_restricted {
            form_params.insert("is_ultra_restricted", param_value.to_string());
        }
        if let Some(param_value) = custom_message {
            form_params.insert("custom_message", param_value.to_string());
        }
        if let Some(param_value) = is_restricted {
            form_params.insert("is_restricted", param_value.to_string());
        }
        if let Some(param_value) = guest_expiration_ts {
            form_params.insert("guest_expiration_ts", param_value.to_string());
        }
        form_params.insert("email", email.to_string());
        if let Some(param_value) = resend {
            form_params.insert("resend", param_value.to_string());
        }
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UsersInviteError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_list(configuration: &configuration::Configuration, params: UsersListParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersListError>> {
        // unbox the parameters
        let token = params.token;
        let team_id = params.team_id;
        let cursor = params.cursor;
        let limit = params.limit;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.users.list", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = cursor {
            req_builder = req_builder.query(&[("cursor", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("limit", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("team_id", &team_id.to_string())]);
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
            let entity: Option<UsersListError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_remove(configuration: &configuration::Configuration, params: UsersRemoveParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersRemoveError>> {
        // unbox the parameters
        let token = params.token;
        let user_id = params.user_id;
        let team_id = params.team_id;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.users.remove", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("token", token.to_string());
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("user_id", user_id.to_string());
        form_params.insert("team_id", team_id.to_string());
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UsersRemoveError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_set_admin(configuration: &configuration::Configuration, params: UsersSetAdminParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersSetAdminError>> {
        // unbox the parameters
        let token = params.token;
        let user_id = params.user_id;
        let team_id = params.team_id;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.users.setAdmin", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("token", token.to_string());
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("user_id", user_id.to_string());
        form_params.insert("team_id", team_id.to_string());
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UsersSetAdminError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_set_expiration(configuration: &configuration::Configuration, params: UsersSetExpirationParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersSetExpirationError>> {
        // unbox the parameters
        let token = params.token;
        let expiration_ts = params.expiration_ts;
        let user_id = params.user_id;
        let team_id = params.team_id;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.users.setExpiration", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("token", token.to_string());
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("expiration_ts", expiration_ts.to_string());
        form_params.insert("user_id", user_id.to_string());
        form_params.insert("team_id", team_id.to_string());
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UsersSetExpirationError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_set_owner(configuration: &configuration::Configuration, params: UsersSetOwnerParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersSetOwnerError>> {
        // unbox the parameters
        let token = params.token;
        let user_id = params.user_id;
        let team_id = params.team_id;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.users.setOwner", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("token", token.to_string());
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("user_id", user_id.to_string());
        form_params.insert("team_id", team_id.to_string());
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UsersSetOwnerError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn users_set_regular(configuration: &configuration::Configuration, params: UsersSetRegularParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersSetRegularError>> {
        // unbox the parameters
        let token = params.token;
        let user_id = params.user_id;
        let team_id = params.team_id;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.users.setRegular", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("token", token.to_string());
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("user_id", user_id.to_string());
        form_params.insert("team_id", team_id.to_string());
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UsersSetRegularError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }
