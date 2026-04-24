# M6 — 5-Minute Walkthrough Script

**Record yourself reading this script. Target: 5:00 flat. Iterate until you don't need notes.**

---

## 0:00–0:30 — Introduction

> "Hi, I'm [Name]. Over the past 8 weeks I built a ZK prover crate in Rust from scratch — field arithmetic, curve operations, NTT, multi-scalar multiplication, and Pedersen commitments. I'm going to walk through the ownership decisions that make it correct and fast."

*[Open your editor. Have `lib.rs` on screen.]*

---

## 0:30–2:00 — DeviceBuffer and PhantomData

> "Let's start with `DeviceBuffer`. This is a GPU memory abstraction. It holds a raw `*mut T` pointing to CUDA memory. The first thing you'll notice is `PhantomData<T>` — this is the ownership signal."

*[Open `device_buffer.rs`]*

> "Without PhantomData, Rust doesn't know this type has any relationship to T. It won't participate in drop check — the compiler won't verify that T is valid for the lifetime of the buffer. And variance would be wrong — you could coerce a DeviceBuffer<&'short str> into a DeviceBuffer<&'static str>."

> "The Drop implementation calls our `cuda_free` stub. This is the contract: we own this memory, and we release it in Drop."

---

## 2:00–3:00 — Generic NTT

*[Open `generic_ntt.rs`]*

> "Here's the NTT. The signature is `fn ntt<F: Field + Copy>(coeffs: &mut [F], omega: F)`. Let me walk through each bound: `Field` because we need add, mul, and inv. `Copy` because we're doing a lot of in-place butterfly swaps and we don't want to clone — we want memcpy semantics."

> "There is no concrete type inside this function. If I change `Fp<u64>` to `Fp<u128>`, the same NTT runs — the compiler generates a new concrete copy via monomorphization."

---

## 3:00–4:00 — Parallel NTT

*[Open `parallel_ntt.rs`]*

> "The parallel version replaces the inner for loop with `par_chunks_mut`. This is safe because: each butterfly pass operates on disjoint index pairs. Rayon's `par_chunks_mut` splits the slice into non-overlapping chunks. Two threads never touch the same element. The borrow checker guarantees this — `&mut [T]` is exclusive."

> "On a machine with 8 cores and 2^20 coefficients, this gave a [X]× speedup. The rest of the work — cache misses, memory bandwidth — is the bottleneck after that."

---

## 4:00–5:00 — End-to-End Test

*[Open `tests/end_to_end.rs`]*

> "The end-to-end test wires everything together: create a polynomial, commit, evaluate at a point, verify the opening is consistent with the commitment. This is not a production PCS — it's a correctness smoke test for the pipeline."

> "This crate is my foundation for building a full Plonk or STARK prover. The next step is to add constraint system evaluation, lookup tables, and real KZG commitments over BN254. Thanks for watching."

---

## Rehearsal Log

| Date | Time | Notes |
|------|------|-------|
|      |      | First read |
|      |      |       |
|      |      | No script needed |
