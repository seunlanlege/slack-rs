use reqwest;
use serde_json;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod admin;
pub mod admin_apps;
pub mod admin_apps_approved;
pub mod admin_apps_requests;
pub mod admin_apps_restricted;
pub mod admin_conversations;
pub mod admin_emoji;
pub mod admin_invite_requests;
pub mod admin_invite_requests_approved;
pub mod admin_invite_requests_denied;
pub mod admin_teams;
pub mod admin_teams_admins;
pub mod admin_teams_owners;
pub mod admin_teams_settings;
pub mod admin_users;
pub mod admin_users_session;
pub mod api;
pub mod apps;
pub mod apps_permissions;
pub mod apps_permissions_resources;
pub mod apps_permissions_scopes;
pub mod apps_permissions_users;
pub mod auth;
pub mod bots;
pub mod channels;
pub mod chat;
pub mod chat_scheduled_messages;
pub mod conversations;
pub mod dialog;
pub mod dnd;
pub mod emoji;
pub mod files;
pub mod files_comments;
pub mod files_remote;
pub mod groups;
pub mod im;
pub mod migration;
pub mod mpim;
pub mod oauth;
pub mod oauth_v2;
pub mod pins;
pub mod reactions;
pub mod reminders;
pub mod rtm;
pub mod search;
pub mod stars;
pub mod team;
pub mod team_profile;
pub mod usergroups;
pub mod usergroups_users;
pub mod users;
pub mod users_profile;
pub mod views;

pub mod configuration;
