# \ChangeApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_changes**](ChangeApi.md#get_all_changes) | **get** /app/rest/changes | Get all changes.
[**get_change**](ChangeApi.md#get_change) | **get** /app/rest/changes/{changeLocator} | Get change matching the locator.
[**get_change_attributes**](ChangeApi.md#get_change_attributes) | **get** /app/rest/changes/{changeLocator}/attributes | Get attributes of the matching change.
[**get_change_duplicates**](ChangeApi.md#get_change_duplicates) | **get** /app/rest/changes/{changeLocator}/duplicates | Get duplicates of the matching change.
[**get_change_field**](ChangeApi.md#get_change_field) | **get** /app/rest/changes/{changeLocator}/{field} | Get a field of the matching change.
[**get_change_first_builds**](ChangeApi.md#get_change_first_builds) | **get** /app/rest/changes/{changeLocator}/firstBuilds | Get first builds of the matching change.
[**get_change_issue**](ChangeApi.md#get_change_issue) | **get** /app/rest/changes/{changeLocator}/issues | Get issues of the matching change.
[**get_change_parent_changes**](ChangeApi.md#get_change_parent_changes) | **get** /app/rest/changes/{changeLocator}/parentChanges | Get parent changes of the matching change.
[**get_change_parent_revisions**](ChangeApi.md#get_change_parent_revisions) | **get** /app/rest/changes/{changeLocator}/parentRevisions | Get parent revisions of the matching change.
[**get_change_related_build_types**](ChangeApi.md#get_change_related_build_types) | **get** /app/rest/changes/{changeLocator}/buildTypes | Get build configurations related to the matching change.
[**get_change_vcs_root**](ChangeApi.md#get_change_vcs_root) | **get** /app/rest/changes/{changeLocator}/vcsRootInstance | Get a VCS root instance of the matching change.



## get_all_changes

> crate::models::Changes get_all_changes(locator, fields)
Get all changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Changes**](changes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_change

> crate::models::Change get_change(change_locator, fields)
Get change matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Change**](change.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_change_attributes

> crate::models::Entries get_change_attributes(change_locator, fields)
Get attributes of the matching change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Entries**](entries.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_change_duplicates

> crate::models::Changes get_change_duplicates(change_locator, fields)
Get duplicates of the matching change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Changes**](changes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_change_field

> String get_change_field(change_locator, field)
Get a field of the matching change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_change_first_builds

> crate::models::Builds get_change_first_builds(change_locator, fields)
Get first builds of the matching change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Builds**](builds.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_change_issue

> crate::models::Issues get_change_issue(change_locator)
Get issues of the matching change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_locator** | **String** |  | [required] |

### Return type

[**crate::models::Issues**](issues.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_change_parent_changes

> crate::models::Changes get_change_parent_changes(change_locator, fields)
Get parent changes of the matching change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Changes**](changes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_change_parent_revisions

> crate::models::Items get_change_parent_revisions(change_locator)
Get parent revisions of the matching change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_locator** | **String** |  | [required] |

### Return type

[**crate::models::Items**](items.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_change_related_build_types

> crate::models::BuildTypes get_change_related_build_types(change_locator, fields)
Get build configurations related to the matching change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BuildTypes**](buildTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_change_vcs_root

> crate::models::VcsRootInstance get_change_vcs_root(change_locator, fields)
Get a VCS root instance of the matching change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::VcsRootInstance**](vcs-root-instance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

