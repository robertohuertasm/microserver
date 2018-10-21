#!/bin/bash
cargo clippy --all -- -D clippy::pedantic -A clippy::non-ascii-literal
cargo fmt
