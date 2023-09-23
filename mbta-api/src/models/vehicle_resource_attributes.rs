/*
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * The version of the OpenAPI document: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VehicleResourceAttributes {
    /// Time at which vehicle information was last updated. Format is ISO8601.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Speed that the vehicle is traveling in meters per second. See [GTFS-realtime Position speed](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-position).
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
    /// The degree of passenger occupancy for the vehicle. See [GTFS-realtime OccupancyStatus](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#enum-vehiclestopstatus).  | _**Value**_                    | _**Description**_                                                                                   | |--------------------------------|-----------------------------------------------------------------------------------------------------| | **MANY_SEATS_AVAILABLE**       | Not crowded: the vehicle has a large percentage of seats available. | | **FEW_SEATS_AVAILABLE**        | Some crowding: the vehicle has a small percentage of seats available. | | **FULL**                       | Crowded: the vehicle is considered full by most measures, but may still be allowing passengers to board. |  
    #[serde(rename = "occupancy_status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub occupancy_status: Option<Option<String>>,
    /// Longitude of the vehicle's current position.  Degrees East, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#Longitudes_on_WGS.C2.A084) coordinate system. See [GTFS-realtime Position longitude](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-position).
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
    /// Latitude of the vehicle's current position. Degrees North, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#A_new_World_Geodetic_System:_WGS.C2.A084) coordinate system. See [GTFS-realtime Position latitude](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-position).
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,
    /// User visible label, such as the one of on the signage on the vehicle.  See [GTFS-realtime VehicleDescriptor label](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-vehicledescriptor).
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Direction in which trip is traveling: `0` or `1`.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.  
    #[serde(rename = "direction_id", skip_serializing_if = "Option::is_none")]
    pub direction_id: Option<i32>,
    /// Index of current stop along trip. See [GTFS-realtime VehiclePosition current_stop_sequence](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-vehicleposition)
    #[serde(rename = "current_stop_sequence", skip_serializing_if = "Option::is_none")]
    pub current_stop_sequence: Option<i32>,
    /// Status of vehicle relative to the stops. See [GTFS-realtime VehicleStopStatus](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#enum-vehiclestopstatus).  | _**Value**_       | _**Description**_                                                                                          | |-------------------|------------------------------------------------------------------------------------------------------------| | **INCOMING_AT**   | The vehicle is just about to arrive at the stop (on a stop display, the vehicle symbol typically flashes). | | **STOPPED_AT**    | The vehicle is standing at the stop.                                                                       | | **IN_TRANSIT_TO** | The vehicle has departed the previous stop and is in transit.                                              |  
    #[serde(rename = "current_status", skip_serializing_if = "Option::is_none")]
    pub current_status: Option<String>,
    /// Carriage-level crowding details. See [GTFS-realtime multi_carriage_details](https://gtfs.org/realtime/reference/#message-CarriageDetails). 
    #[serde(rename = "carriages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub carriages: Option<Option<Vec<crate::models::VehicleResourceAttributesCarriagesInner>>>,
    /// Bearing, in degrees, clockwise from True North, i.e., 0 is North and 90 is East. This can be the compass bearing, or the direction towards the next stop or intermediate location. See [GTFS-realtime Position bearing](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-position).
    #[serde(rename = "bearing", skip_serializing_if = "Option::is_none")]
    pub bearing: Option<i32>,
}

impl VehicleResourceAttributes {
    pub fn new() -> VehicleResourceAttributes {
        VehicleResourceAttributes {
            updated_at: None,
            speed: None,
            occupancy_status: None,
            longitude: None,
            latitude: None,
            label: None,
            direction_id: None,
            current_stop_sequence: None,
            current_status: None,
            carriages: None,
            bearing: None,
        }
    }
}


