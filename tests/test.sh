#!/usr/bin/env bash

echo 'Testing...'

emacs --batch -l ./target/debug/libng_js.dylib -l tests/test.el