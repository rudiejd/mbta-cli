/*
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * The version of the OpenAPI document: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://openapi-generator.tech
 */

/// Stop : A JSON-API document with a single [StopResource](#stopresource) resource



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::ServiceLinks>>,
    /// Included resources
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<crate::models::ServiceIncludedInner>>,
    #[serde(rename = "data")]
    pub data: Box<crate::models::StopResource>,
}

impl Stop {
    /// A JSON-API document with a single [StopResource](#stopresource) resource
    pub fn new(data: crate::models::StopResource) -> Stop {
        Stop {
            links: None,
            included: None,
            data: Box::new(data),
        }
    }
}


