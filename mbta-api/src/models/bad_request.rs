/*
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * The version of the OpenAPI document: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://openapi-generator.tech
 */

/// BadRequest : A JSON-API error document when the server cannot or will not process the request due to something that is perceived to be a client error. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BadRequest {
    #[serde(rename = "errors")]
    pub errors: Vec<crate::models::BadRequestErrorsInner>,
}

impl BadRequest {
    /// A JSON-API error document when the server cannot or will not process the request due to something that is perceived to be a client error. 
    pub fn new(errors: Vec<crate::models::BadRequestErrorsInner>) -> BadRequest {
        BadRequest {
            errors,
        }
    }
}

