extern crate deno;
extern crate emacs;
extern crate v8 as rusty_v8;
#[macro_use]
extern crate lazy_static;

use deno::deno_core::error::AnyError;
use deno::deno_core::FsModuleLoader;
use deno::deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno::deno_runtime::deno_web::BlobStore;
use deno::deno_runtime::permissions::Permissions;
use deno::deno_runtime::worker::MainWorker;
use deno::deno_runtime::worker::WorkerOptions;
use deno::deno_runtime::BootstrapOptions;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use std::thread;

use std::sync::mpsc;

use emacs::{defun, Env, Result, Value};

emacs::plugin_is_GPL_compatible!();

lazy_static! {
    static ref LISP_TO_JS: std::sync::Mutex<Option<std::sync::mpsc::Sender<String>>> = {
        std::sync::Mutex::new(None)
    };

    static ref JS_TO_LISP: std::sync::Mutex<Option<std::sync::mpsc::Receiver<String>>> = {
        std::sync::Mutex::new(None)
    };
}

#[emacs::module(name = "ng-js", defun_prefix = "ng-js", mod_in_name = false)]
fn ng_js(env: &Env) -> Result<()> {
        let (tx, rx): (std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>) = std::sync::mpsc::channel();
        let (jtx, jrx): (std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>) = std::sync::mpsc::channel();
        
        {
            let mut chan = LISP_TO_JS.lock().unwrap();
            *chan = Some(tx.clone());
        }

        {
            let mut chan = JS_TO_LISP.lock().unwrap();
            *chan = Some(jrx);
        }

        std::thread::spawn(move || {
            let result: Result<_> = deno::deno_runtime::tokio_util::run_local(async move {
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
                let mut repl_session = deno::tools::repl::ReplSession::initialize(worker.into_main_worker()).await?;
                
                loop {
                    if let Ok(msg) = rx.recv() {
                        let result = repl_session.evaluate_line_and_get_output(&msg).await?;
                        jtx.send(result.to_string());
                    } else {
                        println!("ERROR");
                    }
                }
                Ok(())   
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


