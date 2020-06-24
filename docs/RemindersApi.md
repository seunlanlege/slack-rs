# \RemindersApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add**](RemindersApi.md#add) | **post** /reminders.add | 
[**complete**](RemindersApi.md#complete) | **post** /reminders.complete | 
[**delete**](RemindersApi.md#delete) | **post** /reminders.delete | 
[**info**](RemindersApi.md#info) | **get** /reminders.info | 
[**list**](RemindersApi.md#list) | **get** /reminders.list | 



## add

> crate::models::RemindersAddSchema add(token, text, user, time)


Creates a reminder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `reminders:write` |  |
**text** | Option<**String**> | The content of the reminder |  |
**user** | Option<**String**> | The user who will receive the reminder. If no user is specified, the reminder will go to user who created it. |  |
**time** | Option<**String**> | When this reminder should happen: the Unix timestamp (up to five years from now), the number of seconds until the reminder (if within 24 hours), or a natural language description (Ex. \\\"in 15 minutes,\\\" or \\\"every Thursday\\\") |  |

### Return type

[**crate::models::RemindersAddSchema**](reminders_add_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete

> crate::models::RemindersCompleteSchema complete(token, reminder)


Marks a reminder as complete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `reminders:write` |  |
**reminder** | Option<**String**> | The ID of the reminder to be marked as complete |  |

### Return type

[**crate::models::RemindersCompleteSchema**](reminders_complete_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete

> crate::models::RemindersDeleteSchema delete(token, reminder)


Deletes a reminder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `reminders:write` |  |
**reminder** | Option<**String**> | The ID of the reminder |  |

### Return type

[**crate::models::RemindersDeleteSchema**](reminders_delete_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info

> crate::models::RemindersInfoSchema info(token, reminder)


Gets information about a reminder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `reminders:read` |  |
**reminder** | Option<**String**> | The ID of the reminder |  |

### Return type

[**crate::models::RemindersInfoSchema**](reminders_info_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> crate::models::RemindersListSchema list(token)


Lists all reminders created by or for a given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `reminders:read` |  |

### Return type

[**crate::models::RemindersListSchema**](reminders_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

