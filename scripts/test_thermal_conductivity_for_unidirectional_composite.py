from struct import pack, unpack
from sys import byteorder
import requests

endianness = 0 if byteorder == 'little' else 1
number_of_model = 2
fiber_content = 0.2
k_for_fiber = 100.0
k_for_matrix = 1.0

bytes = pack(
    "BBxxxxxxddd",
    endianness,
    number_of_model,
    fiber_content,
    k_for_fiber,
    k_for_matrix
)
print("ArgsMessage:", list(bytes))

url = "http://localhost:8080/compute/thermal_conductivity_for_unidirectional_composite"
resp = requests.post(url, data=bytes)
print("Response:", list(resp.content))
(k1,k2,k3) = unpack("ddd", resp.content)
print()
print("k1:", k1)
print("k2:", k2)
print("k3:", k3)
