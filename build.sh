#!/bin/sh
cargo build --release
cp copydog-ffi/copydog.h copydog-qt/src/copydog.h
cp target/release/libcopydog.a copydog-qt/lib/libcopydog.a
