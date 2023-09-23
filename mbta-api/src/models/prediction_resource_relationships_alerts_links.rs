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
pub struct PredictionResourceRelationshipsAlertsLinks {
    /// Relationship link for alerts
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// Related alerts link
    #[serde(rename = "related", skip_serializing_if = "Option::is_none")]
    pub related: Option<String>,
}

impl PredictionResourceRelationshipsAlertsLinks {
    pub fn new() -> PredictionResourceRelationshipsAlertsLinks {
        PredictionResourceRelationshipsAlertsLinks {
            param_self: None,
            related: None,
        }
    }
}


