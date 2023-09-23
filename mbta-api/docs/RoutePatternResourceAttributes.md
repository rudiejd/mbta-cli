# RoutePatternResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**typicality** | Option<**i32**> | Explains how common the route pattern is. For the MBTA, this is within the context of the entire route. Current valid values are: | Value | Description | |-|-| | `0` | Not defined | | `1` | Typical. Pattern is common for the route. Most routes will have only one such pattern per direction. A few routes may have more than 1, such as the Red Line (with one branch to Ashmont and another to Braintree); routes with more than 2 are rare. | | `2` | Pattern is a deviation from the regular route. | | `3` | Pattern represents a highly atypical pattern for the route, such as a special routing which only runs a handful of times per day. | | `4` | Diversions from normal service, such as planned detours, bus shuttles, or snow routes. | | `5` | Canonical trip patterns. |  | [optional]
**sort_order** | Option<**i32**> | Can be used to order the route patterns in a way which is ideal for presentation to customers. Route patterns with smaller sort_order values should be displayed before those with larger values.  | [optional]
**name** | Option<**String**> | User-facing description of where trips on the route pattern serve. These names are published in the form Destination, Destination via Street or Landmark, Origin - Destination, or Origin - Destination via Street or Landmark. Note that names for bus and subway route patterns currently do not include the origin location, but will in the future.  | [optional]
**direction_id** | Option<**i32**> | Direction in which trip is traveling: `0` or `1`.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.   | [optional]
**canonical** | Option<**bool**> | Indicates whether or not the route pattern can be considered canonical and the default set of stops for the given route and direction.  | Value | Description | |-|-| | `true` | Route pattern should be considered canonical for this route in this direction. If branching regularly occurs, this route-direction may have more than one canonical pattern. | | `false` | Route pattern should be not considered canonical for this route in this direction. |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


