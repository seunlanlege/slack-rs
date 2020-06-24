# \AdminInviteRequestsApprovedApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**invite_requests_approved_list**](AdminInviteRequestsApprovedApi.md#invite_requests_approved_list) | **get** /admin.inviteRequests.approved.list | 



## invite_requests_approved_list

> ::std::collections::HashMap<String, serde_json::Value> invite_requests_approved_list(token, cursor, limit, team_id)


List all approved workspace invite requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.invites:read` | [required] |
**cursor** | Option<**String**> | Value of the `next_cursor` field sent as part of the previous API response |  |
**limit** | Option<**i32**> | The number of results that will be returned by the API on each invocation. Must be between 1 - 1000, both inclusive |  |
**team_id** | Option<**String**> | ID for the workspace where the invite requests were made. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

