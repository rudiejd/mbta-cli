# \LiveFacilityApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_period_live_facility_controller_period_index**](LiveFacilityApi.md#api_web_period_live_facility_controller_period_index) | **GET** /live_facilities | 
[**api_web_period_live_facility_controller_period_show**](LiveFacilityApi.md#api_web_period_live_facility_controller_period_show) | **GET** /live_facilities/{id} | 



## api_web_period_live_facility_controller_period_index

> crate::models::LiveFacility api_web_period_live_facility_controller_period_index(page_left_square_bracket_offset_right_square_bracket, page_left_square_bracket_limit_right_square_bracket, sort, include, filter_left_square_bracket_id_right_square_bracket)


Live Facility Data  Live data about a given facility.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_offset_right_square_bracket** | Option<**i32**> | Offset (0-based) of first element in the page |  |
**page_left_square_bracket_limit_right_square_bracket** | Option<**i32**> | Max number of elements to return |  |
**sort** | Option<**String**> | Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any `/data/{index}/attributes` key. Assumes ascending; may be prefixed with '-' for descending  | JSON pointer | Direction | `sort`     | |--------------|-----------|------------| | `/data/{index}/attributes/properties` | ascending | `properties` | | `/data/{index}/attributes/properties` | descending | `-properties` | | `/data/{index}/attributes/updated_at` | ascending | `updated_at` | | `/data/{index}/attributes/updated_at` | descending | `-updated_at` |   |  |
**include** | Option<**String**> | Relationships to include.  * `facility`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)    |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<**String**> | Filter by multiple parking facility ids. **MUST** be a comma-separated (U+002C COMMA, \",\") list. |  |

### Return type

[**crate::models::LiveFacility**](LiveFacility.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_web_period_live_facility_controller_period_show

> crate::models::LiveFacility api_web_period_live_facility_controller_period_show(id, include)


List live parking data for specific parking facility  Live data about a given facility.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for facility | [required] |
**include** | Option<**String**> | Relationships to include.  * `facility`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)    |  |

### Return type

[**crate::models::LiveFacility**](LiveFacility.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

