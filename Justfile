fix:
  cargo fmt
  cargo clippy --allow-dirty --fix -- -D warnings
  cargo clippy -p client --target wasm32-unknown-unknown --allow-dirty --fix -- -D warnings

lint:
  cargo fmt --check
  cargo clippy -- -D warnings
  cargo clippy -p client --target wasm32-unknown-unknown -- -D warnings

test:
  cargo test --all

TARGET := shell("rustc -vV | sed -n 's|host: ||p'")
build:
  cargo build --all --target {{ TARGET }}
  cd client && just build build_web

ci: lint test build
