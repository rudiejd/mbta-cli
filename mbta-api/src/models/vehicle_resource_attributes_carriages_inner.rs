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
pub struct VehicleResourceAttributesCarriagesInner {
    /// The degree of passenger occupancy for the vehicle. See [GTFS-realtime OccupancyStatus](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#enum-vehiclestopstatus).  | _**Value**_                    | _**Description**_                                                                                   | |--------------------------------|-----------------------------------------------------------------------------------------------------| | **MANY_SEATS_AVAILABLE**       | Not crowded: the vehicle has a large percentage of seats available. | | **FEW_SEATS_AVAILABLE**        | Some crowding: the vehicle has a small percentage of seats available. | | **FULL**                       | Crowded: the vehicle is considered full by most measures, but may still be allowing passengers to board. |  
    #[serde(rename = "occupancy_status", skip_serializing_if = "Option::is_none")]
    pub occupancy_status: Option<OccupancyStatus>,
    /// Percentage of vehicle occupied, calculated via weight average
    #[serde(rename = "occupancy_percentage", skip_serializing_if = "Option::is_none")]
    pub occupancy_percentage: Option<i32>,
    /// Carriage-specific label, used as an identifier
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl VehicleResourceAttributesCarriagesInner {
    pub fn new() -> VehicleResourceAttributesCarriagesInner {
        VehicleResourceAttributesCarriagesInner {
            occupancy_status: None,
            occupancy_percentage: None,
            label: None,
        }
    }
}

/// The degree of passenger occupancy for the vehicle. See [GTFS-realtime OccupancyStatus](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#enum-vehiclestopstatus).  | _**Value**_                    | _**Description**_                                                                                   | |--------------------------------|-----------------------------------------------------------------------------------------------------| | **MANY_SEATS_AVAILABLE**       | Not crowded: the vehicle has a large percentage of seats available. | | **FEW_SEATS_AVAILABLE**        | Some crowding: the vehicle has a small percentage of seats available. | | **FULL**                       | Crowded: the vehicle is considered full by most measures, but may still be allowing passengers to board. |  
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OccupancyStatus {
    #[serde(rename = "EMPTY")]
    Empty,
    #[serde(rename = "MANY_SEATS_AVAILABLE")]
    ManySeatsAvailable,
    #[serde(rename = "FEW_SEATS_AVAILABLE")]
    FewSeatsAvailable,
    #[serde(rename = "STANDING_ROOM_ONLY")]
    StandingRoomOnly,
    #[serde(rename = "CRUSHED_STANDING_ROOM_ONLY")]
    CrushedStandingRoomOnly,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "NOT_ACCEPTING_PASSENGERS")]
    NotAcceptingPassengers,
    #[serde(rename = "NO_DATA_AVAILABLE")]
    NoDataAvailable,
    #[serde(rename = "NOT_BOARDABLE")]
    NotBoardable,
}

impl Default for OccupancyStatus {
    fn default() -> OccupancyStatus {
        Self::Empty
    }
}
