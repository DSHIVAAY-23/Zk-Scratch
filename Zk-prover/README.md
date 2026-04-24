# 🔐 ZK Prover — Rust Learning Curriculum

A structured 6–8 week Rust curriculum designed for ZK dev, Blockchain dev, and Technical Writer roles.  
Each module builds on the last, culminating in a full ZK prover crate you can showcase as a portfolio project.

---

## 📁 Module Structure

| Module | Topic | Timeline | Roles |
|--------|-------|----------|-------|
| [M1 — Rust Ownership & Memory](./m1-ownership-memory/README.md) | Stack, heap, lifetimes, borrow checker | Weeks 1–2 | ZK, Blockchain, Writer |
| [M2 — Traits, Generics & Zero-Cost Abstractions](./m2-traits-generics/README.md) | Field trait, NTT generics, monomorphization | Weeks 2–3 | ZK, Blockchain, Writer |
| [M3 — Unsafe Rust & Raw Memory](./m3-unsafe-rust/README.md) | DeviceBuffer, custom allocator, ptr ops | Weeks 3–4 | ZK, Blockchain |
| [M4 — Concurrency: Rayon, Channels, Async](./m4-concurrency/README.md) | Parallel NTT, pipelines, Tokio orchestrator | Weeks 4–5 | ZK, Blockchain |
| [M5 — Cryptographic Primitives](./m5-crypto-primitives/README.md) | Montgomery mul, NTT, MSM, Pedersen, Fiat-Shamir | Weeks 5–6 | ZK, Blockchain, Writer |
| [M6 — Full Prover Integration](./m6-full-prover/README.md) | Wire everything, benchmarks, FFI, blog post | Weeks 6–8 | ZK, Blockchain, Writer |

---

## 🗺️ Learning Path

```
M1 (memory model)
  └── M2 (abstractions)
        └── M3 (unsafe)
              └── M4 (concurrency)
                    └── M5 (crypto math)
                          └── M6 (full prover)
```

---

## 🎯 Final Deliverables

- **Rust crate**: `zk-prover` — a single crate wiring field → curve → poly → NTT → MSM → commitment
- **Benchmark report**: criterion charts comparing naive vs. optimised implementations
- **Technical blog post**: portfolio-ready explanation of the prover
- **Code walkthrough video**: 5-minute explanation of ownership decisions

---

## 🛠️ Prerequisites

- Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install cargo tools: `cargo install cargo-flamegraph cargo-criterion`
- Read: [The Rust Book](https://doc.rust-lang.org/book/), chapters 1–10 before M1
