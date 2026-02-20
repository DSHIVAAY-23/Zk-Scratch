# 🛡️ ZK & Applied Cryptography: From Scratch

Welcome to my personal Zero-Knowledge and Applied Cryptography research repository. 

As the lines between Web3 verifiable compute and AI (zkML) continue to blur, relying on high-level abstractions is a security risk. This repository documents my rigorous, ongoing journey to build ZK primitives, proving systems, and client-side privacy infrastructure completely from scratch. 

My philosophy is simple: **Concrete understanding comes from writing the math in code.**

## 🧠 Core Objectives
1. **Math-to-Code Translation:** Implementing foundational cryptography (Finite Fields, Elliptic Curves, Polynomial Commitments) in pure Rust, without relying on black-box libraries.
2. **Circuit Security & Exploit Research:** Intentionally breaking and analyzing under-constrained circuits to develop a deeply offensive security mindset.
3. **zkVM & zkTLS Infrastructure:** Bridging the gap between isolated ZK math and real-world infrastructure.

## 📂 Repository Architecture
* **`01-math-primitives/`** - Foundational discrete math in Rust.
* **`02-core-snarks/`** - Under-the-hood proving systems (R1CS, Groth16).
* **`03-circom-patterns/`** - Applied circuit engineering and Noir implementations.
* **`04-zk-exploits/`** - Offensive ZK security and constraint vulnerabilities.

## 👨💻 About the Researcher
**Dshivaay**
* **Background:** 3+ Years scaling Web3 architecture (Rust, Solidity, TEEs). 
* **Current Focus:** Architecting privacy-preserving verifiable compute layers and transitioning to full-time solo-founder at Shivaay Labs.

---
*"Don't trust. Cryptographically verify."*
