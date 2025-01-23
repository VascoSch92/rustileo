use std::fs;
use std::path::Path;

#[allow(unused_imports)]
use pyo3::prelude::*;
use toml::Value;

use earth::{
    bearing, destination, great_circle_distance, haversine_distance, tunnel_distance,
    vincenty_distance, CIRCUMFERENCE as EARTH_CIRCUMFERENCE, FLATTENING as EARTH_FLATTENING,
    RADIUS as EARTH_RADIUS, SEMI_MAJOR_AXIS as EARTH_SEMI_MAJOR_AXIS,
    SEMI_MINOR_AXIS as EARTH_SEMI_MINOR_AXIS,
};

mod earth;

fn get_rustileo_version() -> Option<String> {
    // Path to the Cargo.toml file
    let cargo_toml_path = Path::new("Cargo.toml");

    // Check if the file exists
    if !cargo_toml_path.exists() {
        return None;
    }

    // Read the contents of the file
    let cargo_toml_content = fs::read_to_string(cargo_toml_path).ok()?;

    // Parse the contents as TOML
    let parsed_toml: Value = cargo_toml_content.parse::<Value>().ok()?;

    // Extract the version from the [package] section
    parsed_toml
        .get("package")?
        .get("version")?
        .as_str()
        .map(|s| s.to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustileo(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", get_rustileo_version())?;

    let earth = PyModule::new(py, "earth")?;

    // constants
    earth.add("RADIUS", EARTH_RADIUS)?;
    earth.add("CIRCUMFERENCE", EARTH_CIRCUMFERENCE)?;
    earth.add("SEMI_MAJOR_AXIS", EARTH_SEMI_MAJOR_AXIS)?;
    earth.add("SEMI_MINOR_AXIS", EARTH_SEMI_MINOR_AXIS)?;
    earth.add("FLATTENING", EARTH_FLATTENING)?;

    // methods
    earth.add_function(wrap_pyfunction!(great_circle_distance, py)?)?;
    earth.add_function(wrap_pyfunction!(tunnel_distance, py)?)?;
    earth.add_function(wrap_pyfunction!(haversine_distance, py)?)?;
    earth.add_function(wrap_pyfunction!(vincenty_distance, py)?)?;
    earth.add_function(wrap_pyfunction!(bearing, py)?)?;
    earth.add_function(wrap_pyfunction!(destination, py)?)?;


    m.add_submodule(&earth)?;

    Ok(())
}
