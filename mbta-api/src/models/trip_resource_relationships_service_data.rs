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
pub struct TripResourceRelationshipsServiceData {
    /// Type of related service resource
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Related service resource id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl TripResourceRelationshipsServiceData {
    pub fn new() -> TripResourceRelationshipsServiceData {
        TripResourceRelationshipsServiceData {
            r#type: None,
            id: None,
        }
    }
}


