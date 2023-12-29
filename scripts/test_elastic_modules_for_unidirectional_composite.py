from struct import pack, unpack
from sys import byteorder
import requests

endianness = 0 if byteorder == 'little' else 1
number_of_model = 2
fiber_content = 0.2
e_for_fiber = 100.0
nu_for_fiber = 0.3
e_for_matrix = 5.0
nu_for_matrix = 0.2

bytes = pack(
    "BBxxxxxxddddd",
    endianness,
    number_of_model,
    fiber_content,
    e_for_fiber,
    nu_for_fiber,
    e_for_matrix,
    nu_for_matrix
)
print("ArgsMessage:", list(bytes))

url = "http://localhost:8080/compute/elastic_modules_for_unidirectional_composite"
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
