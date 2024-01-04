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
alpha_for_honeycomb = 20e-5

bytes = pack(
    "BBxxxxxxddddd",
    endianness,
    number_of_model,
    l_cell_side_size,
    h_cell_side_size,
    wall_thickness,
    angle,
    alpha_for_honeycomb
)
print("ArgsMessage:", list(bytes))

url = "http://localhost:8080/compute/thermal_expansion_for_honeycomb"
resp = requests.post(url, data=bytes)
print("Response:", list(resp.content))
(alpha1,alpha2,alpha3) = unpack("ddd", resp.content)
print()
print("alpha1:", alpha1)
print("alpha2:", alpha2)
print("alpha3:", alpha3)
