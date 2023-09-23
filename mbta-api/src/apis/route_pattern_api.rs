/*
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * The version of the OpenAPI document: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`api_web_period_route_pattern_controller_period_index`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiWebPeriodRoutePatternControllerPeriodIndexError {
    Status429(crate::models::TooManyRequests),
    Status403(crate::models::Forbidden),
    Status400(crate::models::BadRequest),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_web_period_route_pattern_controller_period_show`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiWebPeriodRoutePatternControllerPeriodShowError {
    Status429(crate::models::TooManyRequests),
    Status406(crate::models::NotAcceptable),
    Status404(crate::models::NotFound),
    Status403(crate::models::Forbidden),
    UnknownValue(serde_json::Value),
}


/// List of route patterns.  Route patterns are used to describe the subsets of a route, representing different possible patterns of where trips may serve. For example, a bus route may have multiple branches, and each branch may be modeled as a separate route pattern per direction. Hierarchically, the route pattern level may be considered to be larger than the trip level and smaller than the route level.  For most MBTA modes, a route pattern will typically represent a unique set of stops that may be served on a route-trip combination. Seasonal schedule changes may result in trips within a route pattern having different routings. In simple changes, such a single bus stop removed or added between one schedule rating and the next (for example, between the Summer and Fall schedules), trips will be maintained on the same route_pattern_id. If the changes are significant, a new route_pattern_id may be introduced.  For Commuter Rail, express or skip-stop trips use the same route pattern as local trips. Some branches do have multiple route patterns when the train takes a different path. For example, `CR-Providence` has two route patterns per direction, one for the Wickford Junction branch and the other for the Stoughton branch.  
pub async fn api_web_period_route_pattern_controller_period_index(configuration: &configuration::Configuration, page_left_square_bracket_offset_right_square_bracket: Option<i32>, page_left_square_bracket_limit_right_square_bracket: Option<i32>, sort: Option<&str>, fields_left_square_bracket_route_pattern_right_square_bracket: Option<&str>, include: Option<&str>, filter_left_square_bracket_id_right_square_bracket: Option<&str>, filter_left_square_bracket_route_right_square_bracket: Option<&str>, filter_left_square_bracket_direction_id_right_square_bracket: Option<&str>, filter_left_square_bracket_stop_right_square_bracket: Option<&str>, filter_left_square_bracket_canonical_right_square_bracket: Option<bool>) -> Result<crate::models::RoutePattern, Error<ApiWebPeriodRoutePatternControllerPeriodIndexError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/route_patterns", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_left_square_bracket_offset_right_square_bracket {
        local_var_req_builder = local_var_req_builder.query(&[("page[offset]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_left_square_bracket_limit_right_square_bracket {
        local_var_req_builder = local_var_req_builder.query(&[("page[limit]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields_left_square_bracket_route_pattern_right_square_bracket {
        local_var_req_builder = local_var_req_builder.query(&[("fields[route_pattern]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_id_right_square_bracket {
        local_var_req_builder = local_var_req_builder.query(&[("filter[id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_route_right_square_bracket {
        local_var_req_builder = local_var_req_builder.query(&[("filter[route]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_direction_id_right_square_bracket {
        local_var_req_builder = local_var_req_builder.query(&[("filter[direction_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_stop_right_square_bracket {
        local_var_req_builder = local_var_req_builder.query(&[("filter[stop]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_canonical_right_square_bracket {
        local_var_req_builder = local_var_req_builder.query(&[("filter[canonical]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("api_key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ApiWebPeriodRoutePatternControllerPeriodIndexError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Show a particular route_pattern by the route's id.  Route patterns are used to describe the subsets of a route, representing different possible patterns of where trips may serve. For example, a bus route may have multiple branches, and each branch may be modeled as a separate route pattern per direction. Hierarchically, the route pattern level may be considered to be larger than the trip level and smaller than the route level.  For most MBTA modes, a route pattern will typically represent a unique set of stops that may be served on a route-trip combination. Seasonal schedule changes may result in trips within a route pattern having different routings. In simple changes, such a single bus stop removed or added between one schedule rating and the next (for example, between the Summer and Fall schedules), trips will be maintained on the same route_pattern_id. If the changes are significant, a new route_pattern_id may be introduced.  For Commuter Rail, express or skip-stop trips use the same route pattern as local trips. Some branches do have multiple route patterns when the train takes a different path. For example, `CR-Providence` has two route patterns per direction, one for the Wickford Junction branch and the other for the Stoughton branch.  
pub async fn api_web_period_route_pattern_controller_period_show(configuration: &configuration::Configuration, id: &str, fields_left_square_bracket_route_pattern_right_square_bracket: Option<&str>, include: Option<&str>) -> Result<crate::models::RoutePattern, Error<ApiWebPeriodRoutePatternControllerPeriodShowError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/route_patterns/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields_left_square_bracket_route_pattern_right_square_bracket {
        local_var_req_builder = local_var_req_builder.query(&[("fields[route_pattern]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("api_key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ApiWebPeriodRoutePatternControllerPeriodShowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

