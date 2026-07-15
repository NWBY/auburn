fmt:
	cargo fmt --manifest-path auburn_core/Cargo.toml

check:
	cargo check --manifest-path auburn_core/Cargo.toml

dev:
	uv run maturin develop

build: fmt check dev

default_app := 'health'

start app=default_app: 
	uv run auburn run examples.{{app}}:app --port 8765

