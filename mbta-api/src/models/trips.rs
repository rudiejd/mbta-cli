/*
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * The version of the OpenAPI document: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://openapi-generator.tech
 */

/// Trips : A page of [TripResource](#tripresource) results



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trips {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::RoutePatternsLinks>>,
    /// Content with [TripResource](#tripresource) objects
    #[serde(rename = "data")]
    pub data: Vec<crate::models::TripResource>,
}

impl Trips {
    /// A page of [TripResource](#tripresource) results
    pub fn new(data: Vec<crate::models::TripResource>) -> Trips {
        Trips {
            links: None,
            data,
        }
    }
}

