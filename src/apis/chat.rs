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

/// struct for passing parameters to the method `delete`
#[derive(Clone, Debug)]
pub struct DeleteParams {
    /// Pass true to delete the message as the authed user with `chat:write:user` scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope.
    pub as_user: Option<bool>,
    /// Authentication token. Requires scope: `chat:write`
    pub token: Option<String>,
    /// Timestamp of the message to be deleted.
    pub ts: Option<f32>,
    /// Channel containing the message to be deleted.
    pub channel: Option<String>
}

/// struct for passing parameters to the method `delete_scheduled_message`
#[derive(Clone, Debug)]
pub struct DeleteScheduledMessageParams {
    /// Authentication token. Requires scope: `chat:write`
    pub token: String,
    /// The channel the scheduled_message is posting to
    pub channel: String,
    /// `scheduled_message_id` returned from call to chat.scheduleMessage
    pub scheduled_message_id: String,
    /// Pass true to delete the message as the authed user with `chat:write:user` scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope.
    pub as_user: Option<bool>
}

/// struct for passing parameters to the method `get_permalink`
#[derive(Clone, Debug)]
pub struct GetPermalinkParams {
    /// Authentication token. Requires scope: `none`
    pub token: String,
    /// A message's `ts` value, uniquely identifying it within a channel
    pub message_ts: String,
    /// The ID of the conversation or channel containing the message
    pub channel: String
}

/// struct for passing parameters to the method `me_message`
#[derive(Clone, Debug)]
pub struct MeMessageParams {
    /// Text of the message to send.
    pub text: Option<String>,
    /// Authentication token. Requires scope: `chat:write:user`
    pub token: Option<String>,
    /// Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name.
    pub channel: Option<String>
}

/// struct for passing parameters to the method `post_ephemeral`
#[derive(Clone, Debug)]
pub struct PostEphemeralParams {
    /// Authentication token. Requires scope: `chat:write`
    pub token: String,
    /// `id` of the user who will receive the ephemeral message. The user should be in the channel specified by the `channel` argument.
    pub user: String,
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name.
    pub channel: String,
    /// Set your bot's user name. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub username: Option<String>,
    /// Provide another message's `ts` value to post this message in a thread. Avoid using a reply's `ts` value; use its parent's value instead. Ephemeral messages in threads are only shown if there is already an active thread.
    pub thread_ts: Option<String>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    pub blocks: Option<String>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    pub attachments: Option<String>,
    /// Pass true to post the message as the authed user. Defaults to true if the chat:write:bot scope is not included. Otherwise, defaults to false.
    pub as_user: Option<bool>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// Change how messages are treated. Defaults to `none`. See [below](#formatting).
    pub parse: Option<String>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    pub text: Option<String>,
    /// Emoji to use as the icon for this message. Overrides `icon_url`. Must be used in conjunction with `as_user` set to `false`, otherwise ignored. See [authorship](#authorship) below.
    pub icon_emoji: Option<String>,
    /// URL to an image to use as the icon for this message. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub icon_url: Option<String>
}

/// struct for passing parameters to the method `post_message`
#[derive(Clone, Debug, Default)]
pub struct PostMessageParams {
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details.
    pub channel: String,
    /// Authentication token. Requires scope: `chat:write`
    pub token: String,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    pub attachments: Option<String>,
    /// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    pub text: Option<String>,
    /// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
    /// Change how messages are treated. Defaults to `none`. See [below](#formatting).
    pub parse: Option<String>,
    /// Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [authorship](#authorship) below.
    pub as_user: Option<bool>,
    /// Disable Slack markup parsing by setting to `false`. Enabled by default.
    pub mrkdwn: Option<bool>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    pub blocks: Option<String>,
    /// Emoji to use as the icon for this message. Overrides `icon_url`. Must be used in conjunction with `as_user` set to `false`, otherwise ignored. See [authorship](#authorship) below.
    pub icon_emoji: Option<String>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`.
    pub reply_broadcast: Option<bool>,
    /// Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead.
    pub thread_ts: Option<String>,
    /// Set your bot's user name. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub username: Option<String>,
    /// URL to an image to use as the icon for this message. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below.
    pub icon_url: Option<String>
}

/// struct for passing parameters to the method `schedule_message`
#[derive(Clone, Debug)]
pub struct ScheduleMessageParams {
    /// Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead.
    pub thread_ts: Option<f32>,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    pub blocks: Option<String>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string.
    pub attachments: Option<String>,
    /// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
    /// How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail.
    pub text: Option<String>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
    /// Change how messages are treated. Defaults to `none`. See [chat.postMessage](chat.postMessage#formatting).
    pub parse: Option<String>,
    /// Authentication token. Requires scope: `chat:write`
    pub token: Option<String>,
    /// Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [chat.postMessage](chat.postMessage#authorship).
    pub as_user: Option<bool>,
    /// Unix EPOCH timestamp of time in future to send the message.
    pub post_at: Option<String>,
    /// Channel, private group, or DM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details.
    pub channel: Option<String>,
    /// Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`.
    pub reply_broadcast: Option<bool>
}

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

/// struct for passing parameters to the method `unfurl`
#[derive(Clone, Debug)]
pub struct UnfurlParams {
    /// Timestamp of the message to add unfurl behavior to.
    pub ts: String,
    /// Authentication token. Requires scope: `links:write`
    pub token: String,
    /// Channel ID of the message
    pub channel: String,
    /// Provide a simply-formatted string to send as an ephemeral message to the user as invitation to authenticate further and enable full unfurling behavior
    pub user_auth_message: Option<String>,
    /// Set to `true` or `1` to indicate the user must install your Slack app to trigger unfurls for this domain
    pub user_auth_required: Option<bool>,
    /// URL-encoded JSON map with keys set to URLs featured in the the message, pointing to their unfurl blocks or message attachments.
    pub unfurls: Option<String>,
    /// Send users to this custom URL where they will complete authentication in your app to fully trigger unfurling. Value should be properly URL-encoded.
    pub user_auth_url: Option<String>
}

/// struct for passing parameters to the method `update`
#[derive(Clone, Debug)]
pub struct UpdateParams {
    /// Timestamp of the message to be updated.
    pub ts: String,
    /// Authentication token. Requires scope: `chat:write`
    pub token: String,
    /// Channel containing the message to be updated.
    pub channel: String,
    /// A JSON-based array of structured blocks, presented as a URL-encoded string.
    pub blocks: Option<String>,
    /// A JSON-based array of structured attachments, presented as a URL-encoded string. This field is required when not presenting `text`.
    pub attachments: Option<String>,
    /// Pass true to update the message as the authed user. [Bot users](/bot-users) in this context are considered authed users.
    pub as_user: Option<String>,
    /// Change how messages are treated. Defaults to `client`, unlike `chat.postMessage`. Accepts either `none` or `full`. See [below](#formatting).
    pub parse: Option<String>,
    /// New text for the message, using the [default formatting rules](/docs/formatting). It's not required when presenting `attachments`.
    pub text: Option<String>,
    /// Find and link channel names and usernames. Defaults to `none`. See [below](#formatting).
    pub link_names: Option<String>
}


/// struct for typed errors of method `delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteError {
    DefaultResponse(crate::models::ChatDeleteErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_scheduled_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteScheduledMessageError {
    DefaultResponse(crate::models::ChatDeleteScheduledMessageErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_permalink`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPermalinkError {
    DefaultResponse(crate::models::ChatGetPermalinkErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `me_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MeMessageError {
    DefaultResponse(crate::models::ChatMeMessageErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `post_ephemeral`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostEphemeralError {
    DefaultResponse(crate::models::ChatPostEphemeralErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `post_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostMessageError {
    DefaultResponse(crate::models::ChatPostMessageErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `schedule_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScheduleMessageError {
    DefaultResponse(crate::models::ChatScheduleMessageErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `scheduled_messages_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScheduledMessagesListError {
    DefaultResponse(crate::models::ChatScheduledMessagesListErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `unfurl`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnfurlError {
    DefaultResponse(crate::models::ChatUnfurlErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateError {
    DefaultResponse(crate::models::ChatUpdateErrorSchema),
    UnknownValue(serde_json::Value),
}


/// Deletes a message.
pub async fn delete(configuration: &configuration::Configuration, params: DeleteParams) -> Result<crate::models::ChatDeleteSuccessSchema, Error<DeleteError>> {
    // unbox the parameters
    let as_user = params.as_user;
    let token = params.token;
    let ts = params.ts;
    let channel = params.channel;


    let client = &configuration.client;

    let uri_str = format!("{}/chat.delete", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = as_user {
        form_params.insert("as_user", param_value.to_string());
    }
    if let Some(param_value) = token {
        form_params.insert("token", param_value.to_string());
    }
    if let Some(param_value) = ts {
        form_params.insert("ts", param_value.to_string());
    }
    if let Some(param_value) = channel {
        form_params.insert("channel", param_value.to_string());
    }
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::ChatDeleteSuccessSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<DeleteError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Deletes a pending scheduled message from the queue.
pub async fn delete_scheduled_message(configuration: &configuration::Configuration, params: DeleteScheduledMessageParams) -> Result<crate::models::ChatDeleteScheduledMessageSchema, Error<DeleteScheduledMessageError>> {
    // unbox the parameters
    let token = params.token;
    let channel = params.channel;
    let scheduled_message_id = params.scheduled_message_id;
    let as_user = params.as_user;


    let client = &configuration.client;

    let uri_str = format!("{}/chat.deleteScheduledMessage", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = as_user {
        form_params.insert("as_user", param_value.to_string());
    }
    form_params.insert("token", token.to_string());
    form_params.insert("channel", channel.to_string());
    form_params.insert("scheduled_message_id", scheduled_message_id.to_string());
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::ChatDeleteScheduledMessageSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<DeleteScheduledMessageError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Retrieve a permalink URL for a specific extant message
pub async fn get_permalink(configuration: &configuration::Configuration, params: GetPermalinkParams) -> Result<crate::models::ChatGetPermalinkSuccessSchema, Error<GetPermalinkError>> {
    // unbox the parameters
    let token = params.token;
    let message_ts = params.message_ts;
    let channel = params.channel;


    let client = &configuration.client;

    let uri_str = format!("{}/chat.getPermalink", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    req_builder = req_builder.query(&[("token", &token.to_string())]);
    req_builder = req_builder.query(&[("message_ts", &message_ts.to_string())]);
    req_builder = req_builder.query(&[("channel", &channel.to_string())]);
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

    let data: Option<crate::models::ChatGetPermalinkSuccessSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<GetPermalinkError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Share a me message into a channel.
pub async fn me_message(configuration: &configuration::Configuration, params: MeMessageParams) -> Result<crate::models::ChatMeMessageSchema, Error<MeMessageError>> {
    // unbox the parameters
    let text = params.text;
    let token = params.token;
    let channel = params.channel;


    let client = &configuration.client;

    let uri_str = format!("{}/chat.meMessage", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = text {
        form_params.insert("text", param_value.to_string());
    }
    if let Some(param_value) = token {
        form_params.insert("token", param_value.to_string());
    }
    if let Some(param_value) = channel {
        form_params.insert("channel", param_value.to_string());
    }
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::ChatMeMessageSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<MeMessageError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Sends an ephemeral message to a user in a channel.
pub async fn post_ephemeral(configuration: &configuration::Configuration, params: PostEphemeralParams) -> Result<crate::models::ChatPostEphemeralSuccessSchema, Error<PostEphemeralError>> {
    // unbox the parameters
    let token = params.token;
    let user = params.user;
    let channel = params.channel;
    let username = params.username;
    let thread_ts = params.thread_ts;
    let blocks = params.blocks;
    let attachments = params.attachments;
    let as_user = params.as_user;
    let link_names = params.link_names;
    let parse = params.parse;
    let text = params.text;
    let icon_emoji = params.icon_emoji;
    let icon_url = params.icon_url;


    let client = &configuration.client;

    let uri_str = format!("{}/chat.postEphemeral", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = username {
        form_params.insert("username", param_value.to_string());
    }
    if let Some(param_value) = thread_ts {
        form_params.insert("thread_ts", param_value.to_string());
    }
    if let Some(param_value) = blocks {
        form_params.insert("blocks", param_value.to_string());
    }
    if let Some(param_value) = attachments {
        form_params.insert("attachments", param_value.to_string());
    }
    if let Some(param_value) = as_user {
        form_params.insert("as_user", param_value.to_string());
    }
    if let Some(param_value) = link_names {
        form_params.insert("link_names", param_value.to_string());
    }
    if let Some(param_value) = parse {
        form_params.insert("parse", param_value.to_string());
    }
    form_params.insert("token", token.to_string());
    if let Some(param_value) = text {
        form_params.insert("text", param_value.to_string());
    }
    form_params.insert("user", user.to_string());
    if let Some(param_value) = icon_emoji {
        form_params.insert("icon_emoji", param_value.to_string());
    }
    if let Some(param_value) = icon_url {
        form_params.insert("icon_url", param_value.to_string());
    }
    form_params.insert("channel", channel.to_string());
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::ChatPostEphemeralSuccessSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<PostEphemeralError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Sends a message to a channel.
pub async fn post_message(configuration: &configuration::Configuration, params: PostMessageParams) -> Result<crate::models::ChatPostMessageSuccessSchema, Error<PostMessageError>> {
    // unbox the parameters
    let channel = params.channel;
    let token = params.token;
    let attachments = params.attachments;
    let unfurl_links = params.unfurl_links;
    let text = params.text;
    let unfurl_media = params.unfurl_media;
    let parse = params.parse;
    let as_user = params.as_user;
    let mrkdwn = params.mrkdwn;
    let blocks = params.blocks;
    let icon_emoji = params.icon_emoji;
    let link_names = params.link_names;
    let reply_broadcast = params.reply_broadcast;
    let thread_ts = params.thread_ts;
    let username = params.username;
    let icon_url = params.icon_url;


    let client = &configuration.client;

    let uri_str = format!("{}/chat.postMessage", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = attachments {
        form_params.insert("attachments", param_value.to_string());
    }
    if let Some(param_value) = unfurl_links {
        form_params.insert("unfurl_links", param_value.to_string());
    }
    if let Some(param_value) = text {
        form_params.insert("text", param_value.to_string());
    }
    if let Some(param_value) = unfurl_media {
        form_params.insert("unfurl_media", param_value.to_string());
    }
    if let Some(param_value) = parse {
        form_params.insert("parse", param_value.to_string());
    }
    if let Some(param_value) = as_user {
        form_params.insert("as_user", param_value.to_string());
    }
    if let Some(param_value) = mrkdwn {
        form_params.insert("mrkdwn", param_value.to_string());
    }
    form_params.insert("channel", channel.to_string());
    if let Some(param_value) = blocks {
        form_params.insert("blocks", param_value.to_string());
    }
    if let Some(param_value) = icon_emoji {
        form_params.insert("icon_emoji", param_value.to_string());
    }
    if let Some(param_value) = link_names {
        form_params.insert("link_names", param_value.to_string());
    }
    if let Some(param_value) = reply_broadcast {
        form_params.insert("reply_broadcast", param_value.to_string());
    }
    if let Some(param_value) = thread_ts {
        form_params.insert("thread_ts", param_value.to_string());
    }
    if let Some(param_value) = username {
        form_params.insert("username", param_value.to_string());
    }
    form_params.insert("token", token.to_string());
    if let Some(param_value) = icon_url {
        form_params.insert("icon_url", param_value.to_string());
    }
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::ChatPostMessageSuccessSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<PostMessageError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Schedules a message to be sent to a channel.
pub async fn schedule_message(configuration: &configuration::Configuration, params: ScheduleMessageParams) -> Result<crate::models::ChatScheduleMessageSuccessSchema, Error<ScheduleMessageError>> {
    // unbox the parameters
    let thread_ts = params.thread_ts;
    let blocks = params.blocks;
    let attachments = params.attachments;
    let unfurl_links = params.unfurl_links;
    let text = params.text;
    let link_names = params.link_names;
    let unfurl_media = params.unfurl_media;
    let parse = params.parse;
    let token = params.token;
    let as_user = params.as_user;
    let post_at = params.post_at;
    let channel = params.channel;
    let reply_broadcast = params.reply_broadcast;


    let client = &configuration.client;

    let uri_str = format!("{}/chat.scheduleMessage", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = thread_ts {
        form_params.insert("thread_ts", param_value.to_string());
    }
    if let Some(param_value) = blocks {
        form_params.insert("blocks", param_value.to_string());
    }
    if let Some(param_value) = attachments {
        form_params.insert("attachments", param_value.to_string());
    }
    if let Some(param_value) = unfurl_links {
        form_params.insert("unfurl_links", param_value.to_string());
    }
    if let Some(param_value) = text {
        form_params.insert("text", param_value.to_string());
    }
    if let Some(param_value) = link_names {
        form_params.insert("link_names", param_value.to_string());
    }
    if let Some(param_value) = unfurl_media {
        form_params.insert("unfurl_media", param_value.to_string());
    }
    if let Some(param_value) = parse {
        form_params.insert("parse", param_value.to_string());
    }
    if let Some(param_value) = token {
        form_params.insert("token", param_value.to_string());
    }
    if let Some(param_value) = as_user {
        form_params.insert("as_user", param_value.to_string());
    }
    if let Some(param_value) = post_at {
        form_params.insert("post_at", param_value.to_string());
    }
    if let Some(param_value) = channel {
        form_params.insert("channel", param_value.to_string());
    }
    if let Some(param_value) = reply_broadcast {
        form_params.insert("reply_broadcast", param_value.to_string());
    }
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::ChatScheduleMessageSuccessSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<ScheduleMessageError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
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

/// Provide custom unfurl behavior for user-posted URLs
pub async fn unfurl(configuration: &configuration::Configuration, params: UnfurlParams) -> Result<crate::models::ChatUnfurlSuccessSchema, Error<UnfurlError>> {
    // unbox the parameters
    let ts = params.ts;
    let token = params.token;
    let channel = params.channel;
    let user_auth_message = params.user_auth_message;
    let user_auth_required = params.user_auth_required;
    let unfurls = params.unfurls;
    let user_auth_url = params.user_auth_url;


    let client = &configuration.client;

    let uri_str = format!("{}/chat.unfurl", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = user_auth_message {
        form_params.insert("user_auth_message", param_value.to_string());
    }
    if let Some(param_value) = user_auth_required {
        form_params.insert("user_auth_required", param_value.to_string());
    }
    if let Some(param_value) = unfurls {
        form_params.insert("unfurls", param_value.to_string());
    }
    form_params.insert("ts", ts.to_string());
    if let Some(param_value) = user_auth_url {
        form_params.insert("user_auth_url", param_value.to_string());
    }
    form_params.insert("token", token.to_string());
    form_params.insert("channel", channel.to_string());
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::ChatUnfurlSuccessSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<UnfurlError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Updates a message.
pub async fn update(configuration: &configuration::Configuration, params: UpdateParams) -> Result<crate::models::ChatUpdateSuccessSchema, Error<UpdateError>> {
    // unbox the parameters
    let ts = params.ts;
    let token = params.token;
    let channel = params.channel;
    let blocks = params.blocks;
    let attachments = params.attachments;
    let as_user = params.as_user;
    let parse = params.parse;
    let text = params.text;
    let link_names = params.link_names;


    let client = &configuration.client;

    let uri_str = format!("{}/chat.update", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = blocks {
        form_params.insert("blocks", param_value.to_string());
    }
    if let Some(param_value) = attachments {
        form_params.insert("attachments", param_value.to_string());
    }
    if let Some(param_value) = as_user {
        form_params.insert("as_user", param_value.to_string());
    }
    form_params.insert("ts", ts.to_string());
    if let Some(param_value) = parse {
        form_params.insert("parse", param_value.to_string());
    }
    form_params.insert("token", token.to_string());
    if let Some(param_value) = text {
        form_params.insert("text", param_value.to_string());
    }
    if let Some(param_value) = link_names {
        form_params.insert("link_names", param_value.to_string());
    }
    form_params.insert("channel", channel.to_string());
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    let data: Option<crate::models::ChatUpdateSuccessSchema> = serde_json::from_str(&content).ok();

    if data.is_some() {
		Ok(data.unwrap())
    } else {
        let entity: Option<UpdateError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

