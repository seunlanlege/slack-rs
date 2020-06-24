# \ChannelsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive**](ChannelsApi.md#archive) | **post** /channels.archive | 
[**create**](ChannelsApi.md#create) | **post** /channels.create | 
[**history**](ChannelsApi.md#history) | **get** /channels.history | 
[**info**](ChannelsApi.md#info) | **get** /channels.info | 
[**invite**](ChannelsApi.md#invite) | **post** /channels.invite | 
[**join**](ChannelsApi.md#join) | **post** /channels.join | 
[**kick**](ChannelsApi.md#kick) | **post** /channels.kick | 
[**leave**](ChannelsApi.md#leave) | **post** /channels.leave | 
[**list**](ChannelsApi.md#list) | **get** /channels.list | 
[**mark**](ChannelsApi.md#mark) | **post** /channels.mark | 
[**rename**](ChannelsApi.md#rename) | **post** /channels.rename | 
[**replies**](ChannelsApi.md#replies) | **get** /channels.replies | 
[**set_purpose**](ChannelsApi.md#set_purpose) | **post** /channels.setPurpose | 
[**set_topic**](ChannelsApi.md#set_topic) | **post** /channels.setTopic | 
[**unarchive**](ChannelsApi.md#unarchive) | **post** /channels.unarchive | 



## archive

> crate::models::ChannelsArchiveSuccessSchema archive(token, channel)


Archives a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**channel** | Option<**String**> | Channel to archive |  |

### Return type

[**crate::models::ChannelsArchiveSuccessSchema**](channels_archive_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create

> crate::models::ChannelsCreateErrorSchema create(token, validate, name)


Creates a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**validate** | Option<**bool**> | Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria. |  |
**name** | Option<**String**> | Name of channel to create |  |

### Return type

[**crate::models::ChannelsCreateErrorSchema**](channels_create_error_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history

> crate::models::ChannelsHistorySuccessSchema history(count, unreads, inclusive, token, oldest, channel, latest)


Fetches history of messages and events from a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**i32**> | Number of messages to return, between 1 and 1000. |  |
**unreads** | Option<**bool**> | Include `unread_count_display` in the output? |  |
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `channels:history` |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**channel** | Option<**String**> | Channel to fetch history for. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |

### Return type

[**crate::models::ChannelsHistorySuccessSchema**](channels_history_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info

> crate::models::ChannelsInfoSuccessSchema info(token, include_locale, channel)


Gets information about a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:read` |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for this channel. Defaults to `false` |  |
**channel** | Option<**String**> | Channel to get info on |  |

### Return type

[**crate::models::ChannelsInfoSuccessSchema**](channels_info_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite

> crate::models::ChannelsInviteErrorSchema invite(token, user, channel)


Invites a user to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**user** | Option<**String**> | User to invite to channel. |  |
**channel** | Option<**String**> | Channel to invite user to. |  |

### Return type

[**crate::models::ChannelsInviteErrorSchema**](channels_invite_error_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join

> crate::models::ChannelsJoinSchema join(token, validate, name)


Joins a channel, creating it if needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**validate** | Option<**bool**> | Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria. |  |
**name** | Option<**String**> | Name of channel to join |  |

### Return type

[**crate::models::ChannelsJoinSchema**](channels_join_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kick

> crate::models::ChannelsKickSchema kick(token, user, channel)


Removes a user from a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**user** | Option<**String**> | User to remove from channel. |  |
**channel** | Option<**String**> | Channel to remove user from. |  |

### Return type

[**crate::models::ChannelsKickSchema**](channels_kick_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave

> crate::models::ChannelsLeaveSchema leave(token, channel)


Leaves a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**channel** | Option<**String**> | Channel to leave |  |

### Return type

[**crate::models::ChannelsLeaveSchema**](channels_leave_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> crate::models::ChannelsListSuccessSchema list(exclude_members, cursor, token, limit, exclude_archived)


Lists all channels in a Slack team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exclude_members** | Option<**bool**> | Exclude the `members` collection from each `channel` |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `channels:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**exclude_archived** | Option<**bool**> | Exclude archived channels from the list |  |

### Return type

[**crate::models::ChannelsListSuccessSchema**](channels_list_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark

> crate::models::ChannelsMarkSuccessSchema mark(token, ts, channel)


Sets the read cursor in a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**ts** | Option<**f32**> | Timestamp of the most recently seen message. |  |
**channel** | Option<**String**> | Channel to set reading cursor in. |  |

### Return type

[**crate::models::ChannelsMarkSuccessSchema**](channels_mark_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rename

> crate::models::ChannelsRenameSchema rename(token, validate, name, channel)


Renames a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**validate** | Option<**bool**> | Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria. |  |
**name** | Option<**String**> | New name for channel. |  |
**channel** | Option<**String**> | Channel to rename |  |

### Return type

[**crate::models::ChannelsRenameSchema**](channels_rename_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replies

> crate::models::ChannelsRepliesSchema replies(thread_ts, token, channel)


Retrieve a thread of messages posted to a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_ts** | Option<**f32**> | Unique identifier of a thread's parent message |  |
**token** | Option<**String**> | Authentication token. Requires scope: `channels:history` |  |
**channel** | Option<**String**> | Channel to fetch thread from |  |

### Return type

[**crate::models::ChannelsRepliesSchema**](channels_replies_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_purpose

> crate::models::ChannelsSetPurposeSchema set_purpose(token, purpose, channel, name_tagging)


Sets the purpose for a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `channels:write` | [required] |
**purpose** | **String** | The new purpose | [required] |
**channel** | **String** | Channel to set the purpose of | [required] |
**name_tagging** | Option<**bool**> | if it is true, treat this like a message and not an unescaped thing |  |

### Return type

[**crate::models::ChannelsSetPurposeSchema**](channels_setPurpose_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_topic

> crate::models::ChannelsSetTopicSchema set_topic(token, topic, channel)


Sets the topic for a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `channels:write` | [required] |
**topic** | **String** | The new topic | [required] |
**channel** | **String** | Channel to set the topic of | [required] |

### Return type

[**crate::models::ChannelsSetTopicSchema**](channels_setTopic_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unarchive

> crate::models::ChannelsUnarchiveSchema unarchive(token, channel)


Unarchives a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `channels:write` | [required] |
**channel** | **String** | Channel to unarchive | [required] |

### Return type

[**crate::models::ChannelsUnarchiveSchema**](channels_unarchive_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

