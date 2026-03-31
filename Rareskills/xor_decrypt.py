# XOR Decryption Challenge

# Hex-encoded ciphertext
ciphertext_hex = "73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d"

# Decode the hex string to bytes
ciphertext = bytes.fromhex(ciphertext_hex)

# Try all possible single-byte keys (0-255)
for key in range(256):
    # XOR each byte of the ciphertext with the key
    plaintext = bytes([byte ^ key for byte in ciphertext])

    # Check if the plaintext is readable (ASCII)
    try:
        decoded = plaintext.decode("ascii")
        print(f"Key: {key}, Message: {decoded}")
    except UnicodeDecodeError:
        # Skip non-readable plaintexts
        continue