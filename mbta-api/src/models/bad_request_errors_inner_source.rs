/*
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * The version of the OpenAPI document: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://openapi-generator.tech
 */

/// BadRequestErrorsInnerSource : A JSON-API error source



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BadRequestErrorsInnerSource {
    /// The name of parameter that caused the error
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
}

impl BadRequestErrorsInnerSource {
    /// A JSON-API error source
    pub fn new() -> BadRequestErrorsInnerSource {
        BadRequestErrorsInnerSource {
            parameter: None,
        }
    }
}


