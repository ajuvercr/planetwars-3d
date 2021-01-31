#!/usr/bin/env bash

SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )" # this source dir

cd "$SRCDIR" # "$SRCDIR" ensures that this script can be run from anywhere.

wasm-pack build --target web --out-name wasm --out-dir ./static -- --features "web"
