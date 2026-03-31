from py_ecc.bn128 import G1, G2, pairing, multiply

# Circuit: z = x * y
# x = 3, y = 2, z = 6
x, y, z = 3, 2, 6

# Witness: a = [1, z, x, y] = [1, 6, 3, 2]
a = [1, z, x, y]

# Encrypt witness
aG1 = [multiply(G1, val) for val in a]
aG2 = [multiply(G2, val) for val in a]

# L = [0,0,1,0] → L·aG1 = 3·G1
L_a_G1 = multiply(G1, x)   # = 3·G1

# R = [0,0,0,1] → R·aG2 = 2·G2
R_a_G2 = multiply(G2, y)   # = 2·G2

# O = [0,1,0,0] → O·aG1 = 6·G1
O_a_G1 = multiply(G1, z)   # = 6·G1

# Verify: e(3G1, 2G2) == e(6G1, G2)
lhs = pairing(R_a_G2, L_a_G1)   # e(3G1, 2G2) → b^6
rhs = pairing(G2, O_a_G1)        # e(6G1, G2)  → b^6

assert lhs == rhs
print("ZK Proof verified! ✅")
print(f"Verifier ne check kiya bina x={x}, y={y}, z={z} jaane!")