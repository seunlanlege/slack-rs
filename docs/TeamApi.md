# \TeamApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**access_logs**](TeamApi.md#access_logs) | **get** /team.accessLogs | 
[**billable_info**](TeamApi.md#billable_info) | **get** /team.billableInfo | 
[**info**](TeamApi.md#info) | **get** /team.info | 
[**integration_logs**](TeamApi.md#integration_logs) | **get** /team.integrationLogs | 
[**profile_get**](TeamApi.md#profile_get) | **get** /team.profile.get | 



## access_logs

> crate::models::TeamAccessLogsSchema access_logs(token, count, page, before)


Gets the access logs for the current team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin` | [required] |
**count** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |
**before** | Option<**String**> | End of time range of logs to include in results (inclusive). |  |

### Return type

[**crate::models::TeamAccessLogsSchema**](team_accessLogs_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billable_info

> crate::models::TeamBillableInfoSchema billable_info(token, user)


Gets billable users information for the current team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin` | [required] |
**user** | Option<**String**> | A user to retrieve the billable information for. Defaults to all users. |  |

### Return type

[**crate::models::TeamBillableInfoSchema**](team_billableInfo_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info

> crate::models::TeamInfoSchema info(token, team)


Gets information about the current team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `team:read` | [required] |
**team** | Option<**String**> | Team to get info on, if omitted, will return information about the current team. Will only return team that the authenticated token is allowed to see through external shared channels |  |

### Return type

[**crate::models::TeamInfoSchema**](team_info_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integration_logs

> crate::models::TeamIntegrationLogsSchema integration_logs(token, count, change_type, app_id, user, service_id, page)


Gets the integration logs for the current team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin` | [required] |
**count** | Option<**i32**> |  |  |
**change_type** | Option<**String**> | Filter logs with this change type. Defaults to all logs. |  |
**app_id** | Option<**i32**> | Filter logs to this Slack app. Defaults to all logs. |  |
**user** | Option<**String**> | Filter logs generated by this user’s actions. Defaults to all logs. |  |
**service_id** | Option<**i32**> | Filter logs to this service. Defaults to all logs. |  |
**page** | Option<**String**> |  |  |

### Return type

[**crate::models::TeamIntegrationLogsSchema**](team_integrationLogs_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profile_get

> crate::models::TeamProfileGetSuccessSchema profile_get(token, visibility)


Retrieve a team's profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users.profile:read` | [required] |
**visibility** | Option<**String**> | Filter by visibility. |  |

### Return type

[**crate::models::TeamProfileGetSuccessSchema**](team_profile_get_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
