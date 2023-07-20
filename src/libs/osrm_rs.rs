use osrm::{Coordinate, Osrm};
use std::time::{Duration, Instant};

// Note: this library treats the input coordinates as f32 instead of f64. This is technically less accurate
pub fn osrm_rs(name: &str, points: Vec<(f64, f64)>) -> Duration {
    // Create an OSRM instance
    let osrm = Osrm::new(format!("./data/{}.osrm", name).as_str()).unwrap();

    // Convert the input points into a vector of Coordinate objects.
    let coordinates = points
        .iter()
        .map(|(lat, lon)| Coordinate {
            latitude: *lat as f32,
            longitude: *lon as f32,
        })
        .collect::<Vec<_>>();

    let speed = Instant::now();

    // Execute the routing request using the OSRM instance.
    osrm.table(&coordinates, &coordinates).unwrap();

    return speed.elapsed();
}
