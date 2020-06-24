# \ViewsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**open**](ViewsApi.md#open) | **get** /views.open | 
[**publish**](ViewsApi.md#publish) | **get** /views.publish | 
[**push**](ViewsApi.md#push) | **get** /views.push | 
[**update**](ViewsApi.md#update) | **get** /views.update | 



## open

> ::std::collections::HashMap<String, serde_json::Value> open(token, trigger_id, view)


Open a view for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**trigger_id** | **String** | Exchange a trigger to post to the user. | [required] |
**view** | **String** | A [view payload](/reference/surfaces/views). This must be a JSON-encoded string. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish

> ::std::collections::HashMap<String, serde_json::Value> publish(token, user_id, view, hash)


Publish a static view for a User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**user_id** | **String** | `id` of the user you want publish a view to. | [required] |
**view** | **String** | A [view payload](/reference/surfaces/views). This must be a JSON-encoded string. | [required] |
**hash** | Option<**String**> | A string that represents view state to protect against possible race conditions. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## push

> ::std::collections::HashMap<String, serde_json::Value> push(token, trigger_id, view)


Push a view onto the stack of a root view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**trigger_id** | **String** | Exchange a trigger to post to the user. | [required] |
**view** | **String** | A [view payload](/reference/surfaces/views). This must be a JSON-encoded string. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update

> ::std::collections::HashMap<String, serde_json::Value> update(token, hash, view_id, external_id, view)


Update an existing view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**hash** | Option<**String**> | A string that represents view state to protect against possible race conditions. |  |
**view_id** | Option<**String**> | A unique identifier of the view to be updated. Either `view_id` or `external_id` is required. |  |
**external_id** | Option<**String**> | A unique identifier of the view set by the developer. Must be unique for all views on a team. Max length of 255 characters. Either `view_id` or `external_id` is required. |  |
**view** | Option<**String**> | A [view payload](/reference/surfaces/views) This must be a JSON-encoded string. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

