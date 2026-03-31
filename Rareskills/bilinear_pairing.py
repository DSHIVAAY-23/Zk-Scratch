# Import necessary functions and constants from the py_ecc library
from py_ecc.bn128 import G1, G2, pairing, add, multiply, eq, curve_order
from py_ecc.bn128 import FQ, FQ2
from py_ecc.bn128 import is_on_curve
from py_ecc.bn128 import b

# Print the generator point G1 (on the elliptic curve group G1)
print("G1:", G1)
# Output: (1, 2)

# Print the generator point G2 (on the elliptic curve group G2)
print("G2:", G2)
# Output: 
# ((10857046999023057135944570762232829481370756359578518086990519993285655852781, 
#   11559732032986387107991004021392285783925812861821192530917403151452391805634), 
#  (8495653923123431417604973247489272438418190587263600148770280649306958101930, 
#   4082367875863433681332203403145435568316851327593401208105741076214120093531))

# G1 and G2 are the generator points for their respective groups.
# Both G1 and G2 have the same order (number of points on the curve).

# Verify the cyclic property of the groups using the curve order
x = 10  # Randomly chosen scalar
assert eq(multiply(G2, x + curve_order), multiply(G2, x))
assert eq(multiply(G1, x + curve_order), multiply(G1, x))

# Scalar multiplication and addition in G1 and G2
# Scalar multiplication is equivalent to repeated addition
print("G1 + G1 == G1 * 2:", eq(add(G1, G1), multiply(G1, 2)))  # True
print("G2 + G2 == G2 * 2:", eq(add(G2, G2), multiply(G2, 2)))  # True

# Note: You can only add elements from the same group.
# The following will raise a TypeError:
# add(G1, G2)

# Demonstrating operator overloading in the library
# Arithmetic operators are overridden for convenience
print("G1 + G1 + G1 == G1 * 3:", G1 + G1 + G1 == G1 * 3)  # True

# The above is equivalent to:
print("eq(add(add(G1, G1), G1), multiply(G1, 3)):", eq(add(add(G1, G1), G1), multiply(G1, 3)))  # True

# Bilinear pairing example
# Pairing is a bilinear map e: G1 x G2 -> GT
# It satisfies the bilinearity property: e(a*P, b*Q) == e(P, Q)^(a*b)
P = G1  # Use the generator point G1 directly
Q = G2  # Point on G2

# Scalars a and b
a = 5
b = 7

# Ensure points are explicitly cast to the correct field types
P = (FQ(P[0]), FQ(P[1]))  # Cast G1 point to FQ

# Debug: Print the point P and check its validity manually
print("Debug: Point P:", P)
print("Debug: Is P on the curve?", is_on_curve(P, b=False))

# Manually verify the curve equation for P using field elements
from py_ecc.bn128 import FQ

x, y = FQ(P[0]), FQ(P[1])  # Convert coordinates to field elements
on_curve = (y**2 == x**3 + FQ(b))  # Check the curve equation
print("Manual verification of P on curve:", on_curve)

# Verify that points are on the curve before pairing
assert on_curve, "P is not on the curve"
assert is_on_curve(Q, b=True), "Q is not on the curve"

# Compute pairings
pairing1 = pairing(multiply(P, a), multiply(Q, b))  # e(a*P, b*Q)
pairing2 = pairing(P, Q) ** (a * b)  # e(P, Q)^(a*b)

# Verify bilinearity
print("Pairing is bilinear:", pairing1 == pairing2)  # True