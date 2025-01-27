use crate::earth::constants::RADIUS;

pub struct Coordinate {
    lat: f64,
    lon: f64,
}

impl Coordinate {
    pub fn new(lat: f64, lon: f64) -> Result<Self, String> {
        if let Err(msg) = Self::validate_coordinate(lat, lon) {
            Err(msg)
        } else {
            Ok(Self { lat, lon })
        }
    }

    fn validate_coordinate(lat: f64, lon: f64) -> Result<(), String> {
        if !(-90.0..=90.0).contains(&lat) {
            return Err("Latitude must be between -90 and 90 degrees.".to_string());
        }
        if !(-180.0..=180.0).contains(&lon) {
            return Err("Longitude must be between -180 and 180 degrees.".to_string());
        }
        Ok(())
    }

    pub fn in_radians(self) -> (f64, f64) {
        (self.lat.to_radians(), self.lon.to_radians())
    }

    pub fn in_cartesian(self) -> (f64, f64, f64) {
        let (lat, lon) = self.in_radians();
        (
            RADIUS * lat.cos() * lon.cos(),
            RADIUS * lat.cos() * lon.sin(),
            RADIUS * lat.sin(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_coordinate_construction() {
        let coordinate_1 = Coordinate::new(0.0, 0.0).unwrap();
        assert_eq!(coordinate_1.lat, 0.0);
        assert_eq!(coordinate_1.lon, 0.0);

        let coordinate_2 = Coordinate::new(-90.0, 180.0).unwrap();
        assert_eq!(coordinate_2.lat, -90.0);
        assert_eq!(coordinate_2.lon, 180.0);

        let coordinate_3 = Coordinate::new(90.0, -180.0).unwrap();
        assert_eq!(coordinate_3.lat, 90.0);
        assert_eq!(coordinate_3.lon, -180.0);
    }

    #[test]
    fn test_wrong_coordinates() {
        let coordinate_1 = Coordinate::new(-91.0, 0.0);
        assert!(
            coordinate_1.is_err(),
            "Latitude must be between -90 and 90 degrees."
        );

        let coordinate_2 = Coordinate::new(91.0, 0.0);
        assert!(
            coordinate_2.is_err(),
            "Latitude must be between -90 and 90 degrees."
        );

        let coordinate_3 = Coordinate::new(0.0, 181.0);
        assert!(
            coordinate_3.is_err(),
            "Longitude must be between -180 and 180 degrees."
        );

        let coordinate_4 = Coordinate::new(0.0, -181.0);
        assert!(
            coordinate_4.is_err(),
            "Latitude must be between -180 and 180 degrees."
        );
    }

    #[test]
    fn test_to_radians() {
        // Test zero coordinates
        let coordinate_1 = Coordinate::new(0.0, 0.0).unwrap();
        assert_eq!(coordinate_1.in_radians(), (0.0, 0.0));

        // Test 90 degree coordinates
        let coordinate_2 = Coordinate::new(90.0, 90.0).unwrap();
        assert_eq!(coordinate_2.in_radians(), (PI / 2.0, PI / 2.0));

        // Test negative coordinates with mixed angles
        let coordinate_3 = Coordinate::new(-45.0, -180.0).unwrap();
        assert_eq!(coordinate_3.in_radians(), (-PI / 4.0, -PI));

        // Test extreme negative latitude with smaller longitude
        let coordinate_4 = Coordinate::new(-90.0, -30.0).unwrap();
        assert_eq!(coordinate_4.in_radians(), (-PI / 2.0, -PI / 6.0));

        // Test coordinates at common angles
        let coordinate_5 = Coordinate::new(30.0, 60.0).unwrap();
        assert_eq!(coordinate_5.in_radians(), (PI / 6.0, PI / 3.0));
    }
}
