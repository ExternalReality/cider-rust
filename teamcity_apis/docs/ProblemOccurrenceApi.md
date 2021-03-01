# \ProblemOccurrenceApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_build_problem_occurrences**](ProblemOccurrenceApi.md#get_all_build_problem_occurrences) | **get** /app/rest/problemOccurrences | Get all build problem occurrences.
[**get_build_problem_occurrence**](ProblemOccurrenceApi.md#get_build_problem_occurrence) | **get** /app/rest/problemOccurrences/{problemLocator} | Get a matching build problem occurrence.



## get_all_build_problem_occurrences

> crate::models::ProblemOccurrences get_all_build_problem_occurrences(locator, fields)
Get all build problem occurrences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProblemOccurrences**](problemOccurrences.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_problem_occurrence

> crate::models::ProblemOccurrence get_build_problem_occurrence(problem_locator, fields)
Get a matching build problem occurrence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**problem_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProblemOccurrence**](problemOccurrence.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

