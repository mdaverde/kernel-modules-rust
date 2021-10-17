//! A kernel module
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::sync;
use kernel::task::{ProcessIterator, Task};

struct ProcIterModule;

module! {
    type: ProcIterModule,
    name: b"proc_iter",
    author: b"milan@mdaverde.com",
    description: b"A kernel module",
    license: b"Dual MIT/GPL",
}

fn print_thread(task: &Task) {
    let comm = task.comm().to_str().unwrap_or("<non-UTF name>");
    // Can one create a String (and therefore use format!) with the current apis?
    let name = if let Some(_) = task.mm() {
        comm
    } else {
        "[kernel]"
    };
    // lock task?
    pr_info!("| {:>5} | {:>5} | {:>16}", task.tgid(), task.pid(), name);
}

fn show_processes() {
    pr_info!("| {:>5} | {:>5} | {:>16} |", "TGID", "PID", "name",);

    let proc_iter = ProcessIterator::new();
    // let first_proc = proc_iter.next(); Can test PhantomData?
    // logs first N tasks; make N a module parameter
    for (_, proc) in (0..1000).zip(proc_iter) {
        for thread in proc.threads() {
            // if *proc != *thread {
            print_thread(&thread);
        }
        sync::cond_resched(); // Should live in mod sync?
    }
}

impl KernelModule for ProcIterModule {
    fn init() -> Result<Self> {
        show_processes();
        Ok(ProcIterModule)
    }
}
