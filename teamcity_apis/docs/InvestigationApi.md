# \InvestigationApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_investigation**](InvestigationApi.md#add_investigation) | **post** /app/rest/investigations | Create a new investigation.
[**add_multiple_investigations**](InvestigationApi.md#add_multiple_investigations) | **post** /app/rest/investigations/multiple | Create multiple new investigations.
[**delete_investigation**](InvestigationApi.md#delete_investigation) | **delete** /app/rest/investigations/{investigationLocator} | Delete investigation matching the locator.
[**get_all_investigations**](InvestigationApi.md#get_all_investigations) | **get** /app/rest/investigations | Get all investigations.
[**get_investigation**](InvestigationApi.md#get_investigation) | **get** /app/rest/investigations/{investigationLocator} | Get investigation matching the locator.
[**replace_investigation**](InvestigationApi.md#replace_investigation) | **put** /app/rest/investigations/{investigationLocator} | Update investigation matching the locator.



## add_investigation

> crate::models::Investigation add_investigation(fields, body)
Create a new investigation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**Investigation**](Investigation.md)> |  |  |

### Return type

[**crate::models::Investigation**](investigation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_multiple_investigations

> crate::models::Investigations add_multiple_investigations(fields, body)
Create multiple new investigations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**Investigations**](Investigations.md)> |  |  |

### Return type

[**crate::models::Investigations**](investigations.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_investigation

> delete_investigation(investigation_locator)
Delete investigation matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**investigation_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_investigations

> crate::models::Investigations get_all_investigations(locator, fields)
Get all investigations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Investigations**](investigations.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_investigation

> crate::models::Investigation get_investigation(investigation_locator, fields)
Get investigation matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**investigation_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Investigation**](investigation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_investigation

> crate::models::Investigation replace_investigation(investigation_locator, fields, body)
Update investigation matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**investigation_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Investigation**](Investigation.md)> |  |  |

### Return type

[**crate::models::Investigation**](investigation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

