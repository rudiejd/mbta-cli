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
pub struct PredictionResourceRelationshipsScheduleData {
    /// Type of related schedule resource
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Related schedule resource id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl PredictionResourceRelationshipsScheduleData {
    pub fn new() -> PredictionResourceRelationshipsScheduleData {
        PredictionResourceRelationshipsScheduleData {
            r#type: None,
            id: None,
        }
    }
}

