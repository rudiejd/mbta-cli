/*
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * The version of the OpenAPI document: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://openapi-generator.tech
 */

/// Alert : A JSON-API document with a single [AlertResource](#alertresource) resource



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::ServiceLinks>>,
    /// Included resources
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<crate::models::ServiceIncludedInner>>,
    #[serde(rename = "data")]
    pub data: Box<crate::models::AlertResource>,
}

impl Alert {
    /// A JSON-API document with a single [AlertResource](#alertresource) resource
    pub fn new(data: crate::models::AlertResource) -> Alert {
        Alert {
            links: None,
            included: None,
            data: Box::new(data),
        }
    }
}


