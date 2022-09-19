#!/usr/bin/env bash

echo 'Testing...'

emacs -l ./target/debug/libng_js.dylib -l ./src/lisp/ng-js-mod.el -l tests/test.el