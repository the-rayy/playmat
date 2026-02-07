fix:
  cargo fmt
  cargo clippy --allow-dirty --fix -- -D warnings

lint:
  cargo fmt --check
  cargo clippy -- -D warnings

test:
  cargo test --all

TARGET := shell("rustc -vV | sed -n 's|host: ||p'")
build:
  cargo build --all --target {{ TARGET }}
  cd client && just build_web

ci: lint test build
