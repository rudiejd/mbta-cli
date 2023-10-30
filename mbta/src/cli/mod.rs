mod command;
pub(crate) mod handlers;

pub use command::*;
pub use handlers::handle_subcommand;

use serde::{Deserialize, Serialize};

// the debug trait prints out the enum variant
#[derive(Debug, Deserialize, clap::ValueEnum, Clone)]
pub enum Direction {
    Inbound = 0,
    Outbound = 1,
    All = 2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObjectData {
    data: Option<Data>,
}

#[derive(Debug, Deserialize)]
struct VehicleRelationships {
    route: Option<ObjectData>,
    stop: Option<ObjectData>,
    trip: Option<ObjectData>,
}

#[derive(Debug, Deserialize)]
struct VehicleAttributes {
    current_status: String,
}

#[derive(Debug, Deserialize)]
struct VehicleData {
    attributes: VehicleAttributes,
    relationships: VehicleRelationships,
    id: String,
}

#[derive(Debug, Deserialize)]
struct VehiclesResponse {
    data: Vec<VehicleData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopAttributes {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopRelationships {
    pub parent_station: Option<ObjectData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopData {
    pub attributes: StopAttributes,
    pub id: String,
    pub relationships: StopRelationships,
}

#[derive(Debug, Deserialize)]
struct StopsResponse {
    data: Vec<StopData>,
}

#[derive(Debug, Deserialize)]
struct PredictionAttributes {
    arrival_time: Option<String>,
    direction_id: usize,
    departure_time: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PredictionRelationships {
    route: ObjectData,
}

#[derive(Debug, Deserialize)]
struct PredictionData {
    attributes: PredictionAttributes,
    relationships: PredictionRelationships,
}

#[derive(Debug, Deserialize)]
struct PredictionsResponse {
    data: Vec<PredictionData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteAttributes {
    long_name: String,
    direction_destinations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteData {
    pub attributes: RouteAttributes,
    pub id: String,
}

#[derive(Debug, Deserialize)]
struct RoutesResponse {
    data: Vec<RouteData>,
}

pub enum Command {
    Trains { service: MbtaService },
}
