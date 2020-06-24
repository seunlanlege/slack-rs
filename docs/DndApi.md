# \DndApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**end_dnd**](DndApi.md#end_dnd) | **post** /dnd.endDnd | 
[**end_snooze**](DndApi.md#end_snooze) | **post** /dnd.endSnooze | 
[**info**](DndApi.md#info) | **get** /dnd.info | 
[**set_snooze**](DndApi.md#set_snooze) | **post** /dnd.setSnooze | 
[**team_info**](DndApi.md#team_info) | **get** /dnd.teamInfo | 



## end_dnd

> crate::models::DndEndDndSchema end_dnd(token)


Ends the current user's Do Not Disturb session immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `dnd:write` | [required] |

### Return type

[**crate::models::DndEndDndSchema**](dnd_endDnd_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## end_snooze

> crate::models::DndEndSnoozeSchema end_snooze(token)


Ends the current user's snooze mode immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `dnd:write` | [required] |

### Return type

[**crate::models::DndEndSnoozeSchema**](dnd_endSnooze_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info

> crate::models::DndInfoSchema info(token, user)


Retrieves a user's current Do Not Disturb status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `dnd:read` |  |
**user** | Option<**String**> | User to fetch status for (defaults to current user) |  |

### Return type

[**crate::models::DndInfoSchema**](dnd_info_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_snooze

> crate::models::DndSetSnoozeSchema set_snooze(num_minutes, token)


Turns on Do Not Disturb mode for the current user, or changes its duration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**num_minutes** | **String** | Number of minutes, from now, to snooze until. | [required] |
**token** | **String** | Authentication token. Requires scope: `dnd:write` | [required] |

### Return type

[**crate::models::DndSetSnoozeSchema**](dnd_setSnooze_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_info

> crate::models::DndTeamInfoSuccessSchema team_info(token, users)


Retrieves the Do Not Disturb status for up to 50 users on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `dnd:read` |  |
**users** | Option<**String**> | Comma-separated list of users to fetch Do Not Disturb status for |  |

### Return type

[**crate::models::DndTeamInfoSuccessSchema**](dnd_teamInfo_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

