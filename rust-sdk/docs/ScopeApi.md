# \ScopeApi

All URIs are relative to *https://socialapp.gomezignacio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_scope**](ScopeApi.md#create_scope) | **POST** /v1/scopes | Create a new scope
[**delete_scope**](ScopeApi.md#delete_scope) | **DELETE** /v1/scopes/{id} | Delete a scope
[**get_scope**](ScopeApi.md#get_scope) | **GET** /v1/scopes/{id} | Returns a scope
[**list_scopes**](ScopeApi.md#list_scopes) | **GET** /v1/scopes | Returns a list of scopes
[**update_scope**](ScopeApi.md#update_scope) | **PUT** /v1/scopes/{id} | Update a scope



## create_scope

> crate::models::Scope create_scope(scope)
Create a new scope

Create a new scope

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | [**Scope**](Scope.md) | Create a new scope | [required] |

### Return type

[**crate::models::Scope**](Scope.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scope

> delete_scope(id)
Delete a scope

Delete a scope

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | id of the scope | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scope

> crate::models::Scope get_scope(id)
Returns a scope

Returns a scope

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the scope | [required] |

### Return type

[**crate::models::Scope**](Scope.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scopes

> Vec<crate::models::Scope> list_scopes(limit, offset)
Returns a list of scopes

Returns a list of scopes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## update_scope

> crate::models::Scope update_scope(id, scope)
Update a scope

Update a scope

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | id of the scope | [required] |
**scope** | Option<[**Scope**](Scope.md)> | Update a scope |  |

### Return type

[**crate::models::Scope**](Scope.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

