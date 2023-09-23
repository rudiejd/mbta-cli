# \ServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_period_service_controller_period_index**](ServiceApi.md#api_web_period_service_controller_period_index) | **GET** /services | 
[**api_web_period_service_controller_period_show**](ServiceApi.md#api_web_period_service_controller_period_show) | **GET** /services/{id} | 



## api_web_period_service_controller_period_index

> crate::models::Services api_web_period_service_controller_period_index(page_left_square_bracket_offset_right_square_bracket, page_left_square_bracket_limit_right_square_bracket, sort, fields_left_square_bracket_service_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, filter_left_square_bracket_route_right_square_bracket)


List of services. Service represents the days of the week, as well as extra days, that a trip is valid. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_offset_right_square_bracket** | Option<**i32**> | Offset (0-based) of first element in the page |  |
**page_left_square_bracket_limit_right_square_bracket** | Option<**i32**> | Max number of elements to return |  |
**sort** | Option<**String**> | Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any `/data/{index}/attributes` key. Assumes ascending; may be prefixed with '-' for descending  | JSON pointer | Direction | `sort`     | |--------------|-----------|------------| | `/data/{index}/attributes/added_dates` | ascending | `added_dates` | | `/data/{index}/attributes/added_dates` | descending | `-added_dates` | | `/data/{index}/attributes/added_dates_notes` | ascending | `added_dates_notes` | | `/data/{index}/attributes/added_dates_notes` | descending | `-added_dates_notes` | | `/data/{index}/attributes/description` | ascending | `description` | | `/data/{index}/attributes/description` | descending | `-description` | | `/data/{index}/attributes/end_date` | ascending | `end_date` | | `/data/{index}/attributes/end_date` | descending | `-end_date` | | `/data/{index}/attributes/rating_description` | ascending | `rating_description` | | `/data/{index}/attributes/rating_description` | descending | `-rating_description` | | `/data/{index}/attributes/rating_end_date` | ascending | `rating_end_date` | | `/data/{index}/attributes/rating_end_date` | descending | `-rating_end_date` | | `/data/{index}/attributes/rating_start_date` | ascending | `rating_start_date` | | `/data/{index}/attributes/rating_start_date` | descending | `-rating_start_date` | | `/data/{index}/attributes/removed_dates` | ascending | `removed_dates` | | `/data/{index}/attributes/removed_dates` | descending | `-removed_dates` | | `/data/{index}/attributes/removed_dates_notes` | ascending | `removed_dates_notes` | | `/data/{index}/attributes/removed_dates_notes` | descending | `-removed_dates_notes` | | `/data/{index}/attributes/schedule_name` | ascending | `schedule_name` | | `/data/{index}/attributes/schedule_name` | descending | `-schedule_name` | | `/data/{index}/attributes/schedule_type` | ascending | `schedule_type` | | `/data/{index}/attributes/schedule_type` | descending | `-schedule_type` | | `/data/{index}/attributes/schedule_typicality` | ascending | `schedule_typicality` | | `/data/{index}/attributes/schedule_typicality` | descending | `-schedule_typicality` | | `/data/{index}/attributes/start_date` | ascending | `start_date` | | `/data/{index}/attributes/start_date` | descending | `-start_date` | | `/data/{index}/attributes/valid_days` | ascending | `valid_days` | | `/data/{index}/attributes/valid_days` | descending | `-valid_days` |   |  |
**fields_left_square_bracket_service_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<**String**> | Filter by multiple IDs. **MUST** be a comma-separated (U+002C COMMA, \",\") list. |  |
**filter_left_square_bracket_route_right_square_bracket** | Option<**String**> | Filter by route. Multiple `route` **MUST** be a comma-separated (U+002C COMMA, \",\") list. |  |

### Return type

[**crate::models::Services**](Services.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_web_period_service_controller_period_show

> crate::models::Service api_web_period_service_controller_period_show(id, fields_left_square_bracket_service_right_square_bracket)


Single service, which represents the days of the week, as well as extra days, that a trip is valid. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for a service | [required] |
**fields_left_square_bracket_service_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

