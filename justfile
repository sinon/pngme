set dotenv-load
build:
    CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly build -Zcodegen-backend
format:
	@cargo fmt --version
	cargo fmt
lint:
	@cargo clippy --version
	cargo clippy -- -D warnings
	cargo doc
test:
    cargo test
bench-all:
    cargo bench -q > benchmarks.txt

python-dev:
	maturin develop --uv -m crates/pngme-python/Cargo.toml

python-dev-release:
	maturin develop --uv -m crates/pngme-python/Cargo.toml --release

python-test: python-dev
    uv pip install -r crates/pngme-python/tests/requirements.txt
    pytest crates/pngme-python/tests
