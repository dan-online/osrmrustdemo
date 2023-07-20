use rsc_osrm::{general::Coordinate, table::TableRequest, Algorithm, EngineConfig, Osrm};
use std::time::{Duration, Instant};

pub fn rsc_osrm(name: &str, points: Vec<(f64, f64)>) -> Duration {
    // Create a new EngineConfig with the given dataset file path.
    let mut config = EngineConfig::new(format!("./data/{}.osrm", name).as_str());

    // Set the configuration to not use shared memory and use the CH (Contraction Hierarchies) algorithm.
    config.use_shared_memory = false;
    config.algorithm = Algorithm::CH;

    // Create a new OSRM instance with the specified configuration.
    let osrm = Osrm::new(&mut config).unwrap();

    // Convert the input points into a vector of Coordinate objects.
    let coordinates = points
        .iter()
        .map(|(lat, lon)| Coordinate {
            name: None,      // Optional name for the coordinate, set to None here.
            latitude: *lat,  // Latitude value extracted from the tuple.
            longitude: *lon, // Longitude value extracted from the tuple.
        })
        .collect::<Vec<_>>();

    let speed = Instant::now();

    // Create a new TableRequest with the vector of coordinates.
    let mut req = TableRequest::new(&coordinates);

    // Execute the routing request using the OSRM instance.
    req.run(&osrm);

    return speed.elapsed();
}
