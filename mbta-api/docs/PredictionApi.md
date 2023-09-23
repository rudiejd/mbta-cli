# \PredictionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_period_prediction_controller_period_index**](PredictionApi.md#api_web_period_prediction_controller_period_index) | **GET** /predictions | 



## api_web_period_prediction_controller_period_index

> crate::models::Predictions api_web_period_prediction_controller_period_index(page_left_square_bracket_offset_right_square_bracket, page_left_square_bracket_limit_right_square_bracket, sort, fields_left_square_bracket_prediction_right_square_bracket, include, filter_left_square_bracket_latitude_right_square_bracket, filter_left_square_bracket_longitude_right_square_bracket, filter_left_square_bracket_radius_right_square_bracket, filter_left_square_bracket_direction_id_right_square_bracket, filter_left_square_bracket_route_type_right_square_bracket, filter_left_square_bracket_stop_right_square_bracket, filter_left_square_bracket_route_right_square_bracket, filter_left_square_bracket_trip_right_square_bracket, filter_left_square_bracket_route_pattern_right_square_bracket)


**NOTE:** A filter **MUST** be present for any predictions to be returned.  List of predictions for trips.  To get the scheduled times instead of the predictions, use `/schedules`.  The predicted arrival time (`//data/{index}/attributes/arrival_time`) and departure time (`/data/{index}/attributes/departure_time`) to/from a stop (`/data/{index}/relationships/stop/data/id`) at a given sequence (`/data/{index}/attriutes/stop_sequence`) along a trip (`/data/{index}/relationships/trip/data/id`) going a direction (`/data/{index}/attributes/direction_id`) along a route (`/data/{index}/relationships/route/data/id`).  See [GTFS Realtime `FeedMesage` `FeedEntity` `TripUpdate` `TripDescriptor`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-tripdescriptor) See [GTFS Realtime `FeedMesage` `FeedEntity` `TripUpdate` `StopTimeUpdate`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-stoptimeupdate)   ## When a vehicle is predicted to be at a stop  `/predictions?filter[stop]=STOP_ID`  ## The predicted schedule for one route  `/predictions?filter[route]=ROUTE_ID`  ## The predicted schedule for a whole trip  `/predictions?filter[trip]=TRIP_ID`  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_offset_right_square_bracket** | Option<**i32**> | Offset (0-based) of first element in the page |  |
**page_left_square_bracket_limit_right_square_bracket** | Option<**i32**> | Max number of elements to return |  |
**sort** | Option<**String**> | Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any `/data/{index}/attributes` key.  | JSON pointer | Direction | `sort`     | |--------------|-----------|------------| | `/data/{index}/attributes/arrival_time` | ascending | `arrival_time` | | `/data/{index}/attributes/arrival_time` | descending | `-arrival_time` | | `/data/{index}/attributes/departure_time` | ascending | `departure_time` | | `/data/{index}/attributes/departure_time` | descending | `-departure_time` | | `/data/{index}/attributes/direction_id` | ascending | `direction_id` | | `/data/{index}/attributes/direction_id` | descending | `-direction_id` | | `/data/{index}/attributes/schedule_relationship` | ascending | `schedule_relationship` | | `/data/{index}/attributes/schedule_relationship` | descending | `-schedule_relationship` | | `/data/{index}/attributes/status` | ascending | `status` | | `/data/{index}/attributes/status` | descending | `-status` | | `/data/{index}/attributes/stop_sequence` | ascending | `stop_sequence` | | `/data/{index}/attributes/stop_sequence` | descending | `-stop_sequence` |  | `/data/{index}/attributes/arrival_time` if present, otherwise `/data/{index}/attributes/departure_time` | ascending | `time` | | `/data/{index}/attributes/arrival_time` if present, otherwise `/data/{index}/attributes/departure_time` | descending | `-time` |   |  |
**fields_left_square_bracket_prediction_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `schedule` * `stop` * `route` * `trip` * `vehicle` * `alerts`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)  ## Example  `https://api-v3.mbta.com/predictions?filter%5Bstop%5D=place-sstat&filter%5Bdirection_id%5D=0&include=stop` returns predictions from South Station with direction_id=0, below is a truncated response with only relevant fields displayed: ```   {     \"data\": [       {         \"id\": \"prediction-CR-Weekday-Fall-18-743-South Station-02-1\",         \"relationships\": {           \"stop\": {             \"data\": {               \"id\": \"South Station-02\",               \"type\": \"stop\"             }           },         },         \"type\": \"prediction\"       }     ],     \"included\": [       {         \"attributes\": {           \"platform_code\": \"2\",         },         \"id\": \"South Station-02\",         \"type\": \"stop\"       }     ],   } ``` Note the stop relationship; use it to cross-reference  stop-id with the included stops to retrieve the platform_code for the given prediction.  ## Note on trips A Vehicle's `trip` is what is currently being served.  A Prediction also has a `vehicle`: this is the vehicle we predict will serve this trip/stop.  Since we know vehicles make future trips, the trip the vehicle is currently servicing can be different from the trips we're making predictions for.  For example: * Vehicle 1234 is currently serving trip A * The block is Trip A → Trip B → Trip C  We'll be making predictions for the rest of trip A, as well as all the stops of trip B and trip C. The `trip` for the Vehicle is always `A`, and all of the Predictions will reference Vehicle 1234.   |  |
**filter_left_square_bracket_latitude_right_square_bracket** | Option<**String**> |  Latitude/Longitude must be both present or both absent. |  |
**filter_left_square_bracket_longitude_right_square_bracket** | Option<**String**> |  Latitude/Longitude must be both present or both absent. |  |
**filter_left_square_bracket_radius_right_square_bracket** | Option<**String**> |  Radius accepts a floating point number, and the default is 0.01.  For example, if you query for: latitude: 42,  longitude: -71,  radius: 0.05 then you will filter between latitudes 41.95 and 42.05, and longitudes -70.95 and -71.05. |  |
**filter_left_square_bracket_direction_id_right_square_bracket** | Option<**String**> | Filter by direction of travel along the route. Must be used in conjuction with `filter[route]` to apply.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.     |  |
**filter_left_square_bracket_route_type_right_square_bracket** | Option<**String**> | Filter by route_type: https://developers.google.com/transit/gtfs/reference/routes-file.  Multiple `route_type` **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Must be used in conjunction with another filter.  |  |
**filter_left_square_bracket_stop_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/stop/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Parent station IDs are treated as though their child stops were also included.   |  |
**filter_left_square_bracket_route_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/route/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.    |  |
**filter_left_square_bracket_trip_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/trip/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.    |  |
**filter_left_square_bracket_route_pattern_right_square_bracket** | Option<**String**> | Filter by `/included/{index}/relationships/route_pattern/data/id` of a trip. Multiple `route_pattern_id` **MUST** be a comma-separated (U+002C COMMA, \",\") list.  |  |

### Return type

[**crate::models::Predictions**](Predictions.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

