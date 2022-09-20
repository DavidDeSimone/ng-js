#!/usr/bin/env bash

echo 'Testing...'

emacs -l ./target/release/libng_js.dylib -l ./src/lisp/ng-js-mod.el -l tests/test.el > logfile.txt
cat logfile.txt 