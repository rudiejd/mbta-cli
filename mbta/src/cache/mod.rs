use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cli::{RouteData, StopData};

//TODO: make the cache generic
pub use cache::{get, write_routes_to_cache, write_stops_to_cache};

mod cache;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MbtaCache {
    pub stops: HashMap<String, StopData>,
    pub routes: HashMap<String, RouteData>,
}
