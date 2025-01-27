#[allow(unused_imports)]
use pyo3::prelude::*;

mod earth;

use crate::earth::constants::{
    CIRCUMFERENCE as EARTH_CIRCUMFERENCE, FLATTENING as EARTH_FLATTENING, RADIUS as EARTH_RADIUS,
    SEMI_MAJOR_AXIS as EARTH_SEMI_MAJOR_AXIS, SEMI_MINOR_AXIS as EARTH_SEMI_MINOR_AXIS,
};
#[allow(unused_imports)]
use crate::earth::interface::{
    are_antipodal, bearing, destination, great_circle_distance, haversine_distance,
    tunnel_distance, vincenty_distance,
};

/// A Python module implemented in Rust.
#[pymodule]
fn rustileo(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.0")?;

    let earth = PyModule::new(py, "earth")?;

    // constants
    earth.add("RADIUS", EARTH_RADIUS)?;
    earth.add("CIRCUMFERENCE", EARTH_CIRCUMFERENCE)?;
    earth.add("SEMI_MAJOR_AXIS", EARTH_SEMI_MAJOR_AXIS)?;
    earth.add("SEMI_MINOR_AXIS", EARTH_SEMI_MINOR_AXIS)?;
    earth.add("FLATTENING", EARTH_FLATTENING)?;

    // methods
    earth.add_function(wrap_pyfunction!(are_antipodal, py)?)?;
    earth.add_function(wrap_pyfunction!(great_circle_distance, py)?)?;
    earth.add_function(wrap_pyfunction!(tunnel_distance, py)?)?;
    earth.add_function(wrap_pyfunction!(haversine_distance, py)?)?;
    earth.add_function(wrap_pyfunction!(vincenty_distance, py)?)?;
    earth.add_function(wrap_pyfunction!(bearing, py)?)?;
    earth.add_function(wrap_pyfunction!(destination, py)?)?;

    m.add_submodule(&earth)?;

    Ok(())
}
