# \MuteApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_muted_tests**](MuteApi.md#get_all_muted_tests) | **get** /app/rest/mutes | Get all muted tests.
[**get_muted_test**](MuteApi.md#get_muted_test) | **get** /app/rest/mutes/{muteLocator} | Get a muted test.
[**mute_multiple_tests**](MuteApi.md#mute_multiple_tests) | **post** /app/rest/mutes/multiple | Mute multiple tests.
[**mute_test**](MuteApi.md#mute_test) | **post** /app/rest/mutes | Mute a test.
[**unmute_test**](MuteApi.md#unmute_test) | **delete** /app/rest/mutes/{muteLocator} | Unmute the matching test.



## get_all_muted_tests

> crate::models::Mutes get_all_muted_tests(locator, fields)
Get all muted tests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Mutes**](mutes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_muted_test

> crate::models::Mute get_muted_test(mute_locator, fields)
Get a muted test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mute_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Mute**](mute.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mute_multiple_tests

> crate::models::Mutes mute_multiple_tests(fields, body)
Mute multiple tests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**Mutes**](Mutes.md)> |  |  |

### Return type

[**crate::models::Mutes**](mutes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mute_test

> crate::models::Mute mute_test(fields, body)
Mute a test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**Mute**](Mute.md)> |  |  |

### Return type

[**crate::models::Mute**](mute.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unmute_test

> unmute_test(mute_locator, body)
Unmute the matching test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mute_locator** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

