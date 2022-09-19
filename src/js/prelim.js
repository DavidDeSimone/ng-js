(function () {
    const global = (1,eval)('this');
    const lispFunctionQueue = [];
    let fileMarker = null;

    global.ngjsSetFileMarker = (s) => { fileMarker = s; };
    global.ngjsCallback = (idx) => { lispFunctionQueue[idx](); lispFunctionQueue[idx] = null; };

    global.lisp = new Proxy({}, {
        get: function(o, k) {
            return function () {
                return new Promise((resolve, reject) => {
                    lispFunctionQueue.push(resolve);
                    const idx = lispFunctionQueue.length - 1;

                    global.send_to_lisp(`(ng-js-eval "callback(${idx})")`);
                    const cmd = ['touch', fileMarker];
                    Deno.run({ cmd }).status();
                });
            };
        }
    });
})();
/*
LispInvoke will return a promise for a change in lisp state
It will return a proxy

await lisp.interactivep; // true

lisp.stringp("x"); // true

lisp.eval(`(interactivep)`);

lisp. -> translates to an async evaluation
*/