# \UsergroupsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create**](UsergroupsApi.md#create) | **post** /usergroups.create | 
[**disable**](UsergroupsApi.md#disable) | **post** /usergroups.disable | 
[**enable**](UsergroupsApi.md#enable) | **post** /usergroups.enable | 
[**list**](UsergroupsApi.md#list) | **get** /usergroups.list | 
[**update**](UsergroupsApi.md#update) | **post** /usergroups.update | 
[**users_list**](UsergroupsApi.md#users_list) | **get** /usergroups.users.list | 
[**users_update**](UsergroupsApi.md#users_update) | **post** /usergroups.users.update | 



## create

> crate::models::UsergroupsCreateSchema create(token, name, handle, description, channels, include_count)


Create a User Group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `usergroups:write` | [required] |
**name** | **String** | A name for the User Group. Must be unique among User Groups. | [required] |
**handle** | Option<**String**> | A mention handle. Must be unique among channels, users and User Groups. |  |
**description** | Option<**String**> | A short description of the User Group. |  |
**channels** | Option<**String**> | A comma separated string of encoded channel IDs for which the User Group uses as a default. |  |
**include_count** | Option<**bool**> | Include the number of users in each User Group. |  |

### Return type

[**crate::models::UsergroupsCreateSchema**](usergroups_create_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable

> crate::models::UsergroupsDisableSchema disable(token, usergroup, include_count)


Disable an existing User Group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `usergroups:write` | [required] |
**usergroup** | **String** | The encoded ID of the User Group to disable. | [required] |
**include_count** | Option<**bool**> | Include the number of users in the User Group. |  |

### Return type

[**crate::models::UsergroupsDisableSchema**](usergroups_disable_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable

> crate::models::UsergroupsEnableSchema enable(token, usergroup, include_count)


Enable a User Group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `usergroups:write` | [required] |
**usergroup** | **String** | The encoded ID of the User Group to enable. | [required] |
**include_count** | Option<**bool**> | Include the number of users in the User Group. |  |

### Return type

[**crate::models::UsergroupsEnableSchema**](usergroups_enable_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list

> crate::models::UsergroupsListSchema list(token, include_users, include_count, include_disabled)


List all User Groups for a team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `usergroups:read` | [required] |
**include_users** | Option<**bool**> | Include the list of users for each User Group. |  |
**include_count** | Option<**bool**> | Include the number of users in each User Group. |  |
**include_disabled** | Option<**bool**> | Include disabled User Groups. |  |

### Return type

[**crate::models::UsergroupsListSchema**](usergroups_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update

> crate::models::UsergroupsUpdateSchema update(token, usergroup, handle, description, channels, include_count, name)


Update an existing User Group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `usergroups:write` | [required] |
**usergroup** | **String** | The encoded ID of the User Group to update. | [required] |
**handle** | Option<**String**> | A mention handle. Must be unique among channels, users and User Groups. |  |
**description** | Option<**String**> | A short description of the User Group. |  |
**channels** | Option<**String**> | A comma separated string of encoded channel IDs for which the User Group uses as a default. |  |
**include_count** | Option<**bool**> | Include the number of users in the User Group. |  |
**name** | Option<**String**> | A name for the User Group. Must be unique among User Groups. |  |

### Return type

[**crate::models::UsergroupsUpdateSchema**](usergroups_update_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list

> crate::models::UsergroupsUsersListSchema users_list(token, usergroup, include_disabled)


List all users in a User Group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `usergroups:read` | [required] |
**usergroup** | **String** | The encoded ID of the User Group to update. | [required] |
**include_disabled** | Option<**bool**> | Allow results that involve disabled User Groups. |  |

### Return type

[**crate::models::UsergroupsUsersListSchema**](usergroups_users_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update

> crate::models::UsergroupsUsersUpdateSchema users_update(token, users, usergroup, include_count)


Update the list of users for a User Group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `usergroups:write` | [required] |
**users** | **String** | A comma separated string of encoded user IDs that represent the entire list of users for the User Group. | [required] |
**usergroup** | **String** | The encoded ID of the User Group to update. | [required] |
**include_count** | Option<**bool**> | Include the number of users in the User Group. |  |

### Return type

[**crate::models::UsergroupsUsersUpdateSchema**](usergroups_users_update_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

