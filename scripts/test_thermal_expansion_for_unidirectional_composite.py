from struct import pack, unpack
from sys import byteorder
import requests

endianness = 0 if byteorder == 'little' else 1
number_of_model = 1
fiber_content = 0.2
e_for_fiber = 100.0
nu_for_fiber = 0.3
alpha_for_fiber = 1e-6
e_for_matrix = 5.0
nu_for_matrix = 0.2
alpha_for_matrix = 20e-5

bytes = pack(
    "BBxxxxxxddddddd",
    endianness,
    number_of_model,
    fiber_content,
    e_for_fiber,
    nu_for_fiber,
    alpha_for_fiber,
    e_for_matrix,
    nu_for_matrix,
    alpha_for_matrix
)
print("ArgsMessage:", list(bytes))

url = "http://localhost:8080/compute/thermal_expansion_for_unidirectional_composite"
resp = requests.post(url, data=bytes)
print("Response:", list(resp.content))
(alpha1,alpha2,alpha3) = unpack("ddd", resp.content)
print()
print("alpha1:", alpha1)
print("alpha2:", alpha2)
print("alpha3:", alpha3)
