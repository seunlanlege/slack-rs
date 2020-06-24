# \AdminAppsRestrictedApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_restricted_list**](AdminAppsRestrictedApi.md#apps_restricted_list) | **get** /admin.apps.restricted.list | 



## apps_restricted_list

> ::std::collections::HashMap<String, serde_json::Value> apps_restricted_list(token, cursor, limit, team_id, enterprise_id)


List restricted apps for an org or workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:read` | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |
**team_id** | Option<**String**> |  |  |
**enterprise_id** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

