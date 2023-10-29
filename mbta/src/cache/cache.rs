mod cache;

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
}
