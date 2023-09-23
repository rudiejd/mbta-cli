# StopResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**wheelchair_boarding** | Option<**i32**> | Whether there are any vehicles with wheelchair boarding or paths to stops that are wheelchair acessible: 0, 1, 2.  Wheelchair boarding (`*_/attributes/wheelchair_boarding`) corresponds to [GTFS wheelchair_boarding](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stopstxt). The MBTA handles parent station inheritance itself, so value can be treated simply:  | Value | Meaning                                       | |-------|-----------------------------------------------| | `0`   | No Information                                | | `1`   | Accessible (if trip is wheelchair accessible) | | `2`   | Inaccessible                                  |   | [optional]
**name** | Option<**String**> | Name of a stop or station in the local and tourist vernacular.  See [GTFS `stops.txt` `stop_name](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stopstxt)  | [optional]
**longitude** | Option<**f32**> | Longitude of the stop or station. Degrees East, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#Longitudes_on_WGS.C2.A084) coordinate system. See [GTFS `stops.txt` `stop_lon`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stopstxt).  | [optional]
**location_type** | Option<**i32**> | The type of the stop.  | Value | Type | Description | | - | - | - | | `0` | Stop | A location where passengers board or disembark from a transit vehicle. | | `1` | Station | A physical structure or area that contains one or more stops. | | `2` | Station Entrance/Exit | A location where passengers can enter or exit a station from the street. The stop entry must also specify a parent_station value referencing the stop ID of the parent station for the entrance. | | `3` | Generic Node | A location within a station, not matching any other location_type, which can be used to link together pathways defined in pathways.txt. |  See also [GTFS `stops.txt` `location_type`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stopstxt).  | [optional]
**latitude** | Option<**f32**> | Latitude of the stop or station.  Degrees North, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#A_new_World_Geodetic_System:_WGS.C2.A084) coordinate system. See [GTFS `stops.txt` `stop_lat`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stopstxt).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


