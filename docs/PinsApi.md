# \PinsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add**](PinsApi.md#add) | **post** /pins.add | 
[**list**](PinsApi.md#list) | **get** /pins.list | 
[**remove**](PinsApi.md#remove) | **post** /pins.remove | 



## add

> crate::models::PinsAddSchema add(token, timestamp, channel)


Pins an item to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `pins:write` |  |
**timestamp** | Option<**f32**> | Timestamp of the message to pin. |  |
**channel** | Option<**String**> | Channel to pin the item in. |  |

### Return type

[**crate::models::PinsAddSchema**](pins_add_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> serde_json::Value list(token, channel)


Lists items pinned to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `pins:read` |  |
**channel** | Option<**String**> | Channel to get pinned items for. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove

> crate::models::PinsRemoveSchema remove(token, file_comment, timestamp, file, channel)


Un-pins an item from a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `pins:write` |  |
**file_comment** | Option<**String**> | File comment to un-pin. |  |
**timestamp** | Option<**f32**> | Timestamp of the message to un-pin. |  |
**file** | Option<**String**> | File to un-pin. |  |
**channel** | Option<**String**> | Channel where the item is pinned to. |  |

### Return type

[**crate::models::PinsRemoveSchema**](pins_remove_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

