# \BuildQueueApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_build_to_queue**](BuildQueueApi.md#add_build_to_queue) | **post** /app/rest/buildQueue | Add a new build to the queue.
[**add_tags_to_build_of_build_queue**](BuildQueueApi.md#add_tags_to_build_of_build_queue) | **post** /app/rest/buildQueue/{buildLocator}/tags | Add tags to the matching build.
[**cancel_queued_build**](BuildQueueApi.md#cancel_queued_build) | **post** /app/rest/buildQueue/{queuedBuildLocator} | Cancel a queued matching build.
[**delete_all_queued_builds**](BuildQueueApi.md#delete_all_queued_builds) | **delete** /app/rest/buildQueue | Delete all queued builds.
[**delete_queued_build**](BuildQueueApi.md#delete_queued_build) | **delete** /app/rest/buildQueue/{queuedBuildLocator} | Delete a queued matching build.
[**get_all_queued_builds**](BuildQueueApi.md#get_all_queued_builds) | **get** /app/rest/buildQueue | Get all queued builds.
[**get_compatible_agents_for_build**](BuildQueueApi.md#get_compatible_agents_for_build) | **get** /app/rest/buildQueue/{queuedBuildLocator}/compatibleAgents | Get compatible agents for a queued matching build.
[**get_queued_build**](BuildQueueApi.md#get_queued_build) | **get** /app/rest/buildQueue/{queuedBuildLocator} | Get a queued matching build.
[**get_queued_build_position**](BuildQueueApi.md#get_queued_build_position) | **get** /app/rest/buildQueue/order/{queuePosition} | Get the queue position of a queued matching build.
[**get_queued_build_tags**](BuildQueueApi.md#get_queued_build_tags) | **get** /app/rest/buildQueue/{buildLocator}/tags | Get tags of the queued matching build.
[**set_queued_build_position**](BuildQueueApi.md#set_queued_build_position) | **put** /app/rest/buildQueue/order/{queuePosition} | Update the queue position of a queued matching build.
[**set_queued_builds_order**](BuildQueueApi.md#set_queued_builds_order) | **put** /app/rest/buildQueue/order | Update the build queue order.



## add_build_to_queue

> crate::models::Build add_build_to_queue(move_to_top, body)
Add a new build to the queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**move_to_top** | Option<**bool**> |  |  |
**body** | Option<[**Build**](Build.md)> |  |  |

### Return type

[**crate::models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_tags_to_build_of_build_queue

> add_tags_to_build_of_build_queue(build_locator, body)
Add tags to the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**body** | Option<[**Tags**](Tags.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_queued_build

> crate::models::Build cancel_queued_build(queued_build_locator, body)
Cancel a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queued_build_locator** | **String** |  | [required] |
**body** | Option<[**BuildCancelRequest**](BuildCancelRequest.md)> |  |  |

### Return type

[**crate::models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_queued_builds

> delete_all_queued_builds(locator, fields)
Delete all queued builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_queued_build

> delete_queued_build(queued_build_locator)
Delete a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queued_build_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_queued_builds

> crate::models::Builds get_all_queued_builds(locator, fields)
Get all queued builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Builds**](builds.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_compatible_agents_for_build

> crate::models::Agents get_compatible_agents_for_build(queued_build_locator, fields)
Get compatible agents for a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queued_build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Agents**](agents.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queued_build

> crate::models::Build get_queued_build(queued_build_locator, fields)
Get a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queued_build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queued_build_position

> crate::models::Build get_queued_build_position(queue_position, fields)
Get the queue position of a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_position** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queued_build_tags

> crate::models::Tags get_queued_build_tags(build_locator, locator, fields)
Get tags of the queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Tags**](tags.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_queued_build_position

> crate::models::Build set_queued_build_position(queue_position, fields, body)
Update the queue position of a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_position** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Build**](Build.md)> |  |  |

### Return type

[**crate::models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_queued_builds_order

> crate::models::Builds set_queued_builds_order(fields, body)
Update the build queue order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**Builds**](Builds.md)> |  |  |

### Return type

[**crate::models::Builds**](builds.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

