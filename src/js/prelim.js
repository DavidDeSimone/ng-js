console.log("This is the prelim test...");
(function () {
    const global = (1,eval)('this');
    const lispFunctionQueue = [];
    let fileMarker = null;

    global.ngjsSetFileMarker = (s) => { fileMarker = s; };

    global.lisp = new Proxy({}, {
        get: function(o, k) {
            return function () {
                // const args = JSON.stringify(arguments); 
                return new Promise((resolve, reject) => {
                    console.log("Sending.... " + fileMarker);
                    global.send_to_lisp();
                    const cmd = ['touch', fileMarker];
                    Deno.run({ cmd }).status()
                    .then(() => resolve());
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