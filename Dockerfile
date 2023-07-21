FROM ubuntu:latest

# set puid and pgid to 1000
ENV PUID 1000
ENV PGID 1000

# Accept any questions
ARG DEBIAN_FRONTEND=noninteractive

# Faster downloads
# RUN sed -i -e 's/http:\/\/archive\.ubuntu\.com\/ubuntu\//mirror:\/\/mirrors\.ubuntu\.com\/mirrors\.txt/' /etc/apt/sources.list

# Install all required libs
RUN --mount=type=cache,target=/var/lib/apt \
    apt-get update -y && \
    apt-get install -y \
        curl \
        git \
        g++ \
        cmake \
        libboost-dev \
        libboost-filesystem-dev \
        libboost-thread-dev \
        libboost-system-dev \
        libboost-regex-dev \
        libxml2-dev \
        libsparsehash-dev \
        libbz2-dev \
        zlib1g-dev \
        libzip-dev \
        libgomp1 \
        liblua5.3-dev \
        pkg-config \
        libgdal-dev \
        libboost-program-options-dev \
        libboost-iostreams-dev \
        libboost-test-dev \
        libtbb-dev \
        libexpat1-dev \
        libclang-dev

# Create libosrm
RUN git clone --branch v5.27.1 https://github.com/Project-OSRM/osrm-backend /osrm-backend
WORKDIR /osrm-backend

# Setup
RUN mkdir build
WORKDIR build

# Build
RUN cmake ..
RUN cmake --build .
RUN cmake --build . --target install

RUN rm -rf /osrm-backend

# Create c_osrm
RUN git clone https://github.com/TehGoat/c_osrm /c_osrm
WORKDIR /c_osrm

RUN mkdir build
WORKDIR build

# Build the c version
RUN cmake ..
RUN cmake --build .
RUN cmake --build . --target install

RUN rm -rf /c_osrm

# Connect the shared libraries
RUN ldconfig /usr/local/lib/

# Install cargo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add safe repos to git
RUN git config --global --add safe.directory /app/rsc_osrm/c_osrm
RUN git config --global --add safe.directory /app/rsc_osrm/c_osrm/osrm-backend

# Mount your volume to /app and you're ready to develop, or use a dev container ofc
WORKDIR /app