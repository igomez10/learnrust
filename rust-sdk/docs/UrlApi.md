# \UrlApi

All URIs are relative to *https://socialapp.gomezignacio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_url**](UrlApi.md#create_url) | **POST** /v1/urls | Create a new url
[**delete_url**](UrlApi.md#delete_url) | **DELETE** /v1/urls/{alias} | Delete a url
[**get_url**](UrlApi.md#get_url) | **GET** /v1/urls/{alias} | Get a url
[**get_url_data**](UrlApi.md#get_url_data) | **GET** /v1/urls/{alias}/data | Returns a url metadata



## create_url

> crate::models::Url create_url(url)
Create a new url

Returns a url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | [**Url**](Url.md) | Create a new url | [required] |

### Return type

[**crate::models::Url**](URL.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_url

> delete_url(alias)
Delete a url

Delete a url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** | The alias of the url | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_url

> get_url(alias)
Get a url

Returns a url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** | The alias of the url | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_url_data

> crate::models::Url get_url_data(alias)
Returns a url metadata

Returns a url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** | The alias of the url | [required] |

### Return type

[**crate::models::Url**](URL.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

