/*
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * The version of the OpenAPI document: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://openapi-generator.tech
 */

/// Vehicles : A page of [VehicleResource](#vehicleresource) results



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vehicles {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::RoutePatternsLinks>>,
    /// Content with [VehicleResource](#vehicleresource) objects
    #[serde(rename = "data")]
    pub data: Vec<crate::models::VehicleResource>,
}

impl Vehicles {
    /// A page of [VehicleResource](#vehicleresource) results
    pub fn new(data: Vec<crate::models::VehicleResource>) -> Vehicles {
        Vehicles {
            links: None,
            data,
        }
    }
}


