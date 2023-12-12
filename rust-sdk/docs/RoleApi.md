# \RoleApi

All URIs are relative to *https://socialapp.gomezignacio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_scope_to_role**](RoleApi.md#add_scope_to_role) | **POST** /v1/roles/{id}/scopes | Add a scope to a role
[**create_role**](RoleApi.md#create_role) | **POST** /v1/roles | Create a new role
[**delete_role**](RoleApi.md#delete_role) | **DELETE** /v1/roles/{id} | Delete a role
[**get_role**](RoleApi.md#get_role) | **GET** /v1/roles/{id} | Returns a role
[**list_roles**](RoleApi.md#list_roles) | **GET** /v1/roles | Returns a list of roles
[**list_scopes_for_role**](RoleApi.md#list_scopes_for_role) | **GET** /v1/roles/{id}/scopes | Returns a list of scopes for a role
[**remove_scope_from_role**](RoleApi.md#remove_scope_from_role) | **DELETE** /v1/roles/{role_id}/scopes/{scope_id} | Remove a scope from a role
[**update_role**](RoleApi.md#update_role) | **PUT** /v1/roles/{id} | Update a role



## add_scope_to_role

> add_scope_to_role(id, request_body)
Add a scope to a role

Add a scope to a role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the role | [required] |
**request_body** | [**Vec<String>**](String.md) | Add a scope to a role | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role

> crate::models::Role create_role(role)
Create a new role

Create a new role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | [**Role**](Role.md) | Create a new role | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> delete_role(id)
Delete a role

Delete a role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | id of the role | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role

> crate::models::Role get_role(id)
Returns a role

Returns a role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the role | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_roles

> Vec<crate::models::Role> list_roles(limit, offset)
Returns a list of roles

Returns a list of roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The numbers of roles to return |  |[default to 20]
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result |  |

### Return type

[**Vec<crate::models::Role>**](Role.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scopes_for_role

> Vec<crate::models::Scope> list_scopes_for_role(id, limit, offset)
Returns a list of scopes for a role

Returns a list of scopes for a role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the role | [required] |
**limit** | Option<**i32**> | The numbers of scopes to return |  |[default to 20]
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result |  |

### Return type

[**Vec<crate::models::Scope>**](Scope.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_scope_from_role

> remove_scope_from_role(role_id, scope_id)
Remove a scope from a role

Remove a scope from a role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i32** | The id of the role | [required] |
**scope_id** | **i32** | The id of the scope | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role

> crate::models::Role update_role(id, role)
Update a role

Update a role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | id of the role | [required] |
**role** | Option<[**Role**](Role.md)> | Update a role |  |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

