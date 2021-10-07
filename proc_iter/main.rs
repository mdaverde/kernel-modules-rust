//! A kernel module
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::task::{ProcessIterator, Task};

struct ProcIterModule;

module! {
    type: ProcIterModule,
    name: b"proc_iter",
    author: b"milan@mdaverde.com",
    description: b"A kernel module",
    license: b"Dual MIT/GPL",
}

fn print_process(task: &Task) {
    pr_info!("| {:>5} | {:>5} |", task.pid(), task.tgid());
}

fn show_processes() {
    let proc_iter = ProcessIterator::new();
    pr_info!("| {:>5} | {:>5} |", "PID", "TGID");
    // logs first 25 tasks
    for (i, proc) in (0..25).zip(proc_iter) {
        print_process(&proc);
        // TODO: consider cond_resched()
    }
}

impl KernelModule for ProcIterModule {
    fn init() -> Result<Self> {
        show_processes();
        Ok(ProcIterModule)
    }
}

impl Drop for ProcIterModule {
    fn drop(&mut self) {}
}
