use rand::Rng;

// Generate a random coordinate within a given distance (in km) of a given location.
pub fn random_coord(location: (f64, f64), distance_in_km: usize) -> (f64, f64) {
    let (lat, lon) = location;

    // Convert distance_in_km to degrees (approximately 111 km per degree of latitude/longitude).
    let distance_in_degrees = distance_in_km as f64 / 111.0;

    // Create a random number generator.
    let mut rng = rand::thread_rng();

    // Generate random latitude and longitude adjustments within the specified distance.
    let lat_adjustment = rng.gen_range(-distance_in_degrees..distance_in_degrees);
    let lon_adjustment = rng.gen_range(-distance_in_degrees..distance_in_degrees);

    // Calculate the new latitude and longitude by adding the adjustments.
    let lat = lat + lat_adjustment;
    let lon = lon + lon_adjustment;

    // Return the new random coordinates.
    (lat, lon)
}
