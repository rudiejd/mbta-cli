/*
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * The version of the OpenAPI document: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://openapi-generator.tech
 */

/// Stops : A page of [StopResource](#stopresource) results



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stops {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::RoutePatternsLinks>>,
    /// Content with [StopResource](#stopresource) objects
    #[serde(rename = "data")]
    pub data: Vec<crate::models::StopResource>,
}

impl Stops {
    /// A page of [StopResource](#stopresource) results
    pub fn new(data: Vec<crate::models::StopResource>) -> Stops {
        Stops {
            links: None,
            data,
        }
    }
}


