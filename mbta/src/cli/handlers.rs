use crate::cache::{self, write_routes_to_cache, write_stops_to_cache};
use std::str::{self};
use chrono::{DateTime, Local};

use super::*;

use anyhow::{anyhow, Result};
use clap::ArgMatches;

const MBTA_API_URL: &str = "https://api-v3.mbta.com";
const DATETIME_FORMAT: &str = "%FT%T%:z";

fn default_stop_data() -> StopData {
    return StopData {
        id: "Unknown".to_string(),
        attributes: StopAttributes {
            name: "Unknown".to_string(),
            description: None
        },
        relationships: StopRelationships { parent_station: None }
    };
}

fn default_route_data() -> RouteData {
    return RouteData {
        id: "Unknown Route".to_string(),
        attributes: RouteAttributes {
            long_name: "Unknown Route".to_string(),
            direction_destinations: Vec::new()
        },
    };
}

fn default_data() -> Data {
    return Data {
        id: "Unknown".to_string(),
    };
}

// only use the cache for this
fn fetch_stop_by_name<'a>(name: &'a String) -> Option<StopData> {
    match cache::get() {
        Some(c) => {
            for stop in c.stops.values() {
                // the actual stations will have parent station as null e.g. https://api-v3.mbta.com/stops/NEC-2287-12 (fake south station)
                // vs https://api-v3.mbta.com/stops/place-sstat (real south station)
                if stop.attributes.name.eq(name) && stop.relationships.parent_station.as_ref().unwrap().data.is_none() {
                    return Some(stop.clone());
                }
            }

            return None;
        }
        None => {
            let res = match reqwest::blocking::get(format!("{}/stops", MBTA_API_URL))
                .expect("MBTA response should have body")
                .text()
            {
                Ok(res) => res.to_string(),
                Err(err) => {
                    dbg!("Encountered error fetching stops from MBTA API {}", err);
                    return None;
                }
            };
            let deserialized = serde_json::from_str::<StopsResponse>(&res).unwrap();

            write_stops_to_cache(&deserialized.data);
            return None;
        }
    };
}

// fetch a stop by stop id. if we can read from the file system cache,
// read from the file system cache. otherwise, invalidate the filesystem stop cache
// TODO make this generic
fn fetch_stop_by_id(id: String) -> StopData {
    // these two paragraphs are probably bad...
    let stop_map = match cache::get() {
        Some(c) => Some(c.stops),
        None => None,
    };

    if let Some(stop_map) = stop_map {
        match stop_map.get(&id) {
            Some(s) => return s.clone(),
            None => (),
        }
    }

    let res = match reqwest::blocking::get(format!("{}/stops", MBTA_API_URL))
        .expect("MBTA response should have body")
        .text()
    {
        Ok(res) => res.to_string(),
        Err(err) => {
            dbg!("Encountered error fetching stops from MBTA API {}", err);
            return default_stop_data();
        }
    };
    let deserialized = serde_json::from_str::<StopsResponse>(&res).unwrap();

    // if we get here, stop was not in the cache
    return match write_stops_to_cache(&deserialized.data).get(&id) {
        Some(s) => s.clone(),
        None => {
            dbg!(
                "Failed to find stop with id {} after invalidating cache!",
                id
            );
            default_stop_data()
        }
    };
}

// fetch a stop by stop id. if we can read from the file system cache,
// read from the file system cache. otherwise, invalidate the filesystem stop cache
// TODO add caching
fn fetch_route_by_id(id: String) -> RouteData {
    // these two paragraphs are probably bad...
    let route_map = match cache::get() {
        Some(c) => Some(c.routes),
        None => None,
    };

    if let Some(route_map) = route_map {
        match route_map.get(&id) {
            Some(s) => return s.clone(),
            None => (),
        }
    }

    let res = match reqwest::blocking::get(format!("{}/routes", MBTA_API_URL))
        .expect("MBTA response should have body")
        .text()
    {
        Ok(res) => res.to_string(),
        Err(err) => {
            dbg!("Encountered error fetching routes from MBTA API {}", err);
            return default_route_data();
        }
    };
    let deserialized = serde_json::from_str::<RoutesResponse>(&res).unwrap();

    // if we get here, route was not in the cache
    return match write_routes_to_cache(&deserialized.data).get(&id) {
        Some(r) => r.clone(),
        None => {
            dbg!(
                "Failed to find route with id {} after invalidating cache!",
                id
            );
            default_route_data()
        }
    };
}

// TODO clean up the sloppy ? error handling here
// ... or is that okay because of anyhow
// idk above my paygrade
pub fn handle_trains_subcommand(args: &ArgMatches) -> Result<()> {
    let (cmd, args) = args.subcommand().expect("list subcommand is required");
    // TODO switch to enum maybe
    let service = args.get_one::<String>("service");

    print!("All active trains");
    if service.is_some() {
        print!(" for service {}", service.unwrap())
    }
    println!();

    let res = match reqwest::blocking::get(format!("{}/vehicles", MBTA_API_URL)) {
        Ok(res) => res.text().unwrap(),
        Err(err) => {
            dbg!("Error sending HTTP request!");
            return Err(err.into());
        }
    };

    let deserialized =
        serde_json::from_str::<VehiclesResponse>(&res).expect("Expected routes response!");
    for vehicle in deserialized.data {
        // TODO figure out what to do with these unwraps
        let route_id = vehicle
            .relationships
            .route
            .unwrap()
            .data
            .unwrap_or(default_data())
            .id;

        // TODO filter this better (is there something i can pass to the API?)
        if service.is_some() && !route_id.contains(service.unwrap()) {
            continue;
        }

        let stop = match vehicle.relationships.stop {
            Some(stop) => match stop.data {
                Some(d) => fetch_stop_by_id(d.id),
                None => default_stop_data(),
            },
            None => default_stop_data(),
        };

        println!(
            "Vehicle {}: {} {} {}",
            vehicle.id, route_id, vehicle.attributes.current_status, stop.attributes.name
        );
    }

    Ok(())
}

pub fn handle_subcommand(cmd: &str, args: &ArgMatches) -> Result<()> {
    match cmd {
        "trains" => handle_trains_subcommand(args),
        "arrivals" => handle_arrivals_subcommand(args),
        _ => unreachable!(),
    }
}

pub fn handle_arrivals_subcommand(args: &ArgMatches) -> Result<()> {
    let stop = args
        .get_one::<String>("stop")
        .expect("Must pass stop to get arrivals!");
    let direction = match args.get_one::<Direction>("direction") {
        Some(dir) => dir.to_owned(),
        // default to all arrivals
        None => Direction::All.to_owned(),
    };

    let potential_stop = fetch_stop_by_name(stop);

    if potential_stop.is_none() {
        return Err(anyhow!("Requested stop {} is not valid!", stop));
    }

    let matched_stop = potential_stop.unwrap();
    let stop_id = matched_stop.id;

    let res = match reqwest::blocking::get(format!(
        "{}/predictions?filter[stop]={}&filter[direction_id]={}",
        // TODO is there a way to not clone here?
        MBTA_API_URL,
        stop_id,
        direction.clone() as u32
    )) {
        Ok(res) => res.text().unwrap(),
        Err(err) => {
            dbg!("Error sending HTTP request!");
            return Err(err.into());
        }
    };

    // TODO probably don't panic here? idk
    let deserialized =
        serde_json::from_str::<PredictionsResponse>(&res).expect("Expected predictions response!");

    println!("Arrivals at {}: ", matched_stop.attributes.name.clone());
    for prediction in deserialized.data {
        let route = match prediction.relationships.route.data {
            Some(data) => fetch_route_by_id(data.id),
            None => default_route_data()
        };

        let arrival_time = match prediction.attributes.arrival_time {
            Some(t) => {
                let dt = DateTime::parse_from_str(&t, DATETIME_FORMAT).unwrap();
                let dt_local = DateTime::<Local>::from_naive_utc_and_offset(
                    dt.naive_utc(),
                    dt.offset().clone(),
                );
                let diff = Local::now() - dt_local;

                // don't show trains that already arrived
                if diff.num_minutes() < 0 {
                    continue;
                }

                diff.num_minutes().to_string()
            }
            // don't show trains with no arrival time
            None => {
                continue;
            }
        };

        println!(
            "{} will arive in {} minutes heading to {}",
            route.attributes.long_name, arrival_time, route.attributes.direction_destinations[prediction.attributes.direction_id]
        );
    }

    Ok(())
}

#[test]
fn deserializes_vehicle() {
    let vehicle_data = "{
  \"data\": [
    {
      \"attributes\": {
        \"bearing\": 225,
        \"carriages\": [],
        \"current_status\": \"STOPPED_AT\",
        \"current_stop_sequence\": 520,
        \"direction_id\": 0,
        \"label\": \"3827\",
        \"latitude\": 42.3295,
        \"longitude\": -71.19235,
        \"occupancy_status\": null,
        \"speed\": 5.6,
        \"updated_at\": \"2023-10-22T19:21:48-04:00\"
      },
      \"id\": \"G-10170\",
      \"links\": {
        \"self\": \"/vehicles/G-10170\"
      },
      \"relationships\": {
        \"route\": {
          \"data\": {
            \"id\": \"Green-D\",
            \"type\": \"route\"
          }
        },
        \"stop\": {
          \"data\": {
            \"id\": \"70171\",
            \"type\": \"stop\"
          }
        },
        \"trip\": {
          \"data\": {
            \"id\": \"58514827\",
            \"type\": \"trip\"
          }
        }
      },
      \"type\": \"vehicle\"
    },
    {
      \"attributes\": {
        \"bearing\": 186,
        \"carriages\": [],
        \"current_status\": \"IN_TRANSIT_TO\",
        \"current_stop_sequence\": 3,
        \"direction_id\": 0,
        \"label\": \"1624\",
        \"latitude\": 42.292651919,
        \"longitude\": -71.11776714,
        \"occupancy_status\": \"FEW_SEATS_AVAILABLE\",
        \"speed\": null,
        \"updated_at\": \"2023-10-22T19:22:12-04:00\"
      },
      \"id\": \"y1624\",
      \"links\": {
        \"self\": \"/vehicles/y1624\"
      },
      \"relationships\": {
        \"route\": {
          \"data\": {
            \"id\": \"32\",
            \"type\": \"route\"
          }
        },
        \"stop\": {
          \"data\": {
            \"id\": \"26495\",
            \"type\": \"stop\"
          }
        },
        \"trip\": {
          \"data\": {
            \"id\": \"58717852\",
            \"type\": \"trip\"
          }
        }
      },
      \"type\": \"vehicle\"
    },
    {
      \"attributes\": {
        \"bearing\": 130,
        \"carriages\": [],
        \"current_status\": \"IN_TRANSIT_TO\",
        \"current_stop_sequence\": 100,
        \"direction_id\": 0,
        \"label\": \"1637\",
        \"latitude\": 42.34895,
        \"longitude\": -71.05328,
        \"occupancy_status\": null,
        \"speed\": null,
        \"updated_at\": \"2023-10-22T19:22:06-04:00\"
      },
      \"id\": \"R-547913F9\",
      \"links\": {
        \"self\": \"/vehicles/R-547913F9\"
      },
      \"relationships\": {
        \"route\": {
          \"data\": {
            \"id\": \"Red\",
            \"type\": \"route\"
          }
        },
        \"stop\": {
          \"data\": {
            \"id\": \"70081\",
            \"type\": \"stop\"
          }
        },
        \"trip\": {
          \"data\": {
            \"id\": \"59548385\",
            \"type\": \"trip\"
          }
        }
      },
      \"type\": \"vehicle\"
    },
    {
      \"attributes\": {
        \"bearing\": 135,
        \"carriages\": [],
        \"current_status\": \"IN_TRANSIT_TO\",
        \"current_stop_sequence\": 1,
        \"direction_id\": 0,
        \"label\": \"2059\",
        \"latitude\": 42.37205,
        \"longitude\": -71.07765,
        \"occupancy_status\": \"MANY_SEATS_AVAILABLE\",
        \"speed\": null,
        \"updated_at\": \"2023-10-22T19:21:19-04:00\"
      },
      \"id\": \"y2059\",
      \"links\": {
        \"self\": \"/vehicles/y2059\"
      },
      \"relationships\": {
        \"route\": {
          \"data\": {
            \"id\": \"80\",
            \"type\": \"route\"
          }
        },
        \"stop\": {
          \"data\": {
            \"id\": \"70500\",
            \"type\": \"stop\"
          }
        },
        \"trip\": {
          \"data\": {
            \"id\": \"58561026\",
            \"type\": \"trip\"
          }
        }
      },
      \"type\": \"vehicle\"
    },
    {
      \"attributes\": {
        \"bearing\": 145,
        \"carriages\": [],
        \"current_status\": \"INCOMING_AT\",
        \"current_stop_sequence\": 8,
        \"direction_id\": 0,
        \"label\": \"3872-3653\",
        \"latitude\": 42.3804,
        \"longitude\": -71.0877,
        \"occupancy_status\": null,
        \"speed\": 8.5,
        \"updated_at\": \"2023-10-22T19:22:03-04:00\"
      },
      \"id\": \"G-10003\",
      \"links\": {
        \"self\": \"/vehicles/G-10003\"
      },
      \"relationships\": {
        \"route\": {
          \"data\": {
            \"id\": \"Green-E\",
            \"type\": \"route\"
          }
        },
        \"stop\": {
          \"data\": {
            \"id\": \"70514\",
            \"type\": \"stop\"
          }
        },
        \"trip\": {
          \"data\": {
            \"id\": \"58515214\",
            \"type\": \"trip\"
          }
        }
      },
      \"type\": \"vehicle\"
    },
    {
      \"attributes\": {
        \"bearing\": 0,
        \"carriages\": [],
        \"current_status\": \"IN_TRANSIT_TO\",
        \"current_stop_sequence\": 9,
        \"direction_id\": 1,
        \"label\": \"1319\",
        \"latitude\": 42.35245949,
        \"longitude\": -71.04699105,
        \"occupancy_status\": \"MANY_SEATS_AVAILABLE\",
        \"speed\": null,
        \"updated_at\": \"2023-10-22T19:17:25-04:00\"
      },
      \"id\": \"y1319\",
      \"links\": {
        \"self\": \"/vehicles/y1319\"
      },
      \"relationships\": {
        \"route\": {
          \"data\": {
            \"id\": \"741\",
            \"type\": \"route\"
          }
        },
        \"stop\": {
          \"data\": {
            \"id\": \"74616\",
            \"type\": \"stop\"
          }
        },
        \"trip\": {
          \"data\": {
            \"id\": \"58792186\",
            \"type\": \"trip\"
          }
        }
      },
      \"type\": \"vehicle\"
        }
      ]
    }";
    let deserialized = serde_json::from_str::<VehiclesResponse>(vehicle_data).unwrap();
    assert_eq!("STOPPED_AT", deserialized.data[0].attributes.current_status);
}

#[test]
fn deserializes_stop() {
    let stop_data = "{
      \"data\": {
        \"attributes\": {
          \"address\": null,
          \"at_street\": null,
          \"description\": \"Newton Centre - Green Line - (D) Riverside\",
          \"latitude\": 42.3294,
          \"location_type\": 0,
          \"longitude\": -71.192622,
          \"municipality\": \"Newton\",
          \"name\": \"Newton Centre\",
          \"on_street\": null,
          \"platform_code\": null,
          \"platform_name\": \"Riverside\",
          \"vehicle_type\": 0,
          \"wheelchair_boarding\": 1
        },
        \"id\": \"70171\",
        \"links\": {
          \"self\": \"/stops/70171\"
        },
        \"relationships\": {
          \"facilities\": {
            \"links\": {
              \"related\": \"/facilities/?filter[stop]=70171\"
            }
          },
          \"parent_station\": {
            \"data\": {
              \"id\": \"place-newto\",
              \"type\": \"stop\"
            }
          },
          \"zone\": {
            \"data\": {
              \"id\": \"RapidTransit\",
              \"type\": \"zone\"
            }
          }
        },
        \"type\": \"stop\"
      },
      \"jsonapi\": {
        \"version\": \"1.0\"
      }
    }";
    let stop_deserialized = serde_json::from_str::<StopsResponse>(stop_data);
    assert_eq!(
        stop_deserialized.unwrap().data[0].attributes.name,
        "Newton Centre"
    )
}
