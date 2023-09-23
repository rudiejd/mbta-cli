# OccupancyResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | The degree of passenger occupancy for the vehicle. See [GTFS-realtime OccupancyStatus](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#enum-vehiclestopstatus).  | _**Value**_                    | _**Description**_                                                                                   | |--------------------------------|-----------------------------------------------------------------------------------------------------| | **MANY_SEATS_AVAILABLE**       | Not crowded: the vehicle has a large percentage of seats available. | | **FEW_SEATS_AVAILABLE**        | Some crowding: the vehicle has a small percentage of seats available. | | **FULL**                       | Crowded: the vehicle is considered full by most measures, but may still be allowing passengers to board. |   | [optional]
**percentage** | Option<**i32**> | Percentage of seats occupied.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


