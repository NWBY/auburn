from __future__ import annotations

import argparse
import importlib
import sys
from pathlib import Path
from typing import Any

from .app import App


def main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(prog="auburn")
    subparsers = parser.add_subparsers(dest="command", required=True)

    run_parser = subparsers.add_parser("run", help="Run an Auburn app")
    run_parser.add_argument("target", help="Import target, for example examples.health:app")
    run_parser.add_argument("--host", default="127.0.0.1")
    run_parser.add_argument("--port", type=int, default=8000)

    dev_parser = subparsers.add_parser("dev", help="Run an Auburn app in development mode")
    dev_parser.add_argument("target", help="Import target, for example examples.health:app")
    dev_parser.add_argument("--host", default="127.0.0.1")
    dev_parser.add_argument("--port", type=int, default=8000)

    bench_parser = subparsers.add_parser("bench", help="Run the first health-route benchmark")
    bench_parser.add_argument("--url", default="http://127.0.0.1:8000/health")
    bench_parser.add_argument("--requests", type=int, default=1000)
    bench_parser.add_argument("--warmup", type=int, default=50)

    args = parser.parse_args(argv)

    if args.command in {"run", "dev"}:
        app = load_app(args.target)
        app.run(host=args.host, port=args.port)
        return

    if args.command == "bench":
        from benchmarks.health_latency import run_benchmark

        run_benchmark(url=args.url, requests=args.requests, warmup=args.warmup)


def load_app(target: str) -> App:
    module_name, separator, attr_name = target.partition(":")
    if not separator:
        raise SystemExit("App target must be in module:attribute form.")

    cwd = str(Path.cwd())
    if cwd not in sys.path:
        sys.path.insert(0, cwd)

    module = importlib.import_module(module_name)
    app: Any = getattr(module, attr_name)
    if not isinstance(app, App):
        raise SystemExit(f"{target} did not resolve to an auburn.App instance.")
    return app
