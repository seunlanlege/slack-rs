# \MpimApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**close**](MpimApi.md#close) | **post** /mpim.close | 
[**history**](MpimApi.md#history) | **get** /mpim.history | 
[**list**](MpimApi.md#list) | **get** /mpim.list | 
[**mark**](MpimApi.md#mark) | **post** /mpim.mark | 
[**open**](MpimApi.md#open) | **post** /mpim.open | 
[**replies**](MpimApi.md#replies) | **get** /mpim.replies | 



## close

> crate::models::MpimCloseSchema close(token, channel)


Closes a multiparty direct message channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `mpim:write` | [required] |
**channel** | **String** | MPIM to close. | [required] |

### Return type

[**crate::models::MpimCloseSchema**](mpim_close_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history

> crate::models::MpimHistorySchema history(count, unreads, inclusive, token, oldest, channel, latest)


Fetches history of messages and events from a multiparty direct message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**i32**> | Number of messages to return, between 1 and 1000. |  |
**unreads** | Option<**bool**> | Include `unread_count_display` in the output? |  |
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `mpim:history` |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**channel** | Option<**String**> | Multiparty direct message to fetch history for. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |

### Return type

[**crate::models::MpimHistorySchema**](mpim_history_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> crate::models::MpimListSchema list(cursor, token, limit)


Lists multiparty direct message channels for the calling user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more details. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `mpim:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. |  |

### Return type

[**crate::models::MpimListSchema**](mpim_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark

> crate::models::MpimMarkSchema mark(token, ts, channel)


Sets the read cursor in a multiparty direct message channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `mpim:write` |  |
**ts** | Option<**f32**> | Timestamp of the most recently seen message. |  |
**channel** | Option<**String**> | multiparty direct message channel to set reading cursor in. |  |

### Return type

[**crate::models::MpimMarkSchema**](mpim_mark_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open

> crate::models::MpimOpenSuccessSchema open(token, users)


This method opens a multiparty direct message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `mpim:write` |  |
**users** | Option<**String**> | Comma separated lists of users.  The ordering of the users is preserved whenever a MPIM group is returned. |  |

### Return type

[**crate::models::MpimOpenSuccessSchema**](mpim_open_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replies

> crate::models::MpimRepliesSchema replies(thread_ts, token, channel)


Retrieve a thread of messages posted to a direct message conversation from a multiparty direct message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_ts** | Option<**f32**> | Unique identifier of a thread's parent message. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `mpim:history` |  |
**channel** | Option<**String**> | Multiparty direct message channel to fetch thread from. |  |

### Return type

[**crate::models::MpimRepliesSchema**](mpim_replies_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

