# \OauthApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**access**](OauthApi.md#access) | **get** /oauth.access | 
[**token**](OauthApi.md#token) | **get** /oauth.token | 
[**v2_access**](OauthApi.md#v2_access) | **get** /oauth.v2.access | 



## access

> ::std::collections::HashMap<String, serde_json::Value> access(code, token, redirect_uri, single_channel, client_id, client_secret)


Exchanges a temporary OAuth verifier code for an access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | Option<**String**> | The `code` param returned via the OAuth callback. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `none` |  |
**redirect_uri** | Option<**String**> | This must match the originally submitted URI (if one was sent). |  |
**single_channel** | Option<**bool**> | Request the user to add your app only to a single channel. Only valid with a [legacy workspace app](https://api.slack.com/legacy-workspace-apps). |  |
**client_id** | Option<**String**> | Issued when you created your application. |  |
**client_secret** | Option<**String**> | Issued when you created your application. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token

> ::std::collections::HashMap<String, serde_json::Value> token(client_secret, code, single_channel, client_id, redirect_uri)


Exchanges a temporary OAuth verifier code for a workspace token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_secret** | Option<**String**> | Issued when you created your application. |  |
**code** | Option<**String**> | The `code` param returned via the OAuth callback. |  |
**single_channel** | Option<**bool**> | Request the user to add your app only to a single channel. |  |
**client_id** | Option<**String**> | Issued when you created your application. |  |
**redirect_uri** | Option<**String**> | This must match the originally submitted URI (if one was sent). |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_access

> ::std::collections::HashMap<String, serde_json::Value> v2_access(code, client_secret, client_id, redirect_uri)


Exchanges a temporary OAuth verifier code for an access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The `code` param returned via the OAuth callback. | [required] |
**client_secret** | Option<**String**> | Issued when you created your application. |  |
**client_id** | Option<**String**> | Issued when you created your application. |  |
**redirect_uri** | Option<**String**> | This must match the originally submitted URI (if one was sent). |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

