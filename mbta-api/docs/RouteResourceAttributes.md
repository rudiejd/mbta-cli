# RouteResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**i32**> | | Value | Name          | Example    | |-------|---------------|------------| | `0`   | Light Rail    | Green Line | | `1`   | Heavy Rail    | Red Line   | | `2`   | Commuter Rail |            | | `3`   | Bus           |            | | `4`   | Ferry         |            |  | [optional]
**text_color** | Option<**String**> | A legible color to use for text drawn against a background of the route's `color` attribute. See [GTFS `routes.txt` `route_text_color`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | [optional]
**sort_order** | Option<**i32**> | Routes sort in ascending order | [optional]
**short_name** | Option<**String**> | This will often be a short, abstract identifier like \"32\", \"100X\", or \"Green\" that riders use to identify a route, but which doesn't give any indication of what places the route serves. See [GTFS `routes.txt` `route_short_name`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | [optional]
**long_name** | Option<**String**> | The full name of a route. This name is generally more descriptive than the `short_name` and will often include the route's destination or stop. See [GTFS `routes.txt` `route_long_name`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | [optional]
**fare_class** | Option<**String**> | Specifies the fare type of the route, which can differ from the service category.  | [optional]
**direction_names** | Option<**Vec<String>**> |  | [optional]
**direction_destinations** | Option<**Vec<String>**> |  | [optional]
**description** | Option<**String**> | Details about stops, schedule, and/or service.  See [GTFS `routes.txt` `route_desc`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | [optional]
**color** | Option<**String**> | A color that corresponds to the route, such as the line color on a map.\" See [GTFS `routes.txt` `route_color`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


