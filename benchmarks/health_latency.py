from __future__ import annotations

import statistics
import time
import urllib.request


def run_benchmark(url: str, requests: int = 1000, warmup: int = 50) -> None:
    for _ in range(warmup):
        _request(url)

    durations: list[float] = []
    started = time.perf_counter()

    for _ in range(requests):
        request_started = time.perf_counter()
        _request(url)
        durations.append((time.perf_counter() - request_started) * 1000)

    elapsed = time.perf_counter() - started
    durations.sort()

    print(f"url={url}")
    print(f"requests={requests}")
    print(f"elapsed_s={elapsed:.4f}")
    print(f"requests_per_s={requests / elapsed:.2f}")
    print(f"latency_min_ms={durations[0]:.4f}")
    print(f"latency_mean_ms={statistics.fmean(durations):.4f}")
    print(f"latency_p50_ms={_percentile(durations, 50):.4f}")
    print(f"latency_p95_ms={_percentile(durations, 95):.4f}")
    print(f"latency_p99_ms={_percentile(durations, 99):.4f}")
    print(f"latency_max_ms={durations[-1]:.4f}")


def _request(url: str) -> bytes:
    request = urllib.request.Request(url, method="GET")
    with urllib.request.urlopen(request, timeout=5) as response:
        return response.read()


def _percentile(values: list[float], percentile: int) -> float:
    if not values:
        return 0.0
    index = round((percentile / 100) * (len(values) - 1))
    return values[index]


if __name__ == "__main__":
    run_benchmark("http://127.0.0.1:8000/health")
