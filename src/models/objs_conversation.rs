/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjsConversation {
    #[serde(rename = "accepted_user", skip_serializing_if = "Option::is_none")]
    pub accepted_user: Option<String>,
    #[serde(rename = "connected_team_ids", skip_serializing_if = "Option::is_none")]
    pub connected_team_ids: Option<Vec<String>>,
    #[serde(rename = "conversation_host_id", skip_serializing_if = "Option::is_none")]
    pub conversation_host_id: Option<String>,
    #[serde(rename = "created")]
    pub created: i32,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "display_counts", skip_serializing_if = "Option::is_none")]
    pub display_counts: Option<crate::models::ObjsConversationMpimDisplayCounts>,
    #[serde(rename = "enterprise_id", skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
    #[serde(rename = "external_connections", skip_serializing_if = "Option::is_none")]
    pub external_connections: Option<serde_json::Value>,
    #[serde(rename = "has_pins", skip_serializing_if = "Option::is_none")]
    pub has_pins: Option<bool>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "internal_team_ids", skip_serializing_if = "Option::is_none")]
    pub internal_team_ids: Option<Vec<String>>,
    #[serde(rename = "is_archived")]
    pub is_archived: bool,
    #[serde(rename = "is_channel")]
    pub is_channel: bool,
    #[serde(rename = "is_ext_shared", skip_serializing_if = "Option::is_none")]
    pub is_ext_shared: Option<bool>,
    #[serde(rename = "is_frozen", skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
    #[serde(rename = "is_general")]
    pub is_general: bool,
    #[serde(rename = "is_global_shared", skip_serializing_if = "Option::is_none")]
    pub is_global_shared: Option<bool>,
    #[serde(rename = "is_group")]
    pub is_group: bool,
    #[serde(rename = "is_im")]
    pub is_im: bool,
    #[serde(rename = "is_member", skip_serializing_if = "Option::is_none")]
    pub is_member: Option<bool>,
    #[serde(rename = "is_moved", skip_serializing_if = "Option::is_none")]
    pub is_moved: Option<i32>,
    #[serde(rename = "is_mpim")]
    pub is_mpim: bool,
    #[serde(rename = "is_non_threadable", skip_serializing_if = "Option::is_none")]
    pub is_non_threadable: Option<bool>,
    #[serde(rename = "is_open", skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    #[serde(rename = "is_org_default", skip_serializing_if = "Option::is_none")]
    pub is_org_default: Option<bool>,
    #[serde(rename = "is_org_mandatory", skip_serializing_if = "Option::is_none")]
    pub is_org_mandatory: Option<bool>,
    #[serde(rename = "is_org_shared")]
    pub is_org_shared: bool,
    #[serde(rename = "is_pending_ext_shared", skip_serializing_if = "Option::is_none")]
    pub is_pending_ext_shared: Option<bool>,
    #[serde(rename = "is_private")]
    pub is_private: bool,
    #[serde(rename = "is_read_only", skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    #[serde(rename = "is_shared")]
    pub is_shared: bool,
    #[serde(rename = "is_starred", skip_serializing_if = "Option::is_none")]
    pub is_starred: Option<bool>,
    #[serde(rename = "is_thread_only", skip_serializing_if = "Option::is_none")]
    pub is_thread_only: Option<bool>,
    #[serde(rename = "last_read", skip_serializing_if = "Option::is_none")]
    pub last_read: Option<String>,
    #[serde(rename = "latest", skip_serializing_if = "Option::is_none")]
    pub latest: Option<serde_json::Value>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "name_normalized")]
    pub name_normalized: String,
    #[serde(rename = "num_members", skip_serializing_if = "Option::is_none")]
    pub num_members: Option<i32>,
    #[serde(rename = "parent_conversation", skip_serializing_if = "Option::is_none")]
    pub parent_conversation: Option<serde_json::Value>,
    #[serde(rename = "pending_connected_team_ids", skip_serializing_if = "Option::is_none")]
    pub pending_connected_team_ids: Option<Vec<String>>,
    #[serde(rename = "pending_shared", skip_serializing_if = "Option::is_none")]
    pub pending_shared: Option<Vec<String>>,
    #[serde(rename = "pin_count", skip_serializing_if = "Option::is_none")]
    pub pin_count: Option<i32>,
    #[serde(rename = "previous_names", skip_serializing_if = "Option::is_none")]
    pub previous_names: Option<Vec<String>>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<f32>,
    #[serde(rename = "purpose")]
    pub purpose: crate::models::ObjsChannelPurpose,
    #[serde(rename = "shared_team_ids", skip_serializing_if = "Option::is_none")]
    pub shared_team_ids: Option<Vec<String>>,
    #[serde(rename = "shares", skip_serializing_if = "Option::is_none")]
    pub shares: Option<Vec<crate::models::ObjsConversationMpimShares>>,
    #[serde(rename = "timezone_count", skip_serializing_if = "Option::is_none")]
    pub timezone_count: Option<i32>,
    #[serde(rename = "topic")]
    pub topic: crate::models::ObjsChannelPurpose,
    #[serde(rename = "unlinked", skip_serializing_if = "Option::is_none")]
    pub unlinked: Option<i32>,
    #[serde(rename = "unread_count", skip_serializing_if = "Option::is_none")]
    pub unread_count: Option<i32>,
    #[serde(rename = "unread_count_display", skip_serializing_if = "Option::is_none")]
    pub unread_count_display: Option<i32>,
    #[serde(rename = "use_case", skip_serializing_if = "Option::is_none")]
    pub use_case: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl ObjsConversation {
    pub fn new(created: i32, creator: String, id: String, is_archived: bool, is_channel: bool, is_general: bool, is_group: bool, is_im: bool, is_mpim: bool, is_org_shared: bool, is_private: bool, is_shared: bool, name: String, name_normalized: String, purpose: crate::models::ObjsChannelPurpose, topic: crate::models::ObjsChannelPurpose) -> ObjsConversation {
        ObjsConversation {
            accepted_user: None,
            connected_team_ids: None,
            conversation_host_id: None,
            created,
            creator,
            display_counts: None,
            enterprise_id: None,
            external_connections: None,
            has_pins: None,
            id,
            internal_team_ids: None,
            is_archived,
            is_channel,
            is_ext_shared: None,
            is_frozen: None,
            is_general,
            is_global_shared: None,
            is_group,
            is_im,
            is_member: None,
            is_moved: None,
            is_mpim,
            is_non_threadable: None,
            is_open: None,
            is_org_default: None,
            is_org_mandatory: None,
            is_org_shared,
            is_pending_ext_shared: None,
            is_private,
            is_read_only: None,
            is_shared,
            is_starred: None,
            is_thread_only: None,
            last_read: None,
            latest: None,
            members: None,
            name,
            name_normalized,
            num_members: None,
            parent_conversation: None,
            pending_connected_team_ids: None,
            pending_shared: None,
            pin_count: None,
            previous_names: None,
            priority: None,
            purpose,
            shared_team_ids: None,
            shares: None,
            timezone_count: None,
            topic,
            unlinked: None,
            unread_count: None,
            unread_count_display: None,
            use_case: None,
            user: None,
            version: None,
        }
    }
}


