# TripResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**wheelchair_accessible** | Option<**i32**> | Indicator of wheelchair accessibility: `0`, `1`, `2`  Wheelchair accessibility (`*_/attributes/wheelchair_accessible`) [as defined in GTFS](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt):  | Value | Meaning                                            | |-------|----------------------------------------------------| | `0`   | No information                                     | | `1`   | Accessible (at stops allowing wheelchair_boarding) | | `2`   | Inaccessible                                       |   | [optional]
**name** | Option<**String**> | The text that appears in schedules and sign boards to identify the trip to passengers, for example, to identify train numbers for commuter rail trips. See [GTFS `trips.txt` `trip_short_name`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt)  | [optional]
**headsign** | Option<**String**> | The text that appears on a sign that identifies the trip's destination to passengers. See [GTFS `trips.txt` `trip_headsign`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt).  | [optional]
**direction_id** | Option<**i32**> | Direction in which trip is traveling: `0` or `1`.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.   | [optional]
**block_id** | Option<**String**> | ID used to group sequential trips with the same vehicle for a given service_id. See [GTFS `trips.txt` `block_id`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt)  | [optional]
**bikes_allowed** | Option<**i32**> | Indicator of whether or not bikes are allowed on this trip: `0`, `1`, `2`  Bikes allowed (`*_/attributes/bikes_allowed`) [as defined in GTFS](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt):  | Value | Meaning                                                                         | |-------|---------------------------------------------------------------------------------| | `0`   | No information                                                                  | | `1`   | Vehicle being used on this particular trip can accommodate at least one bicycle | | `2`   | No bicycles are allowed on this trip                                            |   | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


