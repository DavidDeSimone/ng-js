extern crate emacs;
extern crate deno_runtime;
extern crate tokio;
extern crate v8 as rusty_v8;

use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use std::thread;

use emacs::{defun, Env, Result, Value};

emacs::plugin_is_GPL_compatible!();

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}  

#[emacs::module(name = "ng-js", defun_prefix = "ng-js", mod_in_name = false)]
fn ng_js(env: &Env) -> Result<()> {
    // env.message("Hello, Emacs!")?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .worker_threads(2)
        .max_blocking_threads(32)
        .build()?;

        std::thread::spawn(move || {
            let _: Result<()> = runtime.block_on(async {
                let module_loader = Rc::new(FsModuleLoader);
                let create_web_worker_cb = Arc::new(|_| {
                  todo!("Web workers are not supported in the example");
                });
                let web_worker_event_cb = Arc::new(|_| {
                  todo!("Web workers are not supported in the example");
                });
              
                let options = WorkerOptions {
                  bootstrap: BootstrapOptions {
                    args: vec![],
                    cpu_count: 1,
                    debug_flag: false,
                    enable_testing_features: false,
                    location: None,
                    no_color: false,
                    is_tty: false,
                    runtime_version: "x".to_string(),
                    ts_version: "x".to_string(),
                    unstable: false,
                    user_agent: "hello_runtime".to_string(),
                  },
                  extensions: vec![],
                  unsafely_ignore_certificate_errors: None,
                  root_cert_store: None,
                  seed: None,
                  source_map_getter: None,
                  format_js_error_fn: None,
                  web_worker_preload_module_cb: web_worker_event_cb.clone(),
                  web_worker_pre_execute_module_cb: web_worker_event_cb,
                  create_web_worker_cb,
                  maybe_inspector_server: None,
                  should_break_on_first_statement: false,
                  module_loader,
                  npm_resolver: None,
                  get_error_class_fn: Some(&get_error_class_name),
                  origin_storage_dir: None,
                  blob_store: BlobStore::default(),
                  broadcast_channel: InMemoryBroadcastChannel::default(),
                  shared_array_buffer_store: None,
                  compiled_wasm_module_store: None,
                  stdio: Default::default(),
                };
              
                let js_path =
                  Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/test.js");
                let main_module = deno_core::resolve_path(&js_path.to_string_lossy())?;
                let permissions = Permissions::allow_all();
              
                let mut worker = MainWorker::bootstrap_from_options(
                  main_module.clone(),
                  permissions,
                  options,
                );
                worker.execute_main_module(&main_module).await?;
                worker.run_event_loop(false).await?;
                Ok(())
            });
        }).join();
   


    Ok(())
}

#[defun]
pub fn addx(_: &Env, _left: i64) -> Result<i64> {
    Ok(25)
}
