# \AdminTeamsOwnersApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**teams_owners_list**](AdminTeamsOwnersApi.md#teams_owners_list) | **get** /admin.teams.owners.list | 



## teams_owners_list

> ::std::collections::HashMap<String, serde_json::Value> teams_owners_list(token, team_id, cursor, limit)


List all of the owners on a given workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:read` | [required] |
**team_id** | **String** |  | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page. |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

