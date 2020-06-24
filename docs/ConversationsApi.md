# \ConversationsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive**](ConversationsApi.md#archive) | **post** /conversations.archive | 
[**close**](ConversationsApi.md#close) | **post** /conversations.close | 
[**create**](ConversationsApi.md#create) | **post** /conversations.create | 
[**history**](ConversationsApi.md#history) | **get** /conversations.history | 
[**info**](ConversationsApi.md#info) | **get** /conversations.info | 
[**invite**](ConversationsApi.md#invite) | **post** /conversations.invite | 
[**join**](ConversationsApi.md#join) | **post** /conversations.join | 
[**kick**](ConversationsApi.md#kick) | **post** /conversations.kick | 
[**leave**](ConversationsApi.md#leave) | **post** /conversations.leave | 
[**list**](ConversationsApi.md#list) | **get** /conversations.list | 
[**members**](ConversationsApi.md#members) | **get** /conversations.members | 
[**open**](ConversationsApi.md#open) | **post** /conversations.open | 
[**rename**](ConversationsApi.md#rename) | **post** /conversations.rename | 
[**replies**](ConversationsApi.md#replies) | **get** /conversations.replies | 
[**set_purpose**](ConversationsApi.md#set_purpose) | **post** /conversations.setPurpose | 
[**set_topic**](ConversationsApi.md#set_topic) | **post** /conversations.setTopic | 
[**unarchive**](ConversationsApi.md#unarchive) | **post** /conversations.unarchive | 



## archive

> crate::models::ConversationsArchiveSuccessSchema archive(token, channel)


Archives a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to archive |  |

### Return type

[**crate::models::ConversationsArchiveSuccessSchema**](conversations_archive_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## close

> crate::models::ConversationsCloseSuccessSchema close(token, channel)


Closes a direct message or multi-person direct message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to close. |  |

### Return type

[**crate::models::ConversationsCloseSuccessSchema**](conversations_close_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create

> crate::models::ConversationsCreateSuccessSchema create(token, name, user_ids, is_private)


Initiates a public or private channel-based conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**name** | Option<**String**> | Name of the public or private channel to create |  |
**user_ids** | Option<**String**> | **Required** for workspace apps. A list of between 1 and 30 human users that will be added to the newly-created conversation. This argument has no effect when used by classic Slack apps. |  |
**is_private** | Option<**bool**> | Create a private channel instead of a public one |  |

### Return type

[**crate::models::ConversationsCreateSuccessSchema**](conversations_create_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history

> crate::models::ConversationsHistorySuccessSchema history(inclusive, cursor, token, limit, oldest, channel, latest)


Fetches a conversation's history of messages and events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results only when either timestamp is specified. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:history` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**channel** | Option<**String**> | Conversation ID to fetch history for. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |

### Return type

[**crate::models::ConversationsHistorySuccessSchema**](conversations_history_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info

> crate::models::ConversationsInfoSuccessSchema info(include_num_members, token, channel, include_locale)


Retrieve information about a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_num_members** | Option<**bool**> | Set to `true` to include the member count for the specified conversation. Defaults to `false` |  |
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**channel** | Option<**String**> | Conversation ID to learn more about |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for this conversation. Defaults to `false` |  |

### Return type

[**crate::models::ConversationsInfoSuccessSchema**](conversations_info_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite

> crate::models::ConversationsInviteErrorSchema invite(token, users, channel)


Invites users to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**users** | Option<**String**> | A comma separated list of user IDs. Up to 1000 users may be listed. |  |
**channel** | Option<**String**> | The ID of the public or private channel to invite user(s) to. |  |

### Return type

[**crate::models::ConversationsInviteErrorSchema**](conversations_invite_error_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join

> crate::models::ConversationsJoinSuccessSchema join(token, channel)


Joins an existing conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**channel** | Option<**String**> | ID of conversation to join |  |

### Return type

[**crate::models::ConversationsJoinSuccessSchema**](conversations_join_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kick

> crate::models::ConversationsKickSuccessSchema kick(token, user, channel)


Removes a user from a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**user** | Option<**String**> | User ID to be removed. |  |
**channel** | Option<**String**> | ID of conversation to remove user from. |  |

### Return type

[**crate::models::ConversationsKickSuccessSchema**](conversations_kick_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave

> crate::models::ConversationsLeaveSuccessSchema leave(token, channel)


Leaves a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to leave |  |

### Return type

[**crate::models::ConversationsLeaveSuccessSchema**](conversations_leave_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> crate::models::ConversationsListSuccessSchema list(cursor, token, limit, exclude_archived, types)


Lists all channels in a Slack team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000. |  |
**exclude_archived** | Option<**bool**> | Set to `true` to exclude archived channels from the list |  |
**types** | Option<**String**> | Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im` |  |

### Return type

[**crate::models::ConversationsListSuccessSchema**](conversations_list_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## members

> crate::models::ConversationsMembersSuccessSchema members(cursor, token, limit, channel)


Retrieve members of a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**channel** | Option<**String**> | ID of the conversation to retrieve members for |  |

### Return type

[**crate::models::ConversationsMembersSuccessSchema**](conversations_members_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open

> crate::models::ConversationsOpenSuccessSchema open(token, return_im, users, channel)


Opens or resumes a direct message or multi-person direct message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**return_im** | Option<**bool**> | Boolean, indicates you want the full IM channel definition in the response. |  |
**users** | Option<**String**> | Comma separated lists of users. If only one user is included, this creates a 1:1 DM.  The ordering of the users is preserved whenever a multi-person direct message is returned. Supply a `channel` when not supplying `users`. |  |
**channel** | Option<**String**> | Resume a conversation by supplying an `im` or `mpim`'s ID. Or provide the `users` field instead. |  |

### Return type

[**crate::models::ConversationsOpenSuccessSchema**](conversations_open_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rename

> crate::models::ConversationsRenameSuccessSchema rename(token, name, channel)


Renames a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**name** | Option<**String**> | New name for conversation. |  |
**channel** | Option<**String**> | ID of conversation to rename |  |

### Return type

[**crate::models::ConversationsRenameSuccessSchema**](conversations_rename_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replies

> crate::models::ConversationsRepliesSuccessSchema replies(inclusive, ts, cursor, token, limit, oldest, channel, latest)


Retrieve a thread of messages posted to a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results only when either timestamp is specified. |  |
**ts** | Option<**f32**> | Unique identifier of a thread's parent message. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:history` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**channel** | Option<**String**> | Conversation ID to fetch thread from. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |

### Return type

[**crate::models::ConversationsRepliesSuccessSchema**](conversations_replies_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_purpose

> crate::models::ConversationsSetPurposeSuccessSchema set_purpose(token, purpose, channel)


Sets the purpose for a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**purpose** | Option<**String**> | A new, specialer purpose |  |
**channel** | Option<**String**> | Conversation to set the purpose of |  |

### Return type

[**crate::models::ConversationsSetPurposeSuccessSchema**](conversations_setPurpose_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_topic

> crate::models::ConversationsSetTopicSuccessSchema set_topic(token, topic, channel)


Sets the topic for a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**topic** | Option<**String**> | The new topic string. Does not support formatting or linkification. |  |
**channel** | Option<**String**> | Conversation to set the topic of |  |

### Return type

[**crate::models::ConversationsSetTopicSuccessSchema**](conversations_setTopic_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unarchive

> crate::models::ConversationsUnarchiveSuccessSchema unarchive(token, channel)


Reverses conversation archival.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to unarchive |  |

### Return type

[**crate::models::ConversationsUnarchiveSuccessSchema**](conversations_unarchive_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

