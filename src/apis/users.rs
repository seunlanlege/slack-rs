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

/// struct for passing parameters to the method `conversations`
#[derive(Clone, Debug)]
pub struct ConversationsParams {
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<String>,
    /// Authentication token. Requires scope: `conversations:read`
    pub token: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000.
    pub limit: Option<i32>,
    /// Browse conversations by a specific user ID's membership. Non-public channels are restricted to those where the calling user shares membership.
    pub user: Option<String>,
    /// Set to `true` to exclude archived channels from the list
    pub exclude_archived: Option<bool>,
    /// Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im`
    pub types: Option<String>
}

/// struct for passing parameters to the method `delete_photo`
#[derive(Clone, Debug)]
pub struct DeletePhotoParams {
    /// Authentication token. Requires scope: `users.profile:write`
    pub token: String
}

/// struct for passing parameters to the method `get_presence`
#[derive(Clone, Debug)]
pub struct GetPresenceParams {
    /// Authentication token. Requires scope: `users:read`
    pub token: String,
    /// User to get presence info on. Defaults to the authed user.
    pub user: Option<String>
}

/// struct for passing parameters to the method `identity`
#[derive(Clone, Debug)]
pub struct IdentityParams {
    /// Authentication token. Requires scope: `identity.basic`
    pub token: Option<String>
}

/// struct for passing parameters to the method `info`
#[derive(Clone, Debug)]
pub struct InfoParams {
    /// Authentication token. Requires scope: `users:read`
    pub token: String,
    /// User to get info on
    pub user: Option<String>,
    /// Set this to `true` to receive the locale for this user. Defaults to `false`
    pub include_locale: Option<bool>
}

/// struct for passing parameters to the method `list`
#[derive(Clone, Debug)]
pub struct ListParams {
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail.
    pub cursor: Option<String>,
    /// Authentication token. Requires scope: `users:read`
    pub token: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    pub limit: Option<i32>,
    /// Set this to `true` to receive the locale for users. Defaults to `false`
    pub include_locale: Option<bool>
}

/// struct for passing parameters to the method `lookup_by_email`
#[derive(Clone, Debug)]
pub struct LookupByEmailParams {
    /// Authentication token. Requires scope: `users:read.email`
    pub token: Option<String>,
    /// An email address belonging to a user in the workspace
    pub email: Option<String>
}

/// struct for passing parameters to the method `profile_get`
#[derive(Clone, Debug)]
pub struct ProfileGetParams {
    /// Authentication token. Requires scope: `users.profile:read`
    pub token: Option<String>,
    /// Include labels for each ID in custom profile fields
    pub include_labels: Option<bool>,
    /// User to retrieve profile info for
    pub user: Option<String>
}

/// struct for passing parameters to the method `profile_set`
#[derive(Clone, Debug)]
pub struct ProfileSetParams {
    /// Collection of key:value pairs presented as a URL-encoded JSON hash. At most 50 fields may be set. Each field name is limited to 255 characters.
    pub profile: Option<String>,
    /// Authentication token. Requires scope: `users.profile:write`
    pub token: Option<String>,
    /// ID of user to change. This argument may only be specified by team admins on paid teams.
    pub user: Option<String>,
    /// Value to set a single key to. Usable only if `profile` is not passed.
    pub value: Option<String>,
    /// Name of a single key to set. Usable only if `profile` is not passed.
    pub name: Option<String>
}

/// struct for passing parameters to the method `set_active`
#[derive(Clone, Debug)]
pub struct SetActiveParams {
    /// Authentication token. Requires scope: `users:write`
    pub token: String
}

/// struct for passing parameters to the method `set_photo`
#[derive(Clone, Debug)]
pub struct SetPhotoParams {
    /// File contents via `multipart/form-data`.
    pub image: Option<String>,
    /// Width/height of crop box (always square)
    pub crop_w: Option<i32>,
    /// Authentication token. Requires scope: `users.profile:write`
    pub token: Option<String>,
    /// Y coordinate of top-left corner of crop box
    pub crop_y: Option<i32>,
    /// X coordinate of top-left corner of crop box
    pub crop_x: Option<i32>
}

/// struct for passing parameters to the method `set_presence`
#[derive(Clone, Debug)]
pub struct SetPresenceParams {
    /// Authentication token. Requires scope: `users:write`
    pub token: String,
    /// Either `auto` or `away`
    pub presence: String
}


/// struct for typed errors of method `conversations`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConversationsError {
    DefaultResponse(crate::models::UsersConversationsErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_photo`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePhotoError {
    DefaultResponse(crate::models::UsersDeletePhotoErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_presence`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPresenceError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `identity`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdentityError {
    DefaultResponse(crate::models::UsersIdentityErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `info`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfoError {
    DefaultResponse(crate::models::UsersInfoErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListError {
    DefaultResponse(crate::models::UsersListErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `lookup_by_email`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LookupByEmailError {
    DefaultResponse(crate::models::UsersLookupByEmailErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `profile_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileGetError {
    DefaultResponse(crate::models::UsersProfileGetErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `profile_set`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileSetError {
    DefaultResponse(crate::models::UsersProfileSetErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_active`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetActiveError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_photo`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetPhotoError {
    DefaultResponse(crate::models::UsersSetPhotoErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_presence`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetPresenceError {
    DefaultResponse(crate::models::UsersSetPresenceErrorSchema),
    UnknownValue(serde_json::Value),
}


/// List conversations the calling user may access.
pub async fn conversations(configuration: &configuration::Configuration, params: ConversationsParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ConversationsError>> {
    // unbox the parameters
    let cursor = params.cursor;
    let token = params.token;
    let limit = params.limit;
    let user = params.user;
    let exclude_archived = params.exclude_archived;
    let types = params.types;


    let client = &configuration.client;

    let uri_str = format!("{}/users.conversations", configuration.base_path);
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
    if let Some(ref s) = user {
        req_builder = req_builder.query(&[("user", &s.to_string())]);
    }
    if let Some(ref s) = exclude_archived {
        req_builder = req_builder.query(&[("exclude_archived", &s.to_string())]);
    }
    if let Some(ref s) = types {
        req_builder = req_builder.query(&[("types", &s.to_string())]);
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
        let entity: Option<ConversationsError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Delete the user profile photo
pub async fn delete_photo(configuration: &configuration::Configuration, params: DeletePhotoParams) -> Result<crate::models::UsersDeletePhotoSchema, Error<DeletePhotoError>> {
    // unbox the parameters
    let token = params.token;


    let client = &configuration.client;

    let uri_str = format!("{}/users.deletePhoto", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    form_params.insert("token", token.to_string());
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::UsersDeletePhotoSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<DeletePhotoError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Gets user presence information.
pub async fn get_presence(configuration: &configuration::Configuration, params: GetPresenceParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<GetPresenceError>> {
    // unbox the parameters
    let token = params.token;
    let user = params.user;


    let client = &configuration.client;

    let uri_str = format!("{}/users.getPresence", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    req_builder = req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref s) = user {
        req_builder = req_builder.query(&[("user", &s.to_string())]);
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
        let entity: Option<GetPresenceError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get a user's identity.
pub async fn identity(configuration: &configuration::Configuration, params: IdentityParams) -> Result<serde_json::Value, Error<IdentityError>> {
    // unbox the parameters
    let token = params.token;


    let client = &configuration.client;

    let uri_str = format!("{}/users.identity", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = token {
        req_builder = req_builder.query(&[("token", &s.to_string())]);
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

    let data: Option<serde_json::Value> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<IdentityError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Gets information about a user.
pub async fn info(configuration: &configuration::Configuration, params: InfoParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<InfoError>> {
    // unbox the parameters
    let token = params.token;
    let user = params.user;
    let include_locale = params.include_locale;


    let client = &configuration.client;

    let uri_str = format!("{}/users.info", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    req_builder = req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref s) = user {
        req_builder = req_builder.query(&[("user", &s.to_string())]);
    }
    if let Some(ref s) = include_locale {
        req_builder = req_builder.query(&[("include_locale", &s.to_string())]);
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
        let entity: Option<InfoError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Lists all users in a Slack team.
pub async fn list(configuration: &configuration::Configuration, params: ListParams) -> Result<crate::models::UsersListSchema, Error<ListError>> {
    // unbox the parameters
    let cursor = params.cursor;
    let token = params.token;
    let limit = params.limit;
    let include_locale = params.include_locale;


    let client = &configuration.client;

    let uri_str = format!("{}/users.list", configuration.base_path);
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
    if let Some(ref s) = include_locale {
        req_builder = req_builder.query(&[("include_locale", &s.to_string())]);
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

    let data: Option<crate::models::UsersListSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<ListError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Find a user with an email address.
pub async fn lookup_by_email(configuration: &configuration::Configuration, params: LookupByEmailParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<LookupByEmailError>> {
    // unbox the parameters
    let token = params.token;
    let email = params.email;


    let client = &configuration.client;

    let uri_str = format!("{}/users.lookupByEmail", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = token {
        req_builder = req_builder.query(&[("token", &s.to_string())]);
    }
    if let Some(ref s) = email {
        req_builder = req_builder.query(&[("email", &s.to_string())]);
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
        let entity: Option<LookupByEmailError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Retrieves a user's profile information.
pub async fn profile_get(configuration: &configuration::Configuration, params: ProfileGetParams) -> Result<crate::models::UsersProfileGetSchema, Error<ProfileGetError>> {
    // unbox the parameters
    let token = params.token;
    let include_labels = params.include_labels;
    let user = params.user;


    let client = &configuration.client;

    let uri_str = format!("{}/users.profile.get", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = token {
        req_builder = req_builder.query(&[("token", &s.to_string())]);
    }
    if let Some(ref s) = include_labels {
        req_builder = req_builder.query(&[("include_labels", &s.to_string())]);
    }
    if let Some(ref s) = user {
        req_builder = req_builder.query(&[("user", &s.to_string())]);
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

    let data: Option<crate::models::UsersProfileGetSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<ProfileGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Set the profile information for a user.
pub async fn profile_set(configuration: &configuration::Configuration, params: ProfileSetParams) -> Result<crate::models::UsersProfileSetSchema, Error<ProfileSetError>> {
    // unbox the parameters
    let profile = params.profile;
    let token = params.token;
    let user = params.user;
    let value = params.value;
    let name = params.name;


    let client = &configuration.client;

    let uri_str = format!("{}/users.profile.set", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = profile {
        form_params.insert("profile", param_value.to_string());
    }
    if let Some(param_value) = token {
        form_params.insert("token", param_value.to_string());
    }
    if let Some(param_value) = user {
        form_params.insert("user", param_value.to_string());
    }
    if let Some(param_value) = value {
        form_params.insert("value", param_value.to_string());
    }
    if let Some(param_value) = name {
        form_params.insert("name", param_value.to_string());
    }
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::UsersProfileSetSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<ProfileSetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Marked a user as active. Deprecated and non-functional.
pub async fn set_active(configuration: &configuration::Configuration, params: SetActiveParams) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<SetActiveError>> {
    // unbox the parameters
    let token = params.token;


    let client = &configuration.client;

    let uri_str = format!("{}/users.setActive", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    form_params.insert("token", token.to_string());
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<::std::collections::HashMap<String, serde_json::Value>> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<SetActiveError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Set the user profile photo
pub async fn set_photo(configuration: &configuration::Configuration, params: SetPhotoParams) -> Result<crate::models::UsersSetPhotoSchema, Error<SetPhotoError>> {
    // unbox the parameters
    let image = params.image;
    let crop_w = params.crop_w;
    let token = params.token;
    let crop_y = params.crop_y;
    let crop_x = params.crop_x;


    let client = &configuration.client;

    let uri_str = format!("{}/users.setPhoto", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = image {
        form_params.insert("image", param_value.to_string());
    }
    if let Some(param_value) = crop_w {
        form_params.insert("crop_w", param_value.to_string());
    }
    if let Some(param_value) = token {
        form_params.insert("token", param_value.to_string());
    }
    if let Some(param_value) = crop_y {
        form_params.insert("crop_y", param_value.to_string());
    }
    if let Some(param_value) = crop_x {
        form_params.insert("crop_x", param_value.to_string());
    }
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::UsersSetPhotoSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<SetPhotoError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Manually sets user presence.
pub async fn set_presence(configuration: &configuration::Configuration, params: SetPresenceParams) -> Result<crate::models::UsersSetPresenceSchema, Error<SetPresenceError>> {
    // unbox the parameters
    let token = params.token;
    let presence = params.presence;


    let client = &configuration.client;

    let uri_str = format!("{}/users.setPresence", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    form_params.insert("token", token.to_string());
    form_params.insert("presence", presence.to_string());
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::UsersSetPresenceSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<SetPresenceError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

