//! SigmaOS-process-monitor — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_process_monitor_lib::{Component, ComponentLifecycle};

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-process-monitor: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-process-monitor: ok");
}
