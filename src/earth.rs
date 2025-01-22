use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::{pyfunction, PyResult};

pub const RADIUS: f64 = 6371.0; // km
pub const CIRCUMFERENCE: f64 = 40075.0; // km
pub const SEMI_MAJOR_AXIS: f64 = 6378.137; // km
pub const SEMI_MINOR_AXIS: f64 = 6_356.752_4; // km
pub const FLATTENING: f64 = 1.0 / 298.257_23;

fn validate_coordinate_pair(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> Result<(), String> {
    let latitude_range = -90.0..=90.0;
    let longitude_range = -180.0..=180.0;

    if !latitude_range.contains(&lat1) {
        return Err("Latitude 1 must be between -90 and 90 degrees.".to_string());
    }
    if !longitude_range.contains(&lon1) {
        return Err("Longitude 1 must be between -180 and 180 degrees.".to_string());
    }
    if !latitude_range.contains(&lat2) {
        return Err("Latitude 2 must be between -90 and 90 degrees.".to_string());
    }
    if !longitude_range.contains(&lon2) {
        return Err("Longitude 2 must be between -180 and 180 degrees.".to_string());
    }

    Ok(())
}


fn convert_degrees_to_radians(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> (f64, f64, f64, f64) {
    let lat1 = lat1.to_radians();
    let lon1 = lon1.to_radians();
    let lat2 = lat2.to_radians();
    let lon2 = lon2.to_radians();
    (lat1, lon1, lat2, lon2)
}

#[pyfunction]
#[pyo3(signature = (lat1, lon1, lat2, lon2))]
pub fn tunnel_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> PyResult<f64> {
    if let Err(msg) = validate_coordinate_pair(lat1, lon1, lat2, lon2) {
        return Err(PyValueError::new_err(msg));
    }

    let (lat1, lon1, lat2, lon2) = convert_degrees_to_radians(lat1, lon1, lat2, lon2);

    // Convert to Cartesian coordinates
    let x1 = RADIUS * lat1.cos() * lon1.cos();
    let y1 = RADIUS * lat1.cos() * lon1.sin();
    let z1 = RADIUS * lat1.sin();

    let x2 = RADIUS * lat2.cos() * lon2.cos();
    let y2 = RADIUS * lat2.cos() * lon2.sin();
    let z2 = RADIUS * lat2.sin();

    // Calculate Euclidean distance
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dz = z2 - z1;

    Ok((dx * dx + dy * dy + dz * dz).sqrt())
}

#[pyfunction]
#[pyo3(signature = (lat1, lon1, lat2, lon2))]
pub fn great_circle_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> PyResult<f64> {
    haversine_distance(lat1, lon1, lat2, lon2)
}

#[pyfunction]
#[pyo3(signature = (lat1, lon1, lat2, lon2))]
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> PyResult<f64> {
    if let Err(msg) = validate_coordinate_pair(lat1, lon1, lat2, lon2) {
        return Err(PyValueError::new_err(msg));
    }

    let (lat1, lon1, lat2, lon2) = convert_degrees_to_radians(lat1, lon1, lat2, lon2);

    // Haversine formula
    let d_lat = lat2 - lat1;
    let d_lon = lon2 - lon1;

    let a = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().asin();

    // Calculate the distance
    let distance = RADIUS * c;

    Ok(distance)
}

#[pyfunction]
#[pyo3(signature = (lat1, lon1, lat2, lon2))]
pub fn vincenty_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> PyResult<f64> {
    if let Err(msg) = validate_coordinate_pair(lat1, lon1, lat2, lon2) {
        return Err(PyValueError::new_err(msg));
    }

    // WGS-84 ellipsoidal constants
    const SEMI_MAJOR_AXIS_METER: f64 = SEMI_MAJOR_AXIS * 1000.0; // semi-major axis in meters
    const SEMI_MINOR_AXIS_METER: f64 = SEMI_MINOR_AXIS * 1000.0; // semi-minor axis in meters
    const MAX_ITERATIONS: i32 = 1_000;
    const CONVERGENCE_THRESHOLD: f64 = 1e-8;

    let (phi1, lambda1, phi2, lambda2) = convert_degrees_to_radians(lat1, lon1, lat2, lon2);

    // Check if points are the same
    if (phi1 - phi2).abs() < CONVERGENCE_THRESHOLD &&
        (lambda1 - lambda2).abs() < CONVERGENCE_THRESHOLD {
        return Ok(0.0);
    }

    let reduced_latitude1 = ((1.0 - FLATTENING) * phi1.tan()).atan();
    let reduced_latitude2 = ((1.0 - FLATTENING) * phi2.tan()).atan();

    let omega = lambda2 - lambda1;

    let mut lambda = omega;
    let mut sigma;
    let mut sin_sigma;
    let mut cos_sigma;
    let mut cos2_sigma_m;
    let mut sin_alpha;
    let mut cos2_alpha;
    let mut c;

    for _ in 0..MAX_ITERATIONS {
        let sin_lambda = lambda.sin();
        let cos_lambda = lambda.cos();

        let temp1 = reduced_latitude2.cos() * sin_lambda;
        let temp2 = reduced_latitude1.cos() * reduced_latitude2.sin() -
            reduced_latitude1.sin() * reduced_latitude2.cos() * cos_lambda;
        sin_sigma = (temp1 * temp1 + temp2 * temp2).sqrt();

        if sin_sigma.abs() < CONVERGENCE_THRESHOLD {
            return Ok(0.0); // Points are coincident
        }

        cos_sigma = reduced_latitude1.sin() * reduced_latitude2.sin() +
            reduced_latitude1.cos() * reduced_latitude2.cos() * cos_lambda;

        sigma = sin_sigma.atan2(cos_sigma);

        sin_alpha = reduced_latitude1.cos() * reduced_latitude2.cos() * sin_lambda / sin_sigma;
        cos2_alpha = 1.0 - sin_alpha * sin_alpha;

        cos2_sigma_m = if cos2_alpha != 0.0 {
            cos_sigma - 2.0 * reduced_latitude1.sin() * reduced_latitude2.sin() / cos2_alpha
        } else {
            0.0 // Equatorial line
        };

        c = FLATTENING / 16.0 * cos2_alpha * (4.0 + FLATTENING * (4.0 - 3.0 * cos2_alpha));

        let lambda_prev = lambda;
        lambda = omega + (1.0 - c) * FLATTENING * sin_alpha *
            (sigma + c * sin_sigma * (cos2_sigma_m + c * cos_sigma *
                (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)));

        if (lambda - lambda_prev).abs() < CONVERGENCE_THRESHOLD {
            // Calculate final distance
            let u2 = cos2_alpha * (SEMI_MAJOR_AXIS_METER * SEMI_MAJOR_AXIS_METER - SEMI_MINOR_AXIS_METER * SEMI_MINOR_AXIS_METER) / (SEMI_MINOR_AXIS_METER * SEMI_MINOR_AXIS_METER);
            let a = 1.0 + u2 / 16384.0 * (4096.0 + u2 * (-768.0 + u2 * (320.0 - 175.0 * u2)));
            let b = u2 / 1024.0 * (256.0 + u2 * (-128.0 + u2 * (74.0 - 47.0 * u2)));
            let delta_sigma = b * sin_sigma * (cos2_sigma_m + b / 4.0 *
                (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m) -
                    b / 6.0 * cos2_sigma_m * (-3.0 + 4.0 * sin_sigma * sin_sigma) *
                        (-3.0 + 4.0 * cos2_sigma_m * cos2_sigma_m)));

            let distance = SEMI_MINOR_AXIS_METER * a * (sigma - delta_sigma);

            // Convert to kilometers and return
            return Ok(distance / 1000.0);
        }
    }

    // If we get here, the algorithm didn't converge
    Err(PyValueError::new_err("Vincenty formula failed to converge"))
}

#[pyfunction]
#[pyo3(signature = (lat1, lon1, lat2, lon2))]
pub fn bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> PyResult<f64> {
    if let Err(msg) = validate_coordinate_pair(lat1, lon1, lat2, lon2) {
        return Err(PyValueError::new_err(msg));
    }

    let (lat1, lon1, lat2, lon2) = convert_degrees_to_radians(lat1, lon1, lat2, lon2);

    // Calculate difference in longitudes
    let delta_lon = lon2 - lon1;

    // Calculate bearing using the great circle formula
    let y = delta_lon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * delta_lon.cos();

    // Calculate initial bearing
    let initial_bearing = y.atan2(x).to_degrees();

    Ok((initial_bearing + 360.0) % 360.0)
}


#[pyfunction]
#[pyo3(signature = (lat, lon, distance, bearing))]
pub fn destination(lat: f64, lon: f64, distance: f64, bearing: f64) -> PyResult<(f64, f64)> {
    if !(-90.0..=90.0).contains(&lat) {
        return Err(PyValueError::new_err("Latitude must be between -90 and 90 degrees."));
    }
    if !(-180.0..=180.0).contains(&lon) {
        return Err(PyValueError::new_err("Longitude must be between -180 and 180 degrees."));
    }
    if distance < 0.0 {
        return Err(PyValueError::new_err("Distance cannot be negative."));
    }

    let (radian_lat, radian_lon, _, _) = convert_degrees_to_radians(lat, lon, 0.0, 0.0);
    let bearing_rad = bearing.to_radians();

    // Calculate angular distance
    let angular_distance = distance / RADIUS * 1000.0;

    // Calculate destination point using spherical trigonometry
    let destination_lat = (radian_lat.sin() * angular_distance.cos() +
        radian_lat.cos() * angular_distance.sin() * bearing_rad.cos())
        .asin();

    let destination_lon = radian_lon + (bearing_rad.sin() * angular_distance.sin() * radian_lat.cos())
        .atan2(angular_distance.cos() - radian_lat.sin() * destination_lat.sin());

    // Convert back to degrees and normalize
    let destination_lat_deg = destination_lat.to_degrees();
    let mut destination_lon_deg = destination_lon.to_degrees();

    // Normalize longitude to -180 to 180 degrees
    destination_lon_deg = ((destination_lon_deg + 540.0) % 360.0) - 180.0;

    Ok((destination_lat_deg, destination_lon_deg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    const EPSILON: f64 = 1e-3;  // For floating point comparisons

    #[test]
    fn test_valid_coordinate_pair() {
        assert!(validate_coordinate_pair(40.7128, -74.0060, 51.5074, -0.1278).is_ok());
        assert!(validate_coordinate_pair(-33.8568, 151.2070, 37.7749, -122.4194).is_ok());
        assert!(validate_coordinate_pair(0.0, 0.0, 90.0, 180.0).is_ok());
    }

    #[test]
    fn test_invalid_latitude1() {
        assert!(validate_coordinate_pair(100.0, -74.0060, 51.5074, -0.1278).is_err());
        assert!(validate_coordinate_pair(-100.0, -74.0060, 51.5074, -0.1278).is_err());
    }

    #[test]
    fn test_invalid_longitude1() {
        assert!(validate_coordinate_pair(40.7128, 200.0, 51.5074, -0.1278).is_err());
        assert!(validate_coordinate_pair(40.7128, -200.0, 51.5074, -0.1278).is_err());
    }

    #[test]
    fn test_invalid_latitude2() {
        assert!(validate_coordinate_pair(40.7128, -74.0060, 100.0, -0.1278).is_err());
        assert!(validate_coordinate_pair(40.7128, -74.0060, -100.0, -0.1278).is_err());
    }

    #[test]
    fn test_invalid_longitude2() {
        assert!(validate_coordinate_pair(40.7128, -74.0060, 51.5074, 200.0).is_err());
        assert!(validate_coordinate_pair(40.7128, -74.0060, 51.5074, -200.0).is_err());
    }

    #[test]
    fn test_zero_conversion() {
        let (lat1, lon1, lat2, lon2) = convert_degrees_to_radians(0.0, 0.0, 0.0, 0.0);
        assert!(lat1.abs() < EPSILON);
        assert!(lon1.abs() < EPSILON);
        assert!(lat2.abs() < EPSILON);
        assert!(lon2.abs() < EPSILON);
    }

    #[test]
    fn test_90_degree_conversion() {
        let (lat1, lon1, lat2, lon2) = convert_degrees_to_radians(90.0, 90.0, 90.0, 90.0);
        assert!((lat1 - PI/2.0).abs() < EPSILON);
        assert!((lon1 - PI/2.0).abs() < EPSILON);
        assert!((lat2 - PI/2.0).abs() < EPSILON);
        assert!((lon2 - PI/2.0).abs() < EPSILON);
    }

    #[test]
    fn test_negative_angles() {
        let (lat1, lon1, lat2, lon2) = convert_degrees_to_radians(-45.0, -180.0, -90.0, -30.0);
        assert!((lat1 - (-PI/4.0)).abs() < EPSILON);
        assert!((lon1 - (-PI)).abs() < EPSILON);
        assert!((lat2 - (-PI/2.0)).abs() < EPSILON);
        assert!((lon2 - (-PI/6.0)).abs() < EPSILON);
    }

    #[test]
    fn test_mixed_angles() {
        let (lat1, lon1, lat2, lon2) = convert_degrees_to_radians(45.0, -120.0, -30.0, 150.0);
        assert!((lat1 - PI/4.0).abs() < EPSILON);
        assert!((lon1 - (-2.0*PI/3.0)).abs() < EPSILON);
        assert!((lat2 - (-PI/6.0)).abs() < EPSILON);
        assert!((lon2 - (5.0*PI/6.0)).abs() < EPSILON);
    }

    #[test]
    fn test_full_circle() {
        let (lat1, lon1, lat2, lon2) = convert_degrees_to_radians(360.0, 360.0, 720.0, -360.0);
        assert!((lat1 - 2.0*PI).abs() < EPSILON);
        assert!((lon1 - 2.0*PI).abs() < EPSILON);
        assert!((lat2 - 4.0*PI).abs() < EPSILON);
        assert!((lon2 - (-2.0*PI)).abs() < EPSILON);
    }

    #[test]
    fn test_common_coordinates() {
        // Test with New York coordinates (approximately)
        let (lat1, lon1, _, _) = convert_degrees_to_radians(40.7128, -74.0060, 0.0, 0.0);
        assert!((lat1 - 0.7099).abs() < EPSILON);
        assert!((lon1 - (-1.2916)).abs() < EPSILON);
    }

    #[test]
    fn test_precision() {
        // Test with a very small angle
        let (lat1, _, _, _) = convert_degrees_to_radians(0.0001, 0.0, 0.0, 0.0);
        assert!((lat1 - 1.7453292519943295e-6).abs() < EPSILON);
    }
}
