use super::*;
use anyhow::Result;
use clap::ArgMatches;
use serde::Deserialize;

const MBTA_API_URL: &str = "https://api-v3.mbta.com";

#[derive(Debug, Deserialize)]
struct Data {
    id: String,
}

#[derive(Debug, Deserialize)]
struct ObjectData {
    data: Data,
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
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VehicleData {
    attributes: VehicleAttributes,
    relationships: VehicleRelationships,
}

#[derive(Debug, Deserialize)]
struct VehiclesResponse {
    data: Vec<VehicleData>,
}

#[derive(Debug, Deserialize)]
struct StopAttributes {
    name: String,
}

#[derive(Debug, Deserialize)]
struct StopData {
    attributes: StopAttributes,
}

#[derive(Debug, Deserialize)]
struct StopsResponse {
    data: StopData,
}

fn fetch_stop_by_id(potential_id: Option<ObjectData>) -> Result<String> {
    let id = potential_id.unwrap().data.id;

    // what is this first question mark haha
    // still not sure how to use anyhow🤷
    let stop_resp = match reqwest::blocking::get(format!("{}/stops/{}", MBTA_API_URL, id))?.text() {
        Ok(res) => res.to_string(),
        Err(err) => {
            println!("Encountered error fetching data for stop ID {}!", id);
            return Err(err.into());
        }
    };

    let stop_name = match serde_json::from_str::<StopsResponse>(&stop_resp) {
        Ok(res) => res.data.attributes.name,
        Err(err) => {
            println!("Failed to deserialize stop with ID {}!", id);
            return Err(err.into());
        }
    };

    Ok(stop_name)
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

    let res = reqwest::blocking::get(format!("{}/vehicles", MBTA_API_URL))?;

    let deserialized = serde_json::from_str::<VehiclesResponse>(&res.text()?).unwrap();
    for vehicle in deserialized.data {
        // TODO figure out what to do with these unwraps
        let route_id = vehicle.relationships.route.unwrap().data.id;

        // TODO filter this better (is there something i can pass to the API?)
        if service.is_some() && !route_id.contains(service.unwrap()) {
            continue;
        }

        let stop_name = match fetch_stop_by_id(vehicle.relationships.stop) {
            Ok(stop_name) => stop_name,
            Err(_) => "Unknown Stop".to_string(),
        };

        println!(
            "Vehicle {}: {} {} at {}",
            vehicle.attributes.id.unwrap_or("Unknown ID".to_string()),
            route_id,
            vehicle.attributes.current_status,
            stop_name
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
        stop_deserialized.unwrap().data.attributes.name,
        "Newton Centre"
    )
}
