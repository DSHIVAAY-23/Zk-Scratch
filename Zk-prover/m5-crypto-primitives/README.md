# M5 — Cryptographic Primitives in Rust

**Timeline**: Weeks 5–6  
**Roles**: ZK dev · Blockchain dev · Technical writer

> This module is where the math meets the code. Each primitive you implement here is the real thing — used in production ZK provers. Understand both the algorithm and why each implementation decision exists.

---

## 📂 Files in This Module

```
m5-crypto-primitives/
├── README.md                     ← you are here
├── montgomery_mul.rs             ← Task 1: Montgomery multiplication + test suite
├── cooley_tukey_ntt.rs           ← Task 2: iterative Cooley-Tukey NTT + correctness proof
├── pippenger_msm.rs              ← Task 3: naive Pippenger MSM (c=4, 16 buckets)
├── pedersen_commitment.rs        ← Task 4: Pedersen commitment Com(v, r) = vG + rH
├── fiat_shamir.rs                ← Task 5: tiny Fiat-Shamir transcript using SHA-256
├── primitive_explainer.md        ← Task 6: 600-word technical explanation (writer deliverable)
└── INTERVIEW.md                  ← Task 7: Montgomery form interview Q&A
```

---

## ✅ Tasks

### Task 1 — `montgomery_mul.rs`
**Implement Montgomery multiplication — test against naive modmul on 10,000 random inputs**

- Implement `fn montgomery_mul(a: u64, b: u64, modulus: u64, r: u64, r_inv: u64, m_prime: u64) -> u64`.
- Precompute the Montgomery parameters (R = 2^64, R², M') for your chosen prime.
- Test: generate 10,000 random (a, b) pairs, assert `montgomery_mul(a, b, ...) == (a * b) % modulus` for each.
- Add a `// ALGORITHM:` comment block explaining REDC (Montgomery Reduction) step by step.
- Benchmark: compare `montgomery_mul` vs direct `(a as u128 * b as u128) % modulus as u128` using `std::time::Instant`.

**What to understand**: Montgomery form avoids expensive division by computing in a transformed domain where reduction is cheap (bit shifts and additions). The speedup comes from the divisor being a power-of-two.

---

### Task 2 — `cooley_tukey_ntt.rs`
**Write iterative Cooley-Tukey NTT — prove correctness with `IFFT(FFT(poly)) == poly`**

- Implement `fn ntt(a: &mut [u64], omega: u64, modulus: u64)` — iterative, not recursive.
- Implement `fn intt(a: &mut [u64], omega: u64, modulus: u64)` — inverse NTT (omega → omega_inv, scale by n_inv).
- Correctness proof in tests: for 100 random polynomials of degree 2^k, assert `intt(ntt(poly)) == poly`.
- Add a `// BUTTERFLY:` comment explaining one pass of the butterfly network.
- The polynomial length must be a power of 2 — add an error or panic if not.

**What to understand**: NTT is DFT over a finite field. The primitive n-th root of unity `omega` must satisfy `omega^n ≡ 1 (mod p)`. The iterative version is more cache-friendly than recursive — understand the bit-reversal permutation.

---

### Task 3 — `pippenger_msm.rs`
**Implement naive Pippenger MSM (c=4, 16 buckets) — connect to the ICICLE diagram**

- Implement `fn msm(scalars: &[u64], points: &[CurvePoint<Fp>]) -> CurvePoint<Fp>`.
- Use the Pippenger bucket method with window size `c = 4` (16 buckets per window).
- Algorithm:
  1. For each scalar, extract the 4-bit window and add the corresponding point to the bucket.
  2. Accumulate buckets: running sum from highest bucket to lowest (triangle sum trick).
  3. Combine windows with point doubling.
- Compare result against naive `sum(scalar_i * point_i)` for correctness.
- Write a `// ICICLE CONNECTION:` comment: explain how this maps to ICICLE's GPU bucket scheme.

**What to understand**: Naive MSM is O(n) doublings-and-additions. Pippenger reduces the work to O(n / log n) by batching additions into buckets. This is the core of why GPU provers are fast.

---

### Task 4 — `pedersen_commitment.rs`
**Write a Pedersen commitment: `Com(v, r) = v·G + r·H` over your `CurvePoint` type**

- Use your `CurvePoint<Fp>` from M2.
- Define `G` and `H` as public generator points (hard-code two valid points on your curve).
- Implement `fn commit(v: u64, r: u64) -> CurvePoint<Fp>` — returns `v*G + r*H`.
- Implement the hiding property test: `commit(v, r1) != commit(v, r2)` for `r1 != r2`.
- Implement the binding property test: `commit(v1, r) != commit(v2, r)` for `v1 != v2`.
- Write a `// SECURITY PROPERTIES:` comment block explaining hiding and binding.

**What to understand**: Hiding = commitment reveals nothing about `v`. Binding = cannot find `(v', r')` such that `commit(v', r') == commit(v, r)`. Both properties rely on the hardness of discrete log.

---

### Task 5 — `fiat_shamir.rs`
**Build a tiny Fiat-Shamir transcript using SHA-256 — squeeze challenges from it**

- Implement `struct Transcript { state: Vec<u8> }`.
- `fn absorb(&mut self, label: &[u8], data: &[u8])` — append labeled data to transcript.
- `fn squeeze_challenge(&mut self, label: &[u8]) -> u64` — hash the current state, return 8 bytes as `u64`.
- Use `sha2::Sha256` (add `sha2` crate to `Cargo.toml`).
- Write a test: absorb a commitment, squeeze a challenge, verify the challenge changes when the commitment changes.
- Use it in an interaction: prover commits → squeezes verifier challenge → uses challenge in response.

**What to understand**: Fiat-Shamir heuristic replaces an interactive verifier with a hash function. The transcript must be bound to all prior messages — otherwise an adversary can choose commitments after seeing the challenge.

---

### Task 6 — `primitive_explainer.md`
**Write a 600-word technical explanation of one primitive — the technical writer deliverable**

Choose ONE of: Montgomery multiplication, NTT, MSM, Pedersen commitment, or Fiat-Shamir.

Requirements:
- Target audience: a developer who knows Rust and basic math, but has not studied ZK.
- Word count: 580–620 words.
- Must include: intuition for why the primitive exists, pseudocode or a concrete example, and one sentence on where it appears in production ZK provers.
- No bullet points — this is prose. This is your writer portfolio piece.
- Format it as if it is going to be published on a technical blog (include a title, intro paragraph, and conclusion).

---

## 🎙️ Interview Drill — `INTERVIEW.md`

**Question**: *"Why does Montgomery form speed up modular multiplication?"*

**Explain without code**: the algorithm, not the words. Know REDC cold.

---

## 📚 Reference Reading

- [Montgomery Reduction — Wikipedia](https://en.wikipedia.org/wiki/Montgomery_modular_multiplication)
- [NTT — CP-algorithms.com](https://cp-algorithms.com/algebra/fft.html)
- [Pippenger's algorithm — ZKProof standards](https://zkproof.org/)
- [Pedersen Commitments — Boneh & Shoup](https://toc.cryptobook.us/)
- [Fiat-Shamir heuristic — IACR ePrint](https://eprint.iacr.org/)
