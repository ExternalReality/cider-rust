# \CloudInstanceApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_cloud_images**](CloudInstanceApi.md#get_all_cloud_images) | **get** /app/rest/cloud/images | Get all cloud images.
[**get_all_cloud_instances**](CloudInstanceApi.md#get_all_cloud_instances) | **get** /app/rest/cloud/instances | Get all cloud instances.
[**get_all_cloud_profiles**](CloudInstanceApi.md#get_all_cloud_profiles) | **get** /app/rest/cloud/profiles | Get all cloud profiles.
[**get_cloud_image**](CloudInstanceApi.md#get_cloud_image) | **get** /app/rest/cloud/images/{imageLocator} | Get cloud image matching the locator.
[**get_cloud_instance**](CloudInstanceApi.md#get_cloud_instance) | **get** /app/rest/cloud/instances/{instanceLocator} | Get cloud instance matching the locator.
[**get_cloud_profile**](CloudInstanceApi.md#get_cloud_profile) | **get** /app/rest/cloud/profiles/{profileLocator} | Get cloud profile matching the locator.
[**start_instance**](CloudInstanceApi.md#start_instance) | **post** /app/rest/cloud/instances | Start a new cloud instance.
[**stop_instance**](CloudInstanceApi.md#stop_instance) | **delete** /app/rest/cloud/instances/{instanceLocator} | Stop cloud instance matching the locator.



## get_all_cloud_images

> crate::models::CloudImages get_all_cloud_images(locator, fields)
Get all cloud images.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CloudImages**](cloudImages.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_cloud_instances

> crate::models::CloudInstances get_all_cloud_instances(locator, fields)
Get all cloud instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CloudInstances**](cloudInstances.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_cloud_profiles

> crate::models::CloudProfiles get_all_cloud_profiles(locator, fields)
Get all cloud profiles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CloudProfiles**](cloudProfiles.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_image

> crate::models::CloudImage get_cloud_image(image_locator, fields)
Get cloud image matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CloudImage**](cloudImage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_instance

> crate::models::CloudInstance get_cloud_instance(instance_locator, fields)
Get cloud instance matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CloudInstance**](cloudInstance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_profile

> crate::models::CloudProfile get_cloud_profile(profile_locator, fields)
Get cloud profile matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CloudProfile**](cloudProfile.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_instance

> start_instance(fields, body)
Start a new cloud instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**CloudInstance**](CloudInstance.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_instance

> stop_instance(instance_locator)
Stop cloud instance matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

