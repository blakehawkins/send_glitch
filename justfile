default:
    just --list

check: update clippy cargo-check fmt test outdated

update:
    cargo update

clippy:
    cargo clippy

cargo-check:
    cargo check

fmt:
    cargo fmt

test:
    cargo test

outdated:
    cargo outdated -R
