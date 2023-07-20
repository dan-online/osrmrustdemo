use std::time::{Duration, Instant};

// Benchmarks a given library by taking an input function 'func' and running it 'runs' times with the given 'points'.
pub fn benchmark(
    func: fn(&str, Vec<(f64, f64)>) -> Duration,
    location: &str,
    lib: &str,
    runs: u32,
    points: Vec<(f64, f64)>,
) -> Vec<String> {
    println!("┌ Benchmarking {}", lib);

    // Create an empty vector 'times' to store the durations of each benchmark run.
    let mut run_times = Vec::with_capacity(runs as usize);

    // Run the benchmark 'runs' times.
    for _ in 0..runs {
        let start_time = Instant::now();

        func(location, points.clone());

        let time = start_time.elapsed();
        run_times.push(time);
    }

    let total_time: Duration = run_times.iter().sum();

    // Calculate the average time taken for each benchmark run.
    let average_time = total_time / runs;

    // Sort the 'times' vector to calculate the 95th percentile of the benchmark run times.
    run_times.sort();

    // Calculate the 95th percentile of the benchmark run times.
    let p95 = run_times[(runs as f32 * 0.95) as usize];

    // Calculate the standard deviation of the benchmark run times.
    let stdev = {
        let variance = run_times
            .iter()
            .map(|time| {
                let diff = time.as_nanos() as f32 - average_time.as_nanos() as f32;
                diff * diff
            })
            .sum::<f32>()
            / runs as f32;
        variance.sqrt()
    };

    // Print the total time taken for all benchmark runs.
    println!("└ Success, total time: {:?}", total_time);

    // Return a vector of strings containing benchmark results and statistics.
    vec![
        lib.to_string(),
        format!("{:.2?}", total_time),
        format!("{:.2?}", average_time),
        format!("{:.2?}", p95),
        format!("±{:.2?}", Duration::from_nanos(stdev as u64)),
    ]
}
