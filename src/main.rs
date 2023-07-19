use std::{env::args, time::Instant};

const ORIGIN: (f64, f64) = (25.308135, 51.233283);
const DESTINATION: (f64, f64) = (25.783904, 51.229538);

fn osrm_rs(origin: (f64, f64), destination: (f64, f64)) -> (f64, f64) {
    use osrm::{Coordinate, Osrm};

    let osrm = Osrm::new("./data/gcc-states-latest.osrm").unwrap();

    let origin = Coordinate {
        latitude: origin.0 as f32,
        longitude: origin.1 as f32,
    };

    let destination = Coordinate {
        latitude: destination.0 as f32,
        longitude: destination.1 as f32,
    };

    let route_result = osrm.route(&origin, &destination);

    match route_result {
        Ok(route) => {
            return (route.distance.into(), route.duration.into());
        }
        Err(osrm_error) => {
            eprintln!(" {osrm_error}");
            return (0.0, 0.0);
        }
    }
}

fn rs_osrm(origin: (f64, f64), destination: (f64, f64)) -> (f64, f64) {
    use rs_osrm::{
        engine_config::engine_config_builder::EngineConfigBuilder,
        general::rs_structs::coordinate::Coordinate,
        route_api::route_request_builder::RouteRequestBuilder, Algorithm, Status,
    };

    let osrm = EngineConfigBuilder::new("./data/gcc-states-latest.osrm")
        .set_use_shared_memory(false)
        .set_algorithm(Algorithm::CH)
        .build()
        .unwrap();

    let mut req = RouteRequestBuilder::new(&vec![
        Coordinate::new(origin.0, origin.1),
        Coordinate::new(destination.0, destination.1),
    ])
    .build()
    .unwrap();

    let (status, result) = req.run(&osrm);

    match status {
        Status::Ok => {
            return (result.routes[0].distance, result.routes[0].duration);
        }
        Status::Error => {
            eprintln!(" Error: {:?}", result.message);
            return (0.0, 0.0);
        }
    }
}

fn _rsc_osrm(origin: (f64, f64), destination: (f64, f64)) -> (f64, f64) {
    use rsc_osrm::{
        general::Coordinate, route::RouteRequest, Algorithm, EngineConfig, Osrm, Status,
    };

    let mut config = EngineConfig::new("./data/gcc-states-latest.osrm");

    config.use_shared_memory = false;
    config.verbosity = Some("WARNING".to_string());
    config.algorithm = Algorithm::CH;

    let osrm = Osrm::new(&mut config).unwrap();

    let mut req = RouteRequest::new(&vec![
        Coordinate {
            name: Some("A".to_string()),
            latitude: origin.0,
            longitude: origin.1,
        },
        Coordinate {
            name: Some("B".to_string()),
            latitude: destination.0,
            longitude: destination.1,
        },
    ]);

    let (status, result) = req.run(&osrm);

    match status {
        Status::Ok => {
            return (result.routes[0].distance, result.routes[0].duration);
        }
        Status::Error => {
            eprintln!(" Error: {:?}", result.message);
            return (0.0, 0.0);
        }
    }
}

pub fn benchmark(func: fn((f64, f64), (f64, f64)) -> (f64, f64), name: &str, runs: u32) {
    println!("┌ Benchmarking {name}...");

    let start = Instant::now();
    let mut sample_res: (f64, f64) = (0.0, 0.0);
    for _ in 0..runs {
        if sample_res.0 == 0.0 {
            sample_res = func(ORIGIN, DESTINATION);
        } else {
            func(ORIGIN, DESTINATION);
        }
    }

    println!("│  Distance: {:?}", sample_res.0);
    println!("│  Duration: {:?}", sample_res.1);

    println!(
        "└ {:?} runs in {:?}, avg of {:?}",
        runs,
        start.elapsed(),
        start.elapsed() / runs
    );
}

#[test]
fn test_osrm_rs() {
    let (distance, duration) = osrm_rs(ORIGIN, ORIGIN);
    assert_eq!(distance, 0.0);
    assert_eq!(duration, 0.0);
}

#[test]
fn test_rs_osrm() {
    let (distance, duration) = rs_osrm(ORIGIN, ORIGIN);
    assert_eq!(distance, 0.0);
    assert_eq!(duration, 0.0);
}

#[test]
fn test_rsc_osrm() {
    let (distance, duration) = rsc_osrm(ORIGIN, ORIGIN);
    assert_eq!(distance, 0.0);
    assert_eq!(duration, 0.0);
}

fn main() {
    println!("🚀 OSRM Benchmarking");

    let runs = args()
        .nth(1)
        .and_then(|arg| arg.parse::<u32>().ok())
        .unwrap_or(500);

    benchmark(osrm_rs, "osrm-rs", runs);
    benchmark(rs_osrm, "rs-osrm", runs);
    benchmark(_rsc_osrm, "rsc-osrm", runs);
}
