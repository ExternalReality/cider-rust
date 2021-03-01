# \RootApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_version**](RootApi.md#get_api_version) | **get** /app/rest/apiVersion | Get the API version.
[**get_plugin_info**](RootApi.md#get_plugin_info) | **get** /app/rest/info | Get the plugin info.
[**get_root_endpoints_of_root**](RootApi.md#get_root_endpoints_of_root) | **get** /app/rest | Get root endpoints.
[**get_version**](RootApi.md#get_version) | **get** /app/rest/version | Get the TeamCity server version.



## get_api_version

> String get_api_version()
Get the API version.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_info

> crate::models::Plugin get_plugin_info(fields)
Get the plugin info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Plugin**](plugin.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_root_endpoints_of_root

> String get_root_endpoints_of_root()
Get root endpoints.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version

> String get_version()
Get the TeamCity server version.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

