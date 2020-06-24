# \ReactionsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add**](ReactionsApi.md#add) | **post** /reactions.add | 
[**get**](ReactionsApi.md#get) | **get** /reactions.get | 
[**list**](ReactionsApi.md#list) | **get** /reactions.list | 
[**remove**](ReactionsApi.md#remove) | **post** /reactions.remove | 



## add

> crate::models::ReactionsAddSchema add(token, timestamp, name, channel)


Adds a reaction to an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `reactions:write` | [required] |
**timestamp** | **String** | Timestamp of the message to add reaction to. | [required] |
**name** | **String** | Reaction (emoji) name. | [required] |
**channel** | **String** | Channel where the message to add reaction to was posted. | [required] |

### Return type

[**crate::models::ReactionsAddSchema**](reactions_add_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get

> serde_json::Value get(token, full, file_comment, timestamp, file, channel)


Gets reactions for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `reactions:read` | [required] |
**full** | Option<**bool**> | If true always return the complete reaction list. |  |
**file_comment** | Option<**String**> | File comment to get reactions for. |  |
**timestamp** | Option<**String**> | Timestamp of the message to get reactions for. |  |
**file** | Option<**String**> | File to get reactions for. |  |
**channel** | Option<**String**> | Channel where the message to get reactions for was posted. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> crate::models::ReactionsListSchema list(token, count, full, cursor, limit, user, page)


Lists reactions made by a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `reactions:read` | [required] |
**count** | Option<**i32**> |  |  |
**full** | Option<**bool**> | If true always return the complete reaction list. |  |
**cursor** | Option<**String**> | Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more details. |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. |  |
**user** | Option<**String**> | Show reactions made by this user. Defaults to the authed user. |  |
**page** | Option<**i32**> |  |  |

### Return type

[**crate::models::ReactionsListSchema**](reactions_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove

> crate::models::ReactionsRemoveSchema remove(token, name, file_comment, timestamp, file, channel)


Removes a reaction from an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `reactions:write` | [required] |
**name** | **String** | Reaction (emoji) name. | [required] |
**file_comment** | Option<**String**> | File comment to remove reaction from. |  |
**timestamp** | Option<**String**> | Timestamp of the message to remove reaction from. |  |
**file** | Option<**String**> | File to remove reaction from. |  |
**channel** | Option<**String**> | Channel where the message to remove reaction from was posted. |  |

### Return type

[**crate::models::ReactionsRemoveSchema**](reactions_remove_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

