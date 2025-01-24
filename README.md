# **Welcome to Rustileo**

A set of algorithms related to geodesy, written in Rust 

- 📦 - installable via pip
- 🐍 - compatible with Python **3.9**, **3.10**, **3.11** and **3.12**

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change, and give a look to the
[contribution guidelines](https://github.com/VascoSch92/rustileo/blob/main/CONTRIBUTING.md).

---

- [Installation](#installation)
- [Quickstart](#quickstart)

---
## Installation

Rustileo can be comfortably installed from PyPI using the command

```text
$ pip install rustileo
```

or directly from the source GitHub code with

```text
$ pip install git+https://github.com/VascoSch92/rustileo@xxx
```

where `xxx` is the name of the branch or the tag you would like to install.


## Quickstart

A small example how you can use rustileo:

```python 
from rustileo import earth

lat1, lon1 = 0.0, 0.0
lat2, lon2 = 90.0, 180.0
print(f"{lat1=}, {lon1=}, {lat2=}, {lon2= }")

# Example 1: Calculate tunnel distance between two points
tunnel_dist = earth.tunnel_distance(lat1, lon1, lat2, lon2)
print(f"Tunnel distance: {tunnel_dist:.2f} km")

# Example 2: Calculate great circle distance
gc_dist = earth.great_circle_distance(lat1, lon1, lat2, lon2)
print(f"Great circle distance: {gc_dist:.2f} km")

# Example 3: Calculate Vincenty distance
vincenty_dist = earth.vincenty_distance(lat1, lon1, lat2, lon2)
print(f"Vincenty distance: {vincenty_dist:.2f} km")

# Example 4: Compute the bearing between two points
bearing = earth.bearing(lat1, lon1, lat2, lon2)
print(f"Bearing: {bearing}")

# Example 5: Validate if two points are antipodal
is_antipodal = earth.are_antipodal(lat1, lon1, lat2, lon2)
print(f"Are the points antipodal? {'Yes' if is_antipodal else 'No'}")

# Example 6: Find the destination point given a starting point, distance, and bearing
starting_lat, starting_lon = 0.0, 0.0
distance = 1000  # 1000 km
bearing = 90.0  # East
print(f"Starting point: {(starting_lat, starting_lon)}, distance: {distance}, bearing {bearing}")
dest_lat, dest_lon = earth.destination(starting_lat, starting_lon, distance, bearing)
print(f"Destination: Latitude {dest_lat:.6f}, Longitude {dest_lon:.6f}")
```