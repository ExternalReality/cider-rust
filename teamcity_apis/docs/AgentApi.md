# \AgentApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_agent**](AgentApi.md#delete_agent) | **delete** /app/rest/agents/{agentLocator} | Delete an inactive agent.
[**get_agent**](AgentApi.md#get_agent) | **get** /app/rest/agents/{agentLocator} | Get agent matching the locator.
[**get_agent_field**](AgentApi.md#get_agent_field) | **get** /app/rest/agents/{agentLocator}/{field} | Get a field of the matching agent.
[**get_agent_pool**](AgentApi.md#get_agent_pool) | **get** /app/rest/agents/{agentLocator}/pool | Get the agent pool of the matching agent.
[**get_all_agents**](AgentApi.md#get_all_agents) | **get** /app/rest/agents | Get all known agents.
[**get_authorized_info**](AgentApi.md#get_authorized_info) | **get** /app/rest/agents/{agentLocator}/authorizedInfo | Get the authorization info of the matching agent.
[**get_build_configuration_run_policy**](AgentApi.md#get_build_configuration_run_policy) | **get** /app/rest/agents/{agentLocator}/compatibilityPolicy | Get the build configuration run policy of the matching agent.
[**get_compatible_build_types**](AgentApi.md#get_compatible_build_types) | **get** /app/rest/agents/{agentLocator}/compatibleBuildTypes | Get build types compatible with the matching agent.
[**get_enabled_info**](AgentApi.md#get_enabled_info) | **get** /app/rest/agents/{agentLocator}/enabledInfo | Check if the matching agent is enabled.
[**get_incompatible_build_types**](AgentApi.md#get_incompatible_build_types) | **get** /app/rest/agents/{agentLocator}/incompatibleBuildTypes | Get build types incompatible with the matching agent.
[**set_agent_field**](AgentApi.md#set_agent_field) | **put** /app/rest/agents/{agentLocator}/{field} | Update a field of the matching agent.
[**set_agent_pool**](AgentApi.md#set_agent_pool) | **put** /app/rest/agents/{agentLocator}/pool | Assign the matching agent to the specified agent pool.
[**set_authorized_info**](AgentApi.md#set_authorized_info) | **put** /app/rest/agents/{agentLocator}/authorizedInfo | Update the authorization info of the matching agent.
[**set_build_configuration_run_policy**](AgentApi.md#set_build_configuration_run_policy) | **put** /app/rest/agents/{agentLocator}/compatibilityPolicy | Update build configuration run policy of agent matching locator.
[**set_enabled_info**](AgentApi.md#set_enabled_info) | **put** /app/rest/agents/{agentLocator}/enabledInfo | Update the enablement status of the matching agent.



## delete_agent

> delete_agent(agent_locator)
Delete an inactive agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_agent

> crate::models::Agent get_agent(agent_locator, fields)
Get agent matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Agent**](agent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_agent_field

> String get_agent_field(agent_locator, field)
Get a field of the matching agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_agent_pool

> crate::models::AgentPool get_agent_pool(agent_locator, fields)
Get the agent pool of the matching agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AgentPool**](agentPool.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_agents

> crate::models::Agents get_all_agents(locator, fields)
Get all known agents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Agents**](agents.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorized_info

> crate::models::AuthorizedInfo get_authorized_info(agent_locator, fields)
Get the authorization info of the matching agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AuthorizedInfo**](authorizedInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_configuration_run_policy

> crate::models::CompatibilityPolicy get_build_configuration_run_policy(agent_locator, fields)
Get the build configuration run policy of the matching agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::CompatibilityPolicy**](compatibilityPolicy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_compatible_build_types

> crate::models::BuildTypes get_compatible_build_types(agent_locator, fields)
Get build types compatible with the matching agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::BuildTypes**](buildTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_enabled_info

> crate::models::EnabledInfo get_enabled_info(agent_locator, fields)
Check if the matching agent is enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::EnabledInfo**](enabledInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_incompatible_build_types

> crate::models::Compatibilities get_incompatible_build_types(agent_locator, fields)
Get build types incompatible with the matching agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Compatibilities**](compatibilities.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_agent_field

> String set_agent_field(agent_locator, field, body)
Update a field of the matching agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
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


## set_agent_pool

> crate::models::AgentPool set_agent_pool(agent_locator, fields, body)
Assign the matching agent to the specified agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**AgentPool**](AgentPool.md)> |  |  |

### Return type

[**crate::models::AgentPool**](agentPool.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_authorized_info

> crate::models::AuthorizedInfo set_authorized_info(agent_locator, fields, body)
Update the authorization info of the matching agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**AuthorizedInfo**](AuthorizedInfo.md)> |  |  |

### Return type

[**crate::models::AuthorizedInfo**](authorizedInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_build_configuration_run_policy

> crate::models::CompatibilityPolicy set_build_configuration_run_policy(agent_locator, fields, body)
Update build configuration run policy of agent matching locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**CompatibilityPolicy**](CompatibilityPolicy.md)> |  |  |

### Return type

[**crate::models::CompatibilityPolicy**](compatibilityPolicy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_enabled_info

> crate::models::EnabledInfo set_enabled_info(agent_locator, fields, body)
Update the enablement status of the matching agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**EnabledInfo**](EnabledInfo.md)> |  |  |

### Return type

[**crate::models::EnabledInfo**](enabledInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

