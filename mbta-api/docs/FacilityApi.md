# \FacilityApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_period_facility_controller_period_index**](FacilityApi.md#api_web_period_facility_controller_period_index) | **GET** /facilities | 
[**api_web_period_facility_controller_period_show**](FacilityApi.md#api_web_period_facility_controller_period_show) | **GET** /facilities/{id} | 



## api_web_period_facility_controller_period_index

> crate::models::Facilities api_web_period_facility_controller_period_index(page_left_square_bracket_offset_right_square_bracket, page_left_square_bracket_limit_right_square_bracket, sort, fields_left_square_bracket_facility_right_square_bracket, include, filter_left_square_bracket_stop_right_square_bracket, filter_left_square_bracket_type_right_square_bracket)


List Escalators and Elevators  Amenities at a station stop (`/data/relationships/stop`) such as elevators, escalators, parking lots, and bike storage.  An [MBTA extension](https://groups.google.com/forum/#!topic/gtfs-changes/EzC5m9k45pA).  This spec is not yet finalized.  ## Accessibility  Riders with limited mobility can search any facility, either `ELEVATOR` or `ESCALATOR`, while riders that need wheelchair access can search for `ELEVATOR` only.  The lack of an `ELEVATOR` MAY NOT make a stop wheelchair inaccessible.  Riders should check `/stops/{id}` `/data/attributes/wheelchair_boarding` is `1` to guarantee a path is available from the station entrance to the stop or `0` if it MAY be accessible.  Completely avoid `2` as that is guaranteed to be INACCESSIBLE.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_offset_right_square_bracket** | Option<**i32**> | Offset (0-based) of first element in the page |  |
**page_left_square_bracket_limit_right_square_bracket** | Option<**i32**> | Max number of elements to return |  |
**sort** | Option<**String**> | Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any `/data/{index}/attributes` key. Assumes ascending; may be prefixed with '-' for descending  | JSON pointer | Direction | `sort`     | |--------------|-----------|------------| | `/data/{index}/attributes/latitude` | ascending | `latitude` | | `/data/{index}/attributes/latitude` | descending | `-latitude` | | `/data/{index}/attributes/long_name` | ascending | `long_name` | | `/data/{index}/attributes/long_name` | descending | `-long_name` | | `/data/{index}/attributes/longitude` | ascending | `longitude` | | `/data/{index}/attributes/longitude` | descending | `-longitude` | | `/data/{index}/attributes/properties` | ascending | `properties` | | `/data/{index}/attributes/properties` | descending | `-properties` | | `/data/{index}/attributes/short_name` | ascending | `short_name` | | `/data/{index}/attributes/short_name` | descending | `-short_name` | | `/data/{index}/attributes/type` | ascending | `type` | | `/data/{index}/attributes/type` | descending | `-type` |   |  |
**fields_left_square_bracket_facility_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `stop`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)    |  |
**filter_left_square_bracket_stop_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/stop/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.    |  |
**filter_left_square_bracket_type_right_square_bracket** | Option<**String**> | Filter by type. Multiple types **MUST** be a comma-separated (U+002C COMMA, \",\") list. |  |

### Return type

[**crate::models::Facilities**](Facilities.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_web_period_facility_controller_period_show

> crate::models::Facility api_web_period_facility_controller_period_show(id, fields_left_square_bracket_facility_right_square_bracket, include)


Specific Escalator or Elevator  Amenities at a station stop (`/data/{index}/relationships/stop`) such as elevators, escalators, parking lots, and bike storage.  An [MBTA extension](https://groups.google.com/forum/#!topic/gtfs-changes/EzC5m9k45pA).  This spec is not yet finalized.  ## Accessibility  Riders with limited mobility can search any facility, either `ELEVATOR` or `ESCALATOR`, while riders that need wheelchair access can search for `ELEVATOR` only.  The lack of an `ELEVATOR` MAY NOT make a stop wheelchair inaccessible.  Riders should check `/stops/{id}` `/data/attributes/wheelchair_boarding` is `1` to guarantee a path is available from the station entrance to the stop or `0` if it MAY be accessible.  Completely avoid `2` as that is guaranteed to be INACCESSIBLE.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for facility | [required] |
**fields_left_square_bracket_facility_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `stop`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)    |  |

### Return type

[**crate::models::Facility**](Facility.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

