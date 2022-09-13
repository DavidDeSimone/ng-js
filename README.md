# ng-js

This is a WIP emacs module based on the work in https://github.com/emacs-ng/emacs-ng

The concept is to embedd a JavaScript runtime into emacs for scripting.

Building is `cargo build --lib`. It can be loaded as an emacs module with

`emacs -l target/debug/libng_js.so` or whatever the shared library extension is for your platform.

In elisp, you can load it via `(require 'ng-js)` and run js via `(ng-js-eval "var x = 3")`. This is still very primative and a work in progress. 
