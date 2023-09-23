# InformedEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trip** | Option<**String**> | `id` of the affected Trip. | [optional]
**stop** | Option<**String**> | `id` of the affected Stop. | [optional]
**route_type** | Option<**i32**> | `type` of the affected Route.  | Value | Name          | Example    | |-------|---------------|------------| | `0`   | Light Rail    | Green Line | | `1`   | Heavy Rail    | Red Line   | | `2`   | Commuter Rail |            | | `3`   | Bus           |            | | `4`   | Ferry         |            |  | [optional]
**route** | Option<**String**> | `id` of the affected Route. | [optional]
**facility** | Option<**String**> | `id` of the affected Facility. | [optional]
**direction_id** | Option<**i32**> | `direction_id` of the affected Trip.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.  | [optional]
**activities** | Option<**Vec<String>**> | Activities affected by this alert.  If an entity is a station platform, and the alert only impacts those boarding at that platform and no one else, and the activity `\"BOARD\"` represents customers boarding at the informed entity, then the entity includes `activities` `[\"BOARD\"]`. If the alert affected customers exiting at the platform too, then `activities` is `[\"BOARD\", \"EXIT\"]`.  It should be noted that the `activities` array includes activities that are specifically affected. Thus if there were activities `\"BOARD\"`, `\"EXIT\"`, and `\"USING_WHEELCHAIR\"` [to board or exit], and a station were closed, then the `activities` array would include `\"BOARD\"` and `\"EXIT\"` but it would not be necessary to include the activity `\"USING_WHEELCHAIR\"`. Any rider entering the station who is `\"USING_WHEELCHAIR\"` is also a rider who `\"BOARD\"`s. Using a wheelchair to board is not specifically affected.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


