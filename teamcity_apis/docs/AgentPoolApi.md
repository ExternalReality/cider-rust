# \AgentPoolApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_agent_to_agent_pool**](AgentPoolApi.md#add_agent_to_agent_pool) | **post** /app/rest/agentPools/{agentPoolLocator}/agents | Assign the agent to the matching agent pool.
[**add_project_to_agent_pool**](AgentPoolApi.md#add_project_to_agent_pool) | **post** /app/rest/agentPools/{agentPoolLocator}/projects | Assign the project to the matching agent pool.
[**create_agent_pool**](AgentPoolApi.md#create_agent_pool) | **post** /app/rest/agentPools | Create a new agent pool.
[**delete_agent_pool**](AgentPoolApi.md#delete_agent_pool) | **delete** /app/rest/agentPools/{agentPoolLocator} | Delete the agent pool matching the locator.
[**delete_all_projects_from_agent_pool**](AgentPoolApi.md#delete_all_projects_from_agent_pool) | **delete** /app/rest/agentPools/{agentPoolLocator}/projects | Unassign all projects from the matching agent pool.
[**delete_project_from_agent_pool**](AgentPoolApi.md#delete_project_from_agent_pool) | **delete** /app/rest/agentPools/{agentPoolLocator}/projects/{projectLocator} | Unassign the project from the matching agent pool.
[**get_agent_pool_of_agent_pool**](AgentPoolApi.md#get_agent_pool_of_agent_pool) | **get** /app/rest/agentPools/{agentPoolLocator} | Get the agent pool matching the locator.
[**get_all_agent_pools**](AgentPoolApi.md#get_all_agent_pools) | **get** /app/rest/agentPools | Get all agent pools.
[**get_all_agents_from_agent_pool**](AgentPoolApi.md#get_all_agents_from_agent_pool) | **get** /app/rest/agentPools/{agentPoolLocator}/agents | Get the agent of the matching agent pool.
[**get_all_projects_from_agent_pool**](AgentPoolApi.md#get_all_projects_from_agent_pool) | **get** /app/rest/agentPools/{agentPoolLocator}/projects | Get all projects of the matching agent pool.
[**get_field_from_agent_pool**](AgentPoolApi.md#get_field_from_agent_pool) | **get** /app/rest/agentPools/{agentPoolLocator}/{field} | Get a field of the matching agent pool.
[**set_agent_pool_field**](AgentPoolApi.md#set_agent_pool_field) | **put** /app/rest/agentPools/{agentPoolLocator}/{field} | Update a field of the matching agent pool.
[**set_agent_pool_projects**](AgentPoolApi.md#set_agent_pool_projects) | **put** /app/rest/agentPools/{agentPoolLocator}/projects | Update projects of the matching agent pool.



## add_agent_to_agent_pool

> crate::models::Agent add_agent_to_agent_pool(agent_pool_locator, fields, body)
Assign the agent to the matching agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_pool_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Agent**](Agent.md)> |  |  |

### Return type

[**crate::models::Agent**](agent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_project_to_agent_pool

> crate::models::Project add_project_to_agent_pool(agent_pool_locator, body)
Assign the project to the matching agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_pool_locator** | **String** |  | [required] |
**body** | Option<[**Project**](Project.md)> |  |  |

### Return type

[**crate::models::Project**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_agent_pool

> crate::models::AgentPool create_agent_pool(body)
Create a new agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**AgentPool**](AgentPool.md)> |  |  |

### Return type

[**crate::models::AgentPool**](agentPool.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agent_pool

> delete_agent_pool(agent_pool_locator)
Delete the agent pool matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_pool_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_projects_from_agent_pool

> delete_all_projects_from_agent_pool(agent_pool_locator)
Unassign all projects from the matching agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_pool_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_from_agent_pool

> delete_project_from_agent_pool(agent_pool_locator, project_locator)
Unassign the project from the matching agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_pool_locator** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_agent_pool_of_agent_pool

> crate::models::AgentPool get_agent_pool_of_agent_pool(agent_pool_locator, fields)
Get the agent pool matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_pool_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AgentPool**](agentPool.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_agent_pools

> crate::models::AgentPools get_all_agent_pools(locator, fields)
Get all agent pools.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::AgentPools**](agentPools.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_agents_from_agent_pool

> crate::models::Agents get_all_agents_from_agent_pool(agent_pool_locator, locator, fields)
Get the agent of the matching agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_pool_locator** | **String** |  | [required] |
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


## get_all_projects_from_agent_pool

> crate::models::Projects get_all_projects_from_agent_pool(agent_pool_locator, fields)
Get all projects of the matching agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_pool_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Projects**](projects.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_field_from_agent_pool

> String get_field_from_agent_pool(agent_pool_locator, field)
Get a field of the matching agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_pool_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_agent_pool_field

> String set_agent_pool_field(agent_pool_locator, field, body)
Update a field of the matching agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_pool_locator** | **String** |  | [required] |
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


## set_agent_pool_projects

> crate::models::Projects set_agent_pool_projects(agent_pool_locator, body)
Update projects of the matching agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_pool_locator** | **String** |  | [required] |
**body** | Option<[**Projects**](Projects.md)> |  |  |

### Return type

[**crate::models::Projects**](projects.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

