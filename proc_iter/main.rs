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

fn print_proc(task: &Task) {
    pr_info!(
        "| {:>5} | {:>5} | {:>5} | {:>5} |",
        task.tgid(),
        task.pid(),
        task.uid(),
        task.euid()
    );
}

fn print_thread(task: &Task) {
    pr_info!("{:>9} {:>5}", "", task.pid());
}

fn show_processes() {
    pr_info!(
        "| {:>5} | {:>5} | {:>5} | {:>5} |",
        "TGID",
        "PID",
        "UID",
        "EUID"
    );

    let proc_iter = ProcessIterator::new();
    // let first_proc = proc_iter.next(); Can test PhantomData?
    // logs first N tasks; make N a module parameter
    for (_, proc) in (0..1000).zip(proc_iter) {
        print_proc(&proc);

        for thread in proc.threads() {
            if *proc != *thread {
                // Both are tasks of the same TGID
                print_thread(&thread);
            }
        }
        // TODO: consider cond_resched()
    }
}

impl KernelModule for ProcIterModule {
    fn init() -> Result<Self> {
        show_processes();
        Ok(ProcIterModule)
    }
}