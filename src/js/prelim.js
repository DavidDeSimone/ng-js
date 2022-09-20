(function () {
    const global = (1,eval)('this');
    const lispFunctionQueue = [];
    let fileMarker = null;

    global.ngjsSetFileMarker = (s) => { fileMarker = s; };
    global.ngjsCallback = (idx, json) => {
        let result = null;
        try {
            result = JSON.parse(json);
            console.log(result);
        } catch (e) {
            // Do nothing on purpose
        }

        lispFunctionQueue[idx](result); 
        lispFunctionQueue[idx] = null; 
    };

    global.lisp = new Proxy({}, {
        get: function(o, k) {
            return function () {
                return Promise.resolve().then(new Promise((resolve, reject) => {
                    const process = (funcName, args) => {
                        if (funcName == 'eval') {
                            return `eval ${args[0]}`;
                        }

                        let result = "";
                        for (let i = 0; i < funcName.length; ++i) {
                            if (funcName[i] == funcName[i].toUpperCase()) {
                                result += "-" + funcName[i].toLowerCase();
                            } else {
                                result += funcName[i];
                            }
                        }

                        for (let j = 0; j < args.length; ++j) {
                            const arg = JSON.stringify(args[j]);
                            result += ` ${arg}`;
                        }

                        return result;
                    };


                    lispFunctionQueue.push(resolve);
                    const idx = lispFunctionQueue.length - 1;
                    const func = process(k, arguments);

                    global.send_to_lisp(`
                    (let ((result (${func})))
                        (ng-js-eval 
                            (format "ngjsCallback(${idx}, %S)" (json-serialize (prin1-to-string result)))
                            ))`);
                    const cmd = ['touch', fileMarker];
                    Deno.run({ cmd }).status();
                }));
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