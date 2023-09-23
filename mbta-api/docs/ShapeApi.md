# \ShapeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_period_shape_controller_period_index**](ShapeApi.md#api_web_period_shape_controller_period_index) | **GET** /shapes | 
[**api_web_period_shape_controller_period_show**](ShapeApi.md#api_web_period_shape_controller_period_show) | **GET** /shapes/{id} | 



## api_web_period_shape_controller_period_index

> crate::models::Shapes api_web_period_shape_controller_period_index(filter_left_square_bracket_route_right_square_bracket, page_left_square_bracket_offset_right_square_bracket, page_left_square_bracket_limit_right_square_bracket, sort, fields_left_square_bracket_shape_right_square_bracket)


**NOTE:** `filter[route]` **MUST** be given for any shapes to be returned.  List of shapes.  ## Vertices  ### World  `/data/{index}/attributes/polyline` is in [Encoded Polyline Algorithm Format](https://developers.google.com/maps/documentation/utilities/polylinealgorithm), which encodes the latitude and longitude of a sequence of points in the shape.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_route_right_square_bracket** | **String** | Filter by `/data/{index}/relationships/route/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.    | [required] |
**page_left_square_bracket_offset_right_square_bracket** | Option<**i32**> | Offset (0-based) of first element in the page |  |
**page_left_square_bracket_limit_right_square_bracket** | Option<**i32**> | Max number of elements to return |  |
**sort** | Option<**String**> | Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any `/data/{index}/attributes` key. Assumes ascending; may be prefixed with '-' for descending  | JSON pointer | Direction | `sort`     | |--------------|-----------|------------| | `/data/{index}/attributes/polyline` | ascending | `polyline` | | `/data/{index}/attributes/polyline` | descending | `-polyline` |   |  |
**fields_left_square_bracket_shape_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |

### Return type

[**crate::models::Shapes**](Shapes.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_web_period_shape_controller_period_show

> crate::models::Shape api_web_period_shape_controller_period_show(id, fields_left_square_bracket_shape_right_square_bracket)


Detail of a particular shape.  ## Vertices  ### World  `/data/attributes/polyline` is in [Encoded Polyline Algorithm Format](https://developers.google.com/maps/documentation/utilities/polylinealgorithm), which encodes the latitude and longitude of a sequence of points in the shape.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for shape | [required] |
**fields_left_square_bracket_shape_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |

### Return type

[**crate::models::Shape**](Shape.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

