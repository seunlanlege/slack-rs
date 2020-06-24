# \AppsPermissionsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**permissions_info**](AppsPermissionsApi.md#permissions_info) | **get** /apps.permissions.info | 
[**permissions_request**](AppsPermissionsApi.md#permissions_request) | **get** /apps.permissions.request | 



## permissions_info

> crate::models::AppsPermissionsInfoSchema permissions_info(token)


Returns list of permissions this app has on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `none` |  |

### Return type

[**crate::models::AppsPermissionsInfoSchema**](apps_permissions_info_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permissions_request

> crate::models::AppsPermissionsRequestSchema permissions_request(scopes, token, trigger_id)


Allows an app to request additional scopes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scopes** | **String** | A comma separated list of scopes to request for | [required] |
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**trigger_id** | **String** | Token used to trigger the permissions API | [required] |

### Return type

[**crate::models::AppsPermissionsRequestSchema**](apps_permissions_request_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

