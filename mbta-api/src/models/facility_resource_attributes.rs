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
pub struct FacilityResourceAttributes {
    /// The type of the facility.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Short name of the facility
    #[serde(rename = "short_name", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// A list of name/value pairs that apply to the facility. See [MBTA's facility documentation](https://www.mbta.com/developers/gtfs/f#facilities_properties_definitions) for more information on the possible names and values.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<crate::models::FacilityProperty>>,
    /// Longitude of the facility. Degrees East, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#Longitudes_on_WGS.C2.A084) coordinate system. See [GTFS `facilities.txt` `facility_lon`] 
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
    /// Name of the facility
    #[serde(rename = "long_name", skip_serializing_if = "Option::is_none")]
    pub long_name: Option<String>,
    /// Latitude of the facility.  Degrees North, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#A_new_World_Geodetic_System:_WGS.C2.A084) coordinate system. See [GTFS `facilities.txt` `facility_lat`] 
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,
}

impl FacilityResourceAttributes {
    pub fn new() -> FacilityResourceAttributes {
        FacilityResourceAttributes {
            r#type: None,
            short_name: None,
            properties: None,
            longitude: None,
            long_name: None,
            latitude: None,
        }
    }
}

/// The type of the facility.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "BIKE_STORAGE")]
    BikeStorage,
    #[serde(rename = "BRIDGE_PLATE")]
    BridgePlate,
    #[serde(rename = "ELECTRIC_CAR_CHARGERS")]
    ElectricCarChargers,
    #[serde(rename = "ELEVATED_SUBPLATFORM")]
    ElevatedSubplatform,
    #[serde(rename = "ELEVATOR")]
    Elevator,
    #[serde(rename = "ESCALATOR")]
    Escalator,
    #[serde(rename = "FARE_MEDIA_ASSISTANCE_FACILITY")]
    FareMediaAssistanceFacility,
    #[serde(rename = "FARE_MEDIA_ASSISTANT")]
    FareMediaAssistant,
    #[serde(rename = "FARE_VENDING_MACHINE")]
    FareVendingMachine,
    #[serde(rename = "FARE_VENDING_RETAILER")]
    FareVendingRetailer,
    #[serde(rename = "FULLY_ELEVATED_PLATFORM")]
    FullyElevatedPlatform,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "PARKING_AREA")]
    ParkingArea,
    #[serde(rename = "PICK_DROP")]
    PickDrop,
    #[serde(rename = "PORTABLE_BOARDING_LIFT")]
    PortableBoardingLift,
    #[serde(rename = "RAMP")]
    Ramp,
    #[serde(rename = "TAXI_STAND")]
    TaxiStand,
    #[serde(rename = "TICKET_WINDOW")]
    TicketWindow,
}

impl Default for Type {
    fn default() -> Type {
        Self::BikeStorage
    }
}
