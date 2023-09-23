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
pub struct ServiceResourceAttributes {
    #[serde(rename = "valid_days", skip_serializing_if = "Option::is_none")]
    pub valid_days: Option<Vec<f32>>,
    /// Earliest date which is valid for this service. Format is ISO8601.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Describes how well this schedule represents typical service for the listed `schedule_type`  | Value | Description                                                                 | |-------|-----------------------------------------------------------------------------| | `0`   | Not defined.                                                                | | `1`   | Typical service with perhaps minor modifications                            | | `2`   | Extra service supplements typical schedules                                 | | `3`   | Reduced holiday service is provided by typical Saturday or Sunday schedule  | | `4`   | Major changes in service due to a planned disruption, such as construction  | | `5`   | Major reductions in service for weather events or other atypical situations | | `6`   | Canonical service contains default stopping patterns for selected routes, including temporarily closed stops; not active on any dates | 
    #[serde(rename = "schedule_typicality", skip_serializing_if = "Option::is_none")]
    pub schedule_typicality: Option<i32>,
    /// Description of the schedule type the service_id can be applied. For example, on a holiday, the schedule_type value may be \"Saturday\" or \"Sunday\". Current valid values are \"Weekday\", \"Saturday\", \"Sunday\", or \"Other\" 
    #[serde(rename = "schedule_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<Option<String>>,
    /// Description of when the `service_id` is in effect.
    #[serde(rename = "schedule_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub schedule_name: Option<Option<String>>,
    #[serde(rename = "removed_dates_notes", skip_serializing_if = "Option::is_none")]
    pub removed_dates_notes: Option<Vec<String>>,
    #[serde(rename = "removed_dates", skip_serializing_if = "Option::is_none")]
    pub removed_dates: Option<Vec<String>>,
    /// Earliest date which is a part of the rating (season) which contains this service. Format is ISO8601.
    #[serde(rename = "rating_start_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rating_start_date: Option<Option<String>>,
    /// Latest date which is a part of the rating (season) which contains this service. Format is ISO8601.
    #[serde(rename = "rating_end_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rating_end_date: Option<Option<String>>,
    /// Human-readable description of the rating (season), as it should appear on public-facing websites and applications.
    #[serde(rename = "rating_description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rating_description: Option<Option<String>>,
    /// Latest date which is valid for this service. Format is ISO8601.
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Human-readable description of the service, as it should appear on public-facing websites and applications.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "added_dates_notes", skip_serializing_if = "Option::is_none")]
    pub added_dates_notes: Option<Vec<String>>,
    #[serde(rename = "added_dates", skip_serializing_if = "Option::is_none")]
    pub added_dates: Option<Vec<String>>,
}

impl ServiceResourceAttributes {
    pub fn new() -> ServiceResourceAttributes {
        ServiceResourceAttributes {
            valid_days: None,
            start_date: None,
            schedule_typicality: None,
            schedule_type: None,
            schedule_name: None,
            removed_dates_notes: None,
            removed_dates: None,
            rating_start_date: None,
            rating_end_date: None,
            rating_description: None,
            end_date: None,
            description: None,
            added_dates_notes: None,
            added_dates: None,
        }
    }
}


