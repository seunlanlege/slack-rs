# \ChatScheduledMessagesApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scheduled_messages_list**](ChatScheduledMessagesApi.md#scheduled_messages_list) | **get** /chat.scheduledMessages.list | 



## scheduled_messages_list

> crate::models::ChatScheduledMessagesListSchema scheduled_messages_list(cursor, token, limit, oldest, channel, latest)


Returns a list of scheduled messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | For pagination purposes, this is the `cursor` value returned from a previous call to `chat.scheduledmessages.list` indicating where you want to start this call from. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `none` |  |
**limit** | Option<**i32**> | Maximum number of original entries to return. |  |
**oldest** | Option<**f32**> | A UNIX timestamp of the oldest value in the time range |  |
**channel** | Option<**String**> | The channel of the scheduled messages |  |
**latest** | Option<**f32**> | A UNIX timestamp of the latest value in the time range |  |

### Return type

[**crate::models::ChatScheduledMessagesListSchema**](chat_scheduledMessages_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

