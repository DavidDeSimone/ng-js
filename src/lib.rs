extern crate deno;
extern crate emacs;
extern crate v8 as rusty_v8;
#[macro_use]
extern crate lazy_static;

use deno::deno_runtime::permissions::Permissions;

use std::time::Duration;

use emacs::{defun, Env, Result};

const PRELIM_JS: &str = include_str!("js/prelim.js");

emacs::plugin_is_GPL_compatible!();

lazy_static! {
    static ref LISP_TO_JS: std::sync::Mutex<Option<std::sync::mpsc::Sender<String>>> = {
        std::sync::Mutex::new(None)
    };

    static ref JS_TO_LISP: std::sync::Mutex<Option<std::sync::mpsc::Receiver<String>>> = {
        std::sync::Mutex::new(None)
    };

    static ref NATIVE_TO_JS: std::sync::Mutex<Option<std::sync::mpsc::Sender<String>>> = {
        std::sync::Mutex::new(None)
    };

    static ref JS_TO_NATIVE: std::sync::Mutex<Option<std::sync::mpsc::Receiver<String>>> = {
        std::sync::Mutex::new(None)
    };
}

macro_rules! bind_global_fn {
    ($scope:expr, $global: expr, $fnc:ident) => {{
        let name = v8::String::new($scope, stringify!($fnc)).unwrap();
        let func = v8::Function::new($scope, $fnc).unwrap();
        $global.set($scope, name.into(), func.into());
    }};
}

pub fn send_to_lisp(
    _scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    // @TODO make this read the string based argument
    // and send to lisp
    let chan = NATIVE_TO_JS.lock().unwrap();
    if let Some(tx) = &*chan {
        tx.send("(print \"ello\")".to_string());
    }
}

#[emacs::module(name = "ng-js", defun_prefix = "ng-js", mod_in_name = false)]
fn ng_js(_: &Env) -> Result<()> {
        let (tx, rx): (std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>) = std::sync::mpsc::channel();
        let (jtx, jrx): (std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>) = std::sync::mpsc::channel();
        let (ntx, nrx): (std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>) = std::sync::mpsc::channel();

        {
            let mut chan = LISP_TO_JS.lock().unwrap();
            *chan = Some(tx.clone());
        }

        {
            let mut chan = JS_TO_LISP.lock().unwrap();
            *chan = Some(jrx);
        }

        {
            let mut chan = NATIVE_TO_JS.lock().unwrap();
            *chan = Some(ntx.clone());
        }

        {
            let mut chan = JS_TO_NATIVE.lock().unwrap();
            *chan = Some(nrx);
        }

        std::thread::spawn(move || {
            let result: Result<()> = deno::deno_runtime::tokio_util::run_local(async move {
                let flags = deno::args::flags_from_vec(vec!["deno".to_owned()])?;
                let main_module = deno::deno_core::resolve_url_or_path("./$deno$repl.ts").unwrap();
                let ps = deno::proc_state::ProcState::build(flags).await?; 
                let mut worker = deno::worker::create_main_worker(
                 &ps,
                    main_module.clone(),
                    Permissions::from_options(&ps.options.permissions_options())?,
                    vec![],
                    Default::default(),
                )
                .await?;
                worker.setup_repl().await?;

                let mut main_worker = worker.into_main_worker();
                {
                    let runtime = &mut main_worker.js_runtime;
                    {
                        let context = runtime.global_context();
                        let scope = &mut v8::HandleScope::with_context(runtime.v8_isolate(), context);
                        let context = scope.get_current_context();
                        let global = context.global(scope);

                        bind_global_fn!(scope, global, send_to_lisp);
                    }
                }

                let mut repl_session = deno::tools::repl::ReplSession::initialize(main_worker).await?;
                repl_session.evaluate_line_and_get_output(PRELIM_JS).await?;

                loop {
                    if let Ok(msg) = rx.recv() {
                        let result = repl_session.evaluate_line_and_get_output(&msg).await?;
                        jtx.send(result.to_string());
                    } else {
                        println!("ERROR");
                    }
                }
            });
        });
    Ok(())
}

#[defun]
pub fn eval(_: &Env, payload: String) -> Result<String> {
    let chan = LISP_TO_JS.lock().unwrap();
    let rechan = JS_TO_LISP.lock().unwrap();
    if let Some(tx) = &*chan {
        tx.send(payload);
    }

    if let Some(jrx) = &*rechan {
        if let Ok(msg) = jrx.recv() {
            return Ok(msg.to_string());
        }
    }

    Ok("Failure".to_string())
}

#[defun]
pub fn drain(_: &Env) -> Result<String> {
    let d = Duration::from_millis(10);
    let rechan = JS_TO_NATIVE.lock().unwrap();
    if let Some(jrx) = &*rechan {
        while let Ok(msg) = jrx.recv_timeout(d) {
            return Ok(msg.to_string())
        }
    }

    Ok("".to_string())
}


