# \AppsPermissionsScopesApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**permissions_scopes_list**](AppsPermissionsScopesApi.md#permissions_scopes_list) | **get** /apps.permissions.scopes.list | 



## permissions_scopes_list

> ::std::collections::HashMap<String, serde_json::Value> permissions_scopes_list(token)


Returns list of scopes this app has on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

