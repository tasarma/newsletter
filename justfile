test:
    cargo test

clippy:
    cargo clippy --all-features --all-targets

fmt:
    cargo fmt -- --check

coverage:
    cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
    cargo llvm-cov report --html --output-dir coverage

ci:
    just fmt
    just clippy
    just test

