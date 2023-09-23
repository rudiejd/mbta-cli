# \VehicleApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_period_vehicle_controller_period_index**](VehicleApi.md#api_web_period_vehicle_controller_period_index) | **GET** /vehicles | 
[**api_web_period_vehicle_controller_period_show**](VehicleApi.md#api_web_period_vehicle_controller_period_show) | **GET** /vehicles/{id} | 



## api_web_period_vehicle_controller_period_index

> crate::models::Vehicles api_web_period_vehicle_controller_period_index(page_left_square_bracket_offset_right_square_bracket, page_left_square_bracket_limit_right_square_bracket, sort, fields_left_square_bracket_vehicle_right_square_bracket, include, filter_left_square_bracket_id_right_square_bracket, filter_left_square_bracket_trip_right_square_bracket, filter_left_square_bracket_label_right_square_bracket, filter_left_square_bracket_route_right_square_bracket, filter_left_square_bracket_direction_id_right_square_bracket, filter_left_square_bracket_route_type_right_square_bracket)


List of vehicles (buses, ferries, and trains)  ## Direction  ### World  To figure out which way the vehicle is pointing at the location, use `/data/{index}/attributes/bearing`.  This can be the compass bearing, or the direction towards the next stop or intermediate location.  ### Trip  To get the direction around the stops in the trip use `/data/{index}/attributes/direction_id`.  ## Location  ### World  Use `/data/{index}/attributes/latitude` and `/data/{index}/attributes/longitude` to get the location of a vehicle.  ### Trip  Use `/data/{index}/attributes/current_stop_sequence` to get the stop number along the trip.  Useful for linear stop indicators.  Position relative to the current stop is in `/data/{index}/attributes/current_status`.  ## Movement  ### World  Use `/data/{index}/attributes/speed` to get the speed of the vehicle in meters per second.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_offset_right_square_bracket** | Option<**i32**> | Offset (0-based) of first element in the page |  |
**page_left_square_bracket_limit_right_square_bracket** | Option<**i32**> | Max number of elements to return |  |
**sort** | Option<**String**> | Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any `/data/{index}/attributes` key. Assumes ascending; may be prefixed with '-' for descending  | JSON pointer | Direction | `sort`     | |--------------|-----------|------------| | `/data/{index}/attributes/bearing` | ascending | `bearing` | | `/data/{index}/attributes/bearing` | descending | `-bearing` | | `/data/{index}/attributes/carriages` | ascending | `carriages` | | `/data/{index}/attributes/carriages` | descending | `-carriages` | | `/data/{index}/attributes/current_status` | ascending | `current_status` | | `/data/{index}/attributes/current_status` | descending | `-current_status` | | `/data/{index}/attributes/current_stop_sequence` | ascending | `current_stop_sequence` | | `/data/{index}/attributes/current_stop_sequence` | descending | `-current_stop_sequence` | | `/data/{index}/attributes/direction_id` | ascending | `direction_id` | | `/data/{index}/attributes/direction_id` | descending | `-direction_id` | | `/data/{index}/attributes/label` | ascending | `label` | | `/data/{index}/attributes/label` | descending | `-label` | | `/data/{index}/attributes/latitude` | ascending | `latitude` | | `/data/{index}/attributes/latitude` | descending | `-latitude` | | `/data/{index}/attributes/longitude` | ascending | `longitude` | | `/data/{index}/attributes/longitude` | descending | `-longitude` | | `/data/{index}/attributes/occupancy_status` | ascending | `occupancy_status` | | `/data/{index}/attributes/occupancy_status` | descending | `-occupancy_status` | | `/data/{index}/attributes/speed` | ascending | `speed` | | `/data/{index}/attributes/speed` | descending | `-speed` | | `/data/{index}/attributes/updated_at` | ascending | `updated_at` | | `/data/{index}/attributes/updated_at` | descending | `-updated_at` |   |  |
**fields_left_square_bracket_vehicle_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `trip` * `stop` * `route`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)  | include | Description                                                                                                                                                                                                                                                                                                                                                  | |---------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------| | `trip`  | The trip which the vehicle is currently operating.                                                                                                                                                                                                                                                                                                           | | `stop`  | The vehicle's current (when `current_status` is **STOPPED_AT**) or *next* stop.                                                                                                                                                                                                                                                                              | | `route` | The one route that is designated for that trip, as in GTFS `trips.txt`.  A trip might also provide service on other routes, identified by the MBTA's `multi_route_trips.txt` GTFS extension. `filter[route]` does consider the multi_route_trips GTFS extension, so it is possible to filter for one route and get a different route included in the response. |   |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<**String**> | Filter by multiple IDs. Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list. Cannot be combined with any other filter. |  |
**filter_left_square_bracket_trip_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/trip/data/id`. Multiple `/data/{index}/relationships/trip/data/id` **MUST** be a comma-separated (U+002C COMMA, \",\") list. Cannot be combined with any other filter. |  |
**filter_left_square_bracket_label_right_square_bracket** | Option<**String**> | Filter by label. Multiple `label` **MUST** be a comma-separated (U+002C COMMA, \",\") list.  |  |
**filter_left_square_bracket_route_right_square_bracket** | Option<**String**> | Filter by route. If the vehicle is on a [multi-route trip](https://groups.google.com/forum/#!msg/massdotdevelopers/1egrhNjT9eA/iy6NFymcCgAJ), it will be returned for any of the routes. Multiple `route_id` **MUST** be a comma-separated (U+002C COMMA, \",\") list.  |  |
**filter_left_square_bracket_direction_id_right_square_bracket** | Option<**String**> | Filter by direction of travel along the route. Must be used in conjuction with `filter[route]` to apply.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.   Only used if `filter[route]` is also present.  |  |
**filter_left_square_bracket_route_type_right_square_bracket** | Option<**String**> | Filter by route_type: https://developers.google.com/transit/gtfs/reference/routes-file.  Multiple `route_type` **MUST** be a comma-separated (U+002C COMMA, \",\") list.    |  |

### Return type

[**crate::models::Vehicles**](Vehicles.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_web_period_vehicle_controller_period_show

> crate::models::Vehicle api_web_period_vehicle_controller_period_show(id, fields_left_square_bracket_vehicle_right_square_bracket, include)


Single vehicle (bus, ferry, or train)  ## Direction  ### World  To figure out which way the vehicle is pointing at the location, use `/data/attributes/bearing`.  This can be the compass bearing, or the direction towards the next stop or intermediate location.  ### Trip  To get the direction around the stops in the trip use `/data/attributes/direction_id`.  ## Location  ### World  Use `/data/attributes/latitude` and `/data/attributes/longitude` to get the location of a vehicle.  ### Trip  Use `/data/attributes/current_stop_sequence` to get the stop number along the trip.  Useful for linear stop indicators.  Position relative to the current stop is in `/data/attributes/current_status`.  ## Movement  ### World  Use `/data/attributes/speed` to get the speed of the vehicle in meters per second.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for a vehicle | [required] |
**fields_left_square_bracket_vehicle_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `trip` * `stop` * `route`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)  | include | Description                                                                                                                                                                                                                                                                                                                                                  | |---------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------| | `trip`  | The trip which the vehicle is currently operating.                                                                                                                                                                                                                                                                                                           | | `stop`  | The vehicle's current (when `current_status` is **STOPPED_AT**) or *next* stop.                                                                                                                                                                                                                                                                              | | `route` | The one route that is designated for that trip, as in GTFS `trips.txt`.  A trip might also provide service on other routes, identified by the MBTA's `multi_route_trips.txt` GTFS extension. `filter[route]` does consider the multi_route_trips GTFS extension, so it is possible to filter for one route and get a different route included in the response. |   |  |

### Return type

[**crate::models::Vehicle**](Vehicle.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

