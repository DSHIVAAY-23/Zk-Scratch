# M6 — Technical Blog Post

**Write 800–1200 words here. This is your portfolio artifact.**

---

## Title: [Your Title Here]

### Hook

*Why did you build this? What does it teach?*

---

### Architecture

```
FieldElement (M1/M2)
    └── CurvePoint<F: Field> (M2)
          └── PolynomialVec (M1)
                └── NTT (M2/M4/M5)
                      └── MSM / Pippenger (M5)
                            └── Pedersen Commitment (M5)
                                  └── Fiat-Shamir Transcript (M5)
```

*Explain the pipeline in prose, one paragraph per layer.*

---

### Deep Dive: [Your chosen design decision]

*Pick ONE: Montgomery form / bump allocator / Rayon parallelism / FFI lifetime bounds*

*Explain it in depth — algorithm, code excerpt, why this decision was necessary.*

---

### Benchmark Results

| Primitive | Naive | Optimised | Speedup |
|-----------|-------|-----------|---------|
| modmul    |       |           |         |
| NTT (2²⁰) |       |           |         |
| MSM 10k   |       |           |         |

---

### What's Next

*GPU integration, Plonk constraints, lookup tables — one paragraph.*

---

> Word count: ____  
> Draft: 1 / 2 / Final
