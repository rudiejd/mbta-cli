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
pub struct PredictionResourceRelationshipsSchedule {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PredictionResourceRelationshipsScheduleLinks>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::PredictionResourceRelationshipsScheduleData>>,
}

impl PredictionResourceRelationshipsSchedule {
    pub fn new() -> PredictionResourceRelationshipsSchedule {
        PredictionResourceRelationshipsSchedule {
            links: None,
            data: None,
        }
    }
}

