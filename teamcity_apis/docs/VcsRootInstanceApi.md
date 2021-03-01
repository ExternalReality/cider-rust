# \VcsRootInstanceApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_vcs_root_instance_field**](VcsRootInstanceApi.md#delete_vcs_root_instance_field) | **delete** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/{field} | Remove a field of the matching VCS root instance.
[**delete_vcs_root_instance_repository_state**](VcsRootInstanceApi.md#delete_vcs_root_instance_repository_state) | **delete** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/repositoryState | Delete the last repository state of the matching VCS root instance.
[**download_file**](VcsRootInstanceApi.md#download_file) | **get** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/files/latest/files{path} | Download specific file.
[**get_all_vcs_root_instances**](VcsRootInstanceApi.md#get_all_vcs_root_instances) | **get** /app/rest/vcs-root-instances | Get all VCS root instances.
[**get_file_metadata**](VcsRootInstanceApi.md#get_file_metadata) | **get** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/files/latest/metadata{path} | Get metadata of specific file.
[**get_files_list**](VcsRootInstanceApi.md#get_files_list) | **get** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/files/latest | List all files.
[**get_files_list_for_subpath**](VcsRootInstanceApi.md#get_files_list_for_subpath) | **get** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/files/latest/{path} | List files under this path.
[**get_vcs_root_instance**](VcsRootInstanceApi.md#get_vcs_root_instance) | **get** /app/rest/vcs-root-instances/{vcsRootInstanceLocator} | Get VCS root instance matching the locator.
[**get_vcs_root_instance_creation_date**](VcsRootInstanceApi.md#get_vcs_root_instance_creation_date) | **get** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/repositoryState/creationDate | Get the creation date of the matching VCS root instance.
[**get_vcs_root_instance_field**](VcsRootInstanceApi.md#get_vcs_root_instance_field) | **get** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/{field} | Get a field of the matching VCS root instance.
[**get_vcs_root_instance_properties**](VcsRootInstanceApi.md#get_vcs_root_instance_properties) | **get** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/properties | Get all properties of the matching VCS root instance.
[**get_vcs_root_instance_repository_state**](VcsRootInstanceApi.md#get_vcs_root_instance_repository_state) | **get** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/repositoryState | Get the repository state of the matching VCS root instance.
[**get_zipped_file**](VcsRootInstanceApi.md#get_zipped_file) | **get** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/files/latest/archived{path} | Get specific file zipped.
[**request_pending_changes_check**](VcsRootInstanceApi.md#request_pending_changes_check) | **post** /app/rest/vcs-root-instances/checkingForChangesQueue | Check for the pending changes for all VCS root instances.
[**set_vcs_root_instance_field**](VcsRootInstanceApi.md#set_vcs_root_instance_field) | **put** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/{field} | Get a field of the matching VCS root instance.
[**set_vcs_root_instance_repository_state**](VcsRootInstanceApi.md#set_vcs_root_instance_repository_state) | **put** /app/rest/vcs-root-instances/{vcsRootInstanceLocator}/repositoryState | Update the repository state of the matching VCS root instance.
[**trigger_commit_hook_notification**](VcsRootInstanceApi.md#trigger_commit_hook_notification) | **post** /app/rest/vcs-root-instances/commitHookNotification | Send the commit hook notification.



## delete_vcs_root_instance_field

> delete_vcs_root_instance_field(vcs_root_instance_locator, field)
Remove a field of the matching VCS root instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_instance_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vcs_root_instance_repository_state

> delete_vcs_root_instance_repository_state(vcs_root_instance_locator)
Delete the last repository state of the matching VCS root instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_instance_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_file

> download_file(path, vcs_root_instance_locator)
Download specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**vcs_root_instance_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_vcs_root_instances

> crate::models::VcsRootInstances get_all_vcs_root_instances(locator, fields)
Get all VCS root instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::VcsRootInstances**](vcs-root-instances.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_metadata

> std::path::PathBuf get_file_metadata(path, vcs_root_instance_locator, fields)
Get metadata of specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**vcs_root_instance_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_list

> crate::models::Files get_files_list(vcs_root_instance_locator, base_path, locator, fields)
List all files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_instance_locator** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Files**](files.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_list_for_subpath

> crate::models::Files get_files_list_for_subpath(path, vcs_root_instance_locator, base_path, locator, fields)
List files under this path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**vcs_root_instance_locator** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Files**](files.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root_instance

> crate::models::VcsRootInstance get_vcs_root_instance(vcs_root_instance_locator, fields)
Get VCS root instance matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_instance_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::VcsRootInstance**](vcs-root-instance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root_instance_creation_date

> String get_vcs_root_instance_creation_date(vcs_root_instance_locator)
Get the creation date of the matching VCS root instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_instance_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root_instance_field

> String get_vcs_root_instance_field(vcs_root_instance_locator, field)
Get a field of the matching VCS root instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_instance_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root_instance_properties

> crate::models::Properties get_vcs_root_instance_properties(vcs_root_instance_locator, fields)
Get all properties of the matching VCS root instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_instance_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root_instance_repository_state

> crate::models::Entries get_vcs_root_instance_repository_state(vcs_root_instance_locator, fields)
Get the repository state of the matching VCS root instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_instance_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Entries**](entries.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zipped_file

> get_zipped_file(path, vcs_root_instance_locator, base_path, locator, name)
Get specific file zipped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**vcs_root_instance_locator** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_pending_changes_check

> crate::models::VcsRootInstances request_pending_changes_check(locator, requestor, fields)
Check for the pending changes for all VCS root instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**requestor** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::VcsRootInstances**](vcs-root-instances.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_vcs_root_instance_field

> String set_vcs_root_instance_field(vcs_root_instance_locator, field, body)
Get a field of the matching VCS root instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_instance_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_vcs_root_instance_repository_state

> crate::models::Entries set_vcs_root_instance_repository_state(vcs_root_instance_locator, fields, body)
Update the repository state of the matching VCS root instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_instance_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Entries**](Entries.md)> |  |  |

### Return type

[**crate::models::Entries**](entries.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_commit_hook_notification

> trigger_commit_hook_notification(locator, ok_on_nothing_found)
Send the commit hook notification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**ok_on_nothing_found** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

