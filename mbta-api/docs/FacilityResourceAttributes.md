# FacilityResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The type of the facility. | [optional]
**short_name** | Option<**String**> | Short name of the facility | [optional]
**properties** | Option<[**Vec<crate::models::FacilityProperty>**](FacilityProperty.md)> | A list of name/value pairs that apply to the facility. See [MBTA's facility documentation](https://www.mbta.com/developers/gtfs/f#facilities_properties_definitions) for more information on the possible names and values. | [optional]
**longitude** | Option<**f32**> | Longitude of the facility. Degrees East, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#Longitudes_on_WGS.C2.A084) coordinate system. See [GTFS `facilities.txt` `facility_lon`]  | [optional]
**long_name** | Option<**String**> | Name of the facility | [optional]
**latitude** | Option<**f32**> | Latitude of the facility.  Degrees North, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#A_new_World_Geodetic_System:_WGS.C2.A084) coordinate system. See [GTFS `facilities.txt` `facility_lat`]  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


