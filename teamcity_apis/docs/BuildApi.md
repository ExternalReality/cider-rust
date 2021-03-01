# \BuildApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_build_vcs_label**](BuildApi.md#add_build_vcs_label) | **post** /app/rest/builds/{buildLocator}/vcsLabels | Add a VCS label to the matching build.
[**add_log_message_to_build**](BuildApi.md#add_log_message_to_build) | **post** /app/rest/builds/{buildLocator}/log | Adds a message to the build log. Service messages are accepted.
[**add_problem_to_build**](BuildApi.md#add_problem_to_build) | **post** /app/rest/builds/{buildLocator}/problemOccurrences | Add a build problem to the matching build.
[**add_tags_to_build**](BuildApi.md#add_tags_to_build) | **post** /app/rest/builds/{buildLocator}/tags | Add tags to the matching build.
[**add_tags_to_multiple_builds**](BuildApi.md#add_tags_to_multiple_builds) | **post** /app/rest/builds/multiple/{buildLocator}/tags | Add tags to multiple matching builds.
[**delete_build**](BuildApi.md#delete_build) | **delete** /app/rest/builds/{buildLocator} | Delete build matching the locator.
[**delete_build_comment**](BuildApi.md#delete_build_comment) | **delete** /app/rest/builds/{buildLocator}/comment | Remove the build comment matching the locator.
[**delete_multiple_build_comments**](BuildApi.md#delete_multiple_build_comments) | **delete** /app/rest/builds/multiple/{buildLocator}/comment | Delete comments of multiple matching builds.
[**delete_multiple_builds**](BuildApi.md#delete_multiple_builds) | **delete** /app/rest/builds/multiple/{buildLocator} | Delete multiple builds matching the locator.
[**download_file_of_build**](BuildApi.md#download_file_of_build) | **get** /app/rest/builds/{buildLocator}/artifacts/files{path} | Download specific file.
[**get_aggregated_build_status**](BuildApi.md#get_aggregated_build_status) | **get** /app/rest/builds/aggregated/{buildLocator}/status | Get the build status of aggregated matching builds.
[**get_aggregated_build_status_icon**](BuildApi.md#get_aggregated_build_status_icon) | **get** /app/rest/builds/aggregated/{buildLocator}/statusIcon{suffix} | Get the status icon (in specified format) of aggregated matching builds.
[**get_all_builds**](BuildApi.md#get_all_builds) | **get** /app/rest/builds | Get all builds.
[**get_artifact_dependency_changes**](BuildApi.md#get_artifact_dependency_changes) | **get** /app/rest/builds/{buildLocator}/artifactDependencyChanges | Get artifact dependency changes of the matching build.
[**get_artifacts_directory**](BuildApi.md#get_artifacts_directory) | **get** /app/rest/builds/{buildLocator}/artifactsDirectory | Get the artifacts' directory of the matching build.
[**get_build**](BuildApi.md#get_build) | **get** /app/rest/builds/{buildLocator} | Get build matching the locator.
[**get_build_actual_parameters**](BuildApi.md#get_build_actual_parameters) | **get** /app/rest/builds/{buildLocator}/resulting-properties | Get actual build parameters of the matching build.
[**get_build_field**](BuildApi.md#get_build_field) | **get** /app/rest/builds/{buildLocator}/{field} | Get a field of the matching build.
[**get_build_finish_date**](BuildApi.md#get_build_finish_date) | **get** /app/rest/builds/{buildLocator}/finishDate | Get the finish date of the matching build.
[**get_build_number**](BuildApi.md#get_build_number) | **get** /app/rest/builds/{buildLocator}/number | Get the number of the matching build.
[**get_build_pin_info**](BuildApi.md#get_build_pin_info) | **get** /app/rest/builds/{buildLocator}/pinInfo | Check if the matching build is pinned.
[**get_build_problems**](BuildApi.md#get_build_problems) | **get** /app/rest/builds/{buildLocator}/problemOccurrences | Get build problems of the matching build.
[**get_build_related_issues**](BuildApi.md#get_build_related_issues) | **get** /app/rest/builds/{buildLocator}/relatedIssues | Get related issues of the matching build.
[**get_build_resolved**](BuildApi.md#get_build_resolved) | **get** /app/rest/builds/{buildLocator}/resolved/{value} | Get the resolvement status of the matching build.
[**get_build_resulting_properties**](BuildApi.md#get_build_resulting_properties) | **get** /app/rest/builds/{buildLocator}/resulting-properties/{propertyName} | Update a build parameter of the matching build.
[**get_build_source_file**](BuildApi.md#get_build_source_file) | **get** /app/rest/builds/{buildLocator}/sources/files/{fileName} | Get a source file of the matching build.
[**get_build_statistic_value**](BuildApi.md#get_build_statistic_value) | **get** /app/rest/builds/{buildLocator}/statistics/{name} | Get a statistical value of the matching build.
[**get_build_statistic_values**](BuildApi.md#get_build_statistic_values) | **get** /app/rest/builds/{buildLocator}/statistics | Get all statistical values of the matching build.
[**get_build_status_icon**](BuildApi.md#get_build_status_icon) | **get** /app/rest/builds/{buildLocator}/statusIcon{suffix} | Get the status icon (in specified format) of the matching build.
[**get_build_status_text**](BuildApi.md#get_build_status_text) | **get** /app/rest/builds/{buildLocator}/statusText | Get the build status text of the matching build.
[**get_build_tags**](BuildApi.md#get_build_tags) | **get** /app/rest/builds/{buildLocator}/tags | Get tags of the matching build.
[**get_build_test_occurrences**](BuildApi.md#get_build_test_occurrences) | **get** /app/rest/builds/{buildLocator}/testOccurrences | Get test occurrences of the matching build.
[**get_build_vcs_labels**](BuildApi.md#get_build_vcs_labels) | **get** /app/rest/builds/{buildLocator}/vcsLabels | Get VCS labels of the matching build.
[**get_canceled_info**](BuildApi.md#get_canceled_info) | **get** /app/rest/builds/{buildLocator}/canceledInfo | Check if the matching build is canceled.
[**get_file_metadata_of_build**](BuildApi.md#get_file_metadata_of_build) | **get** /app/rest/builds/{buildLocator}/artifacts/metadata{path} | Get metadata of specific file.
[**get_files_list_for_subpath_of_build**](BuildApi.md#get_files_list_for_subpath_of_build) | **get** /app/rest/builds/{buildLocator}/artifacts/{path} | List files under this path.
[**get_files_list_of_build**](BuildApi.md#get_files_list_of_build) | **get** /app/rest/builds/{buildLocator}/artifacts | List all files.
[**get_multiple_builds**](BuildApi.md#get_multiple_builds) | **get** /app/rest/builds/multiple/{buildLocator} | Get multiple builds matching the locator.
[**get_zipped_file_of_build**](BuildApi.md#get_zipped_file_of_build) | **get** /app/rest/builds/{buildLocator}/artifacts/archived{path} | Get specific file zipped.
[**mark_build_as_running**](BuildApi.md#mark_build_as_running) | **put** /app/rest/builds/{buildLocator}/runningData | Starts the queued build as an agent-less build and returns the corresponding running build.
[**pin_multiple_builds**](BuildApi.md#pin_multiple_builds) | **put** /app/rest/builds/multiple/{buildLocator}/pinInfo | Pin multiple matching builds.
[**remove_multiple_build_tags**](BuildApi.md#remove_multiple_build_tags) | **delete** /app/rest/builds/multiple/{buildLocator}/tags | Remove tags from multiple matching builds.
[**reset_build_finish_properties**](BuildApi.md#reset_build_finish_properties) | **delete** /app/rest/builds/{buildLocator}/caches/finishProperties | Remove build parameters from the matching build.
[**set_build_comment**](BuildApi.md#set_build_comment) | **put** /app/rest/builds/{buildLocator}/comment | Update the comment on the matching build.
[**set_build_finish_date**](BuildApi.md#set_build_finish_date) | **put** /app/rest/builds/{buildLocator}/finishDate | Marks the running build as finished by passing agent the current time of the build to finish.
[**set_build_number**](BuildApi.md#set_build_number) | **put** /app/rest/builds/{buildLocator}/number | Update the number of the matching build.
[**set_build_pin_info**](BuildApi.md#set_build_pin_info) | **put** /app/rest/builds/{buildLocator}/pinInfo | Update the pin info of the matching build.
[**set_build_status_text**](BuildApi.md#set_build_status_text) | **put** /app/rest/builds/{buildLocator}/statusText | Update the build status of the matching build.
[**set_build_tags**](BuildApi.md#set_build_tags) | **put** /app/rest/builds/{buildLocator}/tags | Update tags of the matching build.
[**set_finished_time**](BuildApi.md#set_finished_time) | **put** /app/rest/builds/{buildLocator}/finish | Marks the running build as finished by passing agent the current time of the build to finish.
[**set_multiple_build_comments**](BuildApi.md#set_multiple_build_comments) | **put** /app/rest/builds/multiple/{buildLocator}/comment | Update comments in multiple matching builds.



## add_build_vcs_label

> crate::models::VcsLabels add_build_vcs_label(build_locator, locator, fields, body)
Add a VCS label to the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |
**body** | Option<**String**> |  |  |

### Return type

[**crate::models::VcsLabels**](vcsLabels.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_log_message_to_build

> add_log_message_to_build(build_locator, fields, body)
Adds a message to the build log. Service messages are accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_problem_to_build

> crate::models::ProblemOccurrence add_problem_to_build(build_locator, fields, body)
Add a build problem to the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<**String**> |  |  |

### Return type

[**crate::models::ProblemOccurrence**](problemOccurrence.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_tags_to_build

> crate::models::Tags add_tags_to_build(build_locator, fields, body)
Add tags to the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Tags**](Tags.md)> |  |  |

### Return type

[**crate::models::Tags**](tags.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_tags_to_multiple_builds

> crate::models::MultipleOperationResult add_tags_to_multiple_builds(build_locator, fields, body)
Add tags to multiple matching builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Tags**](Tags.md)> |  |  |

### Return type

[**crate::models::MultipleOperationResult**](multipleOperationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_build

> delete_build(build_locator)
Delete build matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_build_comment

> delete_build_comment(build_locator)
Remove the build comment matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_multiple_build_comments

> crate::models::MultipleOperationResult delete_multiple_build_comments(build_locator, fields)
Delete comments of multiple matching builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::MultipleOperationResult**](multipleOperationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_multiple_builds

> crate::models::MultipleOperationResult delete_multiple_builds(build_locator, fields)
Delete multiple builds matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::MultipleOperationResult**](multipleOperationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_file_of_build

> download_file_of_build(path, build_locator, resolve_parameters, log_build_usage)
Download specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**build_locator** | **String** |  | [required] |
**resolve_parameters** | Option<**bool**> |  |  |
**log_build_usage** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aggregated_build_status

> String get_aggregated_build_status(build_locator)
Get the build status of aggregated matching builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aggregated_build_status_icon

> get_aggregated_build_status_icon(build_locator, suffix)
Get the status icon (in specified format) of aggregated matching builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**suffix** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_builds

> crate::models::Builds get_all_builds(locator, fields)
Get all builds.

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


## get_artifact_dependency_changes

> crate::models::BuildChanges get_artifact_dependency_changes(build_locator, fields)
Get artifact dependency changes of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BuildChanges**](buildChanges.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artifacts_directory

> String get_artifacts_directory(build_locator)
Get the artifacts' directory of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build

> crate::models::Build get_build(build_locator, fields)
Get build matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_actual_parameters

> crate::models::Properties get_build_actual_parameters(build_locator, fields)
Get actual build parameters of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_field

> String get_build_field(build_locator, field)
Get a field of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_finish_date

> String get_build_finish_date(build_locator)
Get the finish date of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_number

> String get_build_number(build_locator)
Get the number of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_pin_info

> crate::models::PinInfo get_build_pin_info(build_locator, fields)
Check if the matching build is pinned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PinInfo**](pinInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_problems

> crate::models::ProblemOccurrences get_build_problems(build_locator, fields)
Get build problems of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ProblemOccurrences**](problemOccurrences.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_related_issues

> crate::models::IssuesUsages get_build_related_issues(build_locator, fields)
Get related issues of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::IssuesUsages**](issuesUsages.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_resolved

> String get_build_resolved(build_locator, value)
Get the resolvement status of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**value** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_resulting_properties

> String get_build_resulting_properties(build_locator, property_name)
Update a build parameter of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**property_name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_source_file

> get_build_source_file(build_locator, file_name)
Get a source file of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**file_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_statistic_value

> String get_build_statistic_value(build_locator, name)
Get a statistical value of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_statistic_values

> crate::models::Properties get_build_statistic_values(build_locator, fields)
Get all statistical values of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_status_icon

> get_build_status_icon(build_locator, suffix)
Get the status icon (in specified format) of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**suffix** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_status_text

> String get_build_status_text(build_locator)
Get the build status text of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_tags

> crate::models::Tags get_build_tags(build_locator, locator, fields)
Get tags of the matching build.

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


## get_build_test_occurrences

> crate::models::TestOccurrences get_build_test_occurrences(build_locator, fields)
Get test occurrences of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::TestOccurrences**](testOccurrences.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_vcs_labels

> crate::models::VcsLabels get_build_vcs_labels(build_locator, fields)
Get VCS labels of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::VcsLabels**](vcsLabels.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_canceled_info

> crate::models::Comment get_canceled_info(build_locator, fields)
Check if the matching build is canceled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Comment**](comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_metadata_of_build

> std::path::PathBuf get_file_metadata_of_build(path, build_locator, fields, resolve_parameters, log_build_usage)
Get metadata of specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**resolve_parameters** | Option<**bool**> |  |  |
**log_build_usage** | Option<**bool**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_list_for_subpath_of_build

> crate::models::Files get_files_list_for_subpath_of_build(path, build_locator, base_path, locator, fields, resolve_parameters, log_build_usage)
List files under this path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**build_locator** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |
**resolve_parameters** | Option<**bool**> |  |  |
**log_build_usage** | Option<**bool**> |  |  |

### Return type

[**crate::models::Files**](files.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_list_of_build

> crate::models::Files get_files_list_of_build(build_locator, base_path, locator, fields, resolve_parameters, log_build_usage)
List all files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |
**resolve_parameters** | Option<**bool**> |  |  |
**log_build_usage** | Option<**bool**> |  |  |

### Return type

[**crate::models::Files**](files.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_multiple_builds

> crate::models::Builds get_multiple_builds(build_locator, fields)
Get multiple builds matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Builds**](builds.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zipped_file_of_build

> get_zipped_file_of_build(path, build_locator, base_path, locator, name, resolve_parameters, log_build_usage)
Get specific file zipped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**build_locator** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**resolve_parameters** | Option<**bool**> |  |  |
**log_build_usage** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_build_as_running

> crate::models::Build mark_build_as_running(build_locator, fields, body)
Starts the queued build as an agent-less build and returns the corresponding running build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<**String**> |  |  |

### Return type

[**crate::models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pin_multiple_builds

> crate::models::MultipleOperationResult pin_multiple_builds(build_locator, fields, body)
Pin multiple matching builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**PinInfo**](PinInfo.md)> |  |  |

### Return type

[**crate::models::MultipleOperationResult**](multipleOperationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_multiple_build_tags

> crate::models::MultipleOperationResult remove_multiple_build_tags(build_locator, fields, body)
Remove tags from multiple matching builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Tags**](Tags.md)> |  |  |

### Return type

[**crate::models::MultipleOperationResult**](multipleOperationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_build_finish_properties

> reset_build_finish_properties(build_locator)
Remove build parameters from the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_build_comment

> set_build_comment(build_locator, body)
Update the comment on the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_build_finish_date

> String set_build_finish_date(build_locator, body)
Marks the running build as finished by passing agent the current time of the build to finish.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_build_number

> String set_build_number(build_locator, body)
Update the number of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_build_pin_info

> crate::models::PinInfo set_build_pin_info(build_locator, fields, body)
Update the pin info of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**PinInfo**](PinInfo.md)> |  |  |

### Return type

[**crate::models::PinInfo**](pinInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_build_status_text

> String set_build_status_text(build_locator, body)
Update the build status of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_build_tags

> crate::models::Tags set_build_tags(build_locator, locator, fields, body)
Update tags of the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Tags**](Tags.md)> |  |  |

### Return type

[**crate::models::Tags**](tags.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_finished_time

> String set_finished_time(build_locator)
Marks the running build as finished by passing agent the current time of the build to finish.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_multiple_build_comments

> crate::models::MultipleOperationResult set_multiple_build_comments(build_locator, fields, body)
Update comments in multiple matching builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<**String**> |  |  |

### Return type

[**crate::models::MultipleOperationResult**](multipleOperationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

