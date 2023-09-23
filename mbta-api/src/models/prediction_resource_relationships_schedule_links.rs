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
pub struct PredictionResourceRelationshipsScheduleLinks {
    /// Relationship link for schedule
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// Related schedule link
    #[serde(rename = "related", skip_serializing_if = "Option::is_none")]
    pub related: Option<String>,
}

impl PredictionResourceRelationshipsScheduleLinks {
    pub fn new() -> PredictionResourceRelationshipsScheduleLinks {
        PredictionResourceRelationshipsScheduleLinks {
            param_self: None,
            related: None,
        }
    }
}

