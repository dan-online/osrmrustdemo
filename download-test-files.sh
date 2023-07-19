#!/bin/sh

# Yoinked and modified from https://github.com/deliveroo/osrm-rs/blob/master/prepare-test-data.sh

mkdir data
cd data

wget -N http://download.geofabrik.de/asia/gcc-states-latest.osm.pbf

echo "osrm-extract"

docker run -t --rm -v $(pwd):/data ghcr.io/project-osrm/osrm-backend:latest osrm-extract -p /opt/foot.lua /data/gcc-states-latest.osm.pbf

echo "osrm-contract"

docker run -t --rm -v $(pwd):/data ghcr.io/project-osrm/osrm-backend:latest osrm-contract /data/gcc-states-latest.osrm