#!/bin/sh
cargo build --release
cp copydog-ffi/copydog.h copydog-qt/src/copydog.h
cp target/release/libcopydogffi.a copydog-qt/lib/libcopydogffi.a
