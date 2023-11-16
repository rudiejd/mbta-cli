use super::*;
use anyhow::{anyhow, Result};
use std::{
    fs::File,
    fs::{self},
    io::Read,
};

const CACHE_FILE_PATH: &str = ".mbtacache";

fn read_cache<'a>() -> Result<MbtaCache> {
    let mut file = match File::open(&CACHE_FILE_PATH) {
        Err(why) => {
            return Err(anyhow!(format!(
                "couldn't read cache {}: {}",
                CACHE_FILE_PATH, why
            )))
        }
        Ok(file) => file,
    };
    let mut cache_serialized = String::new();
    file.read_to_string(&mut cache_serialized);
    let cache = match serde_json::from_str::<MbtaCache>(&cache_serialized) {
        Ok(c) => c,
        //is return Err right here?
        Err(err) => {
            return Err(anyhow!(format!(
                "couldn't deserialize cache {}: {}",
                CACHE_FILE_PATH, err
            )))
        }
    };
    return Ok(cache);
}

pub fn get() -> MbtaCache {
    match read_cache() {
        Ok(c) => c,
        Err(_) => MbtaCache {
            stops: HashMap::<String, StopData>::new(),
            routes: HashMap::<String, RouteData>::new(),
        },
    }
}

pub fn write_stops_to_cache<'a>(stops: &'a Vec<StopData>) -> HashMap<String, StopData> {
    let mut stop_map = HashMap::<String, StopData>::new();
    // assume no duplicate stop ids
    for stop in stops {
        stop_map.insert(stop.id.clone(), stop.clone());
    }

    let current_cache = get();
    let cache = MbtaCache {
        routes: current_cache.routes.clone(),
        stops: stop_map.clone(),
    };

    // just assuming it works lawl
    fs::write(CACHE_FILE_PATH, serde_json::to_string(&cache).unwrap());

    return stop_map;
}

pub fn write_routes_to_cache<'a, 'b>(routes: &'a Vec<RouteData>) -> HashMap<String, RouteData> {
    let mut route_map = HashMap::<String, RouteData>::new();

    for route in routes {
        route_map.insert(route.id.clone(), route.clone());
    }

    let current_cache = get();

    let cache = MbtaCache {
        routes: route_map.clone(),
        stops: current_cache.stops.clone(),
    };

    fs::write(CACHE_FILE_PATH, serde_json::to_string(&cache).unwrap());

    return route_map;
}
