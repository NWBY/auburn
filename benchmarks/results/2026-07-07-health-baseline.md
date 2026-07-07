# Health Baseline: 2026-07-07

## Scenario

Auburn `GET /health`, single-client localhost benchmark.

This is the first baseline for the runnable health server. It is useful for tracking local changes over time, but it is not a throughput claim.

## Commands

Server:

```bash
uv run maturin develop
uv run auburn run examples.health:app --port 8765
```

Benchmark:

```bash
uv run auburn bench --url http://127.0.0.1:8765/health --requests 1000 --json
```

## Result

```json
{
  "url": "http://127.0.0.1:8765/health",
  "requests": 1000,
  "elapsed_s": 0.2501722499728203,
  "requests_per_s": 3997.2458980108468,
  "latency_min_ms": 0.2095840172842145,
  "latency_mean_ms": 0.24992560595273972,
  "latency_p50_ms": 0.24199998006224632,
  "latency_p95_ms": 0.2945830347016454,
  "latency_p99_ms": 0.3487919457256794,
  "latency_max_ms": 0.6391670322045684
}
```

## Environment

```text
OS: macOS 15.0.1 24A348
Architecture: arm64
Python: 3.13.5
uv: 0.11.27 (19fc8b03b 2026-07-06 aarch64-apple-darwin)
rustc: 1.96.1 (31fca3adb 2026-06-26)
cargo: 1.96.1 (356927216 2026-06-26)
```

## Notes

* Benchmark harness uses Python stdlib `urllib.request`.
* Request execution is sequential with one client.
* Server was run from the local editable development install.
* No CPU, memory, or concurrency measurements are included yet.
* No competitor framework comparison is included in this baseline.
