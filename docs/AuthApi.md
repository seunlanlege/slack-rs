# \AuthApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**revoke**](AuthApi.md#revoke) | **get** /auth.revoke | 
[**test**](AuthApi.md#test) | **get** /auth.test | 



## revoke

> crate::models::AuthRevokeSchema revoke(token, test)


Revokes a token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**test** | Option<**bool**> | Setting this parameter to `1` triggers a _testing mode_ where the specified token will not actually be revoked. |  |

### Return type

[**crate::models::AuthRevokeSchema**](auth_revoke_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test

> crate::models::AuthTestSuccessSchema test(token)


Checks authentication & identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |

### Return type

[**crate::models::AuthTestSuccessSchema**](auth_test_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

