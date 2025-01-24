import math

import pytest
from rustileo import earth


@pytest.mark.parametrize(
    argnames="name, value",
    argvalues=[
        ("RADIUS", 6371.0),
        ("CIRCUMFERENCE", 40075.0),
        ("SEMI_MAJOR_AXIS", 6378.137),
        ("SEMI_MINOR_AXIS", 6356.752314245),
        ("FLATTENING", 1.0 / 298.257223563),
    ],
)
def test_constants(name, value):
    assert math.isclose(getattr(earth, name, None), value, rel_tol=0.05)


@pytest.mark.parametrize(
    argnames="lon1, lat1, lon2, lat2, rel_tol, expected_value",
    argvalues=[
        # Test distance between same point should be 0
        (0, 0, 0, 0, 0, 0),
        (-1, 1, -1, 1, 0, 0),
        (40.7128, -74.0060, 40.7128, -74.0060, 0, 0),
        # Antipodal points
        (90, 0, -90, 0, 0, 2 * earth.RADIUS),
        (0, 0, 0, 180, 0, 2 * earth.RADIUS),
        # Well-known distances
        (40.7128, -74.0060, 51.5074, -0.1278, 0.05, 5300),
        (51.5074, -0.1278, 40.7128, -74.0060, 0.05, 5300),
        (35.6895, 139.6917, 34.0522, -118.2437, 0.05, 8100),
        (34.0522, -118.2437, 35.6895, 139.6917, 0.05, 8100),
    ],
)
def test_tunnel_distance(lat1, lon1, lat2, lon2, rel_tol, expected_value):
    computed_value = earth.tunnel_distance(lon1, lat1, lon2, lat2)
    msg = f"Expected: {expected_value}. Got {computed_value}"
    assert math.isclose(computed_value, expected_value, rel_tol=rel_tol), msg


@pytest.mark.parametrize(
    argnames="lat1, lon1, lat2, lon2, error, error_msg",
    argvalues=[
        (-91, 0, 0, 0, ValueError, "Latitude 1 must be between -90 and 90 degrees."),
        (0, 181, 0, 0, ValueError, "Longitude 1 must be between -180 and 180 degrees."),
        (0, 0, 91, 0, ValueError, "Latitude 2 must be between -90 and 90 degrees."),
        (
            0,
            0,
            0,
            -181,
            ValueError,
            "Longitude 2 must be between -180 and 180 degrees.",
        ),
    ],
)
def test_tunnel_distance_exceptions(lat1, lon1, lat2, lon2, error, error_msg):
    with pytest.raises(error, match=error_msg):
        _ = earth.tunnel_distance(lat1, lon1, lat2, lon2)


@pytest.mark.parametrize(
    argnames="lat1, lon1, lat2, lon2, rel_tol, expected_value",
    argvalues=[
        # Test distance between same point should be 0
        (0, 0, 0, 0, 0, 0),
        (-1, 1, -1, 1, 0, 0),
        (40.7128, -74.0060, 40.7128, -74.0060, 0, 0),
        # Antipodal points
        (90, 0, -90, 0, 0.05, 0.5 * earth.CIRCUMFERENCE),
        (0, 0, 0, 180, 0.05, 0.5 * earth.CIRCUMFERENCE),
        # Well-known distances
        (40.7128, -74.0060, 51.5074, -0.1278, 0.05, 5300),
        (51.5074, -0.1278, 40.7128, -74.0060, 0.05, 5300),
        (35.6895, 139.6917, 34.0522, -118.2437, 0.05, 8815),
        (34.0522, -118.2437, 35.6895, 139.6917, 0.05, 8815),
    ],
)
def test_haversine_and_great_circle_distance(
    lat1, lon1, lat2, lon2, rel_tol, expected_value
):
    computed_value_haversine = earth.haversine_distance(lat1, lon1, lat2, lon2)
    computed_value_great_circle = earth.great_circle_distance(lat1, lon1, lat2, lon2)
    msg = f"Expected: {expected_value}. Got {computed_value_haversine}"
    assert math.isclose(computed_value_haversine, expected_value, rel_tol=rel_tol), msg
    assert computed_value_haversine == computed_value_great_circle


@pytest.mark.parametrize(
    argnames="lat1, lon1, lat2, lon2, error, error_msg",
    argvalues=[
        (-91, 0, 0, 0, ValueError, "Latitude 1 must be between -90 and 90 degrees."),
        (0, 181, 0, 0, ValueError, "Longitude 1 must be between -180 and 180 degrees."),
        (0, 0, 91, 0, ValueError, "Latitude 2 must be between -90 and 90 degrees."),
        (
            0,
            0,
            0,
            -181,
            ValueError,
            "Longitude 2 must be between -180 and 180 degrees.",
        ),
    ],
)
def test_haversine_distance_exceptions(lat1, lon1, lat2, lon2, error, error_msg):
    with pytest.raises(error, match=error_msg):
        _ = earth.haversine_distance(lat1, lon1, lat2, lon2)
    with pytest.raises(error, match=error_msg):
        _ = earth.great_circle_distance(lat1, lon1, lat2, lon2)


@pytest.mark.parametrize(
    argnames="lat1, lon1, lat2, lon2, rel_tol, expected_value",
    argvalues=[
        # Test distance between same point should be 0
        (0, 0, 0, 0, 0, 0),
        (-1, 1, -1, 1, 0, 0),
        (40.7128, -74.0060, 40.7128, -74.0060, 0, 0),
        # Antipodal points
        (90, 0, -90, 0, 0.05, 0.5 * earth.CIRCUMFERENCE),
        (0, 0, 0, 180, 0.05, 0.5 * earth.CIRCUMFERENCE),
        # Quasi Antipodal points
        (90, 0, -89, 0, 0.05, 0.5 * earth.CIRCUMFERENCE),
        (89, 0, -90, 0, 0.05, 0.5 * earth.CIRCUMFERENCE),
        (0, 1, 0, 180, 0.05, 0.5 * earth.CIRCUMFERENCE),
        # Well-known distances
        (40.7128, -74.0060, 51.5074, -0.1278, 0.05, 5585),
        (51.5074, -0.1278, 40.7128, -74.0060, 0.05, 5585),
        (35.6895, 139.6917, 34.0522, -118.2437, 0.05, 8815),
        (34.0522, -118.2437, 35.6895, 139.6917, 0.05, 8815),
    ],
)
def test_vincenty_distance(lat1, lon1, lat2, lon2, rel_tol, expected_value):
    computed_value = earth.vincenty_distance(lat1, lon1, lat2, lon2)
    msg = f"Expected: {expected_value}. Got {computed_value}"
    assert math.isclose(computed_value, expected_value, rel_tol=rel_tol), msg


@pytest.mark.parametrize(
    argnames="lat1, lon1, lat2, lon2, error, error_msg",
    argvalues=[
        (-91, 0, 0, 0, ValueError, "Latitude 1 must be between -90 and 90 degrees."),
        (0, 181, 0, 0, ValueError, "Longitude 1 must be between -180 and 180 degrees."),
        (0, 0, 91, 0, ValueError, "Latitude 2 must be between -90 and 90 degrees."),
        (
            0,
            0,
            0,
            -181,
            ValueError,
            "Longitude 2 must be between -180 and 180 degrees.",
        ),
    ],
)
def test_vincenty_distance_exceptions(lat1, lon1, lat2, lon2, error, error_msg):
    with pytest.raises(error, match=error_msg):
        _ = earth.vincenty_distance(lat1, lon1, lat2, lon2)


@pytest.mark.parametrize(
    argnames="lat1, lon1, lat2, lon2, rel_tol, expected_value",
    argvalues=[
        # Bearing between same point should be 0
        (0, 0, 0, 0, 0, 0),
        (-1, 1, -1, 1, 0, 0),
        (40.7128, -74.0060, 40.7128, -74.0060, 0, 0),
        # Antipodal points
        (90, 0, -90, 0, 0, 180),
        (0, 0, 0, 180, 0, 90),
        # Well-known bearings
        (40.7128, -74.0060, 51.5074, -0.1278, 0.05, 51.12),
        (35.6895, 139.6917, 34.0522, -118.2437, 0.05, 55.2),
    ],
)
def test_bearing(lat1, lon1, lat2, lon2, rel_tol, expected_value):
    computed_value = earth.bearing(lat1, lon1, lat2, lon2)
    msg = f"Expected: {expected_value}. Got {computed_value}"
    assert math.isclose(computed_value, expected_value, rel_tol=rel_tol), msg


@pytest.mark.parametrize(
    argnames="lat1, lon1, lat2, lon2, error, error_msg",
    argvalues=[
        (-91, 0, 0, 0, ValueError, "Latitude 1 must be between -90 and 90 degrees."),
        (0, 181, 0, 0, ValueError, "Longitude 1 must be between -180 and 180 degrees."),
        (0, 0, 91, 0, ValueError, "Latitude 2 must be between -90 and 90 degrees."),
        (
            0,
            0,
            0,
            -181,
            ValueError,
            "Longitude 2 must be between -180 and 180 degrees.",
        ),
    ],
)
def test_bearing_exceptions(lat1, lon1, lat2, lon2, error, error_msg):
    with pytest.raises(error, match=error_msg):
        _ = earth.bearing(lat1, lon1, lat2, lon2)


@pytest.mark.parametrize(
    argnames="lat, lon, distance, bearing, rel_tol, expected_lat, expected_lon",
    argvalues=[
        # distance 0 should gives back same coordinates
        (0, 0, 0, 0, 0, 0, 0),
        (-1, 1, 0, 1, 0, -1, 1),
        (40.7128, -74.0060, 0, 0, 0.05, 40.7128, -74.0060),
        # destination north
        (0.0, 0.0, 111.195, 0.0, 0.05, 1, 0),
        # destination east
        (0.0, 0.0, 111.195, 90.0, 1, 0, 1),
        # destination pole
        (89.0, 0.0, 111.195, 0.0, 0.05, 90, -180),
    ],
)
def test_destination(lat, lon, distance, bearing, rel_tol, expected_lat, expected_lon):
    computed_value = earth.destination(lat, lon, distance, bearing)
    msg = f"Expected: ({expected_lat}, {expected_lon}). Got {computed_value}"
    if not math.isclose(
        computed_value[0], expected_lat, rel_tol=rel_tol
    ) or not math.isclose(computed_value[1], expected_lon, rel_tol=rel_tol):
        raise AssertionError(msg)


@pytest.mark.parametrize(
    argnames="lat, lon, distance, bearing, error, error_msg",
    argvalues=[
        (-91, 0, 0, 0, ValueError, "Latitude must be between -90 and 90 degrees."),
        (0, 181, 0, 0, ValueError, "Longitude must be between -180 and 180 degrees."),
        (0, 0, -1, 0, ValueError, "Distance cannot be negative."),
    ],
)
def test_destination_exceptions(lat, lon, distance, bearing, error, error_msg):
    with pytest.raises(error, match=error_msg):
        _ = earth.destination(lat, lon, distance, bearing)
