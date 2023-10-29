mod command;
pub(crate) mod handlers;

pub use command::*;
pub use handlers::handle_subcommand;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Data {
    id: String,
}

#[derive(Debug, Deserialize)]
struct ObjectData {
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
struct StopAttributes {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopData {
    attributes: StopAttributes,
    id: String,
}

#[derive(Debug, Deserialize)]
struct StopsResponse {
    data: Vec<StopData>,
}

#[derive(Debug, Deserialize)]
struct PredictionAttributes {
    arrival_time: Option<String>,
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

pub enum Command {
    Trains { service: MbtaService },
}
