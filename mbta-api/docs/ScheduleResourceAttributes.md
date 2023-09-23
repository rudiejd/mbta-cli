# ScheduleResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timepoint** | Option<**bool**> | | Value   | `*_/attributes/arrival_time` and `*_/attributes/departure_time` | |---------|---------------------------------------------------------------| | `true`  | Exact                                                         | | `false` | Estimates                                                     |  See [GTFS `stop_times.txt` `timepoint`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt)  | [optional]
**stop_sequence** | Option<**i32**> | The sequence the `stop_id` is arrived at during the `trip_id`.  The stop sequence is monotonically increasing along the trip, but the `stop_sequence` along the `trip_id` are not necessarily consecutive.  See [GTFS `stop_times.txt` `stop_sequence`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt)  | [optional]
**stop_headsign** | Option<**String**> | Text identifying destination of the trip, overriding trip-level headsign if present.See [GTFS `stop_times.txt` `stop_headsign`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt)  | [optional]
**pickup_type** | Option<**i32**> | How the vehicle departs from `stop_id`.  | Value | Description                                   | |-------|-----------------------------------------------| | `0`   | Regularly scheduled pickup                    | | `1`   | No pickup available                           | | `2`   | Must phone agency to arrange pickup           | | `3`   | Must coordinate with driver to arrange pickup |  See [GTFS `stop_times.txt` `pickup_type`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt)  | [optional]
**drop_off_type** | Option<**i32**> | How the vehicle arrives at `stop_id`.  | Value | Description                                   | |-------|-----------------------------------------------| | `0`   | Regularly scheduled drop off                  | | `1`   | No drop off available                         | | `2`   | Must phone agency to arrange pickup           | | `3`   | Must coordinate with driver to arrange pickup |  See [GTFS `stop_times.txt` `drop_off_type`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt)  | [optional]
**direction_id** | Option<**i32**> | Direction in which trip is traveling: `0` or `1`.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.   | [optional]
**departure_time** | Option<**String**> | Time when the trip departs the given stop. See [GTFS `stop_times.txt` `departure_time`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt) Format is ISO8601.  | [optional]
**arrival_time** | Option<**String**> | Time when the trip arrives at the given stop. See [GTFS `stop_times.txt` `arrival_time`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt) Format is ISO8601.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


