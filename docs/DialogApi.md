# \DialogApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**open**](DialogApi.md#open) | **get** /dialog.open | 



## open

> crate::models::DialogOpenSchema open(token, trigger_id, dialog)


Open a dialog with a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**trigger_id** | **String** | Exchange a trigger to post to the user. | [required] |
**dialog** | **String** | The dialog definition. This must be a JSON-encoded string. | [required] |

### Return type

[**crate::models::DialogOpenSchema**](dialog_open_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

