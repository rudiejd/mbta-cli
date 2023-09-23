# ServiceResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**valid_days** | Option<**Vec<f32>**> |  | [optional]
**start_date** | Option<[**String**](string.md)> | Earliest date which is valid for this service. Format is ISO8601. | [optional]
**schedule_typicality** | Option<**i32**> | Describes how well this schedule represents typical service for the listed `schedule_type`  | Value | Description                                                                 | |-------|-----------------------------------------------------------------------------| | `0`   | Not defined.                                                                | | `1`   | Typical service with perhaps minor modifications                            | | `2`   | Extra service supplements typical schedules                                 | | `3`   | Reduced holiday service is provided by typical Saturday or Sunday schedule  | | `4`   | Major changes in service due to a planned disruption, such as construction  | | `5`   | Major reductions in service for weather events or other atypical situations | | `6`   | Canonical service contains default stopping patterns for selected routes, including temporarily closed stops; not active on any dates |  | [optional]
**schedule_type** | Option<**String**> | Description of the schedule type the service_id can be applied. For example, on a holiday, the schedule_type value may be \"Saturday\" or \"Sunday\". Current valid values are \"Weekday\", \"Saturday\", \"Sunday\", or \"Other\"  | [optional]
**schedule_name** | Option<**String**> | Description of when the `service_id` is in effect. | [optional]
**removed_dates_notes** | Option<**Vec<String>**> |  | [optional]
**removed_dates** | Option<[**Vec<String>**](string.md)> |  | [optional]
**rating_start_date** | Option<[**String**](string.md)> | Earliest date which is a part of the rating (season) which contains this service. Format is ISO8601. | [optional]
**rating_end_date** | Option<[**String**](string.md)> | Latest date which is a part of the rating (season) which contains this service. Format is ISO8601. | [optional]
**rating_description** | Option<**String**> | Human-readable description of the rating (season), as it should appear on public-facing websites and applications. | [optional]
**end_date** | Option<[**String**](string.md)> | Latest date which is valid for this service. Format is ISO8601. | [optional]
**description** | Option<**String**> | Human-readable description of the service, as it should appear on public-facing websites and applications. | [optional]
**added_dates_notes** | Option<**Vec<String>**> |  | [optional]
**added_dates** | Option<[**Vec<String>**](string.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


