mod benchmark;
mod libs;
mod utils;

use crate::{
    libs::{osrm_rs::osrm_rs, rs_osrm::rs_osrm, rsc_osrm::rsc_osrm},
    utils::random_coord::random_coord,
};
use benchmark::benchmark;
use comfy_table::Table;

fn main() {
    // Define the number of runs for benchmarking
    let runs = 50;

    // Define the location data to be used (in this case, "greater-london-latest")
    let location = "greater-london-latest";

    // Define the coordinates (latitude and longitude) for generating random points
    let coords = (51.509865, -0.118092);

    // Create an empty vector to store random coordinates
    let mut points = Vec::new();

    // Generate 100 random coordinates around the given `coords` at a 5km radius
    for _ in 0..100 {
        points.push(random_coord(coords, 5));
    }

    // Get the number of points generated
    let point_count = points.len();

    // Extract the latitude and longitude of the first generated point
    let demo_lat = points[0].0;
    let demo_lon = points[0].1;

    // Print information about the benchmark setup and the data being used
    println!(
        "🚀 OSRM Benchmarking {runs} runs with {point_count} random points, ex: {demo_lat}, {demo_lon}"
    );

    // Perform the benchmarking for each library (osrm_rs, rs_osrm, rsc_osrm)
    // and get the results for each run (total, average, 95th percentile, standard deviation)
    let osrm_rs_results = benchmark(osrm_rs, &location, "osrm-rs", runs, points.clone());
    let rs_osrm_results = benchmark(rs_osrm, &location, "rs_osrm", runs, points.clone());
    let rsc_osrm_results = benchmark(rsc_osrm, &location, "rsc_osrm", runs, points.clone());

    // Create a table to display the benchmarking results in a tabular format
    let mut table = Table::new();

    // Set the header of the table with appropriate column names
    table
        .set_header(vec![
            "Library",
            "Total",
            "Average",
            "95th Percentile",
            "Standard Deviation",
        ])
        .add_row(osrm_rs_results)
        .add_row(rs_osrm_results)
        .add_row(rsc_osrm_results);

    // Print the table containing the benchmarking results
    println!("{table}");
}
