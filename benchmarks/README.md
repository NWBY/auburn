# Benchmarks

The first benchmark target is `GET /health`, returning `{"ok": true}`.

Start the Auburn example server:

```bash
uv run maturin develop
uv run auburn run examples.health:app
```

In another terminal, run:

```bash
uv run auburn bench --url http://127.0.0.1:8000/health --requests 1000
```

The initial harness records single-client latency and throughput. It is intentionally simple so we can compare framework overhead early, then add concurrency, process resource usage, and competitor apps once the first route is stable.

Recorded results live in `benchmarks/results/`.

First baseline:

* `benchmarks/results/2026-07-07-health-baseline.md`
