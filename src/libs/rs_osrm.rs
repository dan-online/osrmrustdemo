use rs_osrm::{
    engine_config::engine_config_builder::EngineConfigBuilder,
    general::rs_structs::coordinate::Coordinate,
    table_api::table_request_builder::TableRequestBuilder, Algorithm,
};
use std::time::{Duration, Instant};

pub fn rs_osrm(name: &str, points: Vec<(f64, f64)>) -> Duration {
    // Create a new Osrm instance with the given dataset file path.
    let osrm = EngineConfigBuilder::new(format!("./data/{}.osrm", name).as_str())
        .set_use_shared_memory(false)
        .set_algorithm(Algorithm::CH)
        .build()
        .unwrap();

    // Convert the input points into a vector of Coordinate objects.
    let coordinates = points
        .iter()
        .map(|(lat, lon)| Coordinate::new(*lat, *lon)) // Create a new Coordinate with the latitude and longitude from the tuple.
        .collect::<Vec<_>>(); // Collect the Coordinate objects into a vector.

    let speed = Instant::now();

    // Create a new TableRequest with the vector of coordinates.
    let mut req = TableRequestBuilder::new(&coordinates).build().unwrap();

    // Execute the table routing request using the OSRM instance.
    req.run(&osrm);

    return speed.elapsed();
}
