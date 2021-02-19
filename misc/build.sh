#!/bin/bash
# RUSTFLAGS=-g cargo build --release
cargo build --release
time ./target/release/seqfilter
