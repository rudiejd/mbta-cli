# \RouteApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_period_route_controller_period_index**](RouteApi.md#api_web_period_route_controller_period_index) | **GET** /routes | 
[**api_web_period_route_controller_period_show**](RouteApi.md#api_web_period_route_controller_period_show) | **GET** /routes/{id} | 



## api_web_period_route_controller_period_index

> crate::models::Routes api_web_period_route_controller_period_index(page_left_square_bracket_offset_right_square_bracket, page_left_square_bracket_limit_right_square_bracket, sort, fields_left_square_bracket_route_right_square_bracket, include, filter_left_square_bracket_stop_right_square_bracket, filter_left_square_bracket_type_right_square_bracket, filter_left_square_bracket_direction_id_right_square_bracket, filter_left_square_bracket_date_right_square_bracket, filter_left_square_bracket_id_right_square_bracket)


List of routes.  ## Names and Descriptions  There are 3 attributes with increasing details for naming and describing the route.  1. `/data/{index}/attributes/short_name` 2. `/data/{index}/attributes/long_name` 3. `/data/{index}/attributes/description`  ## Directions  `/data/{index}/attributes/direction_names` is the only place to convert the `direction_id` used throughout the rest of the API to human-readable names.  ## Type  `/data/{index}/attributes/type` corresponds to [GTFS `routes.txt` `route_type`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | Value | Name          | Example    | |-------|---------------|------------| | `0`   | Light Rail    | Green Line | | `1`   | Heavy Rail    | Red Line   | | `2`   | Commuter Rail |            | | `3`   | Bus           |            | | `4`   | Ferry         |            |   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_offset_right_square_bracket** | Option<**i32**> | Offset (0-based) of first element in the page |  |
**page_left_square_bracket_limit_right_square_bracket** | Option<**i32**> | Max number of elements to return |  |
**sort** | Option<**String**> | Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any `/data/{index}/attributes` key. Assumes ascending; may be prefixed with '-' for descending  | JSON pointer | Direction | `sort`     | |--------------|-----------|------------| | `/data/{index}/attributes/color` | ascending | `color` | | `/data/{index}/attributes/color` | descending | `-color` | | `/data/{index}/attributes/description` | ascending | `description` | | `/data/{index}/attributes/description` | descending | `-description` | | `/data/{index}/attributes/direction_destinations` | ascending | `direction_destinations` | | `/data/{index}/attributes/direction_destinations` | descending | `-direction_destinations` | | `/data/{index}/attributes/direction_names` | ascending | `direction_names` | | `/data/{index}/attributes/direction_names` | descending | `-direction_names` | | `/data/{index}/attributes/fare_class` | ascending | `fare_class` | | `/data/{index}/attributes/fare_class` | descending | `-fare_class` | | `/data/{index}/attributes/long_name` | ascending | `long_name` | | `/data/{index}/attributes/long_name` | descending | `-long_name` | | `/data/{index}/attributes/short_name` | ascending | `short_name` | | `/data/{index}/attributes/short_name` | descending | `-short_name` | | `/data/{index}/attributes/sort_order` | ascending | `sort_order` | | `/data/{index}/attributes/sort_order` | descending | `-sort_order` | | `/data/{index}/attributes/text_color` | ascending | `text_color` | | `/data/{index}/attributes/text_color` | descending | `-text_color` | | `/data/{index}/attributes/type` | ascending | `type` | | `/data/{index}/attributes/type` | descending | `-type` |   |  |
**fields_left_square_bracket_route_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `stop` * `line` * `route_patterns`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)  `stop` can only be included when `filter[stop]` is also specified.  |  |
**filter_left_square_bracket_stop_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/stop/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.    |  |
**filter_left_square_bracket_type_right_square_bracket** | Option<**String**> | | Value | Name          | Example    | |-------|---------------|------------| | `0`   | Light Rail    | Green Line | | `1`   | Heavy Rail    | Red Line   | | `2`   | Commuter Rail |            | | `3`   | Bus           |            | | `4`   | Ferry         |            |   Multiple `route_type` **MUST** be a comma-separated (U+002C COMMA, \",\") list.  |  |
**filter_left_square_bracket_direction_id_right_square_bracket** | Option<**String**> | Filter by direction of travel along the route. Must be used in conjuction with `filter[route]` to apply.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.   When combined with stop_id, filters by routes which stop at that stop when traveling in a particular direction  |  |
**filter_left_square_bracket_date_right_square_bracket** | Option<**String**> | Filter by date that route is active The active date is the service date. Trips that begin between midnight and 3am are considered part of the previous service day. The format is ISO8601 with the template of YYYY-MM-DD. |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<**String**> | Filter by multiple IDs. Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list. |  |

### Return type

[**crate::models::Routes**](Routes.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_web_period_route_controller_period_show

> crate::models::Route api_web_period_route_controller_period_show(id, fields_left_square_bracket_route_right_square_bracket, include)


Show a particular route by the route's id.  ## Names and Descriptions  There are 3 attributes with increasing details for naming and describing the route.  1. `/data/attributes/short_name` 2. `/data/attributes/long_name` 3. `/data/attributes/description`  ## Directions  `/data/attributes/direction_names` is the only place to convert the `direction_id` used throughout the rest of the API to human-readable names.  ## Type  `/data/attributes/type` corresponds to [GTFS `routes.txt` `route_type`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | Value | Name          | Example    | |-------|---------------|------------| | `0`   | Light Rail    | Green Line | | `1`   | Heavy Rail    | Red Line   | | `2`   | Commuter Rail |            | | `3`   | Bus           |            | | `4`   | Ferry         |            |   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for route | [required] |
**fields_left_square_bracket_route_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `line` * `route_patterns`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)    |  |

### Return type

[**crate::models::Route**](Route.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

