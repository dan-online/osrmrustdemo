#!/bin/sh

# Yoinked and modified from https://github.com/deliveroo/osrm-rs/blob/master/prepare-test-data.sh

# http://download.geofabrik.de/europe/great-britain/england/greater-london-latest.osm.pbf

region="europe/great-britain/england"
name="greater-london-latest"

mkdir data
cd data

wget -N https://download.geofabrik.de/${region}/${name}.osm.pbf

echo "osrm-extract"

docker run -t --rm -v $(pwd):/data ghcr.io/project-osrm/osrm-backend:latest osrm-extract -p /opt/foot.lua /data/${name}.osm.pbf

echo "osrm-contract"

docker run -t --rm -v $(pwd):/data ghcr.io/project-osrm/osrm-backend:latest osrm-contract /data/${name}.osrm