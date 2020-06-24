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

/// struct for passing parameters to the method `emoji_add`
#[derive(Clone, Debug)]
pub struct EmojiAddParams {
    /// The URL of a file to use as an image for the emoji. Square images under 128KB and with transparent backgrounds work best.
    pub url: String,
    /// Authentication token. Requires scope: `admin.teams:write`
    pub token: String,
    /// The name of the emoji to be removed. Colons (`:myemoji:`) around the value are not required, although they may be included.
    pub name: String
}

/// struct for passing parameters to the method `emoji_add_alias`
#[derive(Clone, Debug)]
pub struct EmojiAddAliasParams {
    /// Authentication token. Requires scope: `admin.teams:write`
    pub token: String,
    /// The name of the emoji to be aliased. Colons (`:myemoji:`) around the value are not required, although they may be included.
    pub name: String,
    /// The alias of the emoji.
    pub alias_for: String
}

/// struct for passing parameters to the method `emoji_list`
#[derive(Clone, Debug)]
pub struct EmojiListParams {
    /// Authentication token. Requires scope: `admin.teams:read`
    pub token: String,
    /// Set `cursor` to `next_cursor` returned by the previous call to list items in the next page
    pub cursor: Option<String>,
    /// The maximum number of items to return. Must be between 1 - 1000 both inclusive.
    pub limit: Option<i32>
}

/// struct for passing parameters to the method `emoji_remove`
#[derive(Clone, Debug)]
pub struct EmojiRemoveParams {
    /// Authentication token. Requires scope: `admin.teams:write`
    pub token: String,
    /// The name of the emoji to be removed. Colons (`:myemoji:`) around the value are not required, although they may be included.
    pub name: String
}

/// struct for passing parameters to the method `emoji_rename`
#[derive(Clone, Debug)]
pub struct EmojiRenameParams {
    /// The new name of the emoji.
    pub new_name: String,
    /// Authentication token. Requires scope: `admin.teams:write`
    pub token: String,
    /// The name of the emoji to be renamed. Colons (`:myemoji:`) around the value are not required, although they may be included.
    pub name: String
}


/// struct for typed errors of method `emoji_add`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiAddError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `emoji_add_alias`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiAddAliasError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `emoji_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiListError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `emoji_remove`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiRemoveError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `emoji_rename`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiRenameError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


    pub async fn emoji_add(configuration: &configuration::Configuration, params: EmojiAddParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<EmojiAddError>> {
        // unbox the parameters
        let url = params.url;
        let token = params.token;
        let name = params.name;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.emoji.add", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("url", url.to_string());
        form_params.insert("token", token.to_string());
        form_params.insert("name", name.to_string());
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<EmojiAddError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn emoji_add_alias(configuration: &configuration::Configuration, params: EmojiAddAliasParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<EmojiAddAliasError>> {
        // unbox the parameters
        let token = params.token;
        let name = params.name;
        let alias_for = params.alias_for;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.emoji.addAlias", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("token", token.to_string());
        form_params.insert("name", name.to_string());
        form_params.insert("alias_for", alias_for.to_string());
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<EmojiAddAliasError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn emoji_list(configuration: &configuration::Configuration, params: EmojiListParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<EmojiListError>> {
        // unbox the parameters
        let token = params.token;
        let cursor = params.cursor;
        let limit = params.limit;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.emoji.list", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = cursor {
            req_builder = req_builder.query(&[("cursor", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("token", &token.to_string())]);
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("limit", &s.to_string())]);
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
            let entity: Option<EmojiListError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn emoji_remove(configuration: &configuration::Configuration, params: EmojiRemoveParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<EmojiRemoveError>> {
        // unbox the parameters
        let token = params.token;
        let name = params.name;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.emoji.remove", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("token", token.to_string());
        form_params.insert("name", name.to_string());
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<EmojiRemoveError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn emoji_rename(configuration: &configuration::Configuration, params: EmojiRenameParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<EmojiRenameError>> {
        // unbox the parameters
        let new_name = params.new_name;
        let token = params.token;
        let name = params.name;

        let client = &configuration.client;

        let uri_str = format!("{}/admin.emoji.rename", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("new_name", new_name.to_string());
        form_params.insert("token", token.to_string());
        form_params.insert("name", name.to_string());
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<EmojiRenameError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

