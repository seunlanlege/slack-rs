# \GroupsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive**](GroupsApi.md#archive) | **post** /groups.archive | 
[**create**](GroupsApi.md#create) | **post** /groups.create | 
[**create_child**](GroupsApi.md#create_child) | **post** /groups.createChild | 
[**history**](GroupsApi.md#history) | **get** /groups.history | 
[**info**](GroupsApi.md#info) | **get** /groups.info | 
[**invite**](GroupsApi.md#invite) | **post** /groups.invite | 
[**kick**](GroupsApi.md#kick) | **post** /groups.kick | 
[**leave**](GroupsApi.md#leave) | **post** /groups.leave | 
[**list**](GroupsApi.md#list) | **get** /groups.list | 
[**mark**](GroupsApi.md#mark) | **post** /groups.mark | 
[**open**](GroupsApi.md#open) | **post** /groups.open | 
[**rename**](GroupsApi.md#rename) | **post** /groups.rename | 
[**replies**](GroupsApi.md#replies) | **get** /groups.replies | 
[**set_purpose**](GroupsApi.md#set_purpose) | **post** /groups.setPurpose | 
[**set_topic**](GroupsApi.md#set_topic) | **post** /groups.setTopic | 
[**unarchive**](GroupsApi.md#unarchive) | **post** /groups.unarchive | 



## archive

> crate::models::GroupsArchiveSchema archive(token, channel)


Archives a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**channel** | Option<**String**> | Private channel to archive |  |

### Return type

[**crate::models::GroupsArchiveSchema**](groups_archive_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create

> crate::models::GroupsCreateSuccessSchema create(token, validate, name)


Creates a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**validate** | Option<**bool**> | Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria. |  |
**name** | Option<**String**> | Name of private channel to create |  |

### Return type

[**crate::models::GroupsCreateSuccessSchema**](groups_create_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_child

> crate::models::GroupsCreateChildSchema create_child(token, channel)


Clones and archives a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**channel** | Option<**String**> | Private channel to clone and archive. |  |

### Return type

[**crate::models::GroupsCreateChildSchema**](groups_createChild_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history

> crate::models::GroupsHistorySuccessSchema history(count, unreads, inclusive, token, oldest, channel, latest)


Fetches history of messages and events from a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**i32**> | Number of messages to return, between 1 and 1000. |  |
**unreads** | Option<**bool**> | Include `unread_count_display` in the output? |  |
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `groups:history` |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**channel** | Option<**String**> | Private channel to fetch history for. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |

### Return type

[**crate::models::GroupsHistorySuccessSchema**](groups_history_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info

> crate::models::GroupsInfoSuccessSchema info(token, include_locale, channel)


Gets information about a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:read` |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for this group. Defaults to `false` |  |
**channel** | Option<**String**> | Private channel to get info on |  |

### Return type

[**crate::models::GroupsInfoSuccessSchema**](groups_info_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite

> crate::models::GroupsInviteSuccessSchema invite(token, user, channel)


Invites a user to a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**user** | Option<**String**> | User to invite. |  |
**channel** | Option<**String**> | Private channel to invite user to. |  |

### Return type

[**crate::models::GroupsInviteSuccessSchema**](groups_invite_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kick

> crate::models::GroupsKickSchema kick(token, user, channel)


Removes a user from a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `groups:write` | [required] |
**user** | **String** | User to remove from private channel. | [required] |
**channel** | **String** | Private channel to remove user from. | [required] |

### Return type

[**crate::models::GroupsKickSchema**](groups_kick_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave

> crate::models::GroupsLeaveSchema leave(token, channel)


Leaves a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `groups:write` | [required] |
**channel** | **String** | Private channel to leave | [required] |

### Return type

[**crate::models::GroupsLeaveSchema**](groups_leave_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> crate::models::GroupsListSuccessSchema list(exclude_members, cursor, token, limit, exclude_archived)


Lists private channels that the calling user has access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exclude_members** | Option<**bool**> | Exclude the `members` from each `group` |  |
**cursor** | Option<**String**> | Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more details. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `groups:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. |  |
**exclude_archived** | Option<**bool**> | Don't return archived private channels. |  |

### Return type

[**crate::models::GroupsListSuccessSchema**](groups_list_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark

> crate::models::GroupsMarkSuccessSchema mark(token, ts, channel)


Sets the read cursor in a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**ts** | Option<**f32**> | Timestamp of the most recently seen message. |  |
**channel** | Option<**String**> | Private channel to set reading cursor in. |  |

### Return type

[**crate::models::GroupsMarkSuccessSchema**](groups_mark_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open

> crate::models::GroupsOpenSchema open(token, channel)


Opens a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**channel** | Option<**String**> | Private channel to open. |  |

### Return type

[**crate::models::GroupsOpenSchema**](groups_open_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rename

> crate::models::GroupsRenameSchema rename(token, validate, name, channel)


Renames a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**validate** | Option<**bool**> | Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria. |  |
**name** | Option<**String**> | New name for private channel. |  |
**channel** | Option<**String**> | Private channel to rename |  |

### Return type

[**crate::models::GroupsRenameSchema**](groups_rename_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replies

> crate::models::GroupsRepliesSchema replies(thread_ts, token, channel)


Retrieve a thread of messages posted to a private channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_ts** | Option<**f32**> | Unique identifier of a thread's parent message |  |
**token** | Option<**String**> | Authentication token. Requires scope: `groups:history` |  |
**channel** | Option<**String**> | Private channel to fetch thread from |  |

### Return type

[**crate::models::GroupsRepliesSchema**](groups_replies_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_purpose

> crate::models::GroupsSetPurposeSchema set_purpose(token, purpose, channel)


Sets the purpose for a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**purpose** | Option<**String**> | The new purpose |  |
**channel** | Option<**String**> | Private channel to set the purpose of |  |

### Return type

[**crate::models::GroupsSetPurposeSchema**](groups_setPurpose_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_topic

> crate::models::GroupsSetTopicSchema set_topic(token, topic, channel)


Sets the topic for a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**topic** | Option<**String**> | The new topic |  |
**channel** | Option<**String**> | Private channel to set the topic of |  |

### Return type

[**crate::models::GroupsSetTopicSchema**](groups_setTopic_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unarchive

> crate::models::GroupsUnarchiveSchema unarchive(token, channel)


Unarchives a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**channel** | Option<**String**> | Private channel to unarchive |  |

### Return type

[**crate::models::GroupsUnarchiveSchema**](groups_unarchive_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

