from struct import pack, unpack
from sys import byteorder
import requests
import math

endianness = 0 if byteorder == 'little' else 1

number_of_model = 1
l_cell_side_size = 9.24
h_cell_side_size = 8.4619
wall_thickness = 0.4
angle = math.pi/6
e_for_honeycomb = 7.07
nu_for_honeycomb = 0.2

bytes = pack(
    "BBxxxxxxdddddd",
    endianness,
    number_of_model,
    l_cell_side_size,
    h_cell_side_size,
    wall_thickness,
    angle,
    e_for_honeycomb,
    nu_for_honeycomb
)
print("ArgsMessage:", list(bytes))

url = "http://localhost:8080/compute/elastic_modules_for_honeycomb"
resp = requests.post(url, data=bytes)
print("Response:", list(resp.content))
( e1, e2, e3, nu12, nu13, nu23, g12, g13, g23) = unpack("ddddddddd", resp.content)
print()
print("E1:", e1)
print("E2:", e2)
print("E3:", e3)
print("Nu12:", nu12)
print("Nu13:", nu13)
print("Nu23:", nu23)
print("G12:", g12)
print("G13:", g13)
print("G23:", g23)
