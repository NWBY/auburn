# Auburn

Auburn is an experimental Python web framework with FastAPI-inspired ergonomics and a Rust-native request pipeline.

The first milestone is deliberately small:

```python
from auburn import App

app = App()

@app.get("/health")
def health():
    return {"ok": True}
```

```bash
uv run maturin develop
uv run auburn run examples.health:app
```

See `SPEC.md` for the full design direction and collaboration philosophy.

Implementation work is broken into small pairable slices in `docs/slices/`.

## Development

Auburn uses the Astral Python toolkit:

```bash
uv sync --dev
uv run ruff format .
uv run ruff check .
```

Use `uv run` for Python-side commands so local tooling and dependencies stay consistent.
