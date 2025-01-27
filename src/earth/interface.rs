use crate::earth::coordinates::Coordinate;
use pyo3::exceptions::PyValueError;
#[allow(unused_imports)]
use pyo3::prelude::*;
use pyo3::{pyfunction, PyResult};

use crate::earth::constants::{
    CIRCUMFERENCE, FLATTENING, RADIUS, SEMI_MAJOR_AXIS, SEMI_MINOR_AXIS,
};

fn validate_coordinate_pair(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
) -> Result<(Coordinate, Coordinate), String> {
    let coordinate_1 = Coordinate::new(lat1, lon1);
    let coordinate_2 = Coordinate::new(lat2, lon2);

    if coordinate_1.is_err() {
        return Err(coordinate_1.err().unwrap());
    } else if coordinate_2.is_err() {
        return Err(coordinate_2.err().unwrap());
    }

    Ok((coordinate_1.unwrap(), coordinate_2.unwrap()))
}

#[pyfunction]
#[pyo3(signature = (lat1, lon1, lat2, lon2))]
pub fn are_antipodal(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> bool {
    let lat_condition = lat1 == -lat2;

    let lon_difference = (lon1 - lon2).abs();
    let lon_condition = (lon_difference == 180.0) || (lon_difference == 0.0 && lat1.abs() == 90.0);

    lat_condition && lon_condition
}

#[pyfunction]
#[pyo3(signature = (lat1, lon1, lat2, lon2))]
pub fn tunnel_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> PyResult<f64> {
    let coordinates = validate_coordinate_pair(lat1, lon1, lat2, lon2);
    if coordinates.is_err() {
        return Err(PyValueError::new_err(coordinates.err().unwrap()));
    }

    let (coordinate_1, coordinate_2) = coordinates.unwrap();
    let (x1, y1, z1) = coordinate_1.in_cartesian();
    let (x2, y2, z2) = coordinate_2.in_cartesian();

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
    let coordinates = validate_coordinate_pair(lat1, lon1, lat2, lon2);
    if coordinates.is_err() {
        return Err(PyValueError::new_err(coordinates.err().unwrap()));
    }

    let (coordinate_1, coordinate_2) = coordinates.unwrap();
    let (lat1, lon1) = coordinate_1.in_radians();
    let (lat2, lon2) = coordinate_2.in_radians();

    // Haversine formula
    let d_lat = lat2 - lat1;
    let d_lon = lon2 - lon1;

    let a = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);

    // Calculate the distance
    Ok(RADIUS * 2.0 * a.sqrt().asin())
}

#[pyfunction]
#[pyo3(signature = (lat1, lon1, lat2, lon2))]
pub fn vincenty_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> PyResult<f64> {
    const CONVERGENCE_THRESHOLD: f64 = 1e-8;
    const MAX_ITERATIONS: i32 = 1_000;

    let coordinates = validate_coordinate_pair(lat1, lon1, lat2, lon2);
    if coordinates.is_err() {
        return Err(PyValueError::new_err(coordinates.err().unwrap()));
    } else if (lat1 - lat2).abs() < CONVERGENCE_THRESHOLD
        && (lon1 - lon2).abs() < CONVERGENCE_THRESHOLD
    {
        return Ok(0.0);
    } else if are_antipodal(lat1, lon1, lat2, lon2) {
        return Ok(0.5 * CIRCUMFERENCE);
    }

    let (coordinate_1, coordinate_2) = coordinates.unwrap();
    let (phi1, lambda1) = coordinate_1.in_radians();
    let (phi2, lambda2) = coordinate_2.in_radians();

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
        let temp2 = reduced_latitude1.cos() * reduced_latitude2.sin()
            - reduced_latitude1.sin() * reduced_latitude2.cos() * cos_lambda;
        sin_sigma = (temp1 * temp1 + temp2 * temp2).sqrt();

        if sin_sigma.abs() < CONVERGENCE_THRESHOLD {
            return Ok(0.0); // Points are coincident
        }

        cos_sigma = reduced_latitude1.sin() * reduced_latitude2.sin()
            + reduced_latitude1.cos() * reduced_latitude2.cos() * cos_lambda;

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
        lambda = omega
            + (1.0 - c)
                * FLATTENING
                * sin_alpha
                * (sigma
                    + c * sin_sigma
                        * (cos2_sigma_m
                            + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)));

        if (lambda - lambda_prev).abs() < CONVERGENCE_THRESHOLD {
            // Calculate final distance
            let u2 = cos2_alpha * (SEMI_MAJOR_AXIS.powi(2) - SEMI_MINOR_AXIS.powi(2))
                / SEMI_MINOR_AXIS.powi(2);
            let a = 1.0 + u2 / 16384.0 * (4096.0 + u2 * (-768.0 + u2 * (320.0 - 175.0 * u2)));
            let b = u2 / 1024.0 * (256.0 + u2 * (-128.0 + u2 * (74.0 - 47.0 * u2)));
            let delta_sigma = b
                * sin_sigma
                * (cos2_sigma_m
                    + b / 4.0
                        * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)
                            - b / 6.0
                                * cos2_sigma_m
                                * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                                * (-3.0 + 4.0 * cos2_sigma_m * cos2_sigma_m)));

            return Ok(SEMI_MINOR_AXIS * a * (sigma - delta_sigma));
        }
    }

    // If we get here, the algorithm didn't converge
    Err(PyValueError::new_err("Vincenty formula failed to converge"))
}

#[pyfunction]
#[pyo3(signature = (lat1, lon1, lat2, lon2))]
pub fn bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> PyResult<f64> {
    let coordinates = validate_coordinate_pair(lat1, lon1, lat2, lon2);
    if coordinates.is_err() {
        return Err(PyValueError::new_err(coordinates.err().unwrap()));
    }

    let (coordinate_1, coordinate_2) = coordinates.unwrap();
    let (lat1, lon1) = coordinate_1.in_radians();
    let (lat2, lon2) = coordinate_2.in_radians();

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
    let coordinate = Coordinate::new(lat, lon);

    if coordinate.is_err() {
        return Err(PyValueError::new_err(coordinate.err().unwrap()));
    }
    if distance < 0.0 {
        return Err(PyValueError::new_err("Distance cannot be negative."));
    }

    let (radian_lat, radian_lon) = coordinate.unwrap().in_radians();
    let bearing_rad = bearing.to_radians();

    // Calculate angular distance
    let angular_distance = distance / RADIUS;

    // Calculate destination point using spherical trigonometry
    let destination_lat = (radian_lat.sin() * angular_distance.cos()
        + radian_lat.cos() * angular_distance.sin() * bearing_rad.cos())
    .asin();

    let destination_lon = radian_lon
        + (bearing_rad.sin() * angular_distance.sin() * radian_lat.cos())
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
    const EPSILON: f64 = 1e-3; // For floating point comparisons

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
}
