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
pub struct TripResourceRelationships {
    #[serde(rename = "shape", skip_serializing_if = "Option::is_none")]
    pub shape: Option<Box<crate::models::TripResourceRelationshipsShape>>,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<Box<crate::models::TripResourceRelationshipsService>>,
    #[serde(rename = "route_pattern", skip_serializing_if = "Option::is_none")]
    pub route_pattern: Option<Box<crate::models::TripResourceRelationshipsRoutePattern>>,
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Box<crate::models::TripResourceRelationshipsRoute>>,
    #[serde(rename = "occupancy", skip_serializing_if = "Option::is_none")]
    pub occupancy: Option<Box<crate::models::TripResourceRelationshipsOccupancy>>,
}

impl TripResourceRelationships {
    pub fn new() -> TripResourceRelationships {
        TripResourceRelationships {
            shape: None,
            service: None,
            route_pattern: None,
            route: None,
            occupancy: None,
        }
    }
}


