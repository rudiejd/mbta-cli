# \StopApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_period_stop_controller_period_index**](StopApi.md#api_web_period_stop_controller_period_index) | **GET** /stops | 
[**api_web_period_stop_controller_period_show**](StopApi.md#api_web_period_stop_controller_period_show) | **GET** /stops/{id} | 



## api_web_period_stop_controller_period_index

> crate::models::Stops api_web_period_stop_controller_period_index(page_left_square_bracket_offset_right_square_bracket, page_left_square_bracket_limit_right_square_bracket, sort, fields_left_square_bracket_stop_right_square_bracket, include, filter_left_square_bracket_date_right_square_bracket, filter_left_square_bracket_direction_id_right_square_bracket, filter_left_square_bracket_latitude_right_square_bracket, filter_left_square_bracket_longitude_right_square_bracket, filter_left_square_bracket_radius_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, filter_left_square_bracket_route_type_right_square_bracket, filter_left_square_bracket_route_right_square_bracket, filter_left_square_bracket_service_right_square_bracket, filter_left_square_bracket_location_type_right_square_bracket)


List stops.  ## Accessibility  Wheelchair boarding (`/data/{index}/attributes/wheelchair_boarding`) corresponds to [GTFS wheelchair_boarding](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stopstxt). The MBTA handles parent station inheritance itself, so value can be treated simply:  | Value | Meaning                                       | |-------|-----------------------------------------------| | `0`   | No Information                                | | `1`   | Accessible (if trip is wheelchair accessible) | | `2`   | Inaccessible                                  |   ## Location  ### World  Use `/data/{index}/attributes/latitude` and `/data/{index}/attributes/longitude` to get the location of a stop.  ### Entrance  The stop may be inside a station.  If `/data/{index}/relationships/parent_station/data/id` is present, you should look up the parent station (`/stops/{parent_id}`) and use its location to give direction first to the parent station and then route from there to the stop.    ### Nearby  The `filter[latitude]` and `filter[longitude]` can be used together to find any stops near that latitude and longitude.  The distance is in degrees as if latitude and longitude were on a flat 2D plane and normal Pythagorean distance was calculated.  Over the region MBTA serves, `0.02` degrees is approximately `1` mile. How close is considered nearby, is controlled by `filter[radius]`, which default to `0.01` degrees (approximately a half mile). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_offset_right_square_bracket** | Option<**i32**> | Offset (0-based) of first element in the page |  |
**page_left_square_bracket_limit_right_square_bracket** | Option<**i32**> | Max number of elements to return |  |
**sort** | Option<**String**> | Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any `/data/{index}/attributes` key. Sorting by distance requires `filter[latitude]` and `filter[longitude]` to be set. Assumes ascending; may be prefixed with '-' for descending.  | JSON pointer | Direction | `sort`     | |--------------|-----------|------------| | `/data/{index}/attributes/address` | ascending | `address` | | `/data/{index}/attributes/address` | descending | `-address` | | `/data/{index}/attributes/at_street` | ascending | `at_street` | | `/data/{index}/attributes/at_street` | descending | `-at_street` | | `/data/{index}/attributes/description` | ascending | `description` | | `/data/{index}/attributes/description` | descending | `-description` | | `/data/{index}/attributes/latitude` | ascending | `latitude` | | `/data/{index}/attributes/latitude` | descending | `-latitude` | | `/data/{index}/attributes/location_type` | ascending | `location_type` | | `/data/{index}/attributes/location_type` | descending | `-location_type` | | `/data/{index}/attributes/longitude` | ascending | `longitude` | | `/data/{index}/attributes/longitude` | descending | `-longitude` | | `/data/{index}/attributes/municipality` | ascending | `municipality` | | `/data/{index}/attributes/municipality` | descending | `-municipality` | | `/data/{index}/attributes/name` | ascending | `name` | | `/data/{index}/attributes/name` | descending | `-name` | | `/data/{index}/attributes/on_street` | ascending | `on_street` | | `/data/{index}/attributes/on_street` | descending | `-on_street` | | `/data/{index}/attributes/platform_code` | ascending | `platform_code` | | `/data/{index}/attributes/platform_code` | descending | `-platform_code` | | `/data/{index}/attributes/platform_name` | ascending | `platform_name` | | `/data/{index}/attributes/platform_name` | descending | `-platform_name` | | `/data/{index}/attributes/vehicle_type` | ascending | `vehicle_type` | | `/data/{index}/attributes/vehicle_type` | descending | `-vehicle_type` | | `/data/{index}/attributes/wheelchair_boarding` | ascending | `wheelchair_boarding` | | `/data/{index}/attributes/wheelchair_boarding` | descending | `-wheelchair_boarding` |  | Distance to (`/data/{index}/attributes/latitude`, `/data/{index}/attributes/longitude`) | ascending | `distance` | | Distance to (`/data/{index}/attributes/latitude`, `/data/{index}/attributes/longitude`) | descending | `-distance` |   |  |
**fields_left_square_bracket_stop_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `child_stops` * `connecting_stops` * `facilities` * `parent_station` * `route`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)  Note that `route` can only be included if `filter[route]` is present and has exactly one `/data/{index}/relationships/route/data/id`.  |  |
**filter_left_square_bracket_date_right_square_bracket** | Option<**String**> | Filter by date when stop is in use. Will be ignored unless filter[route] is present. If filter[service] is present, this filter will be ignored. The active date is the service date. Trips that begin between midnight and 3am are considered part of the previous service day. The format is ISO8601 with the template of YYYY-MM-DD. |  |
**filter_left_square_bracket_direction_id_right_square_bracket** | Option<**String**> | Filter by direction of travel along the route. Must be used in conjuction with `filter[route]` to apply.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.     |  |
**filter_left_square_bracket_latitude_right_square_bracket** | Option<**String**> | Latitude in degrees North in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#A_new_World_Geodetic_System:_WGS.C2.A084) coordinate system to search `filter[radius]` degrees around with `filter[longitude]`.  |  |
**filter_left_square_bracket_longitude_right_square_bracket** | Option<**String**> | Longitude in degrees East in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#Longitudes_on_WGS.C2.A084) coordinate system to search `filter[radius]` degrees around with `filter[latitude]`.  |  |
**filter_left_square_bracket_radius_right_square_bracket** | Option<**f32**> | The distance is in degrees as if latitude and longitude were on a flat 2D plane and normal Pythagorean distance was calculated.  Over the region MBTA serves, `0.02` degrees is approximately `1` mile. Defaults to `0.01` degrees (approximately a half mile).  |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/id` (the stop ID). Multiple `/data/{index}/id` **MUST** be a comma-separated (U+002C COMMA, \",\") list.  |  |
**filter_left_square_bracket_route_type_right_square_bracket** | Option<**String**> | Filter by route_type: https://developers.google.com/transit/gtfs/reference/routes-file.  Multiple `route_type` **MUST** be a comma-separated (U+002C COMMA, \",\") list.    |  |
**filter_left_square_bracket_route_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/route/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.    |  |
**filter_left_square_bracket_service_right_square_bracket** | Option<**String**> | Filter by service_id for which stop is in use. Multiple service_ids **MUST** be a comma-separated (U+002C COMMA, \",\") list.  |  |
**filter_left_square_bracket_location_type_right_square_bracket** | Option<**String**> | Filter by location_type https://github.com/mbta/gtfs-documentation/blob/master/reference/gtfs.md#stopstxt. Multiple location_type **MUST** be a comma-separated (U+002C COMMA, \",\") list.  |  |

### Return type

[**crate::models::Stops**](Stops.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_web_period_stop_controller_period_show

> crate::models::Stop api_web_period_stop_controller_period_show(id, fields_left_square_bracket_stop_right_square_bracket, include)


Detail for a specific stop.  ## Accessibility  Wheelchair boarding (`/data/attributes/wheelchair_boarding`) corresponds to [GTFS wheelchair_boarding](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stopstxt). The MBTA handles parent station inheritance itself, so value can be treated simply:  | Value | Meaning                                       | |-------|-----------------------------------------------| | `0`   | No Information                                | | `1`   | Accessible (if trip is wheelchair accessible) | | `2`   | Inaccessible                                  |   ## Location  ### World  Use `/data/attributes/latitude` and `/data/attributes/longitude` to get the location of a stop.  ### Entrance  The stop may be inside a station.  If `/data/relationships/parent_station/data/id` is present, you should look up the parent station (`/stops/{parent_id}`) and use its location to give direction first to the parent station and then route from there to the stop.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for stop | [required] |
**fields_left_square_bracket_stop_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `child_stops` * `connecting_stops` * `facilities` * `parent_station`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)    |  |

### Return type

[**crate::models::Stop**](Stop.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

