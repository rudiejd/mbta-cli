# \RoutePatternApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_period_route_pattern_controller_period_index**](RoutePatternApi.md#api_web_period_route_pattern_controller_period_index) | **GET** /route_patterns | 
[**api_web_period_route_pattern_controller_period_show**](RoutePatternApi.md#api_web_period_route_pattern_controller_period_show) | **GET** /route_patterns/{id} | 



## api_web_period_route_pattern_controller_period_index

> crate::models::RoutePattern api_web_period_route_pattern_controller_period_index(page_left_square_bracket_offset_right_square_bracket, page_left_square_bracket_limit_right_square_bracket, sort, fields_left_square_bracket_route_pattern_right_square_bracket, include, filter_left_square_bracket_id_right_square_bracket, filter_left_square_bracket_route_right_square_bracket, filter_left_square_bracket_direction_id_right_square_bracket, filter_left_square_bracket_stop_right_square_bracket, filter_left_square_bracket_canonical_right_square_bracket)


List of route patterns.  Route patterns are used to describe the subsets of a route, representing different possible patterns of where trips may serve. For example, a bus route may have multiple branches, and each branch may be modeled as a separate route pattern per direction. Hierarchically, the route pattern level may be considered to be larger than the trip level and smaller than the route level.  For most MBTA modes, a route pattern will typically represent a unique set of stops that may be served on a route-trip combination. Seasonal schedule changes may result in trips within a route pattern having different routings. In simple changes, such a single bus stop removed or added between one schedule rating and the next (for example, between the Summer and Fall schedules), trips will be maintained on the same route_pattern_id. If the changes are significant, a new route_pattern_id may be introduced.  For Commuter Rail, express or skip-stop trips use the same route pattern as local trips. Some branches do have multiple route patterns when the train takes a different path. For example, `CR-Providence` has two route patterns per direction, one for the Wickford Junction branch and the other for the Stoughton branch.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_offset_right_square_bracket** | Option<**i32**> | Offset (0-based) of first element in the page |  |
**page_left_square_bracket_limit_right_square_bracket** | Option<**i32**> | Max number of elements to return |  |
**sort** | Option<**String**> | Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any `/data/{index}/attributes` key. Assumes ascending; may be prefixed with '-' for descending  | JSON pointer | Direction | `sort`     | |--------------|-----------|------------| | `/data/{index}/attributes/canonical` | ascending | `canonical` | | `/data/{index}/attributes/canonical` | descending | `-canonical` | | `/data/{index}/attributes/direction_id` | ascending | `direction_id` | | `/data/{index}/attributes/direction_id` | descending | `-direction_id` | | `/data/{index}/attributes/name` | ascending | `name` | | `/data/{index}/attributes/name` | descending | `-name` | | `/data/{index}/attributes/sort_order` | ascending | `sort_order` | | `/data/{index}/attributes/sort_order` | descending | `-sort_order` | | `/data/{index}/attributes/time_desc` | ascending | `time_desc` | | `/data/{index}/attributes/time_desc` | descending | `-time_desc` | | `/data/{index}/attributes/typicality` | ascending | `typicality` | | `/data/{index}/attributes/typicality` | descending | `-typicality` |   |  |
**fields_left_square_bracket_route_pattern_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `route` * `representative_trip`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)  | include | Description | |-|-| | `route` | The route that this pattern belongs to. | | `representative_trip` | A trip that can be considered a canonical trip for the route pattern. This trip can be used to deduce a pattern's canonical set of stops and shape. |   |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<**String**> | Filter by multiple IDs. **MUST** be a comma-separated (U+002C COMMA, \",\") list. |  |
**filter_left_square_bracket_route_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/route/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.    |  |
**filter_left_square_bracket_direction_id_right_square_bracket** | Option<**String**> | Filter by direction of travel along the route. Must be used in conjuction with `filter[route]` to apply.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.     |  |
**filter_left_square_bracket_stop_right_square_bracket** | Option<**String**> | Filter by `/data/{index}/relationships/stop/data/id`.  Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Parent station IDs are treated as though their child stops were also included.   |  |
**filter_left_square_bracket_canonical_right_square_bracket** | Option<**bool**> | Filter by canonical  true: Route pattern should be considered canonical for this route in this direction. If branching regularly occurs, this route-direction may have more than one canonical pattern. false: Route pattern should be not considered canonical for this route in this direction.    |  |

### Return type

[**crate::models::RoutePattern**](RoutePattern.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_web_period_route_pattern_controller_period_show

> crate::models::RoutePattern api_web_period_route_pattern_controller_period_show(id, fields_left_square_bracket_route_pattern_right_square_bracket, include)


Show a particular route_pattern by the route's id.  Route patterns are used to describe the subsets of a route, representing different possible patterns of where trips may serve. For example, a bus route may have multiple branches, and each branch may be modeled as a separate route pattern per direction. Hierarchically, the route pattern level may be considered to be larger than the trip level and smaller than the route level.  For most MBTA modes, a route pattern will typically represent a unique set of stops that may be served on a route-trip combination. Seasonal schedule changes may result in trips within a route pattern having different routings. In simple changes, such a single bus stop removed or added between one schedule rating and the next (for example, between the Summer and Fall schedules), trips will be maintained on the same route_pattern_id. If the changes are significant, a new route_pattern_id may be introduced.  For Commuter Rail, express or skip-stop trips use the same route pattern as local trips. Some branches do have multiple route patterns when the train takes a different path. For example, `CR-Providence` has two route patterns per direction, one for the Wickford Junction branch and the other for the Stoughton branch.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for route_pattern | [required] |
**fields_left_square_bracket_route_pattern_right_square_bracket** | Option<**String**> | Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \",\") list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  |  |
**include** | Option<**String**> | Relationships to include.  * `route` * `representative_trip`  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \",\") list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \".\") list of relationship names. [JSONAPI \"include\" behavior](http://jsonapi.org/format/#fetching-includes)  | include | Description | |-|-| | `route` | The route that this pattern belongs to. | | `representative_trip` | A trip that can be considered a canonical trip for the route pattern. This trip can be used to deduce a pattern's canonical set of stops and shape. |   |  |

### Return type

[**crate::models::RoutePattern**](RoutePattern.md)

### Authorization

[api_key_in_query](../README.md#api_key_in_query), [api_key_in_header](../README.md#api_key_in_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

