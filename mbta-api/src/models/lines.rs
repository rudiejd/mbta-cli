/*
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * The version of the OpenAPI document: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://openapi-generator.tech
 */

/// Lines : A page of [LineResource](#lineresource) results



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lines {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::RoutePatternsLinks>>,
    /// Content with [LineResource](#lineresource) objects
    #[serde(rename = "data")]
    pub data: Vec<crate::models::LineResource>,
}

impl Lines {
    /// A page of [LineResource](#lineresource) results
    pub fn new(data: Vec<crate::models::LineResource>) -> Lines {
        Lines {
            links: None,
            data,
        }
    }
}

