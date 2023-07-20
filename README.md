# OSRM Rust Demo

This is a demo of the bindings libraries for the [OSRM](https://github.com/Project-OSRM/osrm-backend) routing engine. There are 3 main libraries that are used:

- [osrm-rs](osrm-rs) - A deliveroo project
- [rs_osrm](https://github.com/TehGoat/rs_osrm) - The only of the three actually published as a crate
- [rsc_osrm](rsc_osrm) - Another crate but not published

## Usage

This demo runs a benchmark of the 3 libraries using a certain amount of random coordinates within a certain radius of a point. It then outputs a chart with the results for comparison.

The other purpose of this demo is the Dockerfile which provides the perfect environment for running these libraries.

### Running with Docker

#### Building the image

```bash
$ docker build -t osrm-rust-demo .
$ docker run -it --rm -v $(pwd):/app osrm-rust-demo /bin/bash
```

#### Running the demo

##### CLI

```bash
root@<container-id>:/app# cargo run --release
🚀 OSRM Benchmarking 50 runs with 100 random points, ex: 51.50555699584881, -0.15835700509685283
┌ Benchmarking osrm-rs
└ Success, total time: 5.696485744s
┌ Benchmarking rs_osrm
└ Success, total time: 5.500096431s
┌ Benchmarking rsc_osrm
└ Success, total time: 5.428230374s
+----------+-------+----------+-----------------+--------------------+
| Library  | Total | Average  | 95th Percentile | Standard Deviation |
+====================================================================+
| osrm-rs  | 5.70s | 113.93ms | 125.10ms        | ±8.93ms            |
|----------+-------+----------+-----------------+--------------------|
| rs_osrm  | 5.50s | 110.00ms | 122.26ms        | ±7.22ms            |
|----------+-------+----------+-----------------+--------------------|
| rsc_osrm | 5.43s | 108.56ms | 112.93ms        | ±3.03ms            |
+----------+-------+----------+-----------------+--------------------+
```

##### Dev containers

If you're using VSCode, you can use the dev containers feature to run this project in a container. This will give you a fully working environment with all the dependencies and extensions installed. Simply install the "ms-vscode-remote.remote-containers" extension and open the project with the image you built in [the previous step](#building-the-image).

