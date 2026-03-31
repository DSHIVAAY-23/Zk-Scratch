# XOR Flag Decryption Challenge

# Hex-encoded ciphertext
ciphertext_hex = "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104"

# Decode the hex string to bytes
ciphertext = bytes.fromhex(ciphertext_hex)

# Known flag format
flag_format = "crypto{"

# Try all possible single-byte keys (0-255)
for key in range(256):
    # XOR each byte of the ciphertext with the key
    plaintext = bytes([byte ^ key for byte in ciphertext])

    # Check if the plaintext starts with the known flag format
    try:
        decoded = plaintext.decode("ascii")
        if decoded.startswith(flag_format):
            print(f"Key: {key}, Flag: {decoded}")
            break
    except UnicodeDecodeError:
        # Skip non-readable plaintexts
        continue

# Filter and display plaintexts matching the flag format
for key in range(256):
    plaintext = bytes([byte ^ key for byte in ciphertext])
    try:
        decoded = plaintext.decode("ascii")
        if "crypto{" in decoded:
            print(f"Key: {key}, Flag: {decoded}")
    except UnicodeDecodeError:
        continue

# Debug: Print all potential plaintexts for analysis
for key in range(256):
    plaintext = bytes([byte ^ key for byte in ciphertext])
    try:
        decoded = plaintext.decode("ascii")
        print(f"Key: {key}, Plaintext: {decoded}")
    except UnicodeDecodeError:
        continue