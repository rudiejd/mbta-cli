# \TripApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_period_trip_controller_period_index**](TripApi.md#api_web_period_trip_controller_period_index) | **GET** /trips | 
[**api_web_period_trip_controller_period_show**](TripApi.md#api_web_period_trip_controller_period_show) | **GET** /trips/{id} | 



## api_web_period_trip_controller_period_index

> crate::models::Trips api_web_period_trip_controller_period_index(page_left_square_bracket_offset_right_square_bracket, page_left_square_bracket_limit_right_square_bracket, sort, fields_left_square_bracket_trip_right_square_bracket, include, filter_left_square_bracket_date_right_square_bracket, filter_left_square_bracket_direction_id_right_square_bracket, filter_left_square_bracket_route_right_square_bracket, filter_left_square_bracket_route_pattern_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, filter_left_square_bracket_name_right_square_bracket)


**NOTE:** A id, route, route_pattern, or name filter **MUST** be present for any trips to be returned.  List of trips, the journies of a particular vehicle through a set of stops on a primary `route` and zero or more alternative `route`s that can be filtered on.  ## Accessibility  Wheelchair accessibility (`/data/{index}/attributes/wheelchair_accessible`) [as defined in GTFS](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt):  | Value | Meaning                                            | |-------|----------------------------------------------------| | `0`   | No information                                     | | `1`   | Accessible (at stops allowing wheelchair_boarding) | | `2`   | Inaccessible                                       |   ## Grouping  Multiple trips **may** be grouped together using `/data/{index}/attributes/block_id`. A block represents a series of trips scheduled to be operated by the same vehicle.  ## Naming  There are 3 names associated with a trip.  | API Field                   | GTFS              | Show users? | |-----------------------------|-------------------|-------------| | `/data/attributes/headsign` | `trip_headsign`   | Yes         | | `/data/attributes/name`     | `trip_short_name` | Yes         | | `/data/id`                  | `trip_id`         | No          |   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_offset_right_square_bracket** | Option<**i32**> | Offset (0-based) of first element in the page |  |
**page_left_square_bracket_limit_right_square_bracket** | Option<**i32**> | Max number of elements to return |  |
**sort** | Option<**String**> | Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any `/data/{index}/attributes` key. Assumes ascending; may be prefixed with '-' for descending  | JSON pointer | Direction | `sort`     | |--------------|-----------|------------| | `/data/{index}/attributes/percentage` | ascending | `percentage` | | `/data/{index}/attributes/percentage` | descending | `-percentage` | | `/data/{index}/attributes/status` | ascending | `status` | | `/data/{index}/attributes/status` | descending | `-status` | | `/data/{index}/attributes/bikes_allowed` | ascending | `bikes_allowed` | | `/data/{index}/attributes/bikes_allowed` | descending | `-bikes_allowed` | | `/data/{index}/attributes/block_id` | ascending | `block_id` | | `/data/{index}/attributes/block_id` | descending | `-block_id` | | `/data/{index}/attributes/direction_id` | ascending | `direction_id` | | `/data/{index}/attributes/direction_id` | descending | `-direction_id` | | `/data/{index}/attributes/headsign` | ascending | `headsign` | | `/data/{index}/attributes/headsign` | descending | `-headsign` | | `/data/{index}/attributes/name` | ascending | `name` | | `/data/{index}/attributes/name` | descending | `-name` | | `/data/{index}/attributes/wheelchair_accessible` | ascending | `wheelchair_accessible` | | `/data/{index}/attributes/wheelchair_accessible` | descending | `-wheelchair_accessible` |   |  |
**fields_left_square_bracket_trip_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `route` * `vehicle` * `service` * `shape` * `predictions` * `route_pattern` * `stops` * `occupancies`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)  | include         | Description | |-----------------|-------------| | `route`         | The *primary* route for the trip. | | `vehicle`       | The vehicle on this trip. | | `service`       | The service controlling when this trip is active. | | `shape`         | The shape of the trip. | | `route_pattern` | The route pattern for the trip. | | `predictions`   | Predictions of when the `vehicle` on this `trip` will arrive at or depart from each stop on the route(s) on the `trip`. | | `stops`         | The stops this trip goes through. | | `occupancies`   | **EXPERIMENTAL:** The trip's static occupancy data. For information on experimental features, see: https://www.mbta.com/developers/v3-api/versioning.|   |  |
**filter_left_square_bracket_date_right_square_bracket** | Option<**String**> | Filter by trips on a particular date The active date is the service date. Trips that begin between midnight and 3am are considered part of the previous service day. The format is ISO8601 with the template of YYYY-MM-DD. |  |
**filter_left_square_bracket_direction_id_right_square_bracket** | Option<**String**> | Filter by direction of travel along the route. Must be used in conjuction with `filter[route]` to apply.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.     |  |
**filter_left_square_bracket_route_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/route/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.    |  |
**filter_left_square_bracket_route_pattern_right_square_bracket** | Option<**String**> | Filter by route pattern IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list. |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<**String**> | Filter by multiple IDs. **MUST** be a comma-separated (U+002C COMMA, \",\") list. |  |
**filter_left_square_bracket_name_right_square_bracket** | Option<**String**> | Filter by multiple names. **MUST** be a comma-separated (U+002C COMMA, \",\") list. |  |

### Return type

[**crate::models::Trips**](Trips.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_web_period_trip_controller_period_show

> crate::models::Trip api_web_period_trip_controller_period_show(id, fields_left_square_bracket_trip_right_square_bracket, include)


Single trip - the journey of a particular vehicle through a set of stops  ## Accessibility  Wheelchair accessibility (`/data/attributes/wheelchair_accessible`) [as defined in GTFS](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt):  | Value | Meaning                                            | |-------|----------------------------------------------------| | `0`   | No information                                     | | `1`   | Accessible (at stops allowing wheelchair_boarding) | | `2`   | Inaccessible                                       |   ## Grouping  Multiple trips **may** be grouped together using `/data/attributes/block_id`. A block represents a series of trips scheduled to be operated by the same vehicle.  ## Naming  There are 3 names associated with a trip.  | API Field                   | GTFS              | Show users? | |-----------------------------|-------------------|-------------| | `/data/attributes/headsign` | `trip_headsign`   | Yes         | | `/data/attributes/name`     | `trip_short_name` | Yes         | | `/data/id`                  | `trip_id`         | No          |   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for a trip | [required] |
**fields_left_square_bracket_trip_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `route` * `vehicle` * `service` * `shape` * `predictions` * `route_pattern` * `stops` * `occupancies`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)  | include         | Description | |-----------------|-------------| | `route`         | The *primary* route for the trip. | | `vehicle`       | The vehicle on this trip. | | `service`       | The service controlling when this trip is active. | | `shape`         | The shape of the trip. | | `route_pattern` | The route pattern for the trip. | | `predictions`   | Predictions of when the `vehicle` on this `trip` will arrive at or depart from each stop on the route(s) on the `trip`. | | `stops`         | The stops this trip goes through. | | `occupancies`   | **EXPERIMENTAL:** The trip's static occupancy data. For information on experimental features, see: https://www.mbta.com/developers/v3-api/versioning.|   |  |

### Return type

[**crate::models::Trip**](Trip.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

