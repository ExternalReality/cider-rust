# \UserApi

All URIs are relative to *http://localhost:8111*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_role_to_user**](UserApi.md#add_role_to_user) | **post** /app/rest/users/{userLocator}/roles | Add a role to the matching user.
[**add_role_to_user_at_scope**](UserApi.md#add_role_to_user_at_scope) | **put** /app/rest/users/{userLocator}/roles/{roleId}/{scope} | Add a role with the specific scope to the matching user.
[**add_user**](UserApi.md#add_user) | **post** /app/rest/users | Create a new user.
[**add_user_token**](UserApi.md#add_user_token) | **post** /app/rest/users/{userLocator}/tokens | Create a new authentication token for the matching user.
[**delete_user**](UserApi.md#delete_user) | **delete** /app/rest/users/{userLocator} | Delete user matching the locator.
[**delete_user_field**](UserApi.md#delete_user_field) | **delete** /app/rest/users/{userLocator}/{field} | Remove a property of the matching user.
[**delete_user_token**](UserApi.md#delete_user_token) | **delete** /app/rest/users/{userLocator}/tokens/{name} | Remove an authentication token from the matching user.
[**ger_user_field**](UserApi.md#ger_user_field) | **get** /app/rest/users/{userLocator}/{field} | Get a field of the matching user.
[**get_all_user_groups**](UserApi.md#get_all_user_groups) | **get** /app/rest/users/{userLocator}/groups | Get all groups of the matching user.
[**get_all_user_roles**](UserApi.md#get_all_user_roles) | **get** /app/rest/users/{userLocator}/roles | Get all user roles of the matching user.
[**get_all_users**](UserApi.md#get_all_users) | **get** /app/rest/users | Get all users.
[**get_user**](UserApi.md#get_user) | **get** /app/rest/users/{userLocator} | Get user matching the locator.
[**get_user_group**](UserApi.md#get_user_group) | **get** /app/rest/users/{userLocator}/groups/{groupLocator} | Get a user group of the matching user.
[**get_user_permissions**](UserApi.md#get_user_permissions) | **get** /app/rest/users/{userLocator}/permissions | Get all permissions effective for the matching user.
[**get_user_properties**](UserApi.md#get_user_properties) | **get** /app/rest/users/{userLocator}/properties | Get all properties of the matching user.
[**get_user_property**](UserApi.md#get_user_property) | **get** /app/rest/users/{userLocator}/properties/{name} | Get a property of the matching user.
[**get_user_roles_at_scope**](UserApi.md#get_user_roles_at_scope) | **get** /app/rest/users/{userLocator}/roles/{roleId}/{scope} | Get a user role with the specific scope from the matching user.
[**get_user_tokens**](UserApi.md#get_user_tokens) | **get** /app/rest/users/{userLocator}/tokens | Get all authentication tokens of the matching user.
[**remove_user_from_group**](UserApi.md#remove_user_from_group) | **delete** /app/rest/users/{userLocator}/groups/{groupLocator} | Remove the matching user from the specific group.
[**remove_user_property**](UserApi.md#remove_user_property) | **delete** /app/rest/users/{userLocator}/properties/{name} | Remove a property of the matching user.
[**remove_user_remember_me**](UserApi.md#remove_user_remember_me) | **delete** /app/rest/users/{userLocator}/debug/rememberMe | Remove the RememberMe data of the matching user.
[**remove_user_role_at_scope**](UserApi.md#remove_user_role_at_scope) | **delete** /app/rest/users/{userLocator}/roles/{roleId}/{scope} | Remove a role with the specific scope from the matching user.
[**replace_user**](UserApi.md#replace_user) | **put** /app/rest/users/{userLocator} | Update user matching the locator.
[**set_user_field**](UserApi.md#set_user_field) | **put** /app/rest/users/{userLocator}/{field} | Update a field of the matching user.
[**set_user_groups**](UserApi.md#set_user_groups) | **put** /app/rest/users/{userLocator}/groups | Update groups of the matching user.
[**set_user_property**](UserApi.md#set_user_property) | **put** /app/rest/users/{userLocator}/properties/{name} | Update a property of the matching user.
[**set_user_roles**](UserApi.md#set_user_roles) | **put** /app/rest/users/{userLocator}/roles | Update user roles of the matching user.



## add_role_to_user

> crate::models::Role add_role_to_user(user_locator, body)
Add a role to the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**body** | Option<[**Role**](Role.md)> |  |  |

### Return type

[**crate::models::Role**](role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_role_to_user_at_scope

> crate::models::Role add_role_to_user_at_scope(user_locator, role_id, scope)
Add a role with the specific scope to the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**scope** | **String** |  | [required] |

### Return type

[**crate::models::Role**](role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user

> crate::models::User add_user(fields, body)
Create a new user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**User**](User.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_token

> crate::models::Token add_user_token(user_locator, fields, body)
Create a new authentication token for the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Token**](Token.md)> |  |  |

### Return type

[**crate::models::Token**](token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(user_locator)
Delete user matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_field

> delete_user_field(user_locator, field)
Remove a property of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_token

> delete_user_token(user_locator, name)
Remove an authentication token from the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ger_user_field

> String ger_user_field(user_locator, field)
Get a field of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_user_groups

> crate::models::Groups get_all_user_groups(user_locator, fields)
Get all groups of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Groups**](groups.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_user_roles

> crate::models::Roles get_all_user_roles(user_locator)
Get all user roles of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |

### Return type

[**crate::models::Roles**](roles.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_users

> crate::models::Users get_all_users(locator, fields)
Get all users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Users**](users.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::User get_user(user_locator, fields)
Get user matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group

> crate::models::Group get_user_group(user_locator, group_locator, fields)
Get a user group of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**group_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Group**](group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_permissions

> crate::models::PermissionAssignments get_user_permissions(user_locator, locator, fields)
Get all permissions effective for the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::PermissionAssignments**](permissionAssignments.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_properties

> crate::models::Properties get_user_properties(user_locator, fields)
Get all properties of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_property

> String get_user_property(user_locator, name)
Get a property of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_roles_at_scope

> crate::models::Role get_user_roles_at_scope(user_locator, role_id, scope)
Get a user role with the specific scope from the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**scope** | **String** |  | [required] |

### Return type

[**crate::models::Role**](role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_tokens

> crate::models::Tokens get_user_tokens(user_locator, fields)
Get all authentication tokens of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**crate::models::Tokens**](tokens.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_group

> remove_user_from_group(user_locator, group_locator, fields)
Remove the matching user from the specific group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**group_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_property

> remove_user_property(user_locator, name)
Remove a property of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_remember_me

> remove_user_remember_me(user_locator)
Remove the RememberMe data of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_role_at_scope

> remove_user_role_at_scope(user_locator, role_id, scope)
Remove a role with the specific scope from the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**scope** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_user

> crate::models::User replace_user(user_locator, fields, body)
Update user matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**User**](User.md)> |  |  |

### Return type

[**crate::models::User**](user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_field

> String set_user_field(user_locator, field, body)
Update a field of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
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


## set_user_groups

> crate::models::Groups set_user_groups(user_locator, fields, body)
Update groups of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Groups**](Groups.md)> |  |  |

### Return type

[**crate::models::Groups**](groups.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_property

> String set_user_property(user_locator, name, body)
Update a property of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
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


## set_user_roles

> crate::models::Roles set_user_roles(user_locator, body)
Update user roles of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**body** | Option<[**Roles**](Roles.md)> |  |  |

### Return type

[**crate::models::Roles**](roles.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

