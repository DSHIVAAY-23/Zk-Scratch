import galois
import numpy as np
import random

p = 103
GF = galois.GF(p)

xs = GF(np.array([1, 2, 3]))

# Do vectors
v1 = GF(np.array([4, 8, 19]))
v2 = GF(np.array([4, 8, 19]))

# Lagrange se polynomials
p1 = galois.lagrange_poly(xs, v1)
p2 = galois.lagrange_poly(xs, v2)

# Random point
u = random.randint(0, p)

# Sirf ek check!
assert p1(u) == p2(u)
print(f"Vectors equal hain! (checked at u={u}) ✅")

# Agar alag vectors hain:
v3 = GF(np.array([4, 8, 20]))  # last element alag
p3 = galois.lagrange_poly(xs, v3)
print(f"p1(u) = {p1(u)}, p3(u) = {p3(u)}")