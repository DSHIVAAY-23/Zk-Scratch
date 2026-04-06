import numpy as np
import galois
from functools import reduce

# ── Step 1: R1CS Matrices ──────────────────────────
L = np.array([
    [0, 0, 1,  0, 0, 0, 0],
    [0, 0, 0,  0, 1, 0, 0],
    [0, 0, 0, -5, 0, 0, 0],
    [0, 0, 0,  0, 0, 0, 1],
])
R = np.array([
    [0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0],
])
O = np.array([
    [0, 0, 0, 0,  1,  0, 0],
    [0, 0, 0, 0,  0,  1, 0],
    [0, 0, 0, 0,  0,  0, 1],
    [0, 1, 0, 0,  0, -1, 0],
])

# ── Step 2: Finite Field Convert ───────────────────
GF = galois.GF( )
L_galois = GF((L + 79) % 79)
R_galois = GF((R + 79) % 79)
O_galois = GF((O + 79) % 79)

# ── Step 3: Witness ────────────────────────────────
x   = GF(4)
y   = GF(77)       # -2 mod 79
v1  = x * x
v2  = v1 * v1
v3  = GF(74) * y * y   # -5 mod 79
out = v3*v1 + v2
witness = GF(np.array([1, out, x, y, v1, v2, v3]))

assert all(np.equal(
    np.matmul(L_galois, witness) * np.matmul(R_galois, witness),
    np.matmul(O_galois, witness)
))
print("R1CS ✅")

# ── Step 4: Lagrange Interpolation ────────────────
def interpolate_column(col):
    xs = GF(np.array([1,2,3,4]))
    return galois.lagrange_poly(xs, col)

U_polys = np.apply_along_axis(interpolate_column, 0, L_galois)
V_polys = np.apply_along_axis(interpolate_column, 0, R_galois)
W_polys = np.apply_along_axis(interpolate_column, 0, O_galois)

# ── Step 5: u(x), v(x), w(x) ─────────────────────
def inner_product(polys, witness):
    return reduce(lambda a,b: a+b, map(lambda x,y: x*y, polys, witness))

u = inner_product(U_polys, witness)
v = inner_product(V_polys, witness)
w = inner_product(W_polys, witness)

# ── Step 6: h(x) aur Final Check ──────────────────
t = (galois.Poly([1,78], field=GF) * galois.Poly([1,77], field=GF) *
     galois.Poly([1,76], field=GF) * galois.Poly([1,75], field=GF))

h = (u * v - w) // t

assert u * v == w + h * t
print("QAP verified! ✅")
print(f"u(x) = {u}")
print(f"v(x) = {v}")
print(f"h(x) = {h}")
print(f"t(x) = {t}")