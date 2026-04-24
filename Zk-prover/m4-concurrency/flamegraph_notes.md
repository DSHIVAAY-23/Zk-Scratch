# M4 Task 6 — Flamegraph Notes

## Setup
*Describe how you introduced lock contention: how many threads, what data structure, what operation.*

```
// Example: 8 threads all locking the same Mutex<Vec<u64>> 1000 times each
// Command run: cargo flamegraph --bin <your_binary>
```

---

## Observation
*What did the flamegraph show? Which stack frames took the most time?*

- Hotspot frame: `pthread_mutex_lock` / `__lll_lock_wait`
- Percentage of samples in lock wait: ____%
- Screenshot or description of the flame graph:

*(attach or describe the flamegraph here)*

---

## Fix Strategy
Describe which of these you applied:

- [ ] **Lock sharding**: split one `Mutex<Vec>` into N separate mutexes, one per thread
- [ ] **Lock-free data structure**: switched to `crossbeam::atomic::*` or `dashmap`
- [ ] **Reduced critical section**: moved allocations outside the lock, only lock for the write

Implementation summary:
```
(describe what you changed)
```

---

## Result

| Metric | Before | After |
|--------|--------|-------|
| Wall-clock time | ___ms | ___ms |
| Speedup | — | ___× |
| % time in lock | ____% | ____% |

---

## What I Learned
*One paragraph explaining your conclusions about lock contention and how to design around it.*
