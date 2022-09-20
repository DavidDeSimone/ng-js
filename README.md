# emacs ng-js
## Run JavaScript and TypeScript in Vanilla emacs

This project is an emacs dynamic module that will allow you to run JavaScript and TypeScript in any emacs module that supports dynamic modules.

This module is still highly under development and currently experimental. PRs are welcome. The goal will be to eventually list on MELPA and be installable via `cargo install`, however those are still WIP.

## Build

You will need stable [rust](https://www.rust-lang.org/) to install this project. 

In the top directory, run:

`cargo build --release`

This will output our dynamic module in `./target/release/libng_js[EXT]` where EXT is your system's dynamic module extension. For most, this will be a *.so 

You can run the following command when launching emacs to load the module:

`emacs -l target/release/libng_js.so -l src/lisp/ng-js-mod.el`

Within the editor, run

`(require 'ng-js-mod)`

## Usage

Within elisp, you can run:

```lisp
(ng-js-eval "let x = 2 + 3;")
```

This will synchronously evaluate your JavaScript or TypeScript and return the result. 

You can call lisp functions from JavaScript using the special "lisp" object. You can do this via

```js
lisp.bufferMenu();
```

In JavaScript, camelCase names will automatically be changed to their kebab case invocations. If you want to execute a kebab case function without that logic, the following works:

```js
lisp['buffer-menu']();
```

NOTE: The JavaScript/TypeScript runtime is running on a separate thread from the Lisp runtime. It is running in parallel. This means that invocations of lisp functions are done ASYNCHRONOUSLY. The lisp function returns a [Promise](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) that you can chain execution off of. An example would be:

```js
lisp.bufferMenu()
.then(async () => {
    await lisp.getBufferCreate("newBuffer");
    await lisp.switchToBuffer("newBuffer");
    await lisp.setBuffer("newBuffer");
});
```

This can be combined with JS async/await. 

NOTE: Do NOT use top level await in version 0.1 of ng-js. This will cause emacs to lock. await in general works, however top level await is still a work in progress. 

Under the hood, the JavaScript/TypeScript engine is still [deno](deno.land). You can run deno modules. A simple example is using sqlite in emacs:

```js
// Credit to https://deno.land/x/sqlite@v3.5.0 for docs
import { DB } from "https://deno.land/x/sqlite/mod.ts";

const db = new DB("test.db");
db.execute(`
  CREATE TABLE IF NOT EXISTS people (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT
  )
`);

const names = ["Peter Parker", "Clark Kent", "Bruce Wayne"];

for (const name of names) {
  db.query("INSERT INTO people (name) VALUES (?)", [name]);
}


lisp.getOrCreateBuffer("sqldb")
```

## Technology and Performance

Under the hood, this is implemented by running a custom instance of deno in process on another thread. Messages from lisp -> js and js -> lisp are passed via async channels. The primary means of of communicating from js to lisp hooks into lisp filewatcher API. JS updated a sentinel file to let lisp know that there are queue'd lisp commands to run. Once that sentinel fires, lisp will drain the queue of commands. 

Since JavaScript runs on another thread, it should generally not impact emacs execution. There is also a degree of isolation in the event there is a logic error within JavaScript - it shouldn't prevent lisp from being executed further.

This allows for some interesting possibilities for performance. The following is a basic test - it is NOT a formal benchmark, nor is it ANY statement on the performance of this module, or elisp vs. js. This is simply some code you can run on your machine, and a very simple observation of speed possibility of some calculations:

```lisp
(defun fibonacci(n)
  (if (<= n 1)
      n
    (+ (fibonacci (- n 1)) (fibonacci (- n 2)))))

(let ((time (current-time)))
  (fibonacci 35)
  (message "%.06f" (float-time (time-since time))))

  (let ((time (current-time)))
  (ng-js-eval "const fib = (n) => { if (n <= 1) { return n; } return fib(n - 1) + fib(n - 2); }; fib(35);")
  (message "%.06f" (float-time (time-since time))))
```

On my machine, this produces:

```bash
Testing...
3.487964
0.086469
```


## Motivation

This project is based off the work contained in the [emacs-ng](https://github.com/emacs-ng/emacs-ng) fork of emacs. That project included [deno](https://github.com/denoland/deno) into the internals of the emacs editor. 

This projects includes deno as well, in a slightly different way (see [Technology and Performance](#technology-and-performance))

