# AlertResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> | A URL for extra details, such as outline construction or maintenance plans. | [optional]
**updated_at** | Option<**String**> | Date/Time alert last updated. Format is ISO8601. | [optional]
**timeframe** | Option<**String**> | Summarizes when an alert is in effect. | [optional]
**short_header** | Option<**String**> | A shortened version of `*_/attributes/header`. | [optional]
**severity** | Option<**i32**> | How severe the alert it from least (`0`) to most (`10`) severe. | [optional]
**service_effect** | Option<**String**> | Summarizes the service and the impact to that service. | [optional]
**lifecycle** | Option<**String**> | Identifies whether alert is a new or old, in effect or upcoming.  | Value                | |----------------------| | `\"NEW\"`              | | `\"ONGOING\"`          | | `\"ONGOING_UPCOMING\"` | | `\"UPCOMING\"`         |   | [optional]
**informed_entity** | Option<[**Vec<crate::models::InformedEntity>**](InformedEntity.md)> | Entities affected by this alert. | [optional]
**image_alternative_text** | Option<**String**> | Text describing the appearance of the linked image in the image field. | [optional]
**image** | Option<**String**> | URL of an image to be displayed alongside alert. | [optional]
**header** | Option<**String**> | This plain-text string will be highlighted, for example in boldface. See [GTFS Realtime `FeedMessage` `FeedEntity` `Alert` `header_text`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-alert)  | [optional]
**effect_name** | Option<**String**> | Name of the alert | [optional]
**effect** | Option<**String**> | The effect of this problem on the affected entity.  | Value | |-------| | `\"ACCESS_ISSUE\"` | | `\"ADDITIONAL_SERVICE\"` | | `\"AMBER_ALERT\"` | | `\"BIKE_ISSUE\"` | | `\"CANCELLATION\"` | | `\"DELAY\"` | | `\"DETOUR\"` | | `\"DOCK_CLOSURE\"` | | `\"DOCK_ISSUE\"` | | `\"ELEVATOR_CLOSURE\"` | | `\"ESCALATOR_CLOSURE\"` | | `\"EXTRA_SERVICE\"` | | `\"FACILITY_ISSUE\"` | | `\"MODIFIED_SERVICE\"` | | `\"NO_SERVICE\"` | | `\"OTHER_EFFECT\"` | | `\"PARKING_CLOSURE\"` | | `\"PARKING_ISSUE\"` | | `\"POLICY_CHANGE\"` | | `\"SCHEDULE_CHANGE\"` | | `\"SERVICE_CHANGE\"` | | `\"SHUTTLE\"` | | `\"SNOW_ROUTE\"` | | `\"STATION_CLOSURE\"` | | `\"STATION_ISSUE\"` | | `\"STOP_CLOSURE\"` | | `\"STOP_MOVE\"` | | `\"STOP_MOVED\"` | | `\"SUMMARY\"` | | `\"SUSPENSION\"` | | `\"TRACK_CHANGE\"` | | `\"UNKNOWN_EFFECT\"` |  See [GTFS Realtime `FeedMessage` `FeedEntity` `Alert` `effect`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-alert)   | [optional]
**description** | Option<**String**> | This plain-text string will be formatted as the body of the alert (or shown on an explicit \"expand\" request by the user). The information in the description should add to the information of the header. See [GTFS Realtime `FeedMessage` `FeedEntity` `Alert` `description_text`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-alert)  | [optional]
**created_at** | Option<**String**> | Date/Time alert created. Format is ISO8601. | [optional]
**cause** | Option<**String**> | What is causing the alert.  | Value | |-------| | `\"ACCIDENT\"` | | `\"AMTRAK\"` | | `\"AN_EARLIER_MECHANICAL_PROBLEM\"` | | `\"AN_EARLIER_SIGNAL_PROBLEM\"` | | `\"AUTOS_IMPEDING_SERVICE\"` | | `\"COAST_GUARD_RESTRICTION\"` | | `\"CONGESTION\"` | | `\"CONSTRUCTION\"` | | `\"CROSSING_MALFUNCTION\"` | | `\"DEMONSTRATION\"` | | `\"DISABLED_BUS\"` | | `\"DISABLED_TRAIN\"` | | `\"DRAWBRIDGE_BEING_RAISED\"` | | `\"ELECTRICAL_WORK\"` | | `\"FIRE\"` | | `\"FOG\"` | | `\"FREIGHT_TRAIN_INTERFERENCE\"` | | `\"HAZMAT_CONDITION\"` | | `\"HEAVY_RIDERSHIP\"` | | `\"HIGH_WINDS\"` | | `\"HOLIDAY\"` | | `\"HURRICANE\"` | | `\"ICE_IN_HARBOR\"` | | `\"MAINTENANCE\"` | | `\"MECHANICAL_PROBLEM\"` | | `\"MEDICAL_EMERGENCY\"` | | `\"PARADE\"` | | `\"POLICE_ACTION\"` | | `\"POWER_PROBLEM\"` | | `\"SEVERE_WEATHER\"` | | `\"SIGNAL_PROBLEM\"` | | `\"SLIPPERY_RAIL\"` | | `\"SNOW\"` | | `\"SPECIAL_EVENT\"` | | `\"SPEED_RESTRICTION\"` | | `\"SWITCH_PROBLEM\"` | | `\"TIE_REPLACEMENT\"` | | `\"TRACK_PROBLEM\"` | | `\"TRACK_WORK\"` | | `\"TRAFFIC\"` | | `\"UNRULY_PASSENGER\"` | | `\"WEATHER\"` |  See [GTFS Realtime `FeedMessage` `FeedEntity` `Alert` `Cause`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#enum-cause)   | [optional]
**banner** | Option<**String**> | Set if alert is meant to be displayed prominently, such as the top of every page. | [optional]
**active_period** | Option<[**Vec<crate::models::ActivePeriod>**](ActivePeriod.md)> | Date/Time ranges when alert is active. See [GTFS Realtime `FeedMessage` `FeedEntity` `Alert` `active_period`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-alert).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


