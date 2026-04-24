# M4 — Concurrency: Rayon, Channels, Async

**Timeline**: Weeks 4–5  
**Roles**: ZK dev · Blockchain dev

> ZK provers are embarrassingly parallel in some places (NTT butterfly passes) and deeply sequential in others (Fiat-Shamir transcript). Knowing when to use `rayon` vs `tokio` vs `std::sync` is the difference between a prover that runs in 10 seconds and one that runs in 1 second.

---

## 📂 Files in This Module

```
m4-concurrency/
├── README.md                   ← you are here
├── parallel_ntt.rs             ← Task 1: Rayon parallel NTT butterfly passes
├── pipeline.rs                 ← Task 2: mpsc producer-worker-result pipeline
├── work_stealing.rs            ← Task 3: work-stealing queue study + explanation
├── async_orchestrator.rs       ← Task 4: Tokio prover orchestrator (CPU + mock GPU)
├── data_race_demo.rs           ← Task 5: data race Rust prevents — compiler error + fix
├── flamegraph_notes.md         ← Task 6: flamegraph findings and lock contention fix notes
└── INTERVIEW.md                ← Task 7: parallelism vs concurrency Q&A
```

---

## ✅ Tasks

### Task 1 — `parallel_ntt.rs`
**Parallelize NTT butterfly passes using `rayon::par_chunks_mut` — measure speedup**

- Start with your sequential NTT from M2.
- Identify which loops can be parallelized (butterfly passes where elements do not share indices).
- Replace the inner `for` loop over chunks with `par_chunks_mut` from `rayon`.
- Measure wall-clock time: sequential vs parallel for `n = 2^20` coefficients.
- Record results in a comment: `// BENCHMARK: sequential = Xms, parallel = Yms, speedup = Z×`.
- Verify correctness: `IFFT(FFT(input)) == input` after parallelization.

**What to understand**: Rayon uses a work-stealing thread pool. `par_chunks_mut` is safe because the chunks are disjoint — no two threads touch the same memory. This is the "embarrassingly parallel" case.

---

### Task 2 — `pipeline.rs`
**Build a pipeline: producer sends poly coeffs over mpsc, worker runs NTT, sends result back**

- Spawn a producer thread that generates polynomial coefficient vectors and sends them over `std::sync::mpsc::channel`.
- Spawn a worker thread that receives vectors, runs NTT, and sends results back over a second channel.
- Main thread collects results and prints them.
- Handle the case where the producer finishes — worker loop must exit cleanly.
- Add a slowdown (e.g., `thread::sleep`) to the producer — observe backpressure behaviour.

**What to understand**: `mpsc` = multi-producer, single-consumer. Channels are the safe alternative to shared mutable state. The `Sender` type implements `Clone` for multi-producer; `Receiver` does not implement `Clone` — understand why.

---

### Task 3 — `work_stealing.rs`
**Implement a simple work-stealing queue manually (or study Rayon's source) — explain to interviewer**

Option A — Study Rayon's source:
- Read `rayon/src/deque.rs` or the `crossbeam-deque` crate source.
- Annotate 30–40 lines that implement the steal operation.
- Write a `// HOW IT WORKS:` explanation of the Chase-Lev deque.

Option B — Implement a simplified version:
- Use a `VecDeque` protected by a `Mutex` as the local queue.
- Implement `push`, `pop_local` (LIFO), and `steal` (FIFO from front).
- Show why LIFO local + FIFO steal improves cache locality for the thread that created the task.

**What to understand**: Work stealing is why Rayon scales well — idle threads steal work from busy threads' queues without contention on a global queue. Chase-Lev deque uses atomic operations for the steal path.

---

### Task 4 — `async_orchestrator.rs`
**Write an async prover orchestrator in Tokio: overlap CPU NTT with mock GPU transfer**

- Set up a `tokio` runtime (use `#[tokio::main]`).
- Async function `cpu_ntt(batch: Vec<u64>) -> Vec<u64>` — wraps blocking NTT using `spawn_blocking`.
- Async function `gpu_transfer(data: Vec<u64>) -> ()` — simulates GPU transfer with `tokio::time::sleep`.
- In the orchestrator, run `cpu_ntt` and `gpu_transfer` **concurrently** using `tokio::join!`.
- Measure: time with sequential execution vs overlapped execution.
- Write a comment explaining: *"The CPU and GPU are independent resources so concurrent scheduling is correct here."*

**What to understand**: `spawn_blocking` moves CPU-heavy work off the async executor onto a thread pool. `tokio::join!` is concurrency, not parallelism — they share one async thread but overlap I/O wait time.

---

### Task 5 — `data_race_demo.rs`
**Demonstrate a data race that Rust prevents — show the compiler error, explain why**

- Attempt to share a `Vec<u64>` between two threads via raw pointer — this should refuse to compile.
- Leave the broken code commented out with `// WON'T COMPILE:` and the full compiler error message.
- Fix it using `Arc<Mutex<Vec<u64>>>`.
- Write a second example: attempt to mutate shared state without a lock — show the error and fix.
- Final comment: *"The borrow checker enforces the absence of data races at compile time. This is the Send + Sync contract."*

**What to understand**: A data race = two threads accessing the same memory, at least one writes, with no synchronization. Rust makes this a compile error, not a runtime crash.

---

### Task 6 — `flamegraph_notes.md`
**Profile with `cargo flamegraph` — find a lock contention hotspot and fix it**

This is a documentation task, not a code task.

Steps to perform:
1. Add a deliberate contention: have 8 threads all lock the same `Mutex<Vec>` in a tight loop.
2. Run `cargo flamegraph` — observe the `pthread_mutex_lock` or equivalent stacks taking large time.
3. Fix: shard the mutex (8 separate `Mutex<Vec>` for 8 threads), re-profile.
4. Document in this file: flame graph observations before and after, the fix strategy, and the measured improvement.

Template sections:
- **Setup**: how the contention was introduced
- **Observation**: what the flamegraph showed
- **Fix**: sharding / lock-free alternative / reduced critical section
- **Result**: measured speedup after fix

---

## 🎙️ Interview Drill — `INTERVIEW.md`

**Question**: *"What is the difference between parallelism and concurrency? Give a ZK prover example of each."*

**Map to know cold**: Rayon → parallelism. Tokio → concurrency.

---

## 📚 Reference Reading

- [Rayon crate docs](https://docs.rs/rayon)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [The Rust Book — Chapter 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [crossbeam-deque — work-stealing source](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-deque)
