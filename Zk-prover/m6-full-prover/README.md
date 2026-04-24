# M6 — Full Prover Integration & Project Deliverable

**Timeline**: Weeks 6–8  
**Roles**: ZK dev · Blockchain dev · Technical writer

> This is your capstone. You wire every primitive from M1–M5 into a single coherent crate, benchmark it, ship a GPU integration, and produce the portfolio artifacts that get you hired. Do not shortcut this module.

---

## 📂 Files in This Module

```
m6-full-prover/
├── README.md                    ← you are here
├── lib.rs                       ← Task 1: crate root, re-exports, public API
├── integration.rs               ← Task 1: wire field → curve → poly → NTT → MSM → commitment
├── benchmarks/
│   ├── bench_ntt.rs             ← Task 2: criterion NTT benchmark (naive vs optimised)
│   ├── bench_msm.rs             ← Task 2: criterion MSM benchmark
│   └── bench_montgomery.rs      ← Task 2: criterion Montgomery vs naive modmul
├── ffi/
│   ├── icicle_wrapper.rs        ← Task 3: safe Rust wrapper around ICICLE GPU MSM FFI
│   └── icicle_stub.rs           ← stub for build without GPU — compile with cfg feature
├── tests/
│   └── end_to_end.rs            ← Task 4: commit to poly → open at point → verify
├── blog_post.md                 ← Task 5: technical blog post (portfolio artifact)
├── walkthrough_script.md        ← Task 6: 5-minute video walkthrough script
└── INTERVIEW.md                 ← interview walkthrough Q&A
```

---

## ✅ Tasks

### Task 1 — `lib.rs` + `integration.rs`
**Wire M1–M5 into a single crate: field → curve → poly → NTT → MSM → commitment**

- Create a Rust crate `zk-prover` at the root.
- Re-export all public types: `FieldElement`, `CurvePoint`, `PolynomialVec`, `DeviceBuffer`, `Transcript`.
- Write `fn prove(poly: &PolynomialVec) -> (CurvePoint<Fp>, u64)`:
  1. Commit to the polynomial via Pedersen commitment (M5).
  2. Run NTT on the coefficients (M5).
  3. Perform MSM over the evaluations (M5).
  4. Squeeze a Fiat-Shamir challenge from the transcript (M5).
  5. Return the commitment and the challenge.
- This function does not need to be a real ZK proof — it just exercises the full pipeline end-to-end.
- Ensure `cargo build` succeeds with no warnings.

**What to understand**: Crate architecture matters. Everything that another module imports is part of your API surface — think about what is `pub` vs `pub(crate)` vs private.

---

### Task 2 — `benchmarks/`
**Add criterion benchmarks: naive vs. optimised for each primitive — produce a chart**

- Add `criterion` to `Cargo.toml` dev-dependencies.
- Create three benchmark files:
  - `bench_ntt.rs`: `sequential_ntt` vs `parallel_ntt` (from M4) for input sizes `2^16`, `2^20`, `2^24`.
  - `bench_msm.rs`: `naive_msm` (scalar-mul-and-add) vs `pippenger_msm` for 1k, 10k, 100k points.
  - `bench_montgomery.rs`: `native_modmul` vs `montgomery_mul` for 1M iterations.
- Run: `cargo criterion` — criterion generates HTML reports in `target/criterion/`.
- Copy the chart screenshots into a `bench_results/` folder.
- Write a `// ANALYSIS:` comment in each file: what speedup did you observe and why.

**What to understand**: Micro-benchmarks lie if you measure the wrong thing. Criterion uses statistical analysis (Welch t-test) to reduce noise. Always measure with `--release`.

---

### Task 3 — `ffi/icicle_wrapper.rs`
**Integrate ICICLE for GPU MSM via FFI — write the safe Rust wrapper with lifetime bounds**

- Add the `icicle` crate as an optional dependency (feature flag `gpu`).
- Write `unsafe extern "C"` declarations for ICICLE's `msm_cuda` function signature (see ICICLE docs/headers).
- Write a safe public function `fn gpu_msm<'a>(scalars: &'a [u64], points: &'a [CurvePoint<Fp>]) -> CurvePoint<Fp>`.
- Add lifetime bounds ensuring the input slices outlive the GPU computation.
- If the `gpu` feature is not enabled, fall back to `pippenger_msm` from M5 (compile-time dispatch via `cfg`).
- Write a `// SAFETY:` block explaining the FFI invariants you uphold.

**What to understand**: FFI boundaries are always `unsafe` — the Rust compiler cannot verify the C function's behaviour. Your safe wrapper is the proof that the usage is correct. Lifetime annotations at FFI boundaries prevent use-after-free on the Rust side.

---

### Task 4 — `tests/end_to_end.rs`
**Write an end-to-end test: commit to a polynomial, open at a point, verify**

The test must exercise this sequence:
1. Create a `PolynomialVec` with known coefficients.
2. Commit to it: `Com = commit(poly)`.
3. Choose an evaluation point `z`.
4. Evaluate the polynomial at `z`: `v = poly.eval(z)`.
5. Produce an opening proof (simplest version: just `v` + `r` values).
6. Verify: assert that `Com` is consistent with `v` at `z`.

- This is a simplified polynomial commitment scheme — not a production PCS, but it exercises the full pipeline.
- The test must pass with `cargo test`.
- Add a `// ZK ANALOGY:` comment explaining how this relates to KZG or IPA in a real prover.

---

### Task 5 — `blog_post.md`
**Write a technical blog post explaining your prover — this is your portfolio artifact**

Requirements:
- Target audience: hiring manager at a ZK company who can read Rust.
- Length: 800–1200 words.
- Structure:
  - **Hook**: Why you built this and what it teaches.
  - **Architecture section**: Field → Curve → Poly → NTT → MSM → Commitment pipeline with a diagram (ASCII is fine).
  - **Deepdive section**: Pick ONE design decision (e.g., why Montgomery form, why bump allocator, why Rayon over std threads) and explain it in depth.
  - **Benchmark results**: Include the criterion numbers.
  - **What's next**: How you would extend this (GPU, Plonk constraints, lookup tables).
- Tone: confident, technical, precise. No buzzwords. No vague claims.

---

### Task 6 — `walkthrough_script.md`
**Write a 5-minute code walkthrough script — practice explaining ownership decisions out loud**

This file is your script for a screen-recorded walkthrough video.

Structure (approximate timing):
- **0:00–0:30** — Introduction: "This is a ZK prover I built in Rust over 8 weeks. I'm going to walk through the ownership decisions that make it correct."
- **0:30–2:00** — `DeviceBuffer` and PhantomData: show the struct, explain why PhantomData is there, show the Drop impl.
- **2:00–3:00** — Generic NTT: show the function signature, explain each trait bound, show why Copy is needed.
- **3:00–4:00** — Parallel NTT: show the `par_chunks_mut` call, explain why it is safe (disjoint slices).
- **4:00–5:00** — End-to-end test: walk through the commit-open-verify flow, conclude with what you'd add next.

Practice notes:
- Record yourself reading this script once per day until you do not need the script.
- Time yourself — 5 minutes is a hard constraint.
- The goal is to be able to do this **without notes** in an interview.

---

## 🎙️ Interview Drill — `INTERVIEW.md`

**Question**: *"Walk me through a component you built — explain a hard memory ownership decision."*

**Answer lives in your prover code**: point to `DeviceBuffer`, the FFI wrapper, or the bump allocator.

---

## 🏁 Definition of Done

Before this module is complete, verify all of the following:

- [ ] `cargo build` — clean, no warnings
- [ ] `cargo test` — all tests pass including `end_to_end.rs`
- [ ] `cargo criterion` — benchmark HTML reports generated
- [ ] `cargo clippy -- -D warnings` — zero lints
- [ ] `blog_post.md` — 800–1200 words, published or ready to publish
- [ ] `walkthrough_script.md` — 5-minute video recorded at least once

---

## 📚 Reference Reading

- [Criterion.rs user guide](https://bheisler.github.io/criterion.rs/book/)
- [ICICLE GPU library](https://github.com/ingonyama-zk/icicle)
- [KZG polynomial commitments — Dankrad Feist](https://dankradfeist.de/ethereum/2020/06/16/kate-polynomial-commitments.html)
- [arkworks-rs — production ZK prover in Rust](https://github.com/arkworks-rs)
