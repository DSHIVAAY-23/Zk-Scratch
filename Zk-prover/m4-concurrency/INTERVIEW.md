# M4 Interview Practice Sheet

## Question
**"What is the difference between parallelism and concurrency? Give a ZK prover example of each."**

---

## Your 90-Second Answer Template

```
[Suggested structure:]

1. Define concurrency: multiple tasks making progress, not necessarily simultaneously (structured around waiting)
2. Define parallelism: multiple tasks executing simultaneously on multiple cores
3. ZK prover concurrency example → Tokio: async prover orchestrator overlapping CPU proof generation with mock GPU memory transfer — they share a thread but overlap during I/O wait
4. ZK prover parallelism example → Rayon: NTT butterfly passes where each butterfly is independent and can execute on separate cores simultaneously
5. Rust lens: Tokio = concurrency (async/await, single thread, structured around IO waits). Rayon = parallelism (thread pool, real CPU cores, compute-bound work)
```

---

## Marker Concepts

- [ ] Concurrency = structure (multiple things in progress), can happen on one core
- [ ] Parallelism = execution (multiple things running simultaneously), requires multiple cores
- [ ] Rayon → parallelism, Tokio → concurrency — know this cold
- [ ] NTT butterfly passes = embarrassingly parallel (independent subproblems)
- [ ] Proof orchestration = concurrent (CPU + GPU overlap during waits)
- [ ] A single-threaded async runtime demonstrates concurrency without parallelism

---

## Score Yourself

- [ ] Distinguished concurrency from parallelism clearly
- [ ] Named Rayon for parallelism, Tokio for concurrency
- [ ] Used a ZK prover example for each
- [ ] Under 90 seconds
