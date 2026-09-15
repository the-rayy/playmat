fix:
  cargo fmt
  cargo clippy --allow-dirty --fix -- -D warnings
  cargo clippy -p engine --target wasm32-unknown-unknown --allow-dirty --fix -- -D warnings

lint:
  cargo fmt --check
  cargo clippy -- -D warnings
  cargo clippy -p engine --target wasm32-unknown-unknown -- -D warnings

test:
  cargo test --all

TARGET := shell("rustc -vV | sed -n 's|host: ||p'")
build:
  cargo build --all --target {{ TARGET }}

ci: lint test build
