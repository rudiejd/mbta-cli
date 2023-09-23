# \ScheduleApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_period_schedule_controller_period_index**](ScheduleApi.md#api_web_period_schedule_controller_period_index) | **GET** /schedules | 



## api_web_period_schedule_controller_period_index

> crate::models::Schedules api_web_period_schedule_controller_period_index(page_left_square_bracket_offset_right_square_bracket, page_left_square_bracket_limit_right_square_bracket, sort, fields_left_square_bracket_schedule_right_square_bracket, include, filter_left_square_bracket_date_right_square_bracket, filter_left_square_bracket_direction_id_right_square_bracket, filter_left_square_bracket_route_type_right_square_bracket, filter_left_square_bracket_min_time_right_square_bracket, filter_left_square_bracket_max_time_right_square_bracket, filter_left_square_bracket_route_right_square_bracket, filter_left_square_bracket_stop_right_square_bracket, filter_left_square_bracket_trip_right_square_bracket, filter_left_square_bracket_stop_sequence_right_square_bracket)


**NOTE:** `filter[route]`, `filter[stop]`, or `filter[trip]` **MUST** be present for any schedules to be returned.  List of schedules.  To get a realtime prediction instead of the scheduled times, use `/predictions`.  A schedule is the arrival drop off (`/data/{index}/attributes/drop_off_type`) time (`/data/{index}/attributes/arrival_time`) and departure pick up (`/data/{index}/attributes/pickup_type`) time (`/data/{index}/attributes/departure_time`) to/from a stop (`/data/{index}/relationships/stop/data/id`) at a given sequence (`/data/{index}/attributes/stop_sequence`) along a trip (`/data/{index}/relationships/trip/data/id`) going in a direction (`/data/{index}/attributes/direction_id`) on a route (`/data/{index}/relationships/route/data/id`) when the trip is following a service (`/data/{index}/relationships/service/data/id`) to determine when it is active.  See [GTFS `stop_times.txt`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt) for base specification.   ## When a vehicle is scheduled to be at a stop  `/schedules?filter[stop]=STOP_ID`  ## The schedule for one route  `/schedules?filter[route]=ROUTE_ID`  ### When a route is open  Query for the `first` and `last` stops on the route.  `/schedules?filter[route]=ROUTE_ID&filter[stop_sequence]=first,last`  ## The schedule for a whole trip  `/schedule?filter[trip]=TRIP_ID`  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_offset_right_square_bracket** | Option<**i32**> | Offset (0-based) of first element in the page |  |
**page_left_square_bracket_limit_right_square_bracket** | Option<**i32**> | Max number of elements to return |  |
**sort** | Option<**String**> | Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any `/data/{index}/attributes` key.  | JSON pointer | Direction | `sort`     | |--------------|-----------|------------| | `/data/{index}/attributes/arrival_time` | ascending | `arrival_time` | | `/data/{index}/attributes/arrival_time` | descending | `-arrival_time` | | `/data/{index}/attributes/departure_time` | ascending | `departure_time` | | `/data/{index}/attributes/departure_time` | descending | `-departure_time` | | `/data/{index}/attributes/direction_id` | ascending | `direction_id` | | `/data/{index}/attributes/direction_id` | descending | `-direction_id` | | `/data/{index}/attributes/drop_off_type` | ascending | `drop_off_type` | | `/data/{index}/attributes/drop_off_type` | descending | `-drop_off_type` | | `/data/{index}/attributes/pickup_type` | ascending | `pickup_type` | | `/data/{index}/attributes/pickup_type` | descending | `-pickup_type` | | `/data/{index}/attributes/stop_headsign` | ascending | `stop_headsign` | | `/data/{index}/attributes/stop_headsign` | descending | `-stop_headsign` | | `/data/{index}/attributes/stop_sequence` | ascending | `stop_sequence` | | `/data/{index}/attributes/stop_sequence` | descending | `-stop_sequence` | | `/data/{index}/attributes/timepoint` | ascending | `timepoint` | | `/data/{index}/attributes/timepoint` | descending | `-timepoint` |  | `/data/{index}/attributes/arrival_time` if present, otherwise `/data/{index}/attributes/departure_time` | ascending | `time` | | `/data/{index}/attributes/arrival_time` if present, otherwise `/data/{index}/attributes/departure_time` | descending | `-time` |   |  |
**fields_left_square_bracket_schedule_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `stop` * `trip` * `prediction` * `route`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)    |  |
**filter_left_square_bracket_date_right_square_bracket** | Option<**String**> | Filter schedule by date that they are active. The active date is the service date. Trips that begin between midnight and 3am are considered part of the previous service day. The format is ISO8601 with the template of YYYY-MM-DD. |  |
**filter_left_square_bracket_direction_id_right_square_bracket** | Option<**String**> | Filter by direction of travel along the route. Must be used in conjuction with `filter[route]` to apply.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.     |  |
**filter_left_square_bracket_route_type_right_square_bracket** | Option<**String**> | Filter by route_type: https://developers.google.com/transit/gtfs/reference/routes-file.  Multiple `route_type` **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Must be used in conjunction with another filter.  |  |
**filter_left_square_bracket_min_time_right_square_bracket** | Option<**String**> | Time before which schedule should not be returned. To filter times after midnight use more than 24 hours. For example, min_time=24:00 will return schedule information for the next calendar day, since that service is considered part of the current service day. Additionally, min_time=00:00&max_time=02:00 will not return anything. The time format is HH:MM. |  |
**filter_left_square_bracket_max_time_right_square_bracket** | Option<**String**> | Time after which schedule should not be returned. To filter times after midnight use more than 24 hours. For example, min_time=24:00 will return schedule information for the next calendar day, since that service is considered part of the current service day. Additionally, min_time=00:00&max_time=02:00 will not return anything. The time format is HH:MM. |  |
**filter_left_square_bracket_route_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/route/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.    |  |
**filter_left_square_bracket_stop_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/stop/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Parent station IDs are treated as though their child stops were also included.   |  |
**filter_left_square_bracket_trip_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/trip/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.    |  |
**filter_left_square_bracket_stop_sequence_right_square_bracket** | Option<**String**> | Filter by the index of the stop in the trip.  Symbolic values `first` and `last` can be used instead of numeric sequence number too.  |  |

### Return type

[**crate::models::Schedules**](Schedules.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

