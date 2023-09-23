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
pub struct ShapeResourceAttributes {
    /// The sequence of points in [Encoded Polyline Algorithm Format](https://developers.google.com/maps/documentation/utilities/polylinealgorithm). Libraries for decoding polylines are available in many languages, for example:  * [Elixir](https://hex.pm/packages/polyline) * [JavaScript](https://www.npmjs.com/package/polyline) * [Python](https://pypi.org/project/polyline/) 
    #[serde(rename = "polyline", skip_serializing_if = "Option::is_none")]
    pub polyline: Option<String>,
}

impl ShapeResourceAttributes {
    pub fn new() -> ShapeResourceAttributes {
        ShapeResourceAttributes {
            polyline: None,
        }
    }
}


