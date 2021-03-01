# \TestOccurrenceApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_test_occurrences**](TestOccurrenceApi.md#get_all_test_occurrences) | **get** /app/rest/testOccurrences | Get all test occurrences.
[**get_test_occurrence**](TestOccurrenceApi.md#get_test_occurrence) | **get** /app/rest/testOccurrences/{testLocator} | Get a matching test occurrence.



## get_all_test_occurrences

> crate::models::TestOccurrences get_all_test_occurrences(locator, fields)
Get all test occurrences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TestOccurrences**](testOccurrences.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_test_occurrence

> crate::models::TestOccurrence get_test_occurrence(test_locator, fields)
Get a matching test occurrence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TestOccurrence**](testOccurrence.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

