import galois
import numpy as np

GF17 = galois.GF(17)  # Finite field mod 17

xs = GF17(np.array([1, 2, 3, 4]))
ys = GF17(np.array([4, 8, 2, 1]))

p = galois.lagrange_poly(xs, ys)

# Same points se guzarti hai — mod 17 mein!
assert p(1) == GF17(4)  # ✅
assert p(2) == GF17(8)  # ✅
assert p(3) == GF17(2)  # ✅
assert p(4) == GF17(1)  # ✅

print("Lagrange polynomial verified over GF(17) ✅")