mod cache;
use cli::StopData;

pub struct MbtaCache {
    stops: HashMap<String, StopData>,
    routes: HashMap<String, RouteData>

}

