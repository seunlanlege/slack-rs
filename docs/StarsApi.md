# \StarsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add**](StarsApi.md#add) | **post** /stars.add | 
[**list**](StarsApi.md#list) | **get** /stars.list | 
[**remove**](StarsApi.md#remove) | **post** /stars.remove | 



## add

> crate::models::StarsAddSchema add(token, file_comment, timestamp, file, channel)


Adds a star to an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `stars:write` | [required] |
**file_comment** | Option<**String**> | File comment to add star to. |  |
**timestamp** | Option<**String**> | Timestamp of the message to add star to. |  |
**file** | Option<**String**> | File to add star to. |  |
**channel** | Option<**String**> | Channel to add star to, or channel where the message to add star to was posted (used with `timestamp`). |  |

### Return type

[**crate::models::StarsAddSchema**](stars_add_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> crate::models::StarsListSchema list(count, cursor, token, limit, page)


Lists stars for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**String**> |  |  |
**cursor** | Option<**String**> | Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more details. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `stars:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. |  |
**page** | Option<**String**> |  |  |

### Return type

[**crate::models::StarsListSchema**](stars_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove

> crate::models::StarsRemoveSchema remove(token, file_comment, timestamp, channel, file)


Removes a star from an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `stars:write` |  |
**file_comment** | Option<**String**> | File comment to remove star from. |  |
**timestamp** | Option<**f32**> | Timestamp of the message to remove star from. |  |
**channel** | Option<**String**> | Channel to remove star from, or channel where the message to remove star from was posted (used with `timestamp`). |  |
**file** | Option<**String**> | File to remove star from. |  |

### Return type

[**crate::models::StarsRemoveSchema**](stars_remove_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

