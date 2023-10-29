use std::{
    collections::HashMap,
    fs::File,
    fs::{self},
    io::Read,
};

use super::*;

use anyhow::{anyhow, Result};
use clap::ArgMatches;

const MBTA_API_URL: &str = "https://api-v3.mbta.com";
const CACHE_FILE_PATH: &str = ".mbtacache";

// initial naive implementation: just serialize using serde and save dictionary to disk
fn read_cached_stops_from_file<'a>(id: &'a String) -> Result<Option<StopData>> {
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
    let stop_map = match serde_json::from_str::<HashMap<String, StopData>>(&cache_serialized) {
        Ok(dict) => dict,
        //is return Err right here?
        Err(err) => {
            return Err(anyhow!(format!(
                "couldn't deserialize cache {}: {}",
                CACHE_FILE_PATH, err
            )))
        }
    };

    return Ok(stop_map.get(id).cloned());
}

fn default_stop_data() -> StopData {
    return StopData {
        id: "Unknown".to_string(),
        attributes: StopAttributes {
            name: "Unknown Stop".to_string(),
        },
    };
}

fn default_data() -> Data {
    return Data {
        id: "Unknown".to_string(),
    };
}

fn write_stops_cache<'b>(id: &'b String) -> StopData {
    // what is this first question mark
    // still not sure how to use anyhow🤷
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
    let mut stop_map = HashMap::<String, StopData>::new();

    // assume no duplicate stop ids
    for stop in deserialized.data {
        stop_map.insert(stop.id.clone(), stop);
    }

    // just assuming it works lawl
    fs::write(CACHE_FILE_PATH, serde_json::to_string(&stop_map).unwrap());

    return match stop_map.get(id) {
        Some(stop) => stop.clone(),
        None => default_stop_data(),
    };
}

// fetch a stop by stop id. if we can read from the file system cache,
// read from the file system cache. otherwise, invalidate the filesystem stop cache
fn fetch_stop_by_id(potential_id: Option<ObjectData>) -> StopData {
    // TODO this is shitty

    let id = match potential_id.unwrap().data {
        Some(data) => data.id,
        None => return default_stop_data(),
    };

    let stop = match read_cached_stops_from_file(&id) {
        Ok(stop) => stop,

        // we couldn't read the stop cache file. try to write again
        Err(err) => {
            dbg!("Could not read cache...{}", err);
            None
        }
    };

    // we read the stop cache, but there was nothing there for id
    //
    let result = match stop {
        Some(stop) => stop,
        None => {
            // invalidate the cache
            write_stops_cache(&id)
        }
    };

    return result;
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

        let stop = fetch_stop_by_id(vehicle.relationships.stop);
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
    let direction = match args.get_one::<String>("direction") {
        Some(dir) => dir,
        // default to inbound
        None => "1",
    };

    let res = match reqwest::blocking::get(format!(
        "{}/predictions?filter[stop]={}&filter[direction_id]={}",
        MBTA_API_URL, stop, direction
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

    for prediction in deserialized.data {
        let route_id = match prediction.relationships.route.data {
            Some(data) => data.id,
            None => "Unknown Route".to_string(),
        };
        println!(
            "{} will arive at {} going {}",
            route_id,
            prediction
                .attributes
                .arrival_time
                .unwrap_or("unknown".to_string()),
            direction
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
