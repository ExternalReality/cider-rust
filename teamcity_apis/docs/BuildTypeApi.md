# \BuildTypeApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_agent_requirement_to_build_type**](BuildTypeApi.md#add_agent_requirement_to_build_type) | **post** /app/rest/buildTypes/{btLocator}/agent-requirements | Add an agent requirement to the matching build configuration.
[**add_artifact_dependency_to_build_type**](BuildTypeApi.md#add_artifact_dependency_to_build_type) | **post** /app/rest/buildTypes/{btLocator}/artifact-dependencies | Add an artifact dependency to the matching build configuration.
[**add_build_feature_to_build_type**](BuildTypeApi.md#add_build_feature_to_build_type) | **post** /app/rest/buildTypes/{btLocator}/features | Add build feature to the matching build configuration.
[**add_build_step_to_build_type**](BuildTypeApi.md#add_build_step_to_build_type) | **post** /app/rest/buildTypes/{btLocator}/steps | Add a build step to the matching build configuration.
[**add_build_template**](BuildTypeApi.md#add_build_template) | **post** /app/rest/buildTypes/{btLocator}/templates | Add a build template to the matching build configuration.
[**add_parameter_to_build_feature**](BuildTypeApi.md#add_parameter_to_build_feature) | **put** /app/rest/buildTypes/{btLocator}/features/{featureId}/parameters/{parameterName} | Update build feature parameter for the matching build configuration.
[**add_parameter_to_build_step**](BuildTypeApi.md#add_parameter_to_build_step) | **put** /app/rest/buildTypes/{btLocator}/steps/{stepId}/parameters/{parameterName} | Add a parameter to a build step of the matching build configuration.
[**add_snapshot_dependency_to_build_type**](BuildTypeApi.md#add_snapshot_dependency_to_build_type) | **post** /app/rest/buildTypes/{btLocator}/snapshot-dependencies | Add a snapshot dependency to the matching build configuration.
[**add_trigger_to_build_type**](BuildTypeApi.md#add_trigger_to_build_type) | **post** /app/rest/buildTypes/{btLocator}/triggers | Add a trigger to the matching build configuration.
[**add_vcs_root_to_build_type**](BuildTypeApi.md#add_vcs_root_to_build_type) | **post** /app/rest/buildTypes/{btLocator}/vcs-root-entries | Add a VCS root to the matching build.
[**create_build_parameter_of_build_type**](BuildTypeApi.md#create_build_parameter_of_build_type) | **post** /app/rest/buildTypes/{btLocator}/parameters | Create a build parameter.
[**create_build_type**](BuildTypeApi.md#create_build_type) | **post** /app/rest/buildTypes | Create a new build configuration.
[**delete_agent_requirement**](BuildTypeApi.md#delete_agent_requirement) | **delete** /app/rest/buildTypes/{btLocator}/agent-requirements/{agentRequirementLocator} | Remove an agent requirement of the matching build configuration.
[**delete_artifact_dependency**](BuildTypeApi.md#delete_artifact_dependency) | **delete** /app/rest/buildTypes/{btLocator}/artifact-dependencies/{artifactDepLocator} | Remove an artifact dependency from the matching build configuration.
[**delete_build_parameter_of_build_type**](BuildTypeApi.md#delete_build_parameter_of_build_type) | **delete** /app/rest/buildTypes/{btLocator}/parameters/{name} | Delete build parameter.
[**delete_build_parameters_of_build_type**](BuildTypeApi.md#delete_build_parameters_of_build_type) | **delete** /app/rest/buildTypes/{btLocator}/parameters | Delete all build parameters.
[**delete_build_step**](BuildTypeApi.md#delete_build_step) | **delete** /app/rest/buildTypes/{btLocator}/steps/{stepId} | Delete a build step of the matching build configuration.
[**delete_build_step_parameters**](BuildTypeApi.md#delete_build_step_parameters) | **put** /app/rest/buildTypes/{btLocator}/steps/{stepId}/parameters | Update a parameter of a build step of the matching build configuration.
[**delete_build_type**](BuildTypeApi.md#delete_build_type) | **delete** /app/rest/buildTypes/{btLocator} | Delete build configuration matching the locator.
[**delete_feature_of_build_type**](BuildTypeApi.md#delete_feature_of_build_type) | **delete** /app/rest/buildTypes/{btLocator}/features/{featureId} | Remove a build feature of the matching build configuration.
[**delete_snapshot_dependency**](BuildTypeApi.md#delete_snapshot_dependency) | **delete** /app/rest/buildTypes/{btLocator}/snapshot-dependencies/{snapshotDepLocator} | Delete a snapshot dependency of the matching build configuration.
[**delete_trigger**](BuildTypeApi.md#delete_trigger) | **delete** /app/rest/buildTypes/{btLocator}/triggers/{triggerLocator} | Delete a trigger of the matching build configuration.
[**delete_vcs_root_of_build_type**](BuildTypeApi.md#delete_vcs_root_of_build_type) | **delete** /app/rest/buildTypes/{btLocator}/vcs-root-entries/{vcsRootLocator} | Remove a VCS root of the matching build configuration.
[**download_file_of_build_type**](BuildTypeApi.md#download_file_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/vcs/files/latest/files{path} | Download specific file.
[**get_agent_requirement**](BuildTypeApi.md#get_agent_requirement) | **get** /app/rest/buildTypes/{btLocator}/agent-requirements/{agentRequirementLocator} | Get an agent requirement of the matching build configuration.
[**get_agent_requirement_parameter**](BuildTypeApi.md#get_agent_requirement_parameter) | **get** /app/rest/buildTypes/{btLocator}/agent-requirements/{agentRequirementLocator}/{fieldName} | Get a setting of an agent requirement of the matching build configuration.
[**get_aliases**](BuildTypeApi.md#get_aliases) | **get** /app/rest/buildTypes/{btLocator}/aliases | Get external IDs of the matching build configuration.
[**get_all_agent_requirements**](BuildTypeApi.md#get_all_agent_requirements) | **get** /app/rest/buildTypes/{btLocator}/agent-requirements | Get all agent requirements of the matching build configuration.
[**get_all_artifact_dependencies**](BuildTypeApi.md#get_all_artifact_dependencies) | **get** /app/rest/buildTypes/{btLocator}/artifact-dependencies | Get all artifact dependencies of the matching build configuration.
[**get_all_branches_of_build_type**](BuildTypeApi.md#get_all_branches_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/branches | Get all branches of the matching build configuration.
[**get_all_build_feature_parameters**](BuildTypeApi.md#get_all_build_feature_parameters) | **get** /app/rest/buildTypes/{btLocator}/features/{featureId}/parameters | Get all parameters of a build feature of the matching build configuration.
[**get_all_build_features**](BuildTypeApi.md#get_all_build_features) | **get** /app/rest/buildTypes/{btLocator}/features | Get all build features of the matching build configuration.
[**get_all_build_step_parameters**](BuildTypeApi.md#get_all_build_step_parameters) | **get** /app/rest/buildTypes/{btLocator}/steps/{stepId}/parameters | Get all parameters of a build step of the matching build configuration.
[**get_all_build_steps**](BuildTypeApi.md#get_all_build_steps) | **get** /app/rest/buildTypes/{btLocator}/steps | Get all build steps of the matching build configuration.
[**get_all_build_templates**](BuildTypeApi.md#get_all_build_templates) | **get** /app/rest/buildTypes/{btLocator}/templates | Get all build templates of the matching build configuration.
[**get_all_build_types**](BuildTypeApi.md#get_all_build_types) | **get** /app/rest/buildTypes | Get all build configurations.
[**get_all_investigations_of_build_type**](BuildTypeApi.md#get_all_investigations_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/investigations | Get all investigations of the matching build configuration.
[**get_all_snapshot_dependencies**](BuildTypeApi.md#get_all_snapshot_dependencies) | **get** /app/rest/buildTypes/{btLocator}/snapshot-dependencies | Get all snapshot dependencies of the matching build configuration.
[**get_all_triggers**](BuildTypeApi.md#get_all_triggers) | **get** /app/rest/buildTypes/{btLocator}/triggers | Get all triggers of the matching build configuration.
[**get_all_vcs_roots_of_build_type**](BuildTypeApi.md#get_all_vcs_roots_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/vcs-root-entries | Get all VCS roots of the matching build configuration.
[**get_artifact_dependency**](BuildTypeApi.md#get_artifact_dependency) | **get** /app/rest/buildTypes/{btLocator}/artifact-dependencies/{artifactDepLocator} | Get an artifact dependency of the matching build configuration.
[**get_artifact_dependency_parameter**](BuildTypeApi.md#get_artifact_dependency_parameter) | **get** /app/rest/buildTypes/{btLocator}/artifact-dependencies/{artifactDepLocator}/{fieldName} | Get a parameter of an artifact dependency of the matching build configuration.
[**get_build_feature**](BuildTypeApi.md#get_build_feature) | **get** /app/rest/buildTypes/{btLocator}/features/{featureId} | Get a build feature of the matching build configuration.
[**get_build_feature_parameter**](BuildTypeApi.md#get_build_feature_parameter) | **get** /app/rest/buildTypes/{btLocator}/features/{featureId}/parameters/{parameterName} | Get a parameter of a build feature of the matching build configuration.
[**get_build_feature_setting**](BuildTypeApi.md#get_build_feature_setting) | **get** /app/rest/buildTypes/{btLocator}/features/{featureId}/{name} | Get the setting of a build feature of the matching build configuration.
[**get_build_parameter_of_build_type**](BuildTypeApi.md#get_build_parameter_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/parameters/{name} | Get build parameter.
[**get_build_parameter_specification_of_build_type**](BuildTypeApi.md#get_build_parameter_specification_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/parameters/{name}/type/rawValue | Get build parameter specification.
[**get_build_parameter_type_of_build_type**](BuildTypeApi.md#get_build_parameter_type_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/parameters/{name}/type | Get type of build parameter.
[**get_build_parameter_value_of_build_type**](BuildTypeApi.md#get_build_parameter_value_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/parameters/{name}/value | Get value of build parameter.
[**get_build_parameters_of_build_type**](BuildTypeApi.md#get_build_parameters_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/parameters | Get build parameters.
[**get_build_step**](BuildTypeApi.md#get_build_step) | **get** /app/rest/buildTypes/{btLocator}/steps/{stepId} | Get a build step of the matching build configuration.
[**get_build_step_parameter**](BuildTypeApi.md#get_build_step_parameter) | **get** /app/rest/buildTypes/{btLocator}/steps/{stepId}/parameters/{parameterName} | Get a parameter of a build step of the matching build configuration.
[**get_build_step_setting**](BuildTypeApi.md#get_build_step_setting) | **get** /app/rest/buildTypes/{btLocator}/steps/{stepId}/{fieldName} | Get the setting of a build step of the matching build configuration.
[**get_build_template**](BuildTypeApi.md#get_build_template) | **get** /app/rest/buildTypes/{btLocator}/templates/{templateLocator} | Get a template of the matching build configuration.
[**get_build_type**](BuildTypeApi.md#get_build_type) | **get** /app/rest/buildTypes/{btLocator} | Get build configuration matching the locator.
[**get_build_type_build_tags**](BuildTypeApi.md#get_build_type_build_tags) | **get** /app/rest/buildTypes/{btLocator}/buildTags | Get tags of builds of the matching build configuration.
[**get_build_type_builds**](BuildTypeApi.md#get_build_type_builds) | **get** /app/rest/buildTypes/{btLocator}/builds | Get builds of the matching build configuration.
[**get_build_type_field**](BuildTypeApi.md#get_build_type_field) | **get** /app/rest/buildTypes/{btLocator}/{field} | Get a field of the matching build configuration.
[**get_build_type_settings_file**](BuildTypeApi.md#get_build_type_settings_file) | **get** /app/rest/buildTypes/{btLocator}/settingsFile | Get the settings file of the matching build configuration.
[**get_file_metadata_of_build_type**](BuildTypeApi.md#get_file_metadata_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/vcs/files/latest/metadata{path} | Get metadata of specific file.
[**get_files_list_for_subpath_of_build_type**](BuildTypeApi.md#get_files_list_for_subpath_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/vcs/files/latest/{path} | List files under this path.
[**get_files_list_of_build_type**](BuildTypeApi.md#get_files_list_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/vcs/files/latest | List all files.
[**get_snapshot_dependency**](BuildTypeApi.md#get_snapshot_dependency) | **get** /app/rest/buildTypes/{btLocator}/snapshot-dependencies/{snapshotDepLocator} | Get a snapshot dependency of the matching build configuration.
[**get_trigger**](BuildTypeApi.md#get_trigger) | **get** /app/rest/buildTypes/{btLocator}/triggers/{triggerLocator} | Get a trigger of the matching build configuration.
[**get_trigger_parameter**](BuildTypeApi.md#get_trigger_parameter) | **get** /app/rest/buildTypes/{btLocator}/triggers/{triggerLocator}/{fieldName} | Get a parameter of a trigger of the matching build configuration.
[**get_vcs_root**](BuildTypeApi.md#get_vcs_root) | **get** /app/rest/buildTypes/{btLocator}/vcs-root-entries/{vcsRootLocator} | Get a VCS root of the matching build configuration.
[**get_vcs_root_checkout_rules**](BuildTypeApi.md#get_vcs_root_checkout_rules) | **get** /app/rest/buildTypes/{btLocator}/vcs-root-entries/{vcsRootLocator}/checkout-rules | Get checkout rules of a VCS root of the matching build configuration.
[**get_vcs_root_instances_of_build_type**](BuildTypeApi.md#get_vcs_root_instances_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/vcsRootInstances | Get all VCS root instances of the matching build configuration.
[**get_zipped_file_of_build_type**](BuildTypeApi.md#get_zipped_file_of_build_type) | **get** /app/rest/buildTypes/{btLocator}/vcs/files/latest/archived{path} | Get specific file zipped.
[**remove_all_templates**](BuildTypeApi.md#remove_all_templates) | **delete** /app/rest/buildTypes/{btLocator}/templates | Detach all templates from the matching build configuration.
[**remove_template**](BuildTypeApi.md#remove_template) | **delete** /app/rest/buildTypes/{btLocator}/templates/{templateLocator} | Detach a template from the matching build configuration.
[**replace_agent_requirement**](BuildTypeApi.md#replace_agent_requirement) | **put** /app/rest/buildTypes/{btLocator}/agent-requirements/{agentRequirementLocator} | Update an agent requirement of the matching build configuration.
[**replace_all_agent_requirements**](BuildTypeApi.md#replace_all_agent_requirements) | **put** /app/rest/buildTypes/{btLocator}/agent-requirements | Update all agent requirements of the matching build configuration.
[**replace_all_artifact_dependencies**](BuildTypeApi.md#replace_all_artifact_dependencies) | **put** /app/rest/buildTypes/{btLocator}/artifact-dependencies | Update all artifact dependencies of the matching build configuration.
[**replace_all_build_features**](BuildTypeApi.md#replace_all_build_features) | **put** /app/rest/buildTypes/{btLocator}/features | Update all build features of the matching build configuration.
[**replace_all_build_steps**](BuildTypeApi.md#replace_all_build_steps) | **put** /app/rest/buildTypes/{btLocator}/steps | Update all build steps of the matching build configuration.
[**replace_all_snapshot_dependencies**](BuildTypeApi.md#replace_all_snapshot_dependencies) | **put** /app/rest/buildTypes/{btLocator}/snapshot-dependencies | Update all snapshot dependencies of the matching build configuration.
[**replace_all_triggers**](BuildTypeApi.md#replace_all_triggers) | **put** /app/rest/buildTypes/{btLocator}/triggers | Update all triggers of the matching build configuration.
[**replace_all_vcs_roots**](BuildTypeApi.md#replace_all_vcs_roots) | **put** /app/rest/buildTypes/{btLocator}/vcs-root-entries | Update all VCS roots of the matching build configuration.
[**replace_artifact_dependency**](BuildTypeApi.md#replace_artifact_dependency) | **put** /app/rest/buildTypes/{btLocator}/artifact-dependencies/{artifactDepLocator} | Update an artifact dependency of the matching build configuration.
[**replace_build_feature**](BuildTypeApi.md#replace_build_feature) | **put** /app/rest/buildTypes/{btLocator}/features/{featureId} | Update a build feature of the matching build configuration.
[**replace_build_feature_parameters**](BuildTypeApi.md#replace_build_feature_parameters) | **put** /app/rest/buildTypes/{btLocator}/features/{featureId}/parameters | Update a parameter of a build feature of the matching build configuration.
[**replace_build_step**](BuildTypeApi.md#replace_build_step) | **put** /app/rest/buildTypes/{btLocator}/steps/{stepId} | Replace a build step of the matching build configuration.
[**replace_snapshot_dependency**](BuildTypeApi.md#replace_snapshot_dependency) | **put** /app/rest/buildTypes/{btLocator}/snapshot-dependencies/{snapshotDepLocator} | Update a snapshot dependency of the matching build configuration.
[**replace_trigger**](BuildTypeApi.md#replace_trigger) | **put** /app/rest/buildTypes/{btLocator}/triggers/{triggerLocator} | Update a trigger of the matching build configuration.
[**set_agent_requirement_parameter**](BuildTypeApi.md#set_agent_requirement_parameter) | **put** /app/rest/buildTypes/{btLocator}/agent-requirements/{agentRequirementLocator}/{fieldName} | Update a parameter of an agent requirement of the matching build configuration.
[**set_artifact_dependency_parameter**](BuildTypeApi.md#set_artifact_dependency_parameter) | **put** /app/rest/buildTypes/{btLocator}/artifact-dependencies/{artifactDepLocator}/{fieldName} | Update a parameter of an artifact dependency of the matching build configuration.
[**set_build_feature_parameter**](BuildTypeApi.md#set_build_feature_parameter) | **put** /app/rest/buildTypes/{btLocator}/features/{featureId}/{name} | Update a parameter of a build feature of the matching build configuration.
[**set_build_step_parameter**](BuildTypeApi.md#set_build_step_parameter) | **put** /app/rest/buildTypes/{btLocator}/steps/{stepId}/{fieldName} | Update a parameter of a build step of the matching build configuration.
[**set_build_type_field**](BuildTypeApi.md#set_build_type_field) | **put** /app/rest/buildTypes/{btLocator}/{field} | Update a field of the matching build configuration.
[**set_build_type_templates**](BuildTypeApi.md#set_build_type_templates) | **put** /app/rest/buildTypes/{btLocator}/templates | Update all templates of the matching build configuration.
[**set_trigger_parameter**](BuildTypeApi.md#set_trigger_parameter) | **put** /app/rest/buildTypes/{btLocator}/triggers/{triggerLocator}/{fieldName} | Update a parameter of a trigger of the matching build configuration.
[**update_build_parameter_of_build_type**](BuildTypeApi.md#update_build_parameter_of_build_type) | **put** /app/rest/buildTypes/{btLocator}/parameters/{name} | Update build parameter.
[**update_build_parameter_specification_of_build_type**](BuildTypeApi.md#update_build_parameter_specification_of_build_type) | **put** /app/rest/buildTypes/{btLocator}/parameters/{name}/type/rawValue | Update build parameter specification.
[**update_build_parameter_type_of_build_type**](BuildTypeApi.md#update_build_parameter_type_of_build_type) | **put** /app/rest/buildTypes/{btLocator}/parameters/{name}/type | Update type of build parameter.
[**update_build_parameter_value_of_build_type**](BuildTypeApi.md#update_build_parameter_value_of_build_type) | **put** /app/rest/buildTypes/{btLocator}/parameters/{name}/value | Update value of build parameter.
[**update_build_parameters_of_build_type**](BuildTypeApi.md#update_build_parameters_of_build_type) | **put** /app/rest/buildTypes/{btLocator}/parameters | Update build parameters.
[**update_build_type_vcs_root**](BuildTypeApi.md#update_build_type_vcs_root) | **put** /app/rest/buildTypes/{btLocator}/vcs-root-entries/{vcsRootLocator} | Update a VCS root of the matching build configuration.
[**update_build_type_vcs_root_checkout_rules**](BuildTypeApi.md#update_build_type_vcs_root_checkout_rules) | **put** /app/rest/buildTypes/{btLocator}/vcs-root-entries/{vcsRootLocator}/checkout-rules | Update checkout rules of a VCS root of the matching build configuration.



## add_agent_requirement_to_build_type

> crate::models::AgentRequirement add_agent_requirement_to_build_type(bt_locator, fields, body)
Add an agent requirement to the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**AgentRequirement**](AgentRequirement.md)> |  |  |

### Return type

[**crate::models::AgentRequirement**](agent-requirement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_artifact_dependency_to_build_type

> crate::models::ArtifactDependency add_artifact_dependency_to_build_type(bt_locator, fields, body)
Add an artifact dependency to the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**ArtifactDependency**](ArtifactDependency.md)> |  |  |

### Return type

[**crate::models::ArtifactDependency**](artifact-dependency.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_build_feature_to_build_type

> crate::models::Feature add_build_feature_to_build_type(bt_locator, fields, body)
Add build feature to the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Feature**](Feature.md)> |  |  |

### Return type

[**crate::models::Feature**](feature.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_build_step_to_build_type

> crate::models::Step add_build_step_to_build_type(bt_locator, fields, body)
Add a build step to the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Step**](Step.md)> |  |  |

### Return type

[**crate::models::Step**](step.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_build_template

> crate::models::BuildType add_build_template(bt_locator, optimize_settings, fields, body)
Add a build template to the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**optimize_settings** | Option<**bool**> |  |  |
**fields** | Option<**String**> |  |  |
**body** | Option<[**BuildType**](BuildType.md)> |  |  |

### Return type

[**crate::models::BuildType**](buildType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_parameter_to_build_feature

> String add_parameter_to_build_feature(bt_locator, feature_id, parameter_name, body)
Update build feature parameter for the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**feature_id** | **String** |  | [required] |
**parameter_name** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_parameter_to_build_step

> String add_parameter_to_build_step(bt_locator, step_id, parameter_name, body)
Add a parameter to a build step of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**step_id** | **String** |  | [required] |
**parameter_name** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_snapshot_dependency_to_build_type

> crate::models::SnapshotDependency add_snapshot_dependency_to_build_type(bt_locator, fields, body)
Add a snapshot dependency to the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**SnapshotDependency**](SnapshotDependency.md)> |  |  |

### Return type

[**crate::models::SnapshotDependency**](snapshot-dependency.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_trigger_to_build_type

> crate::models::Trigger add_trigger_to_build_type(bt_locator, fields, body)
Add a trigger to the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Trigger**](Trigger.md)> |  |  |

### Return type

[**crate::models::Trigger**](trigger.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_vcs_root_to_build_type

> crate::models::VcsRootEntry add_vcs_root_to_build_type(bt_locator, fields, body)
Add a VCS root to the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**VcsRootEntry**](VcsRootEntry.md)> |  |  |

### Return type

[**crate::models::VcsRootEntry**](vcs-root-entry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_build_parameter_of_build_type

> crate::models::Property create_build_parameter_of_build_type(bt_locator, fields, body)
Create a build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Property**](Property.md)> |  |  |

### Return type

[**crate::models::Property**](property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_build_type

> crate::models::BuildType create_build_type(fields, body)
Create a new build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**BuildType**](BuildType.md)> |  |  |

### Return type

[**crate::models::BuildType**](buildType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agent_requirement

> delete_agent_requirement(bt_locator, agent_requirement_locator)
Remove an agent requirement of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**agent_requirement_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_artifact_dependency

> delete_artifact_dependency(bt_locator, artifact_dep_locator)
Remove an artifact dependency from the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**artifact_dep_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_build_parameter_of_build_type

> delete_build_parameter_of_build_type(name, bt_locator)
Delete build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_build_parameters_of_build_type

> delete_build_parameters_of_build_type(bt_locator)
Delete all build parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_build_step

> delete_build_step(bt_locator, step_id)
Delete a build step of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**step_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_build_step_parameters

> crate::models::Properties delete_build_step_parameters(bt_locator, step_id, fields, body)
Update a parameter of a build step of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**step_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Properties**](Properties.md)> |  |  |

### Return type

[**crate::models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_build_type

> delete_build_type(bt_locator)
Delete build configuration matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_feature_of_build_type

> delete_feature_of_build_type(bt_locator, feature_id)
Remove a build feature of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**feature_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_snapshot_dependency

> delete_snapshot_dependency(bt_locator, snapshot_dep_locator)
Delete a snapshot dependency of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**snapshot_dep_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_trigger

> delete_trigger(bt_locator, trigger_locator)
Delete a trigger of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**trigger_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vcs_root_of_build_type

> delete_vcs_root_of_build_type(bt_locator, vcs_root_locator)
Remove a VCS root of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**vcs_root_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_file_of_build_type

> download_file_of_build_type(path, bt_locator, resolve_parameters)
Download specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |
**resolve_parameters** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_agent_requirement

> crate::models::AgentRequirement get_agent_requirement(bt_locator, agent_requirement_locator, fields)
Get an agent requirement of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**agent_requirement_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AgentRequirement**](agent-requirement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_agent_requirement_parameter

> String get_agent_requirement_parameter(bt_locator, agent_requirement_locator, field_name)
Get a setting of an agent requirement of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**agent_requirement_locator** | **String** |  | [required] |
**field_name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aliases

> crate::models::Items get_aliases(bt_locator, field)
Get external IDs of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**field** | Option<**String**> |  |  |

### Return type

[**crate::models::Items**](items.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_agent_requirements

> crate::models::AgentRequirements get_all_agent_requirements(bt_locator, fields)
Get all agent requirements of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AgentRequirements**](agent-requirements.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_artifact_dependencies

> crate::models::ArtifactDependencies get_all_artifact_dependencies(bt_locator, fields)
Get all artifact dependencies of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ArtifactDependencies**](artifact-dependencies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_branches_of_build_type

> crate::models::Branches get_all_branches_of_build_type(bt_locator, locator, fields)
Get all branches of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Branches**](branches.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_build_feature_parameters

> crate::models::Properties get_all_build_feature_parameters(bt_locator, feature_id, fields)
Get all parameters of a build feature of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**feature_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_build_features

> crate::models::Features get_all_build_features(bt_locator, fields)
Get all build features of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Features**](features.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_build_step_parameters

> crate::models::Properties get_all_build_step_parameters(bt_locator, step_id, fields)
Get all parameters of a build step of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**step_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_build_steps

> crate::models::Steps get_all_build_steps(bt_locator, fields)
Get all build steps of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Steps**](steps.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_build_templates

> crate::models::BuildTypes get_all_build_templates(bt_locator, fields)
Get all build templates of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BuildTypes**](buildTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_build_types

> crate::models::BuildTypes get_all_build_types(locator, fields)
Get all build configurations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BuildTypes**](buildTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_investigations_of_build_type

> crate::models::Investigations get_all_investigations_of_build_type(bt_locator, fields)
Get all investigations of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Investigations**](investigations.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_snapshot_dependencies

> crate::models::SnapshotDependencies get_all_snapshot_dependencies(bt_locator, fields)
Get all snapshot dependencies of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::SnapshotDependencies**](snapshot-dependencies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_triggers

> crate::models::Triggers get_all_triggers(bt_locator, fields)
Get all triggers of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Triggers**](triggers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_vcs_roots_of_build_type

> crate::models::VcsRootEntries get_all_vcs_roots_of_build_type(bt_locator, fields)
Get all VCS roots of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::VcsRootEntries**](vcs-root-entries.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artifact_dependency

> crate::models::ArtifactDependency get_artifact_dependency(bt_locator, artifact_dep_locator, fields)
Get an artifact dependency of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**artifact_dep_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::ArtifactDependency**](artifact-dependency.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artifact_dependency_parameter

> String get_artifact_dependency_parameter(bt_locator, artifact_dep_locator, field_name)
Get a parameter of an artifact dependency of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**artifact_dep_locator** | **String** |  | [required] |
**field_name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_feature

> crate::models::Feature get_build_feature(bt_locator, feature_id, fields)
Get a build feature of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**feature_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Feature**](feature.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_feature_parameter

> String get_build_feature_parameter(bt_locator, feature_id, parameter_name)
Get a parameter of a build feature of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**feature_id** | **String** |  | [required] |
**parameter_name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_feature_setting

> String get_build_feature_setting(bt_locator, feature_id, name)
Get the setting of a build feature of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**feature_id** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_parameter_of_build_type

> crate::models::Property get_build_parameter_of_build_type(name, bt_locator, fields)
Get build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Property**](property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_parameter_specification_of_build_type

> String get_build_parameter_specification_of_build_type(name, bt_locator)
Get build parameter specification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_parameter_type_of_build_type

> crate::models::ModelType get_build_parameter_type_of_build_type(name, bt_locator)
Get type of build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |

### Return type

[**crate::models::ModelType**](type.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_parameter_value_of_build_type

> String get_build_parameter_value_of_build_type(name, bt_locator)
Get value of build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_parameters_of_build_type

> crate::models::Properties get_build_parameters_of_build_type(bt_locator, locator, fields)
Get build parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_step

> crate::models::Step get_build_step(bt_locator, step_id, fields)
Get a build step of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**step_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Step**](step.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_step_parameter

> String get_build_step_parameter(bt_locator, step_id, parameter_name)
Get a parameter of a build step of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**step_id** | **String** |  | [required] |
**parameter_name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_step_setting

> String get_build_step_setting(bt_locator, step_id, field_name)
Get the setting of a build step of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**step_id** | **String** |  | [required] |
**field_name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_template

> crate::models::BuildType get_build_template(bt_locator, template_locator, fields)
Get a template of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**template_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BuildType**](buildType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_type

> crate::models::BuildType get_build_type(bt_locator, fields)
Get build configuration matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BuildType**](buildType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_type_build_tags

> crate::models::Tags get_build_type_build_tags(bt_locator, field)
Get tags of builds of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**field** | Option<**String**> |  |  |

### Return type

[**crate::models::Tags**](tags.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_type_builds

> crate::models::Builds get_build_type_builds(bt_locator, fields)
Get builds of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Builds**](builds.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_type_field

> String get_build_type_field(bt_locator, field)
Get a field of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_type_settings_file

> String get_build_type_settings_file(bt_locator)
Get the settings file of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_metadata_of_build_type

> std::path::PathBuf get_file_metadata_of_build_type(path, bt_locator, fields, resolve_parameters)
Get metadata of specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**resolve_parameters** | Option<**bool**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_list_for_subpath_of_build_type

> crate::models::Files get_files_list_for_subpath_of_build_type(path, bt_locator, base_path, locator, fields, resolve_parameters)
List files under this path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |
**resolve_parameters** | Option<**bool**> |  |  |

### Return type

[**crate::models::Files**](files.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_list_of_build_type

> crate::models::Files get_files_list_of_build_type(bt_locator, base_path, locator, fields, resolve_parameters)
List all files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |
**resolve_parameters** | Option<**bool**> |  |  |

### Return type

[**crate::models::Files**](files.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_snapshot_dependency

> crate::models::SnapshotDependency get_snapshot_dependency(bt_locator, snapshot_dep_locator, fields)
Get a snapshot dependency of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**snapshot_dep_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::SnapshotDependency**](snapshot-dependency.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trigger

> crate::models::Trigger get_trigger(bt_locator, trigger_locator, fields)
Get a trigger of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**trigger_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Trigger**](trigger.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trigger_parameter

> String get_trigger_parameter(bt_locator, trigger_locator, field_name)
Get a parameter of a trigger of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**trigger_locator** | **String** |  | [required] |
**field_name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root

> crate::models::VcsRootEntry get_vcs_root(bt_locator, vcs_root_locator, fields)
Get a VCS root of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**vcs_root_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::VcsRootEntry**](vcs-root-entry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root_checkout_rules

> String get_vcs_root_checkout_rules(bt_locator, vcs_root_locator)
Get checkout rules of a VCS root of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**vcs_root_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root_instances_of_build_type

> crate::models::VcsRootInstances get_vcs_root_instances_of_build_type(bt_locator, fields)
Get all VCS root instances of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::VcsRootInstances**](vcs-root-instances.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zipped_file_of_build_type

> get_zipped_file_of_build_type(path, bt_locator, base_path, locator, name, resolve_parameters)
Get specific file zipped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**resolve_parameters** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_all_templates

> remove_all_templates(bt_locator, inline_settings)
Detach all templates from the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**inline_settings** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_template

> remove_template(bt_locator, template_locator, inline_settings)
Detach a template from the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**template_locator** | **String** |  | [required] |
**inline_settings** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_agent_requirement

> crate::models::AgentRequirement replace_agent_requirement(bt_locator, agent_requirement_locator, fields, body)
Update an agent requirement of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**agent_requirement_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**AgentRequirement**](AgentRequirement.md)> |  |  |

### Return type

[**crate::models::AgentRequirement**](agent-requirement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_all_agent_requirements

> crate::models::AgentRequirements replace_all_agent_requirements(bt_locator, fields, body)
Update all agent requirements of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**AgentRequirements**](AgentRequirements.md)> |  |  |

### Return type

[**crate::models::AgentRequirements**](agent-requirements.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_all_artifact_dependencies

> crate::models::ArtifactDependencies replace_all_artifact_dependencies(bt_locator, fields, body)
Update all artifact dependencies of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**ArtifactDependencies**](ArtifactDependencies.md)> |  |  |

### Return type

[**crate::models::ArtifactDependencies**](artifact-dependencies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_all_build_features

> crate::models::Features replace_all_build_features(bt_locator, fields, body)
Update all build features of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Features**](Features.md)> |  |  |

### Return type

[**crate::models::Features**](features.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_all_build_steps

> crate::models::Steps replace_all_build_steps(bt_locator, fields, body)
Update all build steps of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Steps**](Steps.md)> |  |  |

### Return type

[**crate::models::Steps**](steps.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_all_snapshot_dependencies

> crate::models::SnapshotDependencies replace_all_snapshot_dependencies(bt_locator, fields, body)
Update all snapshot dependencies of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**SnapshotDependencies**](SnapshotDependencies.md)> |  |  |

### Return type

[**crate::models::SnapshotDependencies**](snapshot-dependencies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_all_triggers

> crate::models::Triggers replace_all_triggers(bt_locator, fields, body)
Update all triggers of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Triggers**](Triggers.md)> |  |  |

### Return type

[**crate::models::Triggers**](triggers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_all_vcs_roots

> crate::models::VcsRootEntries replace_all_vcs_roots(bt_locator, fields, body)
Update all VCS roots of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**VcsRootEntries**](VcsRootEntries.md)> |  |  |

### Return type

[**crate::models::VcsRootEntries**](vcs-root-entries.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_artifact_dependency

> crate::models::ArtifactDependency replace_artifact_dependency(bt_locator, artifact_dep_locator, fields, body)
Update an artifact dependency of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**artifact_dep_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**ArtifactDependency**](ArtifactDependency.md)> |  |  |

### Return type

[**crate::models::ArtifactDependency**](artifact-dependency.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_build_feature

> crate::models::Feature replace_build_feature(bt_locator, feature_id, fields, body)
Update a build feature of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**feature_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Feature**](Feature.md)> |  |  |

### Return type

[**crate::models::Feature**](feature.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_build_feature_parameters

> crate::models::Properties replace_build_feature_parameters(bt_locator, feature_id, fields, body)
Update a parameter of a build feature of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**feature_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Properties**](Properties.md)> |  |  |

### Return type

[**crate::models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_build_step

> crate::models::Step replace_build_step(bt_locator, step_id, fields, body)
Replace a build step of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**step_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Step**](Step.md)> |  |  |

### Return type

[**crate::models::Step**](step.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_snapshot_dependency

> crate::models::SnapshotDependency replace_snapshot_dependency(bt_locator, snapshot_dep_locator, fields, body)
Update a snapshot dependency of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**snapshot_dep_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**SnapshotDependency**](SnapshotDependency.md)> |  |  |

### Return type

[**crate::models::SnapshotDependency**](snapshot-dependency.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_trigger

> crate::models::Trigger replace_trigger(bt_locator, trigger_locator, fields, body)
Update a trigger of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**trigger_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Trigger**](Trigger.md)> |  |  |

### Return type

[**crate::models::Trigger**](trigger.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_agent_requirement_parameter

> String set_agent_requirement_parameter(bt_locator, agent_requirement_locator, field_name, body)
Update a parameter of an agent requirement of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**agent_requirement_locator** | **String** |  | [required] |
**field_name** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_artifact_dependency_parameter

> String set_artifact_dependency_parameter(bt_locator, artifact_dep_locator, field_name, body)
Update a parameter of an artifact dependency of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**artifact_dep_locator** | **String** |  | [required] |
**field_name** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_build_feature_parameter

> String set_build_feature_parameter(bt_locator, feature_id, name, body)
Update a parameter of a build feature of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**feature_id** | **String** |  | [required] |
**name** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_build_step_parameter

> String set_build_step_parameter(bt_locator, step_id, field_name, body)
Update a parameter of a build step of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**step_id** | **String** |  | [required] |
**field_name** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_build_type_field

> String set_build_type_field(bt_locator, field, body)
Update a field of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
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


## set_build_type_templates

> crate::models::BuildTypes set_build_type_templates(bt_locator, optimize_settings, fields, body)
Update all templates of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**optimize_settings** | Option<**bool**> |  |  |
**fields** | Option<**String**> |  |  |
**body** | Option<[**BuildTypes**](BuildTypes.md)> |  |  |

### Return type

[**crate::models::BuildTypes**](buildTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_trigger_parameter

> String set_trigger_parameter(bt_locator, trigger_locator, field_name, body)
Update a parameter of a trigger of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**trigger_locator** | **String** |  | [required] |
**field_name** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_parameter_of_build_type

> crate::models::Property update_build_parameter_of_build_type(name, bt_locator, fields, body)
Update build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Property**](Property.md)> |  |  |

### Return type

[**crate::models::Property**](property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_parameter_specification_of_build_type

> String update_build_parameter_specification_of_build_type(name, bt_locator, body)
Update build parameter specification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_parameter_type_of_build_type

> crate::models::ModelType update_build_parameter_type_of_build_type(name, bt_locator, body)
Update type of build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |
**body** | Option<[**ModelType**](ModelType.md)> |  |  |

### Return type

[**crate::models::ModelType**](type.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_parameter_value_of_build_type

> String update_build_parameter_value_of_build_type(name, bt_locator, body)
Update value of build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**bt_locator** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_parameters_of_build_type

> crate::models::Properties update_build_parameters_of_build_type(bt_locator, fields, body)
Update build parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Properties**](Properties.md)> |  |  |

### Return type

[**crate::models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_type_vcs_root

> crate::models::VcsRootEntry update_build_type_vcs_root(bt_locator, vcs_root_locator, fields, body)
Update a VCS root of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**vcs_root_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**VcsRootEntry**](VcsRootEntry.md)> |  |  |

### Return type

[**crate::models::VcsRootEntry**](vcs-root-entry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_type_vcs_root_checkout_rules

> String update_build_type_vcs_root_checkout_rules(bt_locator, vcs_root_locator, body)
Update checkout rules of a VCS root of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_locator** | **String** |  | [required] |
**vcs_root_locator** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

